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

use axum::extract::State;
use axum::Json;
use serde::Serialize;

use commons::models::geo;

use super::sql;
use crate::{AppState, Error};

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
    State(state): State<AppState>,
) -> Result<Json<Vec<geo::Parish>>, Error> {
    Ok(Json(sql::fetch_parishes(&state.pool).await?))
}

#[derive(Serialize)]
pub(crate) struct OsmDiff {
    inserted: usize,
    updated: usize,
}
