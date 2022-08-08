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

use axum::extract::{ContentLengthLimit, Multipart, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_macros::debug_handler;
use chrono::NaiveDate;
use itertools::Itertools;
use serde::Deserialize;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use utoipa_swagger_ui::Config;

use crate::models::{
    requests,
    // This whole ordeal instead of just writing `responses::` because of uitopa
    // The macros do not support module paths
    responses::{
        DateDeparture, Departure, Parish, Route, SpiderMap, SpiderRoute,
        SpiderStop, SpiderSubroute, Subroute, SubrouteStops, UntaggedStopPic
    },
    Calendar,
    Stop
};
use crate::{middleware, Error, State};

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
    .unwrap();

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
        sqlx::query_as!(
            Stop,
            r#"
SELECT *
FROM Stops
    "#
        )
        .fetch_all(&state.pool)
        .await
    } else {
        sqlx::query_as!(
            Stop,
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
    }
    .unwrap();

    Ok((StatusCode::OK, Json(res)).into_response())
}

pub(crate) async fn create_stop(
    Extension(state): Extension<Arc<State>>,
    Json(stop): Json<requests::NewStop>,
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query!(
        r#"
INSERT INTO Stops(name, short_name, street, door, lon, lat, source)
VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING id
    "#,
        stop.name,
        stop.short_name,
        stop.street,
        stop.door,
        stop.lon,
        stop.lat,
        stop.source
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    let returned: HashMap<&str, i64> = {
        let mut map = HashMap::new();
        map.insert("id", res.id);
        map
    };

    Ok((StatusCode::OK, Json(returned)).into_response())
}

pub(crate) async fn update_stop(
    Extension(state): Extension<Arc<State>>,
    Json(stop): Json<requests::NewStop>,
    Path(stop_id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET name=?, short_name=?, street=?, door=?, lon=?, lat=?, source=?
WHERE id=?
    "#,
        stop.name,
        stop.short_name,
        stop.street,
        stop.door,
        stop.lon,
        stop.lat,
        stop.source,
        stop_id
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

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
    let res = sqlx::query_as!(
        Stop,
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
    .unwrap();

    Ok((StatusCode::OK, Json(res)).into_response())
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
    .unwrap();

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
    .unwrap();

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
    .unwrap();

    let mut row_iter = res.into_iter();

    let mut routes = vec![];

    if let Some(row) = row_iter.next() {
        let mut curr_route = Route {
            id: row.route,
            name: row.name,
            code: row.code,
            circular: row.circular.map(|val| val != 0),
            main_subroute: row.main_subroute,
            subroutes: vec![Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                cached_from: row.from_stop,
                cached_to: row.to_stop,
            }],
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
                    subroutes: vec![Subroute {
                        id: row.subroute,
                        flag: row.subroute_flag,
                        cached_from: row.from_stop,
                        cached_to: row.to_stop,
                    }],
                };
            }
        }
        routes.push(curr_route);
    }

    Ok((StatusCode::OK, Json(routes)).into_response())
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
    .unwrap();

    let mut row_iter = res.into_iter();

    if let Some(row) = row_iter.next() {
        let mut curr_route = Route {
            id: row.route,
            name: row.name,
            code: row.code,
            circular: row.circular.map(|val| val != 0),
            main_subroute: row.main_subroute,
            subroutes: vec![Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                cached_from: row.from_stop,
                cached_to: row.to_stop,
            }],
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
    .unwrap();

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
        .map_err(|_err| Error::ValidationFailure)?;

    let res = sqlx::query!(
        r#"
SELECT Subroutes.id as subroute, Departures.time as time, Departures.calendar as calendar
FROM Subroutes
JOIN Departures on Departures.subroute = Subroutes.id
WHERE Subroutes.route=?
    "#,
        route_id
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

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
) -> Result<impl IntoResponse, String> {
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
    .unwrap();

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
    Path((route_id, subroute_id)): Path<(i64, i64)>,
    Json(request): Json<requests::ChangeSubrouteStops>,
) -> Result<impl IntoResponse, String> {
    // Check if the current stops match the requests's check
    if request.from.stops.len() != request.from.diffs.len()
        || request.to.stops.len() != request.to.diffs.len()
    {
        return Err("Size divergence".to_string());
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
    .unwrap();

    // Check for the difference from stored to future
    let stored_len = existing_query_res.len();
    let check_len = request.from.stops.len();
    let to_store_len = request.to.stops.len() as i64;
    let stored_changes = to_store_len as i32 - stored_len as i32;

    if check_len != stored_len {
        return Err("Check mismatch".to_string());
    }

    let check_matched = existing_query_res
        .iter()
        .zip(request.from.stops.iter().zip(request.from.diffs.iter()))
        .all(|(row, (from_stop, from_diff))| {
            row.stop == *from_stop && row.diff == *from_diff
        });

    if !check_matched {
        return Err("Check mismatch".to_string());
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
        .unwrap()
        .rows_affected();

        if deleted_rows != stored_changes.abs() as u64 {
            return Err("Detected an unexpected amount of rows".to_string());
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
            .unwrap();
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
            .unwrap();
        }
    }

    Ok((StatusCode::OK, "").into_response())
}

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 100;

pub(crate) async fn get_untagged_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    paginator: Query<Page>,
) -> Result<impl IntoResponse, Error> {
    let offset = paginator.p.saturating_sub(1) * PAGE_SIZE;
    let res = sqlx::query!(
        r#"
SELECT id, original_filename, sha1, public, sensitive, tagged, uploader,
	upload_date, capture_date, width, height, lon, lat, camera_ref
FROM StopPics
WHERE tagged = 0
LIMIT ? OFFSET ?
    "#,
        PAGE_SIZE,
        offset
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    let mut pics = vec![];
    for row in res {
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
            camera_ref: row.camera_ref
        })
    }

    Ok((StatusCode::OK, Json(pics)).into_response())
}

#[debug_handler]
pub(crate) async fn upload_stop_picture(
    Extension(state): Extension<Arc<State>>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        { 50 * 1024 * 1024 },
    >,
) -> Result<impl IntoResponse, Error> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_err| Error::ValidationFailure)?
    {
        let filename = field
            .file_name()
            .ok_or(Error::ValidationFailure)?
            .to_string();
        let content = field
            .bytes()
            .await
            .map_err(|_err| Error::ValidationFailure)?;
        middleware::upload_stop_picture(
            filename,
            &state.bucket,
            &state.pool,
            content,
        )
        .await?;
    }

    Ok((StatusCode::OK, "").into_response())
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
