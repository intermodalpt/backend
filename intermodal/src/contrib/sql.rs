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
use sqlx::PgPool;

use super::models;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_contribution(
    pool: &PgPool,
    contribution_id: i64,
) -> Result<Option<models::Contribution>> {
    let res = sqlx::query!(
        r#"
SELECT id, author_id, changeset, submission_date, accepted,
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
            changeset: serde_json::from_value(contribution.changeset).unwrap(),
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

pub(crate) async fn fetch_guest_contributions(
    pool: &PgPool,
    contribution: models::Contribution,
) -> Result<Vec<models::Contribution>> {
    let res = sqlx::query!(
        r#"
SELECT id, author_id, changeset, submission_date, accepted,
    evaluator_id, evaluation_date, comment
FROM Contributions
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| models::Contribution {
        id: r.id,
        author_id: r.author_id,
        changeset: serde_json::from_value(r.changeset).unwrap(),
        submission_date: r.submission_date.with_timezone(&Local),
        accepted: r.accepted,
        evaluator_id: r.evaluator_id,
        evaluation_date: r.evaluation_date.map(|d| d.with_timezone(&Local)),
        comment: r.comment,
    })
    .collect::<Vec<models::Contribution>>();
    Ok(res)
}

pub(crate) async fn insert_new_contribution(
    pool: &PgPool,
    contribution: models::Contribution,
) -> Result<i64> {
    let update_date = Local::now();

    let res = sqlx::query!(
        r#"
INSERT INTO Contributions(author_id, changeset, submission_date, comment)
VALUES ($1, $2, $3, $4)
RETURNING id
    "#,
        contribution.author_id,
        serde_json::to_value(&contribution.changeset).unwrap(),
        contribution.submission_date,
        contribution.comment
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
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

    let res = sqlx::query!(
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

    let res = sqlx::query!(
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
