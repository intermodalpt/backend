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
use futures::future;

use commons::models::geo;

use super::{models::responses, sql};
use crate::auth;
use crate::operators::sql as operators_sql;
use crate::{AppState, Error};

pub(crate) async fn get_regions(
    State(state): State<AppState>,
) -> Result<Json<Vec<geo::Region>>, Error> {
    Ok(Json(sql::fetch_regions(&state.pool).await?))
}

pub(crate) async fn get_region(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<responses::RegionWithOperators>, Error> {
    let (region, region_operators) = future::join(
        sql::fetch_region(&state.pool, region_id),
        operators_sql::fetch_region_operators(&state.pool, region_id),
    )
    .await;

    let region = region?.ok_or(Error::NotFoundUpstream)?;
    let region_operators = region_operators?;

    let region_with_operators = responses::RegionWithOperators {
        id: region.id,
        name: region.name,
        geometry: region.geometry,
        center_lat: region.center_lat,
        center_lon: region.center_lon,
        zoom: region.zoom,
        operators: region_operators,
    };
    Ok(Json(region_with_operators))
}

pub(crate) async fn get_operator_regions(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<i32>>, Error> {
    Ok(Json(
        sql::fetch_operator_regions(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn put_operator_into_region(
    State(state): State<AppState>,
    Path((region_id, operator_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyOperatorMeta>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::upsert_operator_into_region(&mut transaction, region_id, operator_id)
        .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) async fn delete_operator_from_region(
    State(state): State<AppState>,
    Path((region_id, operator_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyOperatorMeta>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::delete_operator_from_region(&mut transaction, region_id, operator_id)
        .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) async fn get_route_regions(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<i32>>, Error> {
    Ok(Json(sql::fetch_route_regions(&state.pool, route_id).await?))
}

pub(crate) async fn put_route_into_region(
    State(state): State<AppState>,
    Path((region_id, route_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyRouteBase>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::upsert_route_into_region(&mut transaction, region_id, route_id)
        .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) async fn delete_route_from_region(
    State(state): State<AppState>,
    Path((region_id, route_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::DeleteRoute>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::delete_route_from_region(&mut transaction, region_id, route_id)
        .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) async fn get_stop_regions(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<Vec<i32>>, Error> {
    Ok(Json(sql::fetch_stop_regions(&state.pool, stop_id).await?))
}

pub(crate) async fn put_stop_into_region(
    State(state): State<AppState>,
    Path((region_id, stop_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyStopAttrs>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::upsert_stop_into_region(&mut transaction, region_id, stop_id).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) async fn delete_stop_from_region(
    State(state): State<AppState>,
    Path((region_id, stop_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyStopAttrs>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::delete_stop_from_region(&mut transaction, region_id, stop_id).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
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
    Path((stop_id, parish_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyStopAttrs>,
) -> Result<(), Error> {
    sql::update_stop_parish(&state.pool, stop_id, parish_id).await?;
    Ok(())
}
