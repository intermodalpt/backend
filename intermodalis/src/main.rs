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

#![warn(
    nonstandard_style,
    warnings,
    unused,
    future_incompatible,
    clippy::all,
    clippy::pedantic
)]

mod calendar;
mod errors;
mod handlers;
mod middleware;
mod models;
mod osm;
mod utils;

use errors::Error;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use axum::routing::{get, patch, post};
use axum::{Extension, Json, Router};
use config::Config;
use s3;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) bucket: s3::Bucket,
    pub(crate) pool: SqlitePool,
    pub(crate) stats: Stats,
}

pub(crate) fn build_paths(state: State) -> Router {
    let api_doc = ApiDoc::openapi();
    let config =
        Arc::new(utoipa_swagger_ui::Config::from("/api-doc/openapi.json"));

    let cors = CorsLayer::new()
        .allow_methods([
            Method::HEAD,
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
        ])
        // .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        // Basic roots
        .route("/api/parishes", get(handlers::get_parishes))
        .route("/api/stops", get(handlers::get_stops))
        .route("/api/stops/create", post(handlers::create_stop))
        .route("/api/stops/update/:stop_id", patch(handlers::patch_stop))
        .route(
            "/api/stops/within_boundary/:x0/:y0/:x1/:y1",
            get(handlers::get_bounded_stops),
        )
        .route(
            "/api/stops/:stop_id/pictures",
            get(handlers::get_public_stop_pictures),
        )
        .route(
            "/api/stops/:stop_id/pictures/all",
            get(handlers::get_tagged_stop_pictures),
        )
        .route("/api/stops/:stop_id/spider", get(handlers::get_stop_spider))
        .route("/pictures", get(handlers::get_pictures))
        .route("/pictures/rels", get(handlers::get_picture_stop_rels))
        .route("/api/stops/spider", post(handlers::get_stops_spider))
        .route(
            "/api/routes",
            get(handlers::get_routes).post(handlers::create_route),
        )
        .route(
            "/api/routes/:route_id",
            get(handlers::get_route)
                .patch(handlers::patch_route)
                .delete(handlers::delete_route),
        )
        .route(
            "/api/routes/:route_id/create_subroute",
            post(handlers::create_subroute),
        )
        .route(
            "/api/routes/:route_id/:subroute_id",
            patch(handlers::patch_subroute).delete(handlers::delete_subroute),
        )
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
            patch(handlers::patch_stop_picture_meta)
                .delete(handlers::delete_stop_picture),
        )
        .route(
            "/tagging/stops/untagged",
            get(handlers::get_untagged_stop_pictures),
        )
        .route("/actions/import_osm", get(handlers::import_osm))
        .route("/auth/check", post(handlers::check_auth))
        .route("/stats", get(handlers::get_stats))
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

    let db_pool = SqlitePool::connect("sqlite:db.sqlite").await.expect("");
    let stats = get_stats(&db_pool).await.unwrap();
    let state = State {
        bucket,
        pool: db_pool,
        stats,
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

use crate::calendar::{Calendar, Weekday};
use crate::middleware::get_stats;
use crate::models::responses::Stats;
use models::{
    responses::{
        DateDeparture, Departure, Parish, Route, Subroute, SubrouteStops,
    },
    Stop,
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
