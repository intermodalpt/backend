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
use sqlx::PgPool;

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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_simple_stops(
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_detailed_stops(
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_full_stops(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::FullStop>> {
    sqlx::query!(
r#"SELECT id, name, short_name, locality, street, door, lat, lon, notes, parish,
    tags, updater, update_date, verification_level,
    service_check_date, infrastructure_check_date, verified_position,
    accessibility_meta as "a11y!: sqlx::types::Json<stops::A11yMeta>", osm_id,
    CASE
        WHEN count(stop_operators.stop_id) > 0
        THEN array_agg(
            ROW(stop_operators.operator_id, stop_operators.stop_ref, stop_operators.official_name,
                stop_operators.source))
        ELSE array[]::record[]
    END as "operators!: Vec<responses::OperatorStop>"
FROM Stops
LEFT JOIN stop_operators ON stops.id = stop_operators.stop_id
WHERE id IN (
    SELECT stop_id
    FROM region_stops
    WHERE region_id = $1
)
GROUP BY stops.id
        "#,
        region_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .into_iter()
        .map(|r| {
            let sqlx::types::Json(a11y) = r.a11y;
            Ok(responses::FullStop {
                stop: stops::Stop {
                    id: r.id,
                    name: r.name,
                    short_name: r.short_name,
                    locality: r.locality,
                    street: r.street,
                    door: r.door,
                    lat: r.lat,
                    lon: r.lon,
                    notes: r.notes,
                    parish: r.parish,
                    tags: r.tags,
                    a11y,
                    verification_level: if r.verified_position {
                        r.verification_level as u8 | 0b1100_0000
                    } else {
                        r.verification_level as u8 & 0b0011_1111
                    },
                    service_check_date: r.service_check_date,
                    infrastructure_check_date: r.infrastructure_check_date,
                },
                osm_id: r.osm_id,
                updater: r.updater,
                verified_position: r.verified_position,
                update_date: r.update_date,
                operators: r.operators,
            })
        })
        .collect::<Result<Vec<responses::FullStop>>>()
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
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
        &serde_json::to_value(&stop.a11y).unwrap(),
        user_id,
        update_date,
        i16::from(stop.verification_level),
        stop.service_check_date,
        stop.infrastructure_check_date,
        stop.osm_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(stops::Stop {
        id: res.id,
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
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
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
    routes.active as active
FROM routes
JOIN subroutes ON routes.id = subroutes.route
JOIN subroute_stops ON subroutes.id = subroute_stops.subroute
WHERE subroute_stops.stop = $1"#,
        &stop_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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
