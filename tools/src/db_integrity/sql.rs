/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023  Cláudio Pereira

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use chrono::Local;

use commons::models::history;

pub(crate) enum JsonParseResult<OkData, ErrData> {
    Ok(OkData),
    Err {
        raw: String,
        error: serde_json::Error,
        data: ErrData,
    },
}

pub(crate) async fn fetch_faulty_changeset_logs<'c, E>(
    executor: E,
) -> Vec<JsonParseResult<history::Changeset, history::Changeset>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let mut successes = 0;
    let errors = sqlx::query!(
        r#"
SELECT Changelog.id, Changelog.author_id, Changelog.changes, Changelog.datetime,
    Changelog.contribution_id, Users.username as author_username
FROM Changelog
INNER JOIN Users ON author_id = Users.id
ORDER BY datetime DESC
    "#
    )
    .fetch_all(executor)
    .await
    .unwrap_or_else(|err| panic!("{}", err.to_string()))
    .into_iter()
    .map(|r| {
        let raw_changes = r.changes.to_string();
        let changes: Result<Vec<history::Change>, _> =
            serde_json::from_value(r.changes);
        match changes {
            Ok(changes) => {
                successes += 1;
                JsonParseResult::Ok(history::Changeset {
                    id: r.id,
                    author_id: r.author_id,
                    changes,
                    datetime: r.datetime.with_timezone(&Local),
                    contribution_id: r.contribution_id,
                })
            }
            Err(err) => JsonParseResult::Err {
                raw: raw_changes,
                error: err,
                data: history::Changeset {
                    id: r.id,
                    author_id: r.author_id,
                    changes: vec![],
                    datetime: r.datetime.with_timezone(&Local),
                    contribution_id: r.contribution_id,
                },
            },
        }
    })
    .collect::<Vec<_>>();

    println!("Successes: {}", successes);
    errors
}

pub(crate) async fn update_changeset<'c, E>(
    executor: E,
    contribution_id: i64,
    changes: &[history::Change],
) where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let res = sqlx::query!(
        r#"
UPDATE Changelog
SET changes=$1
WHERE id=$2
    "#,
        serde_json::to_value(changes).unwrap(),
        contribution_id
    )
    .execute(executor)
    .await
    .map_err(|err| dbg!(err));
}
