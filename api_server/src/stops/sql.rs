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

use std::collections::{hash_map, HashMap};

use chrono::Utc;
use sqlx::types::Json;
use sqlx::{FromRow, PgPool, Row};

use commons::models::{routes, stops};

use super::models;
use crate::stops::models::{requests, responses};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_stop(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Option<responses::Stop>> {
    sqlx::query_as!(
        responses::Stop,
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, verification_level, service_check_date, infrastructure_check_date,
    accessibility_meta as "a11y!: sqlx::types::Json<stops::A11yMeta>", osm_id
FROM Stops
WHERE id = $1"#,
        stop_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), stop_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_paired_stop(
    pool: &PgPool,
    osm_id: i64,
) -> Result<Option<responses::SimpleStop>> {
    sqlx::query_as!(
        responses::SimpleStop,
        r#"SELECT id, name, short_name, lat, lon
FROM Stops
WHERE osm_id = $1"#,
        osm_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), osm_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_stops_by_operator_ref(
    pool: &PgPool,
    operator_id: i32,
    stop_ref: &str,
) -> Result<Vec<responses::SimpleStop>> {
    sqlx::query_as!(
        responses::SimpleStop,
        r#"SELECT id, name, short_name, lat, lon
FROM stops
JOIN stop_operators ON stops.id = stop_operators.stop_id
WHERE stop_operators.operator_id= $1 AND stop_operators.stop_ref = $2"#,
        operator_id,
        stop_ref
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id, stop_ref);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_region_simple_stops(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::SimpleStop>> {
    sqlx::query_as!(
        responses::SimpleStop,
        r#"
SELECT id, name, short_name, lat, lon
FROM stops
WHERE id IN (
    SELECT DISTINCT stop_id
    FROM region_stops
    WHERE region_id = $1
)
        "#,
        region_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_region_detailed_stops(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::Stop>> {
    sqlx::query_as!(
        responses::Stop,
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, verification_level, service_check_date, infrastructure_check_date,
    accessibility_meta as "a11y!: sqlx::types::Json<stops::A11yMeta>", osm_id
FROM stops
WHERE id IN (
    SELECT DISTINCT stop_id
    FROM region_stops
    WHERE region_id = $1
)"#,
        region_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), region_id);
        Error::DatabaseExecution
    })
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(crate) async fn fetch_region_full_stops(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::FullStop>> {
    sqlx::query(
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, updater, update_date, verification_level,
    service_check_date, infrastructure_check_date, verified_position,
    accessibility_meta, osm_id,
    CASE
        WHEN count(stop_operators.stop_id) > 0
        THEN array_agg(
            ROW(
                stop_operators.operator_id, stop_operators.stop_ref,
                stop_operators.official_name, stop_operators.source))
        ELSE array[]::record[]
    END as "operators"
FROM Stops
LEFT JOIN stop_operators ON stops.id = stop_operators.stop_id
WHERE id IN (
    SELECT stop_id
    FROM region_stops
    WHERE region_id = $1
)
GROUP BY stops.id
        "#
    ).bind(region_id)
        .fetch_all(pool)
        .await
        .and_then(|res| {
            res.iter()
                .map(responses::FullStop::from_row)
                .collect::<std::result::Result<_,_>>()
        })
        .map_err(|err| {
            tracing::error!(error=err.to_string(), region_id);
            Error::DatabaseExecution
        })
}

pub(crate) async fn fetch_all_stops(
    pool: &PgPool,
) -> Result<Vec<responses::SimpleStop>> {
    sqlx::query_as!(
        responses::SimpleStop,
        "SELECT id, name, short_name, lat, lon FROM stops"
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_all_detailed_stops(
    pool: &PgPool,
) -> Result<Vec<responses::Stop>> {
    sqlx::query_as!(
            responses::Stop,
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, verification_level, service_check_date, infrastructure_check_date,
    accessibility_meta as "a11y!: sqlx::types::Json<stops::A11yMeta>", osm_id
FROM stops"#
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string());
            Error::DatabaseExecution
        })
}

pub(crate) async fn fetch_bounded_stops(
    pool: &PgPool,
    (x0, y0, x1, y1): (f64, f64, f64, f64),
) -> Result<Vec<responses::Stop>> {
    sqlx::query_as!(
        responses::Stop,
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, verification_level, service_check_date, infrastructure_check_date,
    accessibility_meta as "a11y!: sqlx::types::Json<stops::A11yMeta>", osm_id
FROM Stops
WHERE lon >= $1 AND lon <= $2 AND lat <= $3 AND lat >= $4
        "#,
        x0,
        x1,
        y0,
        y1
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string(), x0, x1, y0, y1);
            Error::DatabaseExecution
        })
}

pub(crate) async fn fetch_route_stops(
    pool: &PgPool,
    route_id: i32,
) -> Result<Vec<responses::Stop>> {
    sqlx::query_as!(
        responses::Stop,
r#"SELECT stops.id, name, short_name, locality, street, door, lat, lon, notes,
    parish, tags, verification_level, osm_id,
    service_check_date, infrastructure_check_date,
    accessibility_meta as "a11y!: sqlx::types::Json<stops::A11yMeta>"
FROM stops
JOIN subroute_stops ON stops.id = subroute_stops.stop
JOIN subroutes ON subroute_stops.subroute = subroutes.id
WHERE subroutes.route = $1"#,
        route_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string(), route_id);
            Error::DatabaseExecution
        })
}

pub(crate) async fn fetch_operator_stops(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::Stop>> {
    sqlx::query_as!(
        responses::Stop,
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, verification_level, service_check_date, infrastructure_check_date,
    accessibility_meta as "a11y!: sqlx::types::Json<stops::A11yMeta>", osm_id
FROM stops
JOIN stop_operators ON stops.id = stop_operators.stop_id
WHERE stop_operators.operator_id = $1"#,
        operator_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string(), operator_id);
            Error::DatabaseExecution
        })
}

pub(crate) async fn fetch_operator_full_stops(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::FullStop>> {
    sqlx::query(
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, updater, update_date, verification_level,
    service_check_date, infrastructure_check_date, verified_position,
    accessibility_meta, osm_id,
    CASE
        WHEN count(stop_operators.stop_id) > 0
        THEN array_agg(
            ROW(
                stop_operators.operator_id, stop_operators.stop_ref,
                stop_operators.official_name, stop_operators.source))
        ELSE array[]::record[]
    END as "operators"
FROM Stops
LEFT JOIN stop_operators ON stops.id = stop_operators.stop_id
WHERE id IN (
    SELECT stop_id
    FROM stop_operators
    WHERE operator_id = $1
)
GROUP BY stops.id
"#
    ).bind(operator_id)
        .fetch_all(pool)
        .await
        .and_then(|res| {
            res.iter()
                .map(responses::FullStop::from_row)
                .collect::<std::result::Result<_,_>>()
        })
        .map_err(|err| {
            tracing::error!(error=err.to_string(), operator_id);
            Error::DatabaseExecution
        })
}

pub(crate) async fn insert_stop(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stop: requests::NewStop,
    user_id: i32,
) -> Result<stops::Stop> {
    let update_date = Utc::now();

    let res = sqlx::query!(
        r#"
INSERT INTO Stops(name, short_name, locality, street, door, lon, lat, notes,
    tags, accessibility_meta, updater, update_date, verification_level,
    service_check_date, infrastructure_check_date, osm_id)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
RETURNING id
    "#,
        stop.name,
        stop.short_name,
        stop.locality,
        stop.street,
        stop.door,
        stop.lon,
        stop.lat,
        stop.notes,
        &stop.tags,
        Json(&stop.a11y) as _,
        user_id,
        update_date,
        i16::from(stop.verification_level),
        stop.service_check_date,
        stop.infrastructure_check_date,
        stop.osm_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            stop = ?stop,
            user_id,
            update_date=?update_date);
        Error::DatabaseExecution
    })?;

    Ok(stops::Stop {
        id: res.id,
        osm_id: stop.osm_id,
        name: stop.name,
        short_name: stop.short_name,
        locality: stop.locality,
        street: stop.street,
        door: stop.door,
        lat: stop.lat,
        lon: stop.lon,
        notes: stop.notes,
        parish: None,
        tags: stop.tags,
        a11y: stop.a11y,
        verification_level: stop.verification_level,
        service_check_date: stop.service_check_date,
        infrastructure_check_date: stop.infrastructure_check_date,
    })
}

pub(crate) async fn update_stop(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stop_id: i32,
    changes: requests::ChangeStop,
    user_id: i32,
) -> Result<()> {
    let update_date = Utc::now();

    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET name=$1, short_name=$2, locality=$3, street=$4, door=$5, lon=$6, lat=$7, notes=$8,
    accessibility_meta=$9, updater=$10, update_date=$11, tags=$12, verification_level=$13,
    service_check_date=$14, infrastructure_check_date=$15
WHERE id=$16
    "#,
        changes.name,
        changes.short_name,
        changes.locality,
        changes.street,
        changes.door,
        changes.lon,
        changes.lat,
        changes.notes,
        serde_json::to_value(&changes.a11y).unwrap(),
        user_id,
        update_date,
        &changes.tags,
        i16::from(changes.verification_level),
        changes.service_check_date,
        changes.infrastructure_check_date,
        stop_id
    )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string(), stop_id);
            Error::DatabaseExecution
        })?;

    Ok(())
}

pub(crate) async fn update_stop_position(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stop_id: i32,
    lon: f64,
    lat: f64,
) -> Result<bool> {
    let res = sqlx::query!(
        "UPDATE stops SET lon=$1, lat=$2 WHERE id = $3",
        lon,
        lat,
        stop_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), stop_id);
        Error::DatabaseExecution
    })?;

    Ok(res.rows_affected() != 0)
}

pub(crate) async fn fetch_stop_routes(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<routes::Route>> {
    sqlx::query_as!(
        routes::Route,
        r#"
SELECT routes.id as id,
    routes.type as type_id,
    routes.operator as operator_id,
    routes.code as code,
    routes.name as name,
    routes.circular as circular,
    routes.main_subroute as main_subroute,
    routes.active as active,
    COALESCE(routes.badge_text_color, route_types.badge_text_color) as badge_text_color,
    COALESCE(routes.badge_bg_color, route_types.badge_bg_color) as badge_bg_color
FROM routes
JOIN subroutes ON routes.id = subroutes.route
JOIN subroute_stops ON subroutes.id = subroute_stops.subroute
JOIN route_types ON routes.type = route_types.id
WHERE subroute_stops.stop = $1"#,
        &stop_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), stop_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_stop_spider(
    pool: &PgPool,
    stops: &[i32],
) -> Result<responses::SpiderMap> {
    let res = sqlx::query!(
        r#"
SELECT Routes.id as route_id,
    routes.code as "route_code!: Option<String>",
    routes.name as route_name,
    routes.circular as route_circular,
    subroutes.id as subroute_id,
    subroutes.flag as subroute_flag,
    subroute_stops.stop as stop_id,
    stops.name as stop_name,
    stops.lon as lon,
    stops.lat as lat
FROM routes
JOIN subroutes ON routes.id = subroutes.route
JOIN subroute_stops ON subroutes.id = subroute_stops.subroute
JOIN stops ON stops.id = subroute_stops.stop
WHERE subroutes.id IN (
    SELECT subroutes.id
    FROM subroutes
    JOIN subroute_stops ON subroutes.id = subroute_stops.subroute
    WHERE subroute_stops.stop = ANY($1)
)
ORDER BY subroute_stops.idx"#,
        &stops[..]
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), stops = ?stops);
        Error::DatabaseExecution
    })?;

    let mut routes: HashMap<i32, responses::SpiderRoute> = HashMap::new();
    let mut subroutes: HashMap<i32, responses::SpiderSubroute> = HashMap::new();
    let mut stops: HashMap<i32, responses::SpiderStop> = HashMap::new();

    for row in res {
        if let hash_map::Entry::Vacant(e) = routes.entry(row.route_id) {
            e.insert(responses::SpiderRoute {
                code: row.route_code,
                name: row.route_name,
                circular: row.route_circular,
            });
        }

        if let Some(subroute) = subroutes.get_mut(&row.subroute_id) {
            subroute.stop_sequence.push(row.stop_id);
        } else {
            subroutes.insert(
                row.subroute_id,
                responses::SpiderSubroute {
                    route: row.route_id,
                    flag: row.subroute_flag,
                    stop_sequence: vec![],
                },
            );
        }

        if let hash_map::Entry::Vacant(e) = stops.entry(row.stop_id) {
            e.insert(models::responses::SpiderStop {
                name: row.stop_name,
                lat: row.lat,
                lon: row.lon,
            });
        }
    }

    Ok(responses::SpiderMap {
        routes,
        subroutes,
        stops,
    })
}

impl FromRow<'_, sqlx::postgres::PgRow> for responses::Stop {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        let verified_position: bool = row.try_get("verified_position")?;
        let verification_level: i16 = row.try_get("verification_level")?;

        Ok(Self {
            id: row.try_get("id")?,
            osm_id: row.try_get("osm_id")?,
            name: row.try_get("name")?,
            short_name: row.try_get("short_name")?,
            locality: row.try_get("locality")?,
            street: row.try_get("street")?,
            door: row.try_get("door")?,
            lat: row.try_get("lat")?,
            lon: row.try_get("lon")?,
            notes: row.try_get("notes")?,
            parish: row.try_get("parish")?,
            tags: row.try_get("tags")?,
            a11y: row.try_get("accessibility_meta")?,
            verification_level: if verified_position {
                verification_level | 0b1100_0000
            } else {
                verification_level & 0b0011_1111
            },
            service_check_date: row.try_get("service_check_date")?,
            infrastructure_check_date: row
                .try_get("infrastructure_check_date")?,
        })
    }
}

impl FromRow<'_, sqlx::postgres::PgRow> for responses::FullStop {
    fn from_row(row: &sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        let operators = row
            .try_get::<Vec<(i32, Option<String>, Option<String>, String)>, _>(
                "operators",
            )?
            .into_iter()
            .map(|(operator_id, stop_ref, name, source)| {
                responses::OperatorStopRel {
                    operator_id,
                    stop_ref,
                    name,
                    source,
                }
            })
            .collect();

        Ok(Self {
            stop: responses::Stop::from_row(row)?,
            updater: row.try_get("updater")?,
            verified_position: row.try_get("verified_position")?,
            update_date: row.try_get("update_date")?,
            operators,
        })
    }
}
