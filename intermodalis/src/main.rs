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

#![deny(
    nonstandard_style,
    warnings,
    unused,
    future_incompatible,
    clippy::all,
    clippy::pedantic
)]

mod consts;
mod errors;
mod handlers;
mod models;
mod utils;

use errors::Error;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use axum::routing::get;
use axum::{Extension, Json, Router};
use sqlx::sqlite::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) pool: SqlitePool,
}

pub(crate) fn build_paths(state: State) -> Router {
    let api_doc = ApiDoc::openapi();
    let config = Arc::new(Config::from("/api-doc/openapi.json"));

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        // Basic roots
        .route("/api/parishes", get(handlers::get_parishes))
        .route("/api/stops", get(handlers::get_stops))
        .route("/api/routes", get(handlers::get_routes))
        .route(
            "/api/routes/:route_id/schedule",
            get(handlers::get_schedule),
        )
        .route(
            "/api/routes/:route_id/schedule/:date",
            get(handlers::get_schedule_for_date),
        )
        .route(
            "/api/routes/:route_id/stops",
            get(handlers::get_route_stops),
        )
        .layer(Extension(Arc::new(state)))
        .route(
            "/api-doc/openapi.json",
            get(move || async { Json(api_doc) }),
        )
        .route(
            "/api/docs/*tail",
            get(handlers::serve_swagger_ui).layer(Extension(config)),
        )
        .layer(cors)
}

#[tokio::main]
async fn main() {
    let state = State {
        pool: SqlitePool::connect("sqlite:db.sqlite").await.expect(""),
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(build_paths(state).into_make_service())
        .await
        .expect("Unable to start service");
}

use models::{
    responses::{
        DateDeparture, Departure, Parish, Route, Subroute, SubrouteStops,
    },
    Calendar, Stop, Weekday,
};

#[derive(OpenApi)]
#[openapi(
    handlers(
        handlers::get_parishes,
        handlers::get_stops,
        handlers::get_routes,
        handlers::get_schedule,
        handlers::get_schedule_for_date,
        handlers::get_route_stops,
    ),
    components(
        Stop,
        Calendar,
        Weekday,
        DateDeparture,
        Departure,
        Parish,
        Route,
        Subroute,
        SubrouteStops,
    ),
    tags()
)]
struct ApiDoc;
