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

use commons::models::{history, routes, stops};

use super::models::{requests, responses};
use super::sql;
use crate::responses::IdReturn;
use crate::{auth, contrib, AppState, Error};

pub(crate) async fn get_region_stops(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::SimpleStop>>, Error> {
    Ok(Json(
        sql::fetch_region_simple_stops(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn get_region_detailed_stops(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    Ok(Json(
        sql::fetch_region_detailed_stops(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn get_region_full_stops(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::FullStop>>, Error> {
    Ok(Json(
        sql::fetch_region_full_stops(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn get_all_detailed_stops(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ExpensiveCalls>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    // TODO this made sense back in the black and white days.
    // Consider deprecating.
    Ok(Json(sql::fetch_all_detailed_stops(&state.pool).await?))
}

pub(crate) async fn get_all_stops(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::SimpleStop>>, Error> {
    Ok(Json(sql::fetch_all_stops(&state.pool).await?))
}
pub(crate) async fn get_stop(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<responses::Stop>, Error> {
    Ok(Json(
        sql::fetch_stop(&state.pool, stop_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?,
    ))
}

pub(crate) async fn get_stop_list(
    State(state): State<AppState>,
    Path(stops_ids): Path<String>,
) -> Result<Json<Vec<responses::SimpleStop>>, Error> {
    // parse stop_ids into a comma separated list of integers
    let stops: Vec<i32> = stops_ids
        .split(',')
        .map(|id| {
            id.parse::<i32>()
                .or(Err(Error::MalformedRequest("Invalid list format")))
        })
        .collect::<Result<_, _>>()?;

    Ok(Json(sql::fetch_stop_list(&state.pool, &stops).await?))
}

pub(crate) async fn post_stop(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::CreateStop>,
    Json(stop): Json<requests::NewStop>,
) -> Result<Json<IdReturn<i32>>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let stop = sql::insert_stop(&mut transaction, stop, claims.uid).await?;
    let id = stop.id;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::StopCreation { data: stop.into() }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(IdReturn { id }))
}

pub(crate) async fn patch_stop(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<
        auth::perms::ModifyStopAttrs,
    >,
    Path(stop_id): Path<i32>,
    Json(changes): Json<requests::ChangeStop>,
) -> Result<Json<stops::Stop>, Error> {
    let mut stop: stops::Stop = sql::fetch_stop(&state.pool, stop_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?
        .into();

    let patch = changes.derive_patch(&stop);

    if patch.is_empty() {
        return Ok(Json(stop));
    }

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::StopUpdate {
            original: stop.clone().into(),
            patch: patch.clone(),
        }],
        None,
    )
    .await?;

    sql::update_stop(&mut transaction, stop_id, changes, claims.uid).await?;

    // If this fails then proceed find a better suited job in a fast food chain.
    // The patch was just made, must be valid.
    assert!(patch.apply(&mut stop).is_ok());

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(stop))
}

pub(crate) async fn put_stop_position(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyStopPos>,
    Json(location): Json<requests::Position>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    // TODO log

    let updated = sql::update_stop_position(
        &mut transaction,
        stop_id,
        location.lon,
        location.lat,
    )
    .await?;

    if !updated {
        return Err(Error::NotFoundUpstream);
    }

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn put_stop_vehicle_position(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyStopPos>,
    Json(location): Json<requests::Position>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    // TODO log

    let updated = sql::update_stop_vehicle_position(
        &mut transaction,
        stop_id,
        location.lon,
        location.lat,
    )
    .await?;

    if !updated {
        return Err(Error::NotFoundUpstream);
    }

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_bounded_stops(
    State(state): State<AppState>,
    Path(boundary): Path<(f64, f64, f64, f64)>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    Ok(Json(sql::fetch_bounded_stops(&state.pool, boundary).await?))
}

pub(crate) async fn get_operator_stops(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    Ok(Json(
        sql::fetch_operator_stops(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn get_operator_full_stops(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::FullStop>>, Error> {
    Ok(Json(
        sql::fetch_operator_full_stops(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn get_route_stops(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    Ok(Json(sql::fetch_route_stops(&state.pool, route_id).await?))
}

// TODO move this to the routes module
pub(crate) async fn get_stop_routes(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<Vec<routes::Route>>, Error> {
    Ok(Json(sql::fetch_stop_routes(&state.pool, stop_id).await?))
}

pub(crate) async fn get_stop_spider(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<responses::SpiderMap>, Error> {
    get_stops_spider(State(state), Json(vec![stop_id])).await
}

pub(crate) async fn get_stops_spider(
    State(state): State<AppState>,
    Json(stops): Json<Vec<i32>>,
) -> Result<Json<responses::SpiderMap>, Error> {
    Ok(Json(sql::fetch_stop_spider(&state.pool, &stops).await?))
}

pub(crate) async fn get_osm_paired_stop(
    State(state): State<AppState>,
    Path(osm_id): Path<i64>,
) -> Result<Json<responses::SimpleStop>, Error> {
    sql::fetch_paired_stop(&state.pool, osm_id)
        .await?
        .map(Json)
        .ok_or(Error::NotFoundUpstream)
}

pub(crate) async fn get_stop_by_operator_ref(
    State(state): State<AppState>,
    Path((operator_id, stop_ref)): Path<(i32, String)>,
) -> Result<Json<Vec<responses::SimpleStop>>, Error> {
    Ok(Json(
        sql::fetch_stops_by_operator_ref(&state.pool, operator_id, &stop_ref)
            .await?,
    ))
}

pub(crate) async fn get_region_todo(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::StopTodos>>, Error> {
    Ok(Json(
        sql::fetch_region_todo_stops(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn put_stop_todo(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    Json(todo): Json<Vec<stops::StopTodo>>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_stop_todos(&mut transaction, stop_id, &todo).await?;
    // todo log

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}
