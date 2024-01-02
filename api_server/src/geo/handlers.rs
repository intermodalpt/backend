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

use axum::extract::{Path, State};
use axum::Json;

use commons::models::geo;

use super::sql;
use crate::{AppState, Error};

pub(crate) async fn get_regions(
    State(state): State<AppState>,
) -> Result<Json<Vec<geo::Region>>, Error> {
    Ok(Json(sql::fetch_regions(&state.pool).await?))
}

pub(crate) async fn get_parishes(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<geo::Parish>>, Error> {
    // TODO filter by region
    Ok(Json(sql::fetch_parishes(&state.pool).await?))
}
