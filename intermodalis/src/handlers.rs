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

use crate::models::{
    // This whole ordeal instead of just writing `responses::` because of uitopa
    // The macros do not support module paths
    responses::{
        DateDeparture, Departure, Parish, Route, Subroute, SubrouteStops,
    },
    Calendar,
    Stop,
};
use crate::{Error, State};

use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chrono::NaiveDate;
use itertools::Itertools;
use utoipa_swagger_ui::Config;

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
) -> Result<impl IntoResponse, Error> {
    let res = sqlx::query_as!(
        Stop,
        r#"
--SELECT id, name, short_name, parish, lat, lon, osm_id
SELECT *
FROM Stops
--WHERE source = 'cmet'
    "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Ok((StatusCode::OK, Json(res)).into_response())
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
SELECT Routes.id as route, Routes.flag as flag, Routes.circular as circular,
    Routes.main_subroute as main_subroute,
    Subroutes.id as subroute, Subroutes.verbose_flag as subroute_flag,
    Subroutes.cached_from as from_stop, Subroutes.cached_to as to_stop
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
            flag: row.flag,
            circular: row.circular != 0,
            main_subroute: row.main_subroute,
            subroutes: vec![Subroute {
                id: row.subroute,
                verbose_flag: row.subroute_flag,
                cached_from: row.from_stop,
                cached_to: row.to_stop,
            }],
        };

        for row in row_iter {
            if row.route == curr_route.id {
                curr_route.subroutes.push(Subroute {
                    id: row.subroute,
                    verbose_flag: row.subroute_flag,
                    cached_from: row.from_stop,
                    cached_to: row.to_stop,
                });
            } else {
                routes.push(curr_route);
                curr_route = Route {
                    id: row.route,
                    flag: row.flag,
                    circular: row.circular != 0,
                    main_subroute: row.main_subroute,
                    subroutes: vec![Subroute {
                        id: row.subroute,
                        verbose_flag: row.subroute_flag,
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
            body = [DateDeparture]
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
