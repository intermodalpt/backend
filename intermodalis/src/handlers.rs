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

use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use axum::extract::{ContentLengthLimit, Multipart, Path, Query};
use axum::headers::{authorization::Bearer, Authorization};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, TypedHeader};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa_swagger_ui::Config;

use crate::models::responses::{PublicStopPic, TaggedStopPic};
use crate::models::{
    requests,
    // This whole ordeal instead of just writing `responses::` because of uitopa
    // The macros do not support module paths
    responses::{
        DateDeparture, Departure, Parish, Route, SpiderMap, SubrouteStops,
        UntaggedStopPic,
    },
    Stop,
};
use crate::{middleware, osm, sql, Error, State};

#[utoipa::path(
    get,
    path = "/v1/parishes",
    responses(
        (
            status = 200,
            description = "List of parishes",
            body = [Parish])
    )
)]
pub(crate) async fn get_parishes(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Vec<Parish>>, Error> {
    Ok(Json(sql::fetch_parishes(&state.pool).await?))
}

#[derive(Deserialize)]
pub(crate) struct StopQueryParam {
    #[serde(default)]
    all: bool,
}

#[utoipa::path(
    get,
    path = "/v1/stops",
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
) -> Result<Json<Vec<Stop>>, Error> {
    Ok(Json(sql::fetch_stops(&state.pool, !params.all).await?))
}

pub(crate) async fn create_stop(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(stop): Json<requests::NewStop>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    // FIXME
    if user_id != 1 {
        return Err(Error::Forbidden);
    }

    Ok(Json({
        let mut map = HashMap::new();
        map.insert(
            "id".to_string(),
            sql::insert_stop(&state.pool, stop, user_id).await?,
        );
        map
    }))
}

pub(crate) async fn patch_stop(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(stop): Json<requests::ChangeStop>,
    Path(stop_id): Path<i32>,
) -> Result<(), Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    sql::update_stop(&state.pool, stop_id, stop, user_id).await
}

#[utoipa::path(
    get,
    path = "/v1/stops/{x0}/{y0}/{x1}/{y1}",
    responses(
        (
            status = 200,
            description = "List of stops that fit within a boundary",
            body = [Stop])
    )
)]
pub(crate) async fn get_bounded_stops(
    Extension(state): Extension<Arc<State>>,
    Path(boundary): Path<(f64, f64, f64, f64)>,
) -> Result<Json<Vec<Stop>>, Error> {
    Ok(Json(sql::fetch_bounded_stops(&state.pool, boundary).await?))
}

pub(crate) async fn get_public_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i32>,
) -> Result<Json<Vec<PublicStopPic>>, Error> {
    Ok(Json(
        sql::fetch_public_stop_pictures(&state.pool, stop_id).await?,
    ))
}

pub(crate) async fn get_tagged_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i32>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<TaggedStopPic>>, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    Ok(Json(
        sql::fetch_tagged_stop_pictures(&state.pool, stop_id).await?,
    ))
}

pub(crate) async fn get_picture_stop_rels(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<HashMap<i32, Vec<i32>>>, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    Ok(Json(sql::fetch_picture_stop_rels(&state.pool).await?))
}

pub(crate) async fn get_pictures(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<TaggedStopPic>>, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    Ok(Json(sql::fetch_stop_pictures(&state.pool).await?))
}

#[utoipa::path(get, path = "/v1/stops/{stop_id}/spider")]
pub(crate) async fn get_stop_spider(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i32>,
) -> Result<Json<SpiderMap>, Error> {
    get_stops_spider(Extension(state), Json(vec![stop_id])).await
}

pub(crate) async fn get_stops_spider(
    Extension(state): Extension<Arc<State>>,
    Json(stops): Json<Vec<i32>>,
) -> Result<Json<SpiderMap>, Error> {
    Ok(Json(sql::fetch_stop_spider(&state.pool, &stops).await?))
}

#[utoipa::path(
    get,
    path = "/v1/routes",
    responses(
        (status = 200, description = "List of routes", body = [Route]),
    )
)]
pub(crate) async fn get_routes(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Vec<Route>>, Error> {
    let routes = sql::fetch_routes(&state.pool).await?;
    Ok(Json(routes))
}

pub(crate) async fn create_route(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(route): Json<requests::ChangeRoute>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let id = sql::insert_route(&state.pool, route).await?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn get_route(
    Extension(state): Extension<Arc<State>>,
    Path(route_id): Path<i32>,
) -> Result<Json<Route>, Error> {
    if let Some(route) = sql::fetch_route(&state.pool, route_id).await? {
        Ok(Json(route))
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn patch_route(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(route_id): Path<i32>,
    Json(changes): Json<requests::ChangeRoute>,
) -> Result<(), Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    sql::update_route(&state.pool, route_id, changes).await
}

pub(crate) async fn delete_route(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(route_id): Path<i32>,
) -> Result<(), Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    sql::delete_route(&state.pool, route_id).await
}

pub(crate) async fn create_subroute(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(route_id): Path<i32>,
    Json(subroute): Json<requests::ChangeSubroute>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let id = sql::insert_subroute(&state.pool, route_id, subroute).await?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn patch_subroute(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path((route_id, subroute_id)): Path<(i32, i32)>,
    Json(changes): Json<requests::ChangeSubroute>,
) -> Result<(), Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    sql::update_subroute(&state.pool, route_id, subroute_id, changes).await
}

pub(crate) async fn delete_subroute(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path((route_id, subroute_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let stop_count =
        sql::fetch_subroute_stop_count(&state.pool, subroute_id).await?;
    let departure_count =
        sql::fetch_subroute_departure_count(&state.pool, subroute_id).await?;

    if stop_count > 0 || departure_count > 0 {
        return Err(Error::DependenciesNotMet);
    }

    sql::delete_subroute(&state.pool, route_id, subroute_id).await
}

#[utoipa::path(
    get,
    path = "/v1/routes/{route_id}/schedule",
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
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<Departure>>, Error> {
    Ok(Json(sql::fetch_schedule(&state.pool, route_id).await?))
}

#[utoipa::path(
    get,
    path = "/v1/routes/{route_id}/schedule/{date}",
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
    Path((route_id, date)): Path<(i32, String)>,
) -> Result<Json<Vec<DateDeparture>>, Error> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    Ok(Json(
        sql::fetch_schedule_for_date(&state.pool, route_id, date).await?,
    ))
}

pub(crate) async fn create_subroute_departure(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(departure): Json<requests::ChangeDeparture>,
    Path(subroute_id): Path<i32>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let id = sql::insert_departure(&state.pool, subroute_id, departure).await?;
    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn patch_subroute_departure(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(departure): Json<requests::ChangeDeparture>,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    sql::update_departure(&state.pool, subroute_id, departure_id, departure)
        .await?;
    Ok(())
}

pub(crate) async fn delete_subroute_departure(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    sql::delete_departure(&state.pool, subroute_id, departure_id).await?;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/v1/routes/{route_id}/stops",
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
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<SubrouteStops>>, Error> {
    Ok(Json(sql::fetch_route_stops(&state.pool, route_id).await?))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub(crate) async fn patch_subroute_stops(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path((route_id, subroute_id)): Path<(i32, i32)>,
    Json(request): Json<requests::ChangeSubrouteStops>,
) -> Result<(), Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    if user_id != 1 && user_id != 2 {
        return Err(Error::Forbidden);
    }

    Ok(
        sql::update_subroute_stops(&state.pool, route_id, subroute_id, request)
            .await?,
    )
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
) -> Result<Json<Vec<UntaggedStopPic>>, Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_untagged_stop_pictures(&state.pool, user_id, offset, take)
            .await?,
    ))
}

pub(crate) async fn upload_stop_picture(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        { 500 * 1024 * 1024 },
    >,
) -> Result<(), Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?
    {
        let filename = field
            .file_name()
            .ok_or_else(|| {
                Error::ValidationFailure("File without a filename".to_string())
            })?
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
    }

    Ok(())
}

pub(crate) async fn patch_stop_picture_meta(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(stop_picture_id): Path<i32>,
    Json(stop_pic_meta): Json<requests::ChangeStopPic>,
) -> Result<(), Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    sql::update_stop_picture_meta(
        &state.pool,
        stop_picture_id,
        stop_pic_meta,
        user_id,
    )
    .await
}

pub(crate) async fn delete_stop_picture(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(stop_picture_id): Path<i32>,
) -> Result<(), Error> {
    let user_id = middleware::get_user(auth.token(), &state.pool).await?;

    if user_id != 1 && user_id != 2 {
        return Err(Error::Forbidden);
    }

    middleware::delete_stop_picture(stop_picture_id, &state.bucket, &state.pool)
        .await
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
    Json(&state.stats).into_response()
}

#[derive(Serialize)]
pub(crate) struct OsmDiff {
    inserted: usize,
    updated: usize,
}

pub(crate) async fn import_osm(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<OsmDiff>, Error> {
    let _user_id = middleware::get_user(auth.token(), &state.pool).await?;

    let (inserted, updated) = osm::import(&state.pool).await?;

    Ok(Json(OsmDiff { inserted, updated }))
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
