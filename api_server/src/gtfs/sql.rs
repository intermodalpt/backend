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

use serde_json::json;
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
        .map_err(|err| {
            tracing::error!(error=err.to_string(), operator_id);
            Error::DatabaseExecution
        })?
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
        Json(&data) as _,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), route_id, data = ?data);
        Error::DatabaseExecution
    })?;

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
            ROW(
                subroutes.id,
                validation_current,
                validation_current_ack,
                validation_correspondence,
                validation_correspondence_ack,
                NULLIF(subroutes.validation_gtfs, '{}'::jsonb)))
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
        .map_err(|err| {
            tracing::error!(error=err.to_string(), route_id);
            Error::DatabaseExecution
        })?
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
        Json(&data) as _,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), route_id, data = ?data);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_subroute_validation_data<'c, E>(
    executor: E,
    subroute_id: i32,
) -> Result<Option<models::SubrouteValidationData>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        models::SubrouteValidationData,
        r#"
SELECT -- validation_current as current,
    validation_current_ack as current_ack,
    -- validation_correspondence as correspondence,
    validation_correspondence_ack as correspondence_ack
    -- validation_gtfs as "gtfs!: Json<gtfs::PatternCluster>"
FROM subroutes
WHERE id=$1"#,
        subroute_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), subroute_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn update_subroute_validation_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    current_stops: &[i32],
    correspondence_stops: &[i32],
    validation_gtfs: &gtfs::PatternCluster,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Subroutes
SET validation_current=$1, validation_correspondence=$2, validation_gtfs=$3
WHERE id=$4
    "#,
        current_stops,
        correspondence_stops,
        json!(validation_gtfs),
        subroute_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            subroute_id,
            current_stops = ?current_stops,
            correspondence_stops = ?correspondence_stops,
            validation_gtfs = ?validation_gtfs
        );
        Error::DatabaseExecution
    })?;

    Ok(())
}

/// Sets the `validation_current_ack` field to what will hopefully become the
/// `validation_current` field.
/// We update explicitly (`current_ack`=$1) instead of implicitly
/// (`current_ack`=`current`) to prevent data races
pub(crate) async fn update_subroute_validation_current_ack(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    current_stop_ids: &[i32],
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE subroutes
SET validation_current_ack=$1
WHERE id=$2
    "#,
        current_stop_ids,
        subroute_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(), subroute_id, current_stop_ids = ?current_stop_ids);
        Error::DatabaseExecution
    })?;

    Ok(())
}

/// Sets the `validation_correspondence_ack` field to what will hopefully become the
/// `validation_correspondence` field.
/// We update explicitly (`correspondence_ack`=$1) instead of implicitly
/// (`correspondence_ack`=`correspondence`) to prevent data races
pub(crate) async fn update_subroute_correspondence_ack(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    correspondence_stop_ids: &[i32],
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE subroutes
SET validation_correspondence_ack=$1
WHERE id=$2
    "#,
        correspondence_stop_ids,
        subroute_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            subroute_id,
            correspondence_stop_ids = ?correspondence_stop_ids
        );
        Error::DatabaseExecution
    })?;

    Ok(())
}
