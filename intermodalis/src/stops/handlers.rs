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

use axum::extract::{Path, Query};
use axum::headers::{authorization::Bearer, Authorization};
use axum::{Extension, Json, TypedHeader};
use serde::Deserialize;

use super::{models, sql};
use crate::{auth, Error, State};

#[derive(Deserialize)]
pub(crate) struct StopQueryParam {
    #[serde(default)]
    all: bool,
}

#[utoipa::path(
    get,
    path = "/v1/stops",
    responses(
        (status = 200, description = "List of stops", body = [Stop])
    )
)]
pub(crate) async fn get_stops(
    Extension(state): Extension<Arc<State>>,
    params: Query<StopQueryParam>,
) -> Result<Json<Vec<models::Stop>>, Error> {
    Ok(Json(sql::fetch_stops(&state.pool, !params.all).await?))
}

pub(crate) async fn create_stop(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(stop): Json<models::requests::NewStop>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;

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
    Json(stop): Json<models::requests::ChangeStop>,
    Path(stop_id): Path<i32>,
) -> Result<(), Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;

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
) -> Result<Json<Vec<models::Stop>>, Error> {
    Ok(Json(sql::fetch_bounded_stops(&state.pool, boundary).await?))
}

#[utoipa::path(get, path = "/v1/stops/{stop_id}/spider")]
pub(crate) async fn get_stop_spider(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i32>,
) -> Result<Json<models::responses::SpiderMap>, Error> {
    get_stops_spider(Extension(state), Json(vec![stop_id])).await
}

pub(crate) async fn get_stops_spider(
    Extension(state): Extension<Arc<State>>,
    Json(stops): Json<Vec<i32>>,
) -> Result<Json<models::responses::SpiderMap>, Error> {
    Ok(Json(sql::fetch_stop_spider(&state.pool, &stops).await?))
}
