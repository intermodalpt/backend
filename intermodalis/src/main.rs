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
mod middleware;
mod models;
mod utils;

use errors::Error;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use axum::routing::{get, patch, post};
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
        .allow_methods([Method::GET, Method::POST, Method::PATCH])
        // .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        // Basic roots
        .route("/api/parishes", get(handlers::get_parishes))
        .route("/api/stops", get(handlers::get_stops))
        .route("/api/stops/create", post(handlers::create_stop))
        .route("/api/stops/update/:stop_id", patch(handlers::update_stop))
        .route(
            "/api/stops/within_boundary/:x0/:y0/:x1/:y1",
            get(handlers::get_bounded_stops),
        )
        .route("/api/stops/:stop_id/spider", get(handlers::get_stop_spider))
        .route("/api/stops/spider", post(handlers::get_stops_spider))
        .route("/api/routes", get(handlers::get_routes))
        .route("/api/routes/:route_id", get(handlers::get_route))
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
        .route(
            "/api/routes/:route_id/stops/subroutes/:subroute_id",
            patch(handlers::patch_subroute_stops),
        )
        .route("/upload/stops", post(handlers::upload_stop_picture))
        .route(
            "/upload/stops/:picture_id",
            post(handlers::upload_stop_picture_meta),
        )
        .route(
            "/tagging/stops/untagged",
            get(handlers::get_untagged_stop_pictures),
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
    let settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("SETTINGS"))
        .build()
        .unwrap();

    let credentials = s3::creds::Credentials::new(
        Some(
            &settings
                .get_string("access_key")
                .expect("access_key not set"),
        ),
        Some(
            &settings
                .get_string("secret_key")
                .expect("secret_key not set"),
        ),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket = s3::Bucket::new(
        "stoppics",
        s3::Region::R2 {
            account_id: settings
                .get_string("account_id")
                .expect("account_id not set"),
        },
        credentials,
    )
    .unwrap()
    .with_path_style();

    let state = State {
        bucket,
        pool: SqlitePool::connect("sqlite:db.sqlite").await.expect(""),
    };

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        settings.get_int("port").expect("port not set") as u16,
    ));

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
