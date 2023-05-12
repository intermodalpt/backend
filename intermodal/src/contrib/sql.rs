/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022  Cláudio Pereira

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

use super::{models, responses};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_contribution(
    pool: &PgPool,
    contribution_id: i64,
) -> Result<Option<models::Contribution>> {
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
        Ok(Some(models::Contribution {
            id: contribution.id,
            author_id: contribution.author_id,
            change: serde_json::from_value(contribution.change)
                .map_err(|_e| Error::DatabaseDeserialization)?,
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
) -> Result<Vec<models::Contribution>> {
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
        Ok(models::Contribution {
            id: r.id,
            author_id: r.author_id,
            change: serde_json::from_value(r.change)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        })
    })
    .collect::<Result<Vec<models::Contribution>>>()
}

pub(crate) async fn fetch_undecided_user_contributions(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<models::Contribution>> {
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
        Ok(models::Contribution {
            id: r.id,
            author_id: r.author_id,
            change: serde_json::from_value(r.change)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        })
    })
    .collect::<Result<Vec<models::Contribution>>>()
}

pub(crate) async fn fetch_undecided_contributions(
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
    Authors.username as author_username
FROM Contributions
INNER JOIN Users AS Authors ON author_id = Authors.id
WHERE accepted IS NULL
ORDER BY submission_date ASC
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
            id: r.id,
            author_id: r.author_id,
            author_username: r.author_username,
            change: serde_json::from_value(r.change)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluator_username: None,
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        })
    })
    .collect::<Result<Vec<responses::Contribution>>>()
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
            id: r.id,
            author_id: r.author_id,
            author_username: r.author_username,
            change: serde_json::from_value(r.change)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            submission_date: r.submission_date.with_timezone(&Local),
            accepted: r.accepted,
            evaluator_id: r.evaluator_id,
            evaluator_username: Some(r.evaluator_username),
            evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
            comment: r.comment,
        })
    })
    .collect::<Result<Vec<responses::Contribution>>>()
}

pub(crate) async fn insert_new_contribution(
    pool: &PgPool,
    contribution: models::Contribution,
) -> Result<i64> {
    let res = sqlx::query!(
        r#"
INSERT INTO Contributions(author_id, change, submission_date, comment)
VALUES ($1, $2, $3, $4)
RETURNING id
    "#,
        contribution.author_id,
        serde_json::to_value(&contribution.change)
            .map_err(|_e| Error::DatabaseDeserialization)?,
        contribution.submission_date,
        contribution.comment
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_contribution(
    pool: &PgPool,
    contribution: &models::Contribution,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Contributions
SET change=$1, comment=$2
    "#,
        json!(contribution.change),
        contribution.comment
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn update_guest_contribution_to_accept<'c, E>(
    executor: E,
    contribution_id: i64,
    evaluator_id: i32,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
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
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn update_guest_contribution_to_decline<'c, E>(
    executor: E,
    contribution_id: i64,
    evaluator_id: i32,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
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
    .execute(executor)
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
            changes: serde_json::from_value(r.changes)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            datetime: r.datetime.with_timezone(&Local),
            contribution_id: r.contribution_id,
        })
    })
    .collect()
}

pub(crate) async fn insert_changeset_log<'c, E>(
    executor: E,
    author_id: i32,
    changes: &[models::Change],
    contribution_id: Option<i64>,
) -> Result<i64>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
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
    .fetch_one(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn fetch_user_stop_meta_contributions(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<models::Contribution>> {
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

        let contribution = models::Contribution {
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
            models::Contribution {
                change: models::Change::StopUpdate { .. },
                ..
            }
        ) {
            Some(Ok(contribution))
        } else {
            None
        }
    })
    .collect::<Result<Vec<models::Contribution>>>()
}
