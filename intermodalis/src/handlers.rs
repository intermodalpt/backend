/*
    Intermodalis, transportation information aggregator
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

use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use axum::extract::{ContentLengthLimit, Multipart, Path, Query};
use axum::headers::{authorization::Bearer, Authorization};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, TypedHeader};
use chrono::{Local, NaiveDate};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use utoipa_swagger_ui::Config;

use crate::calendar::Calendar;
use crate::models::responses::{PublicStopPic, TaggedStopPic};
use crate::models::{
    requests,
    // This whole ordeal instead of just writing `responses::` because of uitopa
    // The macros do not support module paths
    responses::{
        DateDeparture, Departure, Parish, Route, SpiderMap, SpiderRoute,
        SpiderStop, SpiderSubroute, Subroute, SubrouteStops, UntaggedStopPic,
    },
    Stop,
};
use crate::{middleware, osm, Error, State};

#[utoipa::path(
    get,
    path = "/api/parishes",
    responses(
        (
            status = 200,
            description = "List of parishes",
            body = [Parish])
    )
)]
pub(crate) async fn get_parishes(
    Extension(state): Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query_as!(
        Parish,
        r#"
SELECT Parishes.id, Parishes.name, Municipalities.name as municipality, Municipalities.zone, Parishes.polygon
FROM Parishes
JOIN Municipalities where Parishes.municipality = Municipalities.id
    "#
    )
    .fetch_all(&state.pool)
    .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok((StatusCode::OK, Json(res)).into_response())
}

#[derive(Deserialize)]
pub(crate) struct StopQueryParam {
    #[serde(default)]
    all: bool,
}

#[utoipa::path(
    get,
    path = "/api/stops",
    responses(
        (
            status = 200,
            description = "List of stops",
            body = [Stop])
    )
)]
pub(crate) async fn get_stops(
    Extension(state): Extension<Arc<State>>,
    params: Query<StopQueryParam>,
) -> Result<impl IntoResponse, Error> {
    let res = if params.0.all {
        middleware::get_stops(&state.pool).await?
    } else {
        sqlx::query!(
            r#"
SELECT *
FROM Stops
WHERE id IN (
    SELECT DISTINCT stop
    FROM SubrouteStops
)
    "#
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .into_iter()
        .map(|row| {
            let tags: Vec<String> =
                if let Ok(tags) = serde_json::from_str(&row.tags) {
                    tags
                } else {
                    // todo warn
                    vec![]
                };

            Stop {
                id: row.id,
                source: row.source,
                name: row.name,
                official_name: row.official_name,
                osm_name: row.osm_name,
                short_name: row.short_name,
                locality: row.locality,
                street: row.street,
                door: row.door,
                parish: row.parish,
                lat: row.lat,
                lon: row.lon,
                external_id: row.external_id,
                succeeded_by: row.succeeded_by,
                notes: row.notes,
                has_crossing: row.has_crossing.map(|val| val != 0),
                has_accessibility: row.has_accessibility.map(|val| val != 0),
                has_abusive_parking: row
                    .has_abusive_parking
                    .map(|val| val != 0),
                has_outdated_info: row.has_outdated_info.map(|val| val != 0),
                is_damaged: row.is_damaged.map(|val| val != 0),
                is_vandalized: row.is_vandalized.map(|val| val != 0),
                has_flag: row.has_flag.map(|val| val != 0),
                has_schedules: row.has_schedules.map(|val| val != 0),
                has_sidewalk: row.has_sidewalk.map(|val| val != 0),
                has_shelter: row.has_shelter.map(|val| val != 0),
                has_bench: row.has_bench.map(|val| val != 0),
                has_trash_can: row.has_trash_can.map(|val| val != 0),
                is_illuminated: row.is_illuminated.map(|val| val != 0),
                has_illuminated_path: row
                    .has_illuminated_path
                    .map(|val| val != 0),
                has_visibility_from_within: row
                    .has_visibility_from_within
                    .map(|val| val != 0),
                has_visibility_from_area: row
                    .has_visibility_from_area
                    .map(|val| val != 0),
                is_visible_from_outside: row
                    .is_visible_from_outside
                    .map(|val| val != 0),
                updater: row.updater,
                update_date: row.update_date,
                tags,
            }
        })
        .collect::<Vec<_>>()
    };

    Ok((StatusCode::OK, Json(res)).into_response())
}

pub(crate) async fn create_stop(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(stop): Json<requests::NewStop>,
) -> Result<impl IntoResponse, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;
    let update_date = Local::now().to_string();

    if user_id != 1 {
        return Err(Error::Forbidden);
    }

    let has_crossing = stop.has_crossing.map(|val| if val { 1 } else { 0 });
    let has_accessibility =
        stop.has_accessibility.map(|val| if val { 1 } else { 0 });
    let has_abusive_parking =
        stop.has_abusive_parking.map(|val| if val { 1 } else { 0 });
    let has_outdated_info =
        stop.has_outdated_info.map(|val| if val { 1 } else { 0 });
    let is_damaged = stop.is_damaged.map(|val| if val { 1 } else { 0 });
    let is_vandalized = stop.is_vandalized.map(|val| if val { 1 } else { 0 });
    let has_flag = stop.has_flag.map(|val| if val { 1 } else { 0 });
    let has_schedules = stop.has_schedules.map(|val| if val { 1 } else { 0 });
    let has_sidewalk = stop.has_sidewalk.map(|val| if val { 1 } else { 0 });
    let has_shelter = stop.has_shelter.map(|val| if val { 1 } else { 0 });
    let has_bench = stop.has_bench.map(|val| if val { 1 } else { 0 });
    let has_trash_can = stop.has_trash_can.map(|val| if val { 1 } else { 0 });
    let is_illuminated = stop.is_illuminated.map(|val| if val { 1 } else { 0 });
    let has_illuminated_path =
        stop.has_illuminated_path.map(|val| if val { 1 } else { 0 });
    let has_visibility_from_within = stop
        .has_visibility_from_within
        .map(|val| if val { 1 } else { 0 });
    let has_visibility_from_area =
        stop.has_visibility_from_area
            .map(|val| if val { 1 } else { 0 });
    let is_visible_from_outside =
        stop.is_visible_from_outside
            .map(|val| if val { 1 } else { 0 });
    let tags = serde_json::to_string(&stop.tags).unwrap_or("[]".to_string());

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
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        tags,
        has_crossing,
        has_accessibility,
        has_abusive_parking,
        has_outdated_info,
        is_damaged,
        is_vandalized,
        has_flag,
        has_schedules,
        has_sidewalk,
        has_shelter,
        has_bench,
        has_trash_can,
        is_illuminated,
        has_illuminated_path,
        has_visibility_from_within,
        has_visibility_from_area,
        is_visible_from_outside,
        user_id,
        update_date,
        stop.source
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let returned: HashMap<&str, i64> = {
        let mut map = HashMap::new();
        map.insert("id", res.id);
        map
    };

    Ok((StatusCode::OK, Json(returned)).into_response())
}

pub(crate) async fn patch_stop(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(stop): Json<requests::ChangeStop>,
    Path(stop_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;
    let update_date = Local::now().to_string();

    let has_crossing = stop.has_crossing.map(|val| if val { 1 } else { 0 });
    let has_accessibility =
        stop.has_accessibility.map(|val| if val { 1 } else { 0 });
    let has_abusive_parking =
        stop.has_abusive_parking.map(|val| if val { 1 } else { 0 });
    let has_outdated_info =
        stop.has_outdated_info.map(|val| if val { 1 } else { 0 });
    let is_damaged = stop.is_damaged.map(|val| if val { 1 } else { 0 });
    let is_vandalized = stop.is_vandalized.map(|val| if val { 1 } else { 0 });
    let has_flag = stop.has_flag.map(|val| if val { 1 } else { 0 });
    let has_schedules = stop.has_schedules.map(|val| if val { 1 } else { 0 });
    let has_sidewalk = stop.has_sidewalk.map(|val| if val { 1 } else { 0 });
    let has_shelter = stop.has_shelter.map(|val| if val { 1 } else { 0 });
    let has_bench = stop.has_bench.map(|val| if val { 1 } else { 0 });
    let has_trash_can = stop.has_trash_can.map(|val| if val { 1 } else { 0 });
    let is_illuminated = stop.is_illuminated.map(|val| if val { 1 } else { 0 });
    let has_illuminated_path =
        stop.has_illuminated_path.map(|val| if val { 1 } else { 0 });
    let has_visibility_from_within = stop
        .has_visibility_from_within
        .map(|val| if val { 1 } else { 0 });
    let has_visibility_from_area =
        stop.has_visibility_from_area
            .map(|val| if val { 1 } else { 0 });
    let is_visible_from_outside =
        stop.is_visible_from_outside
            .map(|val| if val { 1 } else { 0 });
    let tags = serde_json::to_string(&stop.tags).unwrap();

    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET name=?, short_name=?, official_name=?, locality=?, street=?, door=?,
    lon=?, lat=?, notes = ?, tags=?, has_crossing = ?, has_accessibility = ?,
    has_abusive_parking = ?, has_outdated_info = ?, is_damaged = ?,
    is_vandalized = ?, has_flag = ?, has_schedules = ?, has_sidewalk = ?,
    has_shelter = ?, has_bench = ?, has_trash_can = ?, is_illuminated = ?,
    has_illuminated_path = ?, has_visibility_from_within = ?,
    has_visibility_from_area = ?, is_visible_from_outside = ?,
    updater=?, update_date=?
WHERE id=?
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
        tags,
        has_crossing,
        has_accessibility,
        has_abusive_parking,
        has_outdated_info,
        is_damaged,
        is_vandalized,
        has_flag,
        has_schedules,
        has_sidewalk,
        has_shelter,
        has_bench,
        has_trash_can,
        is_illuminated,
        has_illuminated_path,
        has_visibility_from_within,
        has_visibility_from_area,
        is_visible_from_outside,
        user_id,
        update_date,
        stop_id
    )
    .execute(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok((StatusCode::OK, "").into_response())
}

#[utoipa::path(
    get,
    path = "/api/stops/{x0}/{y0}/{x1}/{y1}",
    responses(
        (
            status = 200,
            description = "List of stops that fit within a boundary",
            body = [Stop])
    )
)]
pub(crate) async fn get_bounded_stops(
    Extension(state): Extension<Arc<State>>,
    Path((x0, y0, x1, y1)): Path<(f64, f64, f64, f64)>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
SELECT *
FROM Stops
WHERE lon >= ? AND lon <= ? AND lat <= ? AND lat >= ? AND id IN (
    SELECT DISTINCT stop FROM SubrouteStops
)
    "#,
        x0,
        x1,
        y0,
        y1
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        let tags: Vec<String> =
            if let Ok(tags) = serde_json::from_str(&row.tags) {
                tags
            } else {
                // todo warn
                vec![]
            };

        Stop {
            id: row.id,
            source: row.source,
            name: row.name,
            official_name: row.official_name,
            osm_name: row.osm_name,
            short_name: row.short_name,
            locality: row.locality,
            street: row.street,
            door: row.door,
            parish: row.parish,
            lat: row.lat,
            lon: row.lon,
            external_id: row.external_id,
            succeeded_by: row.succeeded_by,
            notes: row.notes,
            has_crossing: row.has_crossing.map(|val| val != 0),
            has_accessibility: row.has_accessibility.map(|val| val != 0),
            has_abusive_parking: row.has_abusive_parking.map(|val| val != 0),
            has_outdated_info: row.has_outdated_info.map(|val| val != 0),
            is_damaged: row.is_damaged.map(|val| val != 0),
            is_vandalized: row.is_vandalized.map(|val| val != 0),
            has_flag: row.has_flag.map(|val| val != 0),
            has_schedules: row.has_schedules.map(|val| val != 0),
            has_sidewalk: row.has_sidewalk.map(|val| val != 0),
            has_shelter: row.has_shelter.map(|val| val != 0),
            has_bench: row.has_bench.map(|val| val != 0),
            has_trash_can: row.has_trash_can.map(|val| val != 0),
            is_illuminated: row.is_illuminated.map(|val| val != 0),
            has_illuminated_path: row.has_illuminated_path.map(|val| val != 0),
            has_visibility_from_within: row
                .has_visibility_from_within
                .map(|val| val != 0),
            has_visibility_from_area: row
                .has_visibility_from_area
                .map(|val| val != 0),
            is_visible_from_outside: row
                .is_visible_from_outside
                .map(|val| val != 0),
            updater: row.updater,
            update_date: row.update_date,
            tags,
        }
    })
    .collect::<Vec<_>>();

    Ok((StatusCode::OK, Json(res)).into_response())
}

pub(crate) async fn get_public_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
SELECT StopPics.id, StopPics.sha1, StopPics.capture_date, StopPics.lon, StopPics.lat, StopPics.tags, StopPics.quality
FROM StopPics
JOIN StopPicStops on StopPicStops.pic = StopPics.id
WHERE StopPics.tagged = 0 AND StopPics.sensitive = 0
    AND StopPics.public = 1 AND StopPicStops.stop=?
ORDER BY StopPics.capture_date DESC
    "#,
        stop_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut pics = vec![];
    for row in res {
        let tags: Vec<String> =
            if let Ok(tags) = serde_json::from_str(&row.tags) {
                tags
            } else {
                // todo warn
                vec![]
            };

        pics.push(PublicStopPic {
            id: row.id,
            sha1: row.sha1,
            capture_date: row.capture_date,
            lon: row.lon.unwrap_or(f32::NAN),
            lat: row.lat.unwrap_or(f32::NAN),
            quality: row.quality,
            tags,
        });
    }
    Ok((StatusCode::OK, Json(pics)).into_response())
}

pub(crate) async fn get_tagged_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i64>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let res = sqlx::query!(
        r#"
SELECT StopPics.id, StopPics.original_filename, StopPics.sha1, StopPics.public,
    StopPics.sensitive, StopPics.tagged, StopPics.uploader,
    StopPics.upload_date, StopPics.capture_date, StopPics.quality,
    StopPics.width, StopPics.height, StopPics.lon, StopPics.lat,
    StopPics.camera_ref, StopPics.tags, StopPics.notes
FROM StopPics
JOIN StopPicStops ON StopPicStops.pic = StopPics.id
WHERE StopPics.tagged = 1 AND StopPicStops.stop=?
ORDER BY quality DESC
    "#,
        stop_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut pics = vec![];
    for row in res {
        let tags: Vec<String> =
            if let Ok(tags) = serde_json::from_str(&row.tags) {
                tags
            } else {
                // todo warn
                vec![]
            };

        pics.push(TaggedStopPic {
            id: row.id,
            original_filename: row.original_filename,
            sha1: row.sha1,
            public: row.public != 0,
            sensitive: row.sensitive != 0,
            uploader: row.uploader,
            upload_date: row.upload_date,
            capture_date: row.capture_date,
            lon: row.lon.unwrap_or(f32::NAN),
            lat: row.lat.unwrap_or(f32::NAN),
            quality: row.quality,
            width: row.width as u32,
            height: row.height as u32,
            camera_ref: row.camera_ref,
            tags,
            notes: row.notes,
        });
    }
    Ok((StatusCode::OK, Json(pics)).into_response())
}

pub(crate) async fn get_picture_stop_rels(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let res = sqlx::query!(
        r#"
SELECT stop, pic
FROM  StopPicStops
ORDER BY stop ASC
    "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut stops = HashMap::<i64, Vec<i64>>::new();

    for row in res {
        if let Some(pics) = stops.get_mut(&row.stop) {
            pics.push(row.pic);
        } else {
            stops.insert(row.stop, vec![row.pic]);
        }
    }
    Ok((StatusCode::OK, Json(stops)).into_response())
}

pub(crate) async fn get_pictures(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let res = sqlx::query!("SELECT * FROM StopPics")
        .fetch_all(&state.pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut pics = vec![];

    for row in res {
        let tags: Vec<String> =
            if let Ok(tags) = serde_json::from_str(&row.tags) {
                tags
            } else {
                // todo warn
                vec![]
            };

        pics.push(TaggedStopPic {
            id: row.id,
            original_filename: row.original_filename,
            sha1: row.sha1,
            public: row.public != 0,
            sensitive: row.sensitive != 0,
            uploader: row.uploader,
            upload_date: row.upload_date,
            capture_date: row.capture_date,
            lon: row.lon.unwrap_or(f32::NAN),
            lat: row.lat.unwrap_or(f32::NAN),
            quality: row.quality,
            width: row.width as u32,
            height: row.height as u32,
            camera_ref: row.camera_ref,
            tags,
            notes: row.notes,
        });
    }

    Ok((StatusCode::OK, Json(pics)).into_response())
}

#[utoipa::path(get, path = "/api/stops/{stop_id}/spider")]
pub(crate) async fn get_stop_spider(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
SELECT Routes.id as route_id, Routes.code as route_code,
    Routes.name as route_name, Routes.circular as route_circular,
    Subroutes.id as subroute_id, Subroutes.flag as subroute_flag,
    SubrouteStops.stop as stop_id,
    Stops.name as stop_name,
    Stops.lon as lon,
    Stops.lat as lat
FROM Routes
JOIN Subroutes ON Routes.id = Subroutes.route
JOIN SubrouteStops ON Subroutes.id = SubrouteStops.subroute
JOIN Stops ON Stops.id = SubrouteStops.stop
WHERE Subroutes.id IN (
    SELECT Subroutes.id
    FROM Subroutes
    JOIN SubrouteStops ON Subroutes.id = SubrouteStops.subroute
    WHERE SubrouteStops.stop = ?
)
ORDER BY SubrouteStops.idx
    "#,
        stop_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut routes: HashMap<i64, SpiderRoute> = HashMap::new();
    let mut subroutes: HashMap<i64, SpiderSubroute> = HashMap::new();
    let mut stops: HashMap<i64, SpiderStop> = HashMap::new();

    for row in res {
        if !routes.contains_key(&row.route_id) {
            routes.insert(
                row.route_id,
                SpiderRoute {
                    code: row.route_code,
                    name: row.route_name,
                    circular: row
                        .route_circular
                        .map(|val| val != 0)
                        .unwrap_or(false),
                },
            );
        }

        if let Some(subroute) = subroutes.get_mut(&row.subroute_id) {
            subroute.stop_sequence.push(row.stop_id);
        } else {
            subroutes.insert(
                row.subroute_id,
                SpiderSubroute {
                    route: row.route_id,
                    flag: row.subroute_flag,
                    stop_sequence: vec![],
                },
            );
        }

        if !stops.contains_key(&row.stop_id) {
            stops.insert(
                row.stop_id,
                SpiderStop {
                    name: row.stop_name,
                    lat: row.lat,
                    lon: row.lon,
                },
            );
        }
    }

    let map = SpiderMap {
        routes,
        subroutes,
        stops,
    };

    Ok((StatusCode::OK, Json(map)).into_response())
}

struct SpiderRow {
    route_id: i64,
    route_code: String,
    route_name: String,
    route_circular: Option<i64>,
    subroute_id: i64,
    subroute_flag: Option<String>,
    stop_id: i64,
    stop_name: Option<String>,
    lon: Option<f32>,
    lat: Option<f32>,
}

pub(crate) async fn get_stops_spider(
    Extension(state): Extension<Arc<State>>,
    Json(stops): Json<Vec<i64>>,
) -> Result<impl IntoResponse, Error> {
    let stop_ids = stops.iter().join(",");

    let res = sqlx::query(&format!(
        "\
SELECT Routes.id as route_id,
    Routes.code as route_code,
    Routes.name as route_name,
    Routes.circular as route_circular,
    Subroutes.id as subroute_id,
    Subroutes.flag as subroute_flag,
    SubrouteStops.stop as stop_id,
    Stops.name as stop_name,
    Stops.lon as lon,
    Stops.lat as lat
FROM Routes
JOIN Subroutes ON Routes.id = Subroutes.route
JOIN SubrouteStops ON Subroutes.id = SubrouteStops.subroute
JOIN Stops ON Stops.id = SubrouteStops.stop
WHERE Subroutes.id IN (
    SELECT Subroutes.id
    FROM Subroutes
    JOIN SubrouteStops ON Subroutes.id = SubrouteStops.subroute
    WHERE SubrouteStops.stop IN ({stop_ids})
)
ORDER BY SubrouteStops.idx"
    ))
    .map(|row: SqliteRow| SpiderRow {
        route_id: row.get(0),
        route_code: row.get(1),
        route_name: row.get(2),
        route_circular: row.get(3),
        subroute_id: row.get(4),
        subroute_flag: row.get(5),
        stop_id: row.get(6),
        stop_name: row.get(7),
        lon: row.get(8),
        lat: row.get(9),
    })
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut routes: HashMap<i64, SpiderRoute> = HashMap::new();
    let mut subroutes: HashMap<i64, SpiderSubroute> = HashMap::new();
    let mut stops: HashMap<i64, SpiderStop> = HashMap::new();

    for row in res {
        if !routes.contains_key(&row.route_id) {
            routes.insert(
                row.route_id,
                SpiderRoute {
                    code: row.route_code,
                    name: row.route_name,
                    circular: row
                        .route_circular
                        .map(|val| val != 0)
                        .unwrap_or(false),
                },
            );
        }

        if let Some(subroute) = subroutes.get_mut(&row.subroute_id) {
            subroute.stop_sequence.push(row.stop_id);
        } else {
            subroutes.insert(
                row.subroute_id,
                SpiderSubroute {
                    route: row.route_id,
                    flag: row.subroute_flag,
                    stop_sequence: vec![],
                },
            );
        }

        if !stops.contains_key(&row.stop_id) {
            stops.insert(
                row.stop_id,
                SpiderStop {
                    name: row.stop_name,
                    lat: row.lat,
                    lon: row.lon,
                },
            );
        }
    }

    let map = SpiderMap {
        routes,
        subroutes,
        stops,
    };

    Ok((StatusCode::OK, Json(map)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/routes",
    responses(
        (status = 200, description = "List of routes", body = [Route]),
    )
)]
pub(crate) async fn get_routes(
    Extension(state): Extension<Arc<State>>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
SELECT Routes.id as route,
    Routes.code as code,
    Routes.name as name,
    Routes.circular as circular,
    Routes.main_subroute as main_subroute,
    Routes.active as active,
    Routes.badge_bg as bg_color,
    Routes.badge_text as text_color,
    Subroutes.id as subroute,
    Subroutes.flag as subroute_flag,
    Subroutes.cached_from as from_stop,
    Subroutes.cached_to as to_stop
FROM Routes
LEFT JOIN Subroutes on Routes.id = Subroutes.route
ORDER BY Routes.id asc
"#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut row_iter = res.into_iter();

    let mut routes = vec![];

    if let Some(row) = row_iter.next() {
        let mut curr_route = Route {
            id: row.route,
            name: row.name,
            code: row.code,
            circular: row.circular.map(|val| val != 0),
            main_subroute: row.main_subroute,
            badge_text: row.text_color,
            badge_bg: row.bg_color,
            subroutes: vec![Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                cached_from: row.from_stop,
                cached_to: row.to_stop,
            }],
            active: row.active != 0,
        };

        for row in row_iter {
            if row.route == curr_route.id {
                curr_route.subroutes.push(Subroute {
                    id: row.subroute,
                    flag: row.subroute_flag,
                    cached_from: row.from_stop,
                    cached_to: row.to_stop,
                });
            } else {
                routes.push(curr_route);
                curr_route = Route {
                    id: row.route,
                    code: row.code,
                    name: row.name,
                    circular: row.circular.map(|val| val != 0),
                    main_subroute: row.main_subroute,
                    badge_text: row.text_color,
                    badge_bg: row.bg_color,
                    subroutes: vec![Subroute {
                        id: row.subroute,
                        flag: row.subroute_flag,
                        cached_from: row.from_stop,
                        cached_to: row.to_stop,
                    }],
                    active: row.active != 0,
                };
            }
        }
        routes.push(curr_route);
    }

    Ok((StatusCode::OK, Json(routes)).into_response())
}

pub(crate) async fn create_route(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(route): Json<requests::ChangeRoute>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let res = sqlx::query!(
        r#"
INSERT INTO Routes(code, name, main_subroute, operator, badge_text, badge_bg, active)
VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING id
    "#,
        route.code,
        route.name,
        route.main_subroute,
        route.operator,
        route.badge_text,
        route.badge_bg,
        route.active
    )
        .fetch_one(&state.pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let returned: HashMap<&str, i64> = {
        let mut map = HashMap::new();
        map.insert("id", res.id);
        map
    };

    Ok((StatusCode::OK, Json(returned)).into_response())
}

pub(crate) async fn get_route(
    Extension(state): Extension<Arc<State>>,
    Path(route_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
SELECT Routes.id as route,
    Routes.code as code,
    Routes.name as name,
    Routes.circular as circular,
    Routes.main_subroute as main_subroute,
    Routes.active as active,
    Routes.badge_bg as bg_color,
    Routes.badge_text as text_color,
    Subroutes.id as subroute,
    Subroutes.flag as subroute_flag,
    Subroutes.cached_from as from_stop,
    Subroutes.cached_to as to_stop
FROM Routes
LEFT JOIN Subroutes on Routes.id = Subroutes.route
WHERE Routes.id = ?
ORDER BY Routes.id asc
"#,
        route_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut row_iter = res.into_iter();

    if let Some(row) = row_iter.next() {
        let mut curr_route = Route {
            id: row.route,
            name: row.name,
            code: row.code,
            circular: row.circular.map(|val| val != 0),
            main_subroute: row.main_subroute,
            badge_text: row.text_color,
            badge_bg: row.bg_color,
            subroutes: vec![Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                cached_from: row.from_stop,
                cached_to: row.to_stop,
            }],
            active: row.active != 0,
        };

        for row in row_iter {
            curr_route.subroutes.push(Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                cached_from: row.from_stop,
                cached_to: row.to_stop,
            });
        }
        Ok((StatusCode::OK, Json(curr_route)).into_response())
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn patch_route(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(route_id): Path<i64>,
    Json(route): Json<requests::ChangeRoute>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let _res = sqlx::query!(
        r#"
UPDATE Routes
SET code=?, name=?, main_subroute=?, operator=?,
    badge_text=?, badge_bg=?, active=?
WHERE id=?
    "#,
        route.code,
        route.name,
        route.main_subroute,
        route.operator,
        route.badge_text,
        route.badge_bg,
        route.active,
        route_id
    )
    .execute(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok((StatusCode::OK, "").into_response())
}

pub(crate) async fn delete_route(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(route_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let subroute_count: i32 = sqlx::query!(
        r#"
SELECT count(*) as count
FROM Subroutes
WHERE route=?
"#,
        route_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .count;

    if subroute_count > 0 {
        return Err(Error::DependenciesNotMet);
    }

    let deleted_rows = sqlx::query!(
        r#"
DELETE FROM Routes
WHERE id=?
    "#,
        route_id
    )
    .execute(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .rows_affected();

    if deleted_rows != 1 {
        todo!();
    }

    Ok((StatusCode::OK, "").into_response())
}

pub(crate) async fn create_subroute(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(route_id): Path<i64>,
    Json(subroute): Json<requests::ChangeSubroute>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let circular_pseudo_bool = if subroute.circular { 1 } else { 0 };
    let res = sqlx::query!(
        r#"
INSERT INTO Subroutes(route, flag, circular)
VALUES (?, ?, ?)
RETURNING id
    "#,
        route_id,
        subroute.flag,
        circular_pseudo_bool,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let returned: HashMap<&str, i64> = {
        let mut map = HashMap::new();
        map.insert("id", res.id);
        map
    };

    Ok((StatusCode::OK, Json(returned)).into_response())
}

pub(crate) async fn patch_subroute(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(route_id): Path<i64>,
    Path(subroute_id): Path<i64>,
    Json(route): Json<requests::ChangeSubroute>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let _res = sqlx::query!(
        r#"
UPDATE Subroutes
SET flag=?, circular=?
WHERE id=? AND route=?
    "#,
        route.flag,
        route.circular,
        subroute_id,
        route_id,
    )
    .execute(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok((StatusCode::OK, "").into_response())
}

pub(crate) async fn delete_subroute(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path((route_id, subroute_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let stop_count: i32 = sqlx::query!(
        r#"
SELECT count(*) as count
FROM SubrouteStops
WHERE subroute=?
"#,
        subroute_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .count;

    if stop_count > 0 {
        return Err(Error::DependenciesNotMet);
    }

    let deleted_rows = sqlx::query!(
        r#"
DELETE FROM Subroutes
WHERE id=? AND route=?
    "#,
        subroute_id,
        route_id
    )
    .execute(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .rows_affected();

    if deleted_rows != 1 {
        todo!();
    }

    Ok((StatusCode::OK, "").into_response())
}

#[utoipa::path(
    get,
    path = "/api/routes/{route_id}/schedule",
    params(
        (
            "route_id",
            path,
            description = "Route identifier"
        ),
    ),
    responses(
        (
            status = 200,
            description = "Route schedule",
            body = [Departure]
        ),
        (
            status = 404,
            description = "Route does not exist"
        ),
    )
)]
pub(crate) async fn get_schedule(
    Extension(state): Extension<Arc<State>>,
    Path(route_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
SELECT Subroutes.id as subroute,
    Departures.time as time,
    Departures.calendar as calendar
FROM Subroutes
JOIN Departures on Departures.subroute = Subroutes.id
WHERE Subroutes.route=?
    "#,
        route_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut departures = vec![];
    for row in res {
        departures.push(Departure {
            subroute: row.subroute,
            time: row.time,
            calendar: serde_json::from_str(&row.calendar)
                .map_err(|_err| Error::DatabaseDeserialization)?,
        });
    }

    Ok((StatusCode::OK, Json(departures)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/routes/{route_id}/schedule/{date}",
    params(
        (
            "route_id",
            path,
            description = "Route identifier"
        ),
    ),
    params(
        (
            "date",
            path,
            description = "Date of the schedule, in the YYYY-MM-DD format"
        ),
    ),
    responses(
        (
            status = 200,
            description = "Route schedule for a specific day",
            body = [DateDeparture]
        ),
        (
            status = 400,
            description = "Invalid date"
        ),
        (
            status = 404,
            description = "Route does not exist"
        ),
    )
)]
pub(crate) async fn get_schedule_for_date(
    Extension(state): Extension<Arc<State>>,
    Path((route_id, date)): Path<(i64, String)>,
) -> Result<impl IntoResponse, Error> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let res = sqlx::query!(
        r#"
SELECT Subroutes.id as subroute, Departures.time as time, Departures.calendar as calendar
FROM Subroutes
JOIN Departures on Departures.subroute = Subroutes.id
WHERE Subroutes.route=?
ORDER BY time asc
    "#,
        route_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut departures = vec![];
    for row in res {
        let calendar: Calendar = serde_json::from_str(&row.calendar)
            .map_err(|_err| Error::DatabaseDeserialization)?;
        if calendar.includes(date) {
            departures.push(DateDeparture {
                subroute: row.subroute,
                time: row.time,
            });
        }
    }

    Ok((StatusCode::OK, Json(departures)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/routes/{route_id}/stops",
    params(
        (
            "route_id",
            path,
            description = "Route identifier"
        ),
    ),
    responses(
        (
            status = 200,
            description = "Stops a route makes along its subroutes",
        ),
        (
            status = 404,
            description = "Route does not exist"
        ),
    )
)]
pub(crate) async fn get_route_stops(
    Extension(state): Extension<Arc<State>>,
    Path(route_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
SELECT Subroutes.id as subroute, SubrouteStops.stop as stop, SubrouteStops.time_to_next as diff
FROM Subroutes
JOIN SubrouteStops on SubrouteStops.subroute = Subroutes.id
WHERE Subroutes.route=?
ORDER BY Subroutes.id ASC, SubrouteStops.idx ASC
    "#,
        route_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let subroute_stops = res
        .into_iter()
        .group_by(|row| row.subroute)
        .into_iter()
        .map(|(subroute, group)| {
            (
                subroute,
                group
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|stop| (stop.stop, stop.diff))
                    .unzip(),
            )
        })
        .map(|(key, (stops, diffs))| SubrouteStops {
            subroute: key,
            stops,
            diffs,
        })
        .collect::<Vec<_>>();

    Ok((StatusCode::OK, Json(subroute_stops)).into_response())
}

pub(crate) async fn patch_subroute_stops(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path((route_id, subroute_id)): Path<(i64, i64)>,
    Json(request): Json<requests::ChangeSubrouteStops>,
) -> Result<impl IntoResponse, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    if user_id != 1 {
        return Err(Error::Forbidden);
    }

    // Check if the current stops match the requests's check
    if request.from.stops.len() != request.from.diffs.len()
        || request.to.stops.len() != request.to.diffs.len()
    {
        return Err(Error::ValidationFailure("Size divergence".to_string()));
    }

    let existing_query_res = sqlx::query!(
        r#"
SELECT SubrouteStops.stop as stop, SubrouteStops.time_to_next as diff
FROM Subroutes
JOIN SubrouteStops on SubrouteStops.subroute = Subroutes.id
WHERE Subroutes.route=? AND Subroutes.id=?
ORDER BY SubrouteStops.idx ASC
    "#,
        route_id,
        subroute_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    // Check for the difference from stored to future
    let stored_len = existing_query_res.len();
    let check_len = request.from.stops.len();
    let to_store_len = request.to.stops.len() as i64;
    let stored_changes = to_store_len as i32 - stored_len as i32;

    if check_len != stored_len {
        return Err(Error::ValidationFailure("Check mismatch".to_string()));
    }

    let check_matched = existing_query_res
        .iter()
        .zip(request.from.stops.iter().zip(request.from.diffs.iter()))
        .all(|(row, (from_stop, from_diff))| {
            row.stop == *from_stop && row.diff == *from_diff
        });

    if !check_matched {
        return Err(Error::ValidationFailure("Check mismatch".to_string()));
    }

    let existing_duplicates_count = existing_query_res
        .iter()
        .zip(request.to.stops.iter().zip(request.to.diffs.iter()))
        .filter(|(row, (from_stop, from_diff))| {
            row.stop == **from_stop && row.diff == **from_diff
        })
        .count();

    if stored_changes == 0 && existing_duplicates_count == stored_len {
        return Ok((StatusCode::OK, "No changes").into_response());
    }

    if stored_changes < 0 {
        let deleted_rows = sqlx::query!(
            r#"
DELETE FROM SubrouteStops
WHERE Subroute=? AND idx>=?
    "#,
            subroute_id,
            to_store_len
        )
        .execute(&state.pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .rows_affected();

        if deleted_rows != stored_changes.abs() as u64 {
            return Err(Error::Processing(
                "Detected an unexpected amount of rows".to_string(),
            ));
        }
    } else if stored_changes > 0 {
        let additional_entries = request
            .to
            .stops
            .iter()
            .zip(request.to.diffs.iter())
            .skip(stored_len)
            .enumerate();

        for (index, (stop, diff)) in additional_entries {
            let index = (stored_len + index) as i64;
            let _res = sqlx::query!(
                r#"
INSERT INTO SubrouteStops(subroute, stop, time_to_next, idx)
VALUES (?, ?, ?, ?)
    "#,
                subroute_id,
                stop,
                diff,
                index
            )
            .execute(&state.pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    };

    if existing_duplicates_count != stored_len {
        // Update the already existing records
        let overlapping_entries = request
            .to
            .stops
            .iter()
            .zip(request.to.diffs.into_iter())
            .take(stored_len)
            .enumerate();

        for (index, (stop, diff)) in overlapping_entries {
            let index = index as i64;
            let _res = sqlx::query!(
                r#"
UPDATE SubrouteStops
SET stop=?, time_to_next=?
WHERE  subroute=? AND idx=?
    "#,
                stop,
                diff,
                subroute_id,
                index
            )
            .execute(&state.pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    }

    Ok((StatusCode::OK, "").into_response())
}

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 20;

pub(crate) async fn get_untagged_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    paginator: Query<Page>,
) -> Result<impl IntoResponse, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let offset = paginator.p * PAGE_SIZE;
    let res = sqlx::query!(
        r#"
SELECT id, original_filename, sha1, public, sensitive, tagged, uploader,
	upload_date, capture_date, width, height, lon, lat, camera_ref, tags, notes
FROM StopPics
WHERE tagged = 0 AND uploader = ?
ORDER BY capture_date ASC
LIMIT ? OFFSET ?
    "#,
        user_id,
        PAGE_SIZE,
        offset
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut pics = vec![];
    for row in res {
        let tags: Vec<String> =
            if let Ok(tags) = serde_json::from_str(&row.tags) {
                tags
            } else {
                // todo warn
                vec![]
            };

        pics.push(UntaggedStopPic {
            id: row.id,
            original_filename: row.original_filename,
            sha1: row.sha1,
            public: row.public != 0,
            sensitive: row.sensitive != 0,
            uploader: row.uploader,
            upload_date: row.upload_date,
            capture_date: row.capture_date,
            lon: row.lon,
            lat: row.lat,
            width: row.width as u32,
            height: row.height as u32,
            camera_ref: row.camera_ref,
            tags,
            notes: row.notes,
        })
    }

    Ok((StatusCode::OK, Json(pics)).into_response())
}

pub(crate) async fn upload_stop_picture(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        { 500 * 1024 * 1024 },
    >,
) -> Result<impl IntoResponse, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?
    {
        let filename = field
            .file_name()
            .ok_or(Error::ValidationFailure(
                "File without a filename".to_string(),
            ))?
            .to_string();
        let content = field
            .bytes()
            .await
            .map_err(|err| Error::ValidationFailure(err.to_string()))?;

        let res = middleware::upload_stop_picture(
            user_id,
            filename.clone(),
            &state.bucket,
            &state.pool,
            &content,
        )
        .await;

        if res.is_err() {
            sleep(Duration::from_secs(1));
        } else {
            continue;
        }
        // Retry, just in case
        middleware::upload_stop_picture(
            user_id,
            filename.clone(),
            &state.bucket,
            &state.pool,
            &content,
        )
        .await?;
    }

    Ok((StatusCode::OK, "").into_response())
}

pub(crate) async fn patch_stop_picture_meta(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(stop_picture_id): Path<i64>,
    Json(stop_pic_meta): Json<requests::ChangeStopPic>,
) -> Result<impl IntoResponse, Error> {
    let updater = middleware::get_user(auth.token(), &state.pool).await?;
    let update_date = Local::now().to_string();

    // TODO add updater and update date
    let stop_ids = stop_pic_meta.stops.iter().join(",");

    let tags = serde_json::to_string(&stop_pic_meta.tags).unwrap();
    let _res = sqlx::query!(
        r#"
UPDATE StopPics
SET public=?, sensitive=?, lon=?, lat=?, tags=?, quality=?, updater=?,
    update_date=?, tagged=1
WHERE id=?
    "#,
        stop_pic_meta.public,
        stop_pic_meta.sensitive,
        stop_pic_meta.lon,
        stop_pic_meta.lat,
        tags,
        stop_pic_meta.quality,
        updater,
        update_date,
        stop_picture_id
    )
    .execute(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let _res = sqlx::query(&format!(
        r#"
DELETE FROM StopPicStops
WHERE pic=? AND stop NOT IN ({stop_ids})
    "#
    ))
    .bind(stop_picture_id)
    .execute(&state.pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    for stop_id in stop_pic_meta.stops {
        let _res = sqlx::query!(
            r#"
INSERT OR IGNORE INTO StopPicStops(pic, stop)
VALUES (?, ?)
    "#,
            stop_picture_id,
            stop_id
        )
        .execute(&state.pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    Ok((StatusCode::OK, "").into_response())
}

pub(crate) async fn delete_stop_picture(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(stop_picture_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    if user_id != 1 {
        return Err(Error::Forbidden);
    }

    middleware::delete_stop_picture(
        stop_picture_id,
        &state.bucket,
        &state.pool,
    )
    .await?;

    Ok((StatusCode::OK, "").into_response())
}

pub(crate) async fn check_auth(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, Error> {
    if middleware::try_get_user(auth.token(), &state.pool)
        .await?
        .is_some()
    {
        Ok((StatusCode::OK, "Success").into_response())
    } else {
        Ok((StatusCode::UNAUTHORIZED, "Failure").into_response())
    }
}

pub(crate) async fn get_stats(
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(&state.stats)).into_response()
}

pub(crate) async fn import_osm(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    #[derive(Serialize)]
    struct Diff {
        inserted: usize,
        updated: usize,
    }

    let (inserted, updated) = osm::import(&state.pool).await?;

    Ok((StatusCode::OK, Json(Diff { inserted, updated })).into_response())
}

#[allow(clippy::unused_async)]
pub(crate) async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file.map_or_else(
            || StatusCode::NOT_FOUND.into_response(),
            |file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            },
        ),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            .into_response(),
    }
}
