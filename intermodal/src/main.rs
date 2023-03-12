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

#![warn(
    nonstandard_style,
    warnings,
    unused,
    future_incompatible,
    clippy::all,
    clippy::pedantic
)]

mod auth;
mod calendar;
mod contrib;
mod errors;
mod geo;
mod misc;
mod operators;
mod pics;
mod routes;
mod stops;
mod utils;

use errors::Error;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use axum::routing::{delete, get, patch, post};
use axum::{Extension, Json, Router};
use config::Config;
use sqlx::postgres::PgPool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) bucket: s3::Bucket,
    pub(crate) pool: PgPool,
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
        .route("/v1/parishes", get(geo::handlers::get_parishes))
        .route("/v1/stops", get(stops::handlers::get_stops))
        .route("/v1/stops/create", post(stops::handlers::create_stop))
        .route(
            "/v1/stops/update/:stop_id",
            patch(stops::handlers::patch_stop),
        )
        .route(
            "/v1/stops/within_boundary/:x0/:y0/:x1/:y1",
            get(stops::handlers::get_bounded_stops),
        )
        .route(
            "/v1/stops/:stop_id/pictures",
            get(pics::handlers::get_public_stop_pictures),
        )
        .route(
            "/v1/stops/:stop_id/pictures/all",
            get(pics::handlers::get_tagged_stop_pictures),
        )
        .route(
            "/v1/stops/:stop_id/spider",
            get(stops::handlers::get_stop_spider),
        )
        .route("/v1/pictures", get(pics::handlers::get_pictures))
        .route(
            "/v1/pictures/rels",
            get(pics::handlers::get_picture_stop_rels),
        )
        .route("/v1/stops/spider", post(stops::handlers::get_stops_spider))
        .route(
            "/v1/routes",
            get(routes::handlers::get_routes)
                .post(routes::handlers::create_route),
        )
        .route(
            "/v1/routes/:route_id",
            get(routes::handlers::get_route)
                .patch(routes::handlers::patch_route)
                .delete(routes::handlers::delete_route),
        )
        .route(
            "/v1/routes/:route_id/create_subroute",
            post(routes::handlers::create_subroute),
        )
        .route(
            "/v1/routes/:route_id/:subroute_id",
            patch(routes::handlers::patch_subroute)
                .delete(routes::handlers::delete_subroute),
        )
        .route(
            "/v1/routes/:route_id/schedule",
            get(routes::handlers::get_schedule),
        )
        .route(
            "/v1/schedules/:subroute_id",
            post(routes::handlers::create_subroute_departure),
        )
        .route(
            "/v1/schedules/:subroute_id/:departure_id",
            patch(routes::handlers::patch_subroute_departure)
                .delete(routes::handlers::delete_subroute_departure),
        )
        .route(
            "/v1/routes/:route_id/stops",
            get(routes::handlers::get_route_stops),
        )
        .route(
            "/v1/routes/:route_id/stops/subroutes/:subroute_id",
            patch(routes::handlers::patch_subroute_stops),
        )
        .route(
            "/v1/upload/stops",
            post(pics::handlers::upload_stop_picture),
        )
        .route(
            "/v1/upload/stops/:picture_id",
            patch(pics::handlers::patch_stop_picture_meta)
                .delete(pics::handlers::delete_stop_picture),
        )
        .route(
            "/v1/tagging/stops/untagged",
            get(pics::handlers::get_untagged_stop_pictures),
        )
        .route(
            "/v1/contrib/upload/stops",
            post(pics::handlers::upload_stop_picture),
        )
        .route(
            "/v1/contrib/changelog/contributions",
            get(contrib::handlers::get_user_contributions),
        )
        .route(
            "/v1/contrib/contributions/undecided",
            get(contrib::handlers::get_latest_undecided_contributions),
        )
        .route(
            "/v1/contrib/contributions/decided",
            get(contrib::handlers::get_latest_decided_contributions),
        )
        .route(
            "/v1/contrib/changelog",
            get(contrib::handlers::get_changelog),
        )
        .route(
            "/v1/contrib/pics",
            post(contrib::handlers::post_contrib_stop_picture),
        )
        .route(
            "/v1/contrib/pics/:contribution_id",
            patch(contrib::handlers::patch_contrib_stop_picture_meta),
        )
        .route(
            "/v1/contrib/stops/update/:stop_id",
            post(contrib::handlers::post_contrib_stop_data),
        )
        .route(
            "/v1/contrib/:contribution_id/accept",
            post(contrib::handlers::post_accept_contrib_data),
        )
        .route(
            "/v1/contrib/:contribution_id/decline",
            post(contrib::handlers::post_decline_contrib_data),
        )
        .route("/v1/news", get(operators::handlers::get_news))
        .route("/v1/operators", get(operators::handlers::get_operators))
        .route(
            "/v1/calendars",
            get(operators::handlers::get_calendars)
        )
        .route(
            "/v1/operators/:operator_id/calendars",
            get(operators::handlers::get_operator_calendars)
                .post(operators::handlers::post_operator_calendar),
        )
        .route(
            "/v1/operators/:operator_id/calendars/:operator_id",
            delete(operators::handlers::delete_operator_calendar),
        )
        .route(
            "/v1/operators/:operator_id/news",
            get(operators::handlers::get_operator_news),
        )
        .route("/v1/actions/import_osm", get(geo::handlers::import_osm))
        .route("/v1/auth/login", post(auth::handlers::post_login))
        .route("/v1/auth/register", post(auth::handlers::post_register))
        .route("/v1/auth/check", get(auth::handlers::check_auth))
        .route("/v1/stats", get(misc::handlers::get_stats))
        .layer(Extension(Arc::new(state)))
        .route(
            "/api-doc/openapi.json",
            get(move || async { Json(api_doc) }),
        )
        .route(
            "/docs/*tail",
            get(misc::handlers::serve_swagger_ui).layer(Extension(config)),
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

    let _ = auth::SECRET_KEY.set(Box::leak(Box::new(
        settings
            .get_string("jwt_secret")
            .expect("jwt_secret not set"),
    )));

    let credentials = s3::creds::Credentials::new(
        Some(
            &settings
                .get_string("s3_access_key")
                .expect("s3_access_key not set"),
        ),
        Some(
            &settings
                .get_string("s3_secret_key")
                .expect("s3_secret_key not set"),
        ),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket = s3::Bucket::new(
        &settings
            .get_string("s3_bucket_name")
            .expect("s3_bucket_name not set"),
        s3::Region::R2 {
            account_id: settings
                .get_string("s3_account_id")
                .expect("s3_account_id not set"),
        },
        credentials,
    )
    .unwrap()
    .with_path_style();

    let pool = PgPool::connect(&settings.get_string("db").expect("db not set"))
        .await
        .expect("Unable to connect to the database");
    let stats = misc::sql::get_stats(&pool).await.unwrap();
    let state = State {
        bucket,
        pool,
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

use calendar::models::{Calendar, Weekday};
use geo::models::Parish;
use misc::models::responses::Stats;
use routes::models::responses::{
    DateDeparture, Departure, Route, Subroute, SubrouteStops,
};
use stops::models::Stop;

#[derive(OpenApi)]
#[openapi(
    handlers(
        geo::handlers::get_parishes,
        stops::handlers::get_stops,
        routes::handlers::get_routes,
        routes::handlers::get_schedule,
        routes::handlers::get_route_stops,
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
