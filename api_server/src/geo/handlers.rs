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
use std::collections::HashMap;

use commons::models::geo;

use super::sql;
use crate::{AppState, Error};

pub(crate) async fn get_regions(
    State(state): State<AppState>,
) -> Result<Json<Vec<geo::Region>>, Error> {
    Ok(Json(sql::fetch_regions(&state.pool).await?))
}

pub(crate) async fn put_operator_into_region(
    State(state): State<AppState>,
    Path((region_id, operator_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::upsert_operator_into_region(&mut transaction, region_id, operator_id)
        .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn delete_operator_from_region(
    State(state): State<AppState>,
    Path((region_id, operator_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::delete_operator_from_region(&mut transaction, region_id, operator_id)
        .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn put_route_into_region(
    State(state): State<AppState>,
    Path((region_id, route_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::upsert_route_into_region(&mut transaction, region_id, route_id)
        .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn delete_route_from_region(
    State(state): State<AppState>,
    Path((region_id, route_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::delete_route_from_region(&mut transaction, region_id, route_id)
        .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn put_stop_into_region(
    State(state): State<AppState>,
    Path((region_id, stop_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::upsert_stop_into_region(&mut transaction, region_id, stop_id).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn delete_stop_from_region(
    State(state): State<AppState>,
    Path((region_id, stop_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::delete_stop_from_region(&mut transaction, region_id, stop_id).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn get_region_stops_osm_quality(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<HashMap<i32, Option<bool>>>, Error> {
    Ok(Json(
        sql::fetch_region_osm_quality(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn get_parishes(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<geo::Parish>>, Error> {
    // TODO filter by region
    Ok(Json(sql::fetch_parishes(&state.pool).await?))
}

pub(crate) async fn put_stop_parish(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    Path(parish_id): Path<i32>,
) -> Result<(), Error> {
    sql::update_stop_parish(&state.pool, stop_id, parish_id).await?;
    Ok(())
}
