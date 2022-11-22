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
use std::collections::{hash_map, HashMap};

use super::models;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_stop(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Option<models::Stop>> {
    let res = sqlx::query!(
        r#"
SELECT id, source, name, official_name, osm_name, short_name, locality, street,
    door, lat, lon, external_id, succeeded_by, notes, updater, update_date,
    parish, tags, accessibility_meta
FROM Stops
WHERE id = $1
    "#,
        stop_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .map(|r| models::Stop {
        id: r.id,
        source: r.source,
        name: r.name,
        official_name: r.official_name,
        osm_name: r.osm_name,
        short_name: r.short_name,
        locality: r.locality,
        street: r.street,
        door: r.door,
        lat: r.lat,
        lon: r.lon,
        external_id: r.external_id,
        succeeded_by: r.succeeded_by,
        notes: r.notes,
        updater: r.updater,
        update_date: r.update_date,
        parish: r.parish,
        tags: r.tags,
        accessibility_meta: serde_json::from_value(r.accessibility_meta)
            .unwrap(),
    });

    Ok(res)
}

pub(crate) async fn fetch_stops(
    pool: &PgPool,
    filter_used: bool,
) -> Result<Vec<models::Stop>> {
    Ok(if filter_used {
        sqlx::query!(
            r#"
SELECT id, source, name, official_name, osm_name, short_name, locality, street,
door, lat, lon, external_id, succeeded_by, notes, updater, update_date,
parish, tags, accessibility_meta
FROM Stops
WHERE id IN (
    SELECT DISTINCT stop
    FROM subroute_stops
)
        "#
        )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .into_iter()
        .map(|r| models::Stop {
            id: r.id,
            source: r.source,
            name: r.name,
            official_name: r.official_name,
            osm_name: r.osm_name,
            short_name: r.short_name,
            locality: r.locality,
            street: r.street,
            door: r.door,
            lat: r.lat,
            lon: r.lon,
            external_id: r.external_id,
            succeeded_by: r.succeeded_by,
            notes: r.notes,
            updater: r.updater,
            update_date: r.update_date,
            parish: r.parish,
            tags: r.tags,
            accessibility_meta: serde_json::from_value(r.accessibility_meta)
                .unwrap(),
        })
        .collect()
    } else {
        sqlx::query!(
"SELECT id, source, name, official_name, osm_name, short_name, locality, street,
    door, lat, lon, external_id, succeeded_by, notes, updater, update_date,
    parish, tags, accessibility_meta
FROM stops")
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .into_iter()
        .map(|r| models::Stop {
            id: r.id,
            source: r.source,
            name: r.name,
            official_name: r.official_name,
            osm_name: r.osm_name,
            short_name: r.short_name,
            locality: r.locality,
            street: r.street,
            door: r.door,
            lat: r.lat,
            lon: r.lon,
            external_id: r.external_id,
            succeeded_by: r.succeeded_by,
            notes: r.notes,
            updater: r.updater,
            update_date: r.update_date,
            parish: r.parish,
            tags: r.tags,
            accessibility_meta: serde_json::from_value(r.accessibility_meta)
                .unwrap(),
        })
        .collect()
    })
}

pub(crate) async fn fetch_bounded_stops(
    pool: &PgPool,
    (x0, y0, x1, y1): (f64, f64, f64, f64),
) -> Result<Vec<models::Stop>> {
    let res = sqlx::query!(
        r#"
SELECT id, source, name, official_name, osm_name, short_name, locality, street,
    door, lat, lon, external_id, succeeded_by, notes, updater, update_date,
    parish, tags, accessibility_meta
FROM Stops
WHERE lon >= $1 AND lon <= $2 AND lat <= $3 AND lat >= $4 AND id IN (
    SELECT DISTINCT stop FROM subroute_stops
)
        "#,
        x0,
        x1,
        y0,
        y1
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| models::Stop {
        id: r.id,
        source: r.source,
        name: r.name,
        official_name: r.official_name,
        osm_name: r.osm_name,
        short_name: r.short_name,
        locality: r.locality,
        street: r.street,
        door: r.door,
        lat: r.lat,
        lon: r.lon,
        external_id: r.external_id,
        succeeded_by: r.succeeded_by,
        notes: r.notes,
        updater: r.updater,
        update_date: r.update_date,
        parish: r.parish,
        tags: r.tags,
        accessibility_meta: serde_json::from_value(r.accessibility_meta)
            .unwrap(),
    })
    .collect();

    Ok(res)
}

pub(crate) async fn insert_stop(
    pool: &PgPool,
    stop: models::requests::NewStop,
    user_id: i32,
) -> Result<i32> {
    let update_date = Local::now().to_string();

    let res = sqlx::query!(
        r#"
INSERT INTO Stops(name, short_name, official_name, locality, street, door,
    lon, lat, notes, tags, accessibility_meta, updater, update_date, source)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
RETURNING id
    "#,
        stop.name,
        stop.short_name,
        stop.official_name,
        stop.locality,
        stop.street,
        stop.door,
        stop.lon,
        stop.lat,
        stop.notes,
        &stop.tags,
        &serde_json::to_value(&stop.accessibility_meta).unwrap(),
        user_id,
        update_date,
        stop.source
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_stop<'c, E>(
    executor: E,
    stop_id: i32,
    changes: models::requests::ChangeStop,
    user_id: i32,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let update_date = Local::now().to_string();

    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET name=$1, short_name=$2, official_name=$3, locality=$4, street=$5, door=$6,
    lon=$7, lat=$8, notes=$9, accessibility_meta=$10 , updater=$11, update_date=$12
WHERE id=$13
    "#,
        changes.name,
        changes.short_name,
        changes.official_name,
        changes.locality,
        changes.street,
        changes.door,
        changes.lon,
        changes.lat,
        changes.notes,
        serde_json::to_value(&changes.accessibility_meta).unwrap(),
        user_id,
        update_date,
        stop_id
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_stop_spider(
    pool: &PgPool,
    stops: &[i32],
) -> Result<models::responses::SpiderMap> {
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

    let mut routes: HashMap<i32, models::responses::SpiderRoute> =
        HashMap::new();
    let mut subroutes: HashMap<i32, models::responses::SpiderSubroute> =
        HashMap::new();
    let mut stops: HashMap<i32, models::responses::SpiderStop> = HashMap::new();

    for row in res {
        if let hash_map::Entry::Vacant(e) = routes.entry(row.route_id) {
            e.insert(models::responses::SpiderRoute {
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
                models::responses::SpiderSubroute {
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

    Ok(models::responses::SpiderMap {
        routes,
        subroutes,
        stops,
    })
}