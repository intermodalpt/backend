/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023 - 2024  Cláudio Pereira

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

use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use axum_client_ip::SecureClientIpSource;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::AppState;
use crate::{auth, contrib, geo, gtfs, misc, operators, pics, routes, stops};

#[allow(clippy::too_many_lines)]
pub fn build_paths(state: AppState) -> Router {
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
        .route("/v1/regions", get(geo::handlers::get_regions))
        .route(
            "/v1/regions/:region_id/parishes",
            get(geo::handlers::get_parishes),
        )
        .route(
            "/v1/regions/:region_id/stops",
            get(stops::handlers::get_stops),
        )
        .route(
            "/v1/regions/:region_id/stops/detailed",
            get(stops::handlers::get_detailed_stops),
        )
        .route(
            "/v1/regions/:region_id/stops/full",
            get(stops::handlers::get_full_stops),
        )
        .route(
            "/v1/regions/:region_id/stops/:stop_id",
            put(geo::handlers::put_stop_into_region)
                .delete(geo::handlers::delete_stop_from_region),
        )
        .route(
            "/v1/regions/:region_id/operators/:operator_id",
            put(geo::handlers::put_operator_into_region)
                .delete(geo::handlers::delete_operator_from_region),
        )
        .route(
            "/v1/regions/:region_id/routes",
            get(routes::handlers::get_routes),
        )
        .route(
            "/v1/regions/:region_id/routes/full",
            get(routes::handlers::get_full_routes),
        )
        .route(
            "/v1/regions/:region_id/routes/:route_id",
            put(geo::handlers::put_route_into_region)
                .delete(geo::handlers::delete_route_from_region),
        )
        .route(
            "/v1/regions/:region_id/osm_stops_quality",
            get(geo::handlers::get_region_stops_osm_quality),
        )
        .route("/v1/stops/:stop_id", get(stops::handlers::get_stop))
        .route("/v1/stops/create", post(stops::handlers::create_stop))
        .route("/v1/stops/all", get(stops::handlers::get_all_stops))
        .route(
            "/v1/stops/osm_meta",
            get(stops::handlers::get_stops_osm_meta),
        )
        .route(
            "/v1/stops/update/:stop_id",
            patch(stops::handlers::patch_stop),
        )
        .route(
            "/v1/stops/within_boundary/:x0/:y0/:x1/:y1",
            get(stops::handlers::get_bounded_stops),
        )
        .route(
            "/v1/stops/:stop_id/osm_meta",
            get(stops::handlers::get_stop_osm_meta)
                .patch(stops::handlers::patch_stop_osm_meta),
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
            "/v1/stops/:stop_id/parish/:parish_id",
            put(geo::handlers::put_stop_parish),
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
        .route("/v1/routes", post(routes::handlers::create_route))
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
            "/v1/routes/:route_id/validation",
            get(gtfs::handlers::get_route_validation_data)
                .put(gtfs::handlers::put_route_validation_data),
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
            post(pics::handlers::upload_dangling_stop_picture),
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
        .route(
            "/v1/stop_pics/uploaded_by/:user_id",
            post(pics::handlers::get_user_stop_pictures),
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
        .route(
            "/v1/operators",
            get(operators::handlers::get_operators)
                .post(operators::handlers::post_operator),
        )
        .route(
            "/v1/operators/:operator_id",
            patch(operators::handlers::patch_operator),
        )
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
            "/v1/operators/:operator_id/validation",
            get(gtfs::handlers::get_operator_validation_data)
                .put(gtfs::handlers::put_operator_validation_data),
        )
        .route(
            "/v1/operators/:operator_id/logo",
            post(pics::handlers::post_upload_operator_logo),
        )
        .route("/v1/calendars", get(operators::handlers::get_calendars))
        .route(
            "/v1/actions/migrate_stop/:original_id/:replacement_id",
            post(routes::handlers::post_replace_stop_across_routes),
        )
        .route("/v1/auth/login", post(auth::handlers::post_login))
        .route("/v1/auth/register", post(auth::handlers::post_register))
        .route("/v1/auth/check", get(auth::handlers::check_auth))
        .route("/v1/stats", get(misc::handlers::get_stats))
        .route(
            "/v1/user/change_password",
            post(auth::handlers::post_user_change_password),
        )
        .route(
            "/v1/admin/change_password",
            post(auth::handlers::post_admin_change_password),
        )
        .route("/v1/admin/audit_log", post(auth::handlers::get_audit_log))
        .route(
            "/v1/admin/audit_log/user/:user_id",
            post(auth::handlers::get_user_audit_log),
        )
        .with_state(state)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(30 * 1024 * 1024 /* 30mb */))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(cors)
}

#[derive(OpenApi)]
#[openapi(paths(), components(schemas()), tags())]
struct ApiDoc;
