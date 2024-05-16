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

use sqlx::PgPool;
use std::collections::HashMap;

use commons::models::geo;

use super::models::responses;

use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_regions(pool: &PgPool) -> Result<Vec<geo::Region>> {
    sqlx::query_as!(
        geo::Region,
        r#"
SELECT id, name, geometry, center_lat, center_lon, zoom
FROM regions
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_region(
    pool: &PgPool,
    region_id: i32,
) -> Result<Option<geo::Region>> {
    sqlx::query_as!(
        geo::Region,
        r#"
SELECT id, name, geometry, center_lat, center_lon, zoom
FROM regions
WHERE id = $1
    "#,
        region_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_operator_regions(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<i32>> {
    Ok(sqlx::query!(
        r#"
SELECT region_id
FROM region_operators
WHERE operator_id = $1
"#,
        operator_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| row.region_id)
    .collect())
}

pub(crate) async fn upsert_operator_into_region(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    region_id: i32,
    operator_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO region_operators (region_id, operator_id)
VALUES ($1, $2)
ON CONFLICT DO NOTHING;
    "#,
        region_id,
        operator_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id, operator_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_operator_from_region(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    region_id: i32,
    operator_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM region_operators
WHERE region_id=$1 AND operator_id=$2;
    "#,
        region_id,
        operator_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id, operator_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_route_regions(
    pool: &PgPool,
    route_id: i32,
) -> Result<Vec<i32>> {
    Ok(sqlx::query!(
        r#"
SELECT region_id
FROM region_routes
WHERE route_id = $1
"#,
        route_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), route_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| row.region_id)
    .collect())
}

pub(crate) async fn upsert_route_into_region(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    region_id: i32,
    route_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO region_routes (region_id, route_id)
VALUES ($1, $2)
ON CONFLICT DO NOTHING;
    "#,
        region_id,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id, route_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_route_from_region(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    region_id: i32,
    route_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM region_routes
WHERE region_id=$1 AND route_id=$2;
    "#,
        region_id,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id, route_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_stop_regions(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<i32>> {
    Ok(sqlx::query!(
        r#"
SELECT region_id
FROM region_stops
WHERE stop_id = $1
"#,
        stop_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), stop_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| row.region_id)
    .collect())
}

pub(crate) async fn upsert_stop_into_region(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    region_id: i32,
    stop_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO region_stops (region_id, stop_id)
VALUES ($1, $2)
ON CONFLICT DO NOTHING;
    "#,
        region_id,
        stop_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), stop_id, region_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_stop_from_region(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    region_id: i32,
    stop_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM region_stops
WHERE region_id=$1 AND stop_id=$2;
    "#,
        region_id,
        stop_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), stop_id, region_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_parishes(pool: &PgPool) -> Result<Vec<geo::Parish>> {
    sqlx::query_as!(
        geo::Parish,
        r#"
SELECT parishes.id, parishes.name, parishes.short_name, municipalities.name as municipality,
    municipalities.zone, parishes.polygon, parishes.geojson
FROM parishes
JOIN municipalities ON parishes.municipality = municipalities.id
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string());
        Error::DatabaseExecution
    })
}

pub(crate) async fn update_stop_parish(
    pool: &PgPool,
    stop_id: i32,
    parish: i32,
) -> Result<()> {
    sqlx::query!(
        "UPDATE stops SET parish = $1 WHERE id = $2",
        parish,
        stop_id
    )
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), parish, stop_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_region_osm_quality(
    pool: &PgPool,
    region_id: i32,
) -> Result<HashMap<i32, bool>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_id, osm_map_quality
FROM Stops
JOIN region_stops ON region_stops.stop_id = Stops.id
WHERE region_stops.region_id = $1
    "#,
        region_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| (row.stop_id, row.osm_map_quality))
    .collect())
}
