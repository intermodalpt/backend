/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2024  Cláudio Pereira

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

use sqlx::types::Json;
use sqlx::PgPool;

use commons::models::gtfs;

use super::models::{self, responses};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_operator_validation_data(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Option<Option<gtfs::OperatorValidation>>> {
    Ok(sqlx::query!(
        r#"
SELECT operators.validation as "validation!: Option<Json<gtfs::OperatorValidation>>"
FROM operators
WHERE operators.id=$1
    "#,
        operator_id
    )
        .fetch_optional(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .map(|r| r.validation.map(|data| data.0)))
}

pub(crate) async fn update_operator_validation_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    data: gtfs::OperatorValidation,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Operators
SET validation=$1
WHERE id=$2
    "#,
        Json(data) as _,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_route_validation_data<'c, E>(
    executor: E,
    route_id: i32,
) -> Result<Option<responses::RouteValidation>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    Ok(sqlx::query!(
        r#"
SELECT routes.id, routes.validation as "validation!: Option<sqlx::types::Json<gtfs::RouteValidation>>",
    CASE
        WHEN count(subroutes.id) > 0
        THEN array_agg(
            ROW(subroutes.id, NULLIF(subroutes.validation, '{}'::jsonb)))
        ELSE array[]::record[]
    END as "subroutes!: Vec<models::SubrouteValidationPair>"
FROM routes
LEFT JOIN subroutes ON subroutes.route = routes.id
WHERE routes.id=$1
GROUP BY routes.id
    "#,
        route_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .map(|row| responses::RouteValidation {
        validation: row.validation,
        subroutes: row
            .subroutes
            .into_iter()
            .map(|pair| (pair.id, pair.validation))
            .collect(),
    }))
}

pub(crate) async fn update_route_validation_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    data: gtfs::RouteValidation,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Routes
SET validation=$1
WHERE id=$2
    "#,
        Json(data) as _,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_subroute_validation_data<'c, E>(
    executor: E,
    subroute_id: i32,
) -> Result<Option<gtfs::SubrouteValidation>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let res = sqlx::query!(
        r#"
SELECT subroutes.validation as "validation!: Json<gtfs::SubrouteValidation>"
FROM subroutes
WHERE subroutes.id=$1
    "#,
        subroute_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.map(|row| row.validation.0))
}

pub(crate) async fn update_subroute_validation_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    data: gtfs::SubrouteValidation,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Subroutes
SET validation=$1
WHERE id=$2
    "#,
        Json(data) as _,
        subroute_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}
