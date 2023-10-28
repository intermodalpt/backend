/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

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
mod contrib;
mod errors;
mod geo;
mod gtfs;
mod misc;
mod operators;
mod pics;
mod routes;
mod stops;
mod utils;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use config::Config;
use sqlx::postgres::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use errors::Error;
use misc::models::responses::Stats;

pub(crate) type AppState = Arc<State>;

pub(crate) struct State {
    pub(crate) bucket: s3::Bucket,
    pub(crate) pool: PgPool,
    pub(crate) stats: Stats,
    pub(crate) cached: Cached,
}

struct Cached {
    gtfs_stops: RwLock<HashMap<i32, Arc<Vec<commons::models::gtfs::GTFSStop>>>>,
    tml_routes: RwLock<HashMap<i32, Arc<Vec<gtfs::models::TMLRoute>>>>,
}

pub(crate) fn build_paths(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::HEAD,
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        // .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
        .route("/v1/parishes", get(geo::handlers::get_parishes))
        .route("/v1/stops", get(stops::handlers::get_stops))
        .route("/v1/stops/full", get(stops::handlers::get_full_stops))
        .route("/v1/stops/:stop_id", get(stops::handlers::get_stop))
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
            get(pics::handlers::get_stop_pictures),
        )
        .route(
            "/v1/stops/:stop_id/pano",
            get(pics::handlers::get_stop_pano),
        )
        .route(
            "/v1/stops/:stop_id/routes",
            get(stops::handlers::get_stop_routes),
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
        .route("/v1/stop_pics/map", get(pics::handlers::get_pictures_map))
        .route(
            "/v1/stop_pics/dangling",
            get(pics::handlers::get_dangling_stop_pictures)
                .post(pics::handlers::upload_dangling_stop_picture),
        )
        .route(
            "/v1/stop_pics/latest",
            get(pics::handlers::get_latest_stop_pictures),
        )
        .route(
            "/v1/stop_pics/unpositioned",
            get(pics::handlers::get_unpositioned_stop_pictures),
        )
        .route(
            "/v1/stop_pics/linked/:stop_id",
            post(pics::handlers::upload_stop_picture),
        )
        .route("/v1/stop_pics/pano/all", get(pics::handlers::get_panos))
        .route(
            "/v1/stop_pics/pano",
            post(pics::handlers::upload_pano_picture),
        )
        .route(
            "/v1/stop_pics/pano/:pano_id/onion",
            get(pics::handlers::get_onion_skin),
        )
        .route(
            "/v1/stop_pics/:picture_id",
            get(pics::handlers::get_stop_picture_meta)
                .patch(pics::handlers::patch_stop_picture_meta)
                .delete(pics::handlers::delete_picture),
        )
        .route(
            "/v1/stop_pics/by_stop",
            get(pics::handlers::get_picture_count_by_stop),
        )
        .route(
            "/v1/issues",
            get(operators::handlers::get_issues)
                .post(operators::handlers::post_issue),
        )
        .route(
            "/v1/issues/:issue_id",
            get(operators::handlers::get_issue)
                .patch(operators::handlers::patch_issue),
        )
        .route(
            "/v1/contrib/upload/stops",
            post(pics::handlers::upload_dangling_stop_picture),
        )
        .route(
            "/v1/contrib/contributions/own/decided",
            get(contrib::handlers::get_decided_own_contributions),
        )
        .route(
            "/v1/contrib/contributions/own/undecided",
            get(contrib::handlers::get_undecided_own_contributions),
        )
        .route(
            "/v1/contrib/contributions/:user_id/decided",
            get(contrib::handlers::get_decided_user_contributions),
        )
        .route(
            "/v1/contrib/contributions/:user_id/undecided",
            get(contrib::handlers::get_undecided_user_contributions),
        )
        .route(
            "/v1/contrib/pending_stop_patch/own",
            get(contrib::handlers::get_pending_stop_patch),
        )
        .route(
            "/v1/contrib/contributions/undecided/contributors",
            get(contrib::handlers::get_undecided_contribution_contributors),
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
        .route("/v1/calendars", get(operators::handlers::get_calendars))
        .route(
            "/v1/operators/:operator_id/calendars",
            get(operators::handlers::get_operator_calendars)
                .post(operators::handlers::post_operator_calendar),
        )
        .route(
            "/v1/operators/:operator_id/calendars/date/:date",
            get(operators::handlers::get_operator_calendars_for_date),
        )
        .route(
            "/v1/operators/:operator_id/calendars/:calendar_id",
            delete(operators::handlers::delete_operator_calendar),
        )
        .route(
            "/v1/operators/:operator_id/issues",
            get(operators::handlers::get_operator_issues),
        )
        .route(
            "/v1/operators/:operator_id/stops",
            get(operators::handlers::get_operator_stops),
        )
        .route(
            "/v1/operators/:operator_id/stops/:stop_id",
            put(operators::handlers::put_operator_stop)
                .delete(operators::handlers::delete_operator_stop),
        )
        .route(
            "/v1/operators/:operator_id/gtfs/stops",
            get(gtfs::handlers::get_gtfs_stops),
        )
        .route(
            "/v1/operators/:operator_id/gtfs/stops/sliding",
            get(gtfs::handlers::get_gtfs_stop_sliding_windows),
        )
        .route(
            "/v1/operators/:operator_id/gtfs/routes",
            get(gtfs::handlers::get_gtfs_route_trips),
        )
        .route(
            "/v1/operators/:operator_id/gtfs/update",
            post(gtfs::handlers::post_update_operator_gtfs),
        )
        .route(
            "/v1/operators/:operator_id/routes",
            get(routes::handlers::get_operator_routes),
        )
        .route(
            "/v1/operators/:operator_id/news",
            get(operators::handlers::get_operator_news),
        )
        .route(
            "/v1/actions/migrate_stop/:original_id/:replacement_id",
            post(routes::handlers::post_replace_stop_across_routes),
        )
        .route("/v1/auth/login", post(auth::handlers::post_login))
        .route("/v1/auth/register", post(auth::handlers::post_register))
        .route("/v1/auth/check", get(auth::handlers::check_auth))
        .route("/v1/stats", get(misc::handlers::get_stats))
        .with_state(state)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(30 * 1024 * 1024 /* 30mb */))
        .layer(cors)
}

#[tokio::main]
async fn main() {
    env_logger::init();

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

    let _ = pics::IMG_ROOT.set(Box::leak(Box::new(
        settings.get_string("img_root").expect("img_root not set"),
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
    let state = Arc::new(State {
        bucket,
        pool,
        stats,
        cached: Cached {
            gtfs_stops: RwLock::new(HashMap::new()),
            tml_routes: RwLock::new(HashMap::new()),
        },
    });

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        settings.get_int("port").expect("port not set") as u16,
    ));

    axum::Server::bind(&addr)
        .serve(build_paths(state).into_make_service())
        .await
        .expect("Unable to start service");
}

#[derive(OpenApi)]
#[openapi(paths(), components(schemas()), tags())]
struct ApiDoc;
