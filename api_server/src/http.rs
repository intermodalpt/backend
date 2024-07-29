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

use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Request};
use axum::http::{header, Method};
use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use axum_client_ip::SecureClientIpSource;
use axum_extra::headers::{authorization::Bearer, Authorization};
use headers::Header;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::{self, TraceLayer};

use crate::state::AppState;
use crate::{
    auth, contrib, geo, gtfs, info, operators, osm, pics, routes, stops,
};

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
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_origin(Any);

    Router::new()
        .route("/v1/regions", get(geo::handlers::get_regions))
        .route("/v1/regions/:region_id", get(geo::handlers::get_region))
        .route(
            "/v1/regions/:region_id/parishes",
            get(geo::handlers::get_parishes),
        )
        .route(
            "/v1/regions/:region_id/stops",
            get(stops::handlers::get_region_stops),
        )
        .route(
            "/v1/regions/:region_id/stops/detailed",
            get(stops::handlers::get_region_detailed_stops),
        )
        .route(
            "/v1/regions/:region_id/stops/full",
            get(stops::handlers::get_region_full_stops),
        )
        .route(
            "/v1/regions/:region_id/stops/todo",
            get(stops::handlers::get_region_todo),
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
            "/v1/regions/:region_id/map_features",
            get(osm::handlers::get_region_stops_map_features),
        )
        .route(
            "/v1/regions/:region_id/news",
            get(info::handlers::get_region_news),
        )
        .route(
            "/v1/stops",
            get(stops::handlers::get_all_stops)
                .post(stops::handlers::post_stop),
        )
        .route(
            "/v1/stops/detailed",
            get(stops::handlers::get_all_detailed_stops),
        )
        .route(
            "/v1/stops/map_features",
            get(osm::handlers::get_stops_map_features),
        )
        .route(
            "/v1/stops/:stop_id",
            get(stops::handlers::get_stop).patch(stops::handlers::patch_stop),
        )
        .route(
            "/v1/stops/within_boundary/:x0/:y0/:x1/:y1",
            get(stops::handlers::get_bounded_stops),
        )
        .route(
            "/v1/stops/:stop_id/position",
            put(stops::handlers::put_stop_position),
        )
        .route(
            "/v1/stops/:stop_id/osm",
            get(osm::handlers::get_paired_osm_stop),
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
            "/v1/stops/:stop_id/regions",
            get(geo::handlers::get_stop_regions),
        )
        .route(
            "/v1/stops/:stop_id/parish/:parish_id",
            put(geo::handlers::put_stop_parish),
        )
        .route(
            "/v1/stops/:stop_id/map_features",
            put(osm::handlers::put_stop_map_features),
        )
        .route(
            "/v1/stops/:stop_id/todo",
            put(stops::handlers::put_stop_todo),
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
        .route("/v1/routes", post(routes::handlers::post_route))
        .route(
            "/v1/routes/:route_id",
            get(routes::handlers::get_route)
                .patch(routes::handlers::patch_route)
                .delete(routes::handlers::delete_route),
        )
        .route(
            "/v1/routes/:route_id/full",
            get(routes::handlers::get_route_full),
        )
        .route(
            "/v1/routes/:route_id/subroutes",
            post(routes::handlers::create_subroute),
        )
        .route(
            "/v1/routes/:route_id/schedule",
            get(routes::handlers::get_schedule),
        )
        .route(
            "/v1/routes/:route_id/regions",
            get(geo::handlers::get_route_regions),
        )
        .route(
            "/v1/routes/:route_id/validation",
            get(gtfs::handlers::get_route_validation_data)
                .patch(gtfs::handlers::patch_route_validation_data),
        )
        .route(
            "/v1/routes/:route_id/assign_unmatched_validation",
            post(gtfs::handlers::post_assign_subroute_validation),
        )
        .route(
            "/v1/routes/:route_id/stops", // <- TODO change URL
            get(routes::handlers::get_subroute_stops),
        )
        .route(
            "/v1/routes/:route_id/stops/full", // <- TODO simplify url, these are not full stops
            get(stops::handlers::get_route_stops),
        )
        .route(
            "/v1/subroutes/:subroute_id",
            patch(routes::handlers::patch_subroute)
                .delete(routes::handlers::delete_subroute),
        )
        .route(
            "/v1/subroutes/:subroute_id/stops",
            patch(routes::handlers::patch_subroute_stops),
        )
        .route(
            "/v1/subroutes/:subroute_id/validation/current_ack",
            post(gtfs::handlers::post_subroute_validation_current_ack),
        )
        .route(
            "/v1/subroutes/:subroute_id/validation/correspondence_ack",
            post(gtfs::handlers::post_subroute_validation_correspondence_ack),
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
        .route(
            "/v1/operators",
            get(operators::handlers::get_operators)
                .post(operators::handlers::post_operator),
        )
        .route(
            "/v1/operators/:operator_id",
            get(operators::handlers::get_operator)
                .patch(operators::handlers::patch_operator),
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
            "/v1/operators/:operator_id/stop_rels",
            get(operators::handlers::get_operator_stop_rels),
        )
        .route(
            "/v1/operators/:operator_id/stops",
            get(stops::handlers::get_operator_stops),
        )
        .route(
            "/v1/operators/:operator_id/stops/full",
            get(stops::handlers::get_operator_full_stops),
        )
        .route(
            "/v1/operators/:operator_id/stop_by_ref/:stop_ref",
            get(stops::handlers::get_stop_by_operator_ref),
        )
        .route(
            "/v1/operators/:operator_id/stops/:stop_id",
            put(operators::handlers::put_operator_stop)
                .delete(operators::handlers::delete_operator_stop),
        )
        .route(
            "/v1/operators/:operator_id/routes",
            get(routes::handlers::get_operator_routes),
        )
        .route(
            "/v1/operators/:operator_id/routes/full",
            get(routes::handlers::get_operator_full_routes),
        )
        .route(
            "/v1/operators/:operator_id/routes/types",
            get(operators::handlers::get_operator_route_types)
                .post(operators::handlers::post_operator_route_type),
        )
        .route(
            "/v1/operators/:operator_id/routes/types/:type_id",
            patch(operators::handlers::patch_operator_route_type)
                .delete(operators::handlers::delete_operator_route_type),
        )
        .route(
            "/v1/operators/:operator_id/issues",
            get(operators::handlers::get_operator_issues),
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
            "/v1/operators/:operator_id/regions",
            get(geo::handlers::get_operator_regions),
        )
        .route(
            "/v1/operators/:operator_id/news",
            get(info::handlers::get_operator_news),
        )
        .route(
            "/v1/operators/:operator_id/external_news",
            get(info::handlers::get_operator_external_news),
        )
        .route(
            "/v1/operators/:operator_id/external_news/pending",
            get(info::handlers::get_operator_pending_external_news),
        )
        .route(
            "/v1/operators/:operator_id/validation",
            get(gtfs::handlers::get_operator_validation_data)
                .patch(gtfs::handlers::patch_operator_validation_data),
        )
        .route(
            "/v1/operators/:operator_id/logo",
            post(pics::handlers::post_upload_operator_logo),
        )
        .route("/v1/calendars", get(operators::handlers::get_calendars))
        .route(
            "/v1/news",
            get(info::handlers::get_news).post(info::handlers::post_news_item),
        )
        .route("/v1/news/images", post(pics::handlers::post_news_image))
        .route(
            "/v1/news/images/:image_id",
            patch(pics::handlers::patch_news_image_meta),
        )
        .route(
            "/v1/news/:item_id",
            get(info::handlers::get_news_item)
                .patch(info::handlers::patch_news_item),
        )
        .route(
            "/v1/news/:item_id/full",
            get(info::handlers::get_full_news_item),
        )
        .route(
            "/v1/news/images/import_external/:external_image_id",
            post(pics::handlers::post_import_external_news_image),
        )
        .route(
            "/v1/news/external",
            get(info::handlers::get_external_news)
                .post(info::handlers::post_external_news_item),
        )
        .route(
            "/v1/news/external/:item_id",
            get(info::handlers::get_external_news_item)
                .patch(info::handlers::patch_external_news_item)
                .delete(info::handlers::delete_external_news_item),
        )
        .route(
            "/v1/news/external/:item_id/images",
            post(pics::handlers::post_external_news_image),
        )
        .route(
            "/v1/news/external/:item_id/screenshot",
            put(pics::handlers::put_external_news_screenshot),
        )
        .route(
            "/v1/news/external/:item_id/full",
            get(info::handlers::get_full_external_news_item),
        )
        .route(
            "/v1/news/external/pending",
            get(info::handlers::get_pending_external_news),
        )
        .route(
            "/v1/news/external/:source/known_urls",
            get(info::handlers::get_external_news_source_known_urls),
        )
        .route(
            "/v1/news/external/:source/dump",
            get(info::handlers::get_external_news_source_dump),
        )
        .route(
            "/v1/osm/stops",
            get(osm::handlers::get_osm_stops)
                .patch(osm::handlers::patch_osm_stops),
        )
        .route(
            "/v1/osm/stops/:id",
            get(osm::handlers::get_osm_stop_history)
                .delete(osm::handlers::delete_osm_stop),
        )
        .route(
            "/v1/osm/stops/:id/paired",
            get(stops::handlers::get_osm_paired_stop),
        )
        .route(
            "/v1/osm/stops/versions",
            get(osm::handlers::get_osm_stop_versions),
        )
        .route(
            "/v1/actions/migrate_stop/:original_id/:replacement_id",
            post(routes::handlers::post_replace_stop_across_routes),
        )
        .route("/v1/auth/login", post(auth::handlers::post_login))
        .route(
            "/v1/auth/renew",
            get(auth::handlers::get_renew_access_token),
        )
        .route("/v1/auth/register", post(auth::handlers::post_register))
        .route(
            "/v1/auth/mtokens",
            get(auth::handlers::get_management_tokens)
                .post(auth::handlers::post_create_management_token),
        )
        .route(
            "/v1/auth/mtokens/:token_id",
            delete(auth::handlers::delete_revoke_management_token),
        )
        .route("/v1/auth/get_captcha", get(auth::handlers::get_captcha))
        .route(
            "/v1/auth/register/username_check",
            post(auth::handlers::post_username_availability),
        )
        .route("/v1/auth/check", get(auth::handlers::check_auth))
        .route(
            "/v1/auth/permissions/user/:user_id",
            get(auth::handlers::get_user_permissions)
                .post(auth::handlers::post_assign_user_permissions),
        )
        .route(
            "/v1/auth/permissions/assignment/:assignment_id",
            delete(auth::handlers::delete_user_permissions),
        )
        .route(
            "/v1/user/change_password",
            post(auth::handlers::post_user_change_password),
        )
        .route(
            "/v1/admin/change_password",
            post(auth::handlers::post_admin_change_password),
        )
        .route("/v1/admin/audit/log", post(auth::handlers::get_audit_log))
        .route(
            "/v1/admin/audit/log/user/:user_id",
            post(auth::handlers::get_user_audit_log),
        )
        .route(
            "/v1/admin/audit/user/:user_id/sessions",
            post(auth::handlers::get_user_sessions),
        )
        .route(
            "/v1/admin/audit/sessions/:session_id/accesses",
            post(auth::handlers::get_session_accesses),
        )
        .with_state(state)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(30 * 1024 * 1024 /* 30mb */))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    type JwtH = Authorization<Bearer>;

                    let mut jwt_headers =
                        request.headers().get_all(JwtH::name()).iter();
                    // We're possibly decoding claims twice (here and in the handlers).
                    // I don't know if this has any measurable performance impact
                    // Maybe we could somehow have a layer to make it available here
                    // and for the handlers
                    let claim_ids = JwtH::decode(&mut jwt_headers)
                        .ok()
                        .and_then(|Authorization(bearer)| {
                            if let Ok(claims) =
                                auth::jwt::decode_access_claims(bearer.token())
                            {
                                Some((claims.uid, claims.jti))
                            } else if let Ok(claims) =
                                auth::jwt::decode_refresh_claims(bearer.token())
                            {
                                Some((claims.uid, claims.jti))
                            } else {
                                None
                            }
                        });

                    let request_id = uuid::Uuid::new_v4();

                    if let Some((uid, jti)) = claim_ids {
                        tracing::span!(
                            tracing::Level::INFO,
                            "req",
                            method = display(request.method()),
                            uri = display(request.uri()),
                            version = debug(request.version()),
                            uid = display(uid),
                            jti = display(jti),
                            request_id = display(request_id)
                        )
                    } else {
                        tracing::span!(
                            tracing::Level::INFO,
                            "req",
                            method = display(request.method()),
                            uri = display(request.uri()),
                            version = debug(request.version()),
                            request_id = display(request_id)
                        )
                    }
                })
                .on_response(
                    trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        )
}
