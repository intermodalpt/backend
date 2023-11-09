/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

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

use super::responses;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_contribution(
    pool: &PgPool,
    contribution_id: i64,
) -> Result<Option<history::Contribution>> {
    let res = sqlx::query!(
        r#"
SELECT id, author_id, change, submission_date, accepted,
    evaluator_id, evaluation_date, comment
FROM Contributions
WHERE id=$1
    "#,
        contribution_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if let Some(contribution) = res {
        Ok(Some(history::Contribution {
            id: contribution.id,
            author_id: contribution.author_id,
            change: serde_json::from_value(contribution.change).map_err(
                |e| {
                    log::error!("Error deserializing: {}", e);
                    Error::DatabaseDeserialization
                },
            )?,
            submission_date: contribution.submission_date.with_timezone(&Local),
            accepted: contribution.accepted,
            evaluator_id: contribution.evaluator_id,
            evaluation_date: contribution
                .evaluation_date
                .map(|d| d.with_timezone(&Local)),
            comment: contribution.comment,
        }))
    } else {
        Ok(None)
    }
}

pub(crate) async fn fetch_decided_user_contributions(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<history::Contribution>> {
    sqlx::query!(
        r#"
SELECT id, author_id, change, submission_date, accepted,
    evaluator_id, evaluation_date, comment
FROM Contributions
WHERE accepted IS NOT NULL AND author_id=$1
ORDER BY submission_date DESC
LIMIT $2 OFFSET $3
    "#,
        user_id,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| {
        Ok(history::Contribution {
            id: r.id,
            author_id: r.author_id,
            change: serde_json::from_value(r.change).map_err(|e| {
                log::error!("Error deserializing: {}", e);
                Error::DatabaseDeserialization
            })?,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        })
    })
    .collect::<Result<Vec<history::Contribution>>>()
}

pub(crate) async fn fetch_undecided_user_contributions(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<history::Contribution>> {
    sqlx::query!(
        r#"
SELECT id, author_id, change, submission_date, accepted,
    evaluator_id, evaluation_date, comment
FROM Contributions
WHERE accepted is NULL AND author_id=$1
ORDER BY submission_date DESC
LIMIT $2 OFFSET $3
    "#,
        user_id,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| {
        Ok(history::Contribution {
            id: r.id,
            author_id: r.author_id,
            change: serde_json::from_value(r.change).map_err(|e| {
                log::error!("Error deserializing: {}", e);
                Error::DatabaseDeserialization
            })?,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        })
    })
    .collect::<Result<Vec<history::Contribution>>>()
}

pub(crate) async fn fetch_undecided_contributions(
    pool: &PgPool,
    filter_uid: Option<i32>,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::Contribution>> {
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
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| {
        Ok(responses::Contribution {
            contribution: history::Contribution {
                id: r.id,
                author_id: r.author_id,
                change: serde_json::from_value(r.change).map_err(|e| {
                    log::error!("Error deserializing: {}", e);
                    Error::DatabaseDeserialization
                })?,
                submission_date: r.submission_date.with_timezone(&Local),
                accepted: r.accepted,
                evaluator_id: r.evaluator_id,
                evaluation_date: r
                    .evaluation_date
                    .map(|d| d.with_timezone(&Local)),
                comment: r.comment,
            },
            author_username: r.author_username,
            evaluator_username: None,
        })
    })
    .collect::<Result<Vec<responses::Contribution>>>()
}

pub(crate) async fn count_undecided_contributions(
    pool: &PgPool,
    filter_uid: Option<i32>,
) -> Result<i64> {
    // -1 means no filter
    let filter_uid = filter_uid.unwrap_or(-1);

    Ok(sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Contributions
WHERE Contributions.accepted IS NULL
    AND ($1 = -1 OR Contributions.author_id = $1)
    "#,
        filter_uid
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0))
}

pub(crate) async fn fetch_undecided_contribution_contributors(
    pool: &PgPool,
) -> Result<Vec<responses::Contributor>> {
    sqlx::query_as!(
        responses::Contributor,
        r#"
SELECT Users.id, Users.username, Users.works_for
FROM Users
WHERE Users.id IN (
    SELECT DISTINCT author_id
    FROM Contributions
    WHERE accepted IS NULL
)
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_decided_contributions(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::Contribution>> {
    sqlx::query!(
        r#"
SELECT Contributions.id, Contributions.author_id, Contributions.change,
    Contributions.submission_date, Contributions.accepted,
    Contributions.evaluator_id, Contributions.evaluation_date,
    Contributions.comment,
    Authors.username as author_username,
    Evaluators.username as evaluator_username
FROM Contributions
INNER JOIN Users AS Authors ON author_id = Authors.id
LEFT JOIN Users AS Evaluators ON evaluator_id = Evaluators.id
WHERE accepted IS NOT NULL
ORDER BY evaluation_date DESC
LIMIT $1 OFFSET $2
    "#,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| {
        Ok(responses::Contribution {
            contribution: history::Contribution {
                id: r.id,
                author_id: r.author_id,
                change: serde_json::from_value(r.change).map_err(|e| {
                    log::error!("Error deserializing: {}", e);
                    Error::DatabaseDeserialization
                })?,
                submission_date: r.submission_date.with_timezone(&Local),
                accepted: r.accepted,
                evaluator_id: r.evaluator_id,
                evaluation_date: r
                    .evaluation_date
                    .map(|d| d.with_timezone(&Local)),
                comment: r.comment,
            },
            author_username: r.author_username,
            evaluator_username: Some(r.evaluator_username),
        })
    })
    .collect::<Result<Vec<responses::Contribution>>>()
}

pub(crate) async fn count_decided_contributions(pool: &PgPool) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Contributions
WHERE accepted IS NOT NULL
    "#
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0))
}

pub(crate) async fn insert_new_contribution(
    pool: &PgPool,
    contribution: history::Contribution,
) -> Result<i64> {
    let res = sqlx::query!(
        r#"
INSERT INTO Contributions(author_id, change, submission_date, comment)
VALUES ($1, $2, $3, $4)
RETURNING id
    "#,
        contribution.author_id,
        serde_json::to_value(&contribution.change).map_err(|e| {
            log::error!("Error deserializing: {}", e);
            Error::DatabaseDeserialization
        })?,
        contribution.submission_date,
        contribution.comment
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

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
    let comment = comment.as_ref().map(String::as_str);

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
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn update_guest_contribution_to_accept(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    contribution_id: i64,
    evaluator_id: i32,
) -> Result<()> {
    let date = Local::now();

    sqlx::query!(
        r#"
UPDATE Contributions
SET accepted=true, evaluator_id=$1, evaluation_date=$2
WHERE id=$3
    "#,
        evaluator_id,
        date,
        contribution_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn update_guest_contribution_to_decline(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    contribution_id: i64,
    evaluator_id: i32,
) -> Result<()> {
    let date = Local::now();

    sqlx::query!(
        r#"
UPDATE Contributions
SET accepted=false, evaluator_id=$1, evaluation_date=$2
WHERE id=$3
    "#,
        evaluator_id,
        date,
        contribution_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_changeset_logs<'c, E>(
    executor: E,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::Changeset>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query!(
        r#"
SELECT Changelog.id, Changelog.author_id, Changelog.changes, Changelog.datetime,
    Changelog.contribution_id, Users.username as author_username
FROM Changelog
INNER JOIN Users ON author_id = Users.id
ORDER BY datetime DESC
LIMIT $1 OFFSET $2
    "#,
        take,
        skip
    )
    .fetch_all(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| {
        Ok(responses::Changeset {
            id: r.id,
            author_id: r.author_id,
            author_username: r.author_username,
            changes: serde_json::from_value(r.changes).map_err(|e| {
                log::error!("Error deserializing: {}", e);
                Error::DatabaseDeserialization
            })?,
            datetime: r.datetime.with_timezone(&Local),
            contribution_id: r.contribution_id,
        })
    })
    .collect()
}

pub(crate) async fn count_changeset_logs(pool: &PgPool) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Changelog
INNER JOIN Users ON author_id = Users.id
    "#
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0))
}

pub(crate) async fn insert_changeset_log(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    author_id: i32,
    changes: &[history::Change],
    contribution_id: Option<i64>,
) -> Result<i64> {
    let res = sqlx::query!(
        r#"
INSERT INTO Changelog(author_id, changes, contribution_id)
VALUES ($1, $2, $3)
RETURNING id
    "#,
        author_id,
        serde_json::to_value(changes).unwrap(),
        contribution_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn fetch_user_stop_meta_contributions(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<history::Contribution>> {
    sqlx::query!(
        r#"
SELECT id, author_id, change, submission_date, accepted,
    evaluator_id, evaluation_date, comment
FROM Contributions
WHERE accepted is NULL AND author_id=$1
ORDER BY submission_date ASC
    "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .filter_map(|r| {
        let change = match serde_json::from_value(r.change) {
            Ok(change) => change,
            Err(_e) => return Some(Err(Error::DatabaseDeserialization)),
        };

        let contribution = history::Contribution {
            id: r.id,
            author_id: r.author_id,
            change,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        };
        if matches!(
            contribution,
            history::Contribution {
                change: history::Change::StopUpdate { .. },
                ..
            }
        ) {
            Some(Ok(contribution))
        } else {
            None
        }
    })
    .collect::<Result<Vec<history::Contribution>>>()
}
