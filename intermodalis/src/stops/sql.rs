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

pub(crate) async fn fetch_stops(
    pool: &PgPool,
    filter_used: bool,
) -> Result<Vec<models::Stop>> {
    Ok(if filter_used {
        sqlx::query_as!(
            models::Stop,
            r#"
SELECT *
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
    } else {
        sqlx::query_as!(models::Stop, "SELECT * FROM stops")
            .fetch_all(pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    })
}

pub(crate) async fn fetch_bounded_stops(
    pool: &PgPool,
    (x0, y0, x1, y1): (f64, f64, f64, f64),
) -> Result<Vec<models::Stop>> {
    sqlx::query_as!(
        models::Stop,
        r#"
SELECT *
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
    lon, lat, notes, tags, has_crossing, has_accessibility,
    has_abusive_parking, has_outdated_info, is_damaged,
    is_vandalized, has_flag, has_schedules, has_sidewalk,
    has_shelter, has_bench, has_trash_can, is_illuminated,
    has_illuminated_path, has_visibility_from_within,
    has_visibility_from_area, is_visible_from_outside,
    updater, update_date, source)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30)
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
        stop.has_crossing,
        stop.has_accessibility,
        stop.has_abusive_parking,
        stop.has_outdated_info,
        stop.is_damaged,
        stop.is_vandalized,
        stop.has_flag,
        stop.has_schedules,
        stop.has_sidewalk,
        stop.has_shelter,
        stop.has_bench,
        stop.has_trash_can,
        stop.is_illuminated,
        stop.has_illuminated_path,
        stop.has_visibility_from_within,
        stop.has_visibility_from_area,
        stop.is_visible_from_outside,
        user_id,
        update_date,
        stop.source
    )
        .fetch_one(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_stop(
    pool: &PgPool,
    stop_id: i32,
    changes: models::requests::ChangeStop,
    user_id: i32,
) -> Result<()> {
    let update_date = Local::now().to_string();

    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET name=$1, short_name=$2, official_name=$3, locality=$4, street=$5, door=$6,
    lon=$7, lat=$8, notes=$9, tags=$10, has_crossing=$11, has_accessibility=$12,
    has_abusive_parking=$13, has_outdated_info=$14, is_damaged=$15,
    is_vandalized=$16, has_flag=$17, has_schedules=$18, has_sidewalk=$19,
    has_shelter=$20, has_bench=$21, has_trash_can=$22, is_illuminated=$23,
    has_illuminated_path=$24, has_visibility_from_within=$25,
    has_visibility_from_area=$26, is_visible_from_outside=$27,
    updater=$28, update_date=$29
WHERE id=$30
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
        &changes.tags,
        changes.has_crossing,
        changes.has_accessibility,
        changes.has_abusive_parking,
        changes.has_outdated_info,
        changes.is_damaged,
        changes.is_vandalized,
        changes.has_flag,
        changes.has_schedules,
        changes.has_sidewalk,
        changes.has_shelter,
        changes.has_bench,
        changes.has_trash_can,
        changes.is_illuminated,
        changes.has_illuminated_path,
        changes.has_visibility_from_within,
        changes.has_visibility_from_area,
        changes.is_visible_from_outside,
        user_id,
        update_date,
        stop_id
    )
    .execute(pool)
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
