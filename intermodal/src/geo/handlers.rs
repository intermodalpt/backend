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

use std::sync::Arc;

use axum::headers::{authorization::Bearer, Authorization};
use axum::{Extension, Json, TypedHeader};
use serde::Serialize;

use super::{models, osm, sql};
use crate::{auth, Error, State};

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
) -> Result<Json<Vec<models::Parish>>, Error> {
    Ok(Json(sql::fetch_parishes(&state.pool).await?))
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
    let _user_id = auth::get_user(auth.token(), &state.pool).await?;

    let (inserted, updated) = osm::import(&state.pool).await?;

    Ok(Json(OsmDiff { inserted, updated }))
}