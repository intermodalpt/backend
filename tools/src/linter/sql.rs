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
use serde_json::json;
use sqlx::PgPool;

use commons::models::history;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) async fn update_contribution<'c, E>(
    executor: E,
    id: i64,
    change: &history::Change,
    comment: &Option<String>,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    // FIXME the hell?
    let comment = comment.as_ref().map(|s| s.as_str());

    sqlx::query!(
        r#"
UPDATE Contributions
SET change=$1, comment=$2
WHERE id=$3
    "#,
        json!(change),
        comment,
        id
    )
    .execute(executor)
    .await?;
    Ok(())
}

pub(crate) async fn fetch_undecided_contributions(
    pool: &PgPool,
    filter_uid: Option<i32>,
    skip: i64,
    take: i64,
) -> Result<Vec<history::Contribution>> {
    // -1 means no filter
    let filter_uid = filter_uid.unwrap_or(-1);

    sqlx::query!(
        r#"
SELECT Contributions.id, Contributions.author_id, Contributions.change,
    Contributions.submission_date, Contributions.accepted,
    Contributions.evaluator_id, Contributions.evaluation_date,
    Contributions.comment,
    Authors.username as author_username
FROM Contributions
INNER JOIN Users AS Authors ON Contributions.author_id = Authors.id
WHERE Contributions.accepted IS NULL
    AND ($1 = -1 OR Contributions.author_id = $1)
ORDER BY submission_date ASC
LIMIT $2 OFFSET $3
    "#,
        filter_uid,
        take,
        skip
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| {
        Ok(history::Contribution {
            id: r.id,
            author_id: r.author_id,
            change: serde_json::from_value(r.change)?,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        })
    })
    .collect::<Result<Vec<history::Contribution>>>()
}
