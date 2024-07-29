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

use commons::models::{history, routes};

use super::models::{requests, responses};
use super::sql;
use crate::{auth, contrib, AppState, Error};

pub(crate) async fn get_routes(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::Route>>, Error> {
    Ok(Json(sql::fetch_routes(&state.pool, region_id).await?))
}

pub(crate) async fn get_full_routes(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::FullRoute>>, Error> {
    Ok(Json(sql::fetch_full_routes(&state.pool, region_id).await?))
}

pub(crate) async fn get_operator_routes(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::Route>>, Error> {
    Ok(Json(
        sql::fetch_operator_routes(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn get_operator_full_routes(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::FullRoute>>, Error> {
    Ok(Json(
        sql::fetch_operator_full_routes(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn post_route(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::CreateRoute>,
    Json(route): Json<requests::ChangeRoute>,
) -> Result<Json<routes::Route>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let route = sql::insert_route(&mut transaction, route).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::RouteCreation {
            data: route.clone().into(),
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(route))
}

pub(crate) async fn get_route(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<responses::Route>, Error> {
    sql::fetch_route_with_subroutes(&state.pool, route_id)
        .await?
        .map(Json)
        .ok_or(Error::NotFoundUpstream)
}

pub(crate) async fn get_route_full(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<responses::FullRoute>, Error> {
    sql::fetch_full_route_with_subroutes(&state.pool, route_id)
        .await?
        .map(Json)
        .ok_or(Error::NotFoundUpstream)
}

pub(crate) async fn patch_route(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<
        auth::perms::ModifyRouteBase,
    >,
    Path(route_id): Path<i32>,
    Json(changes): Json<requests::ChangeRoute>,
) -> Result<Json<routes::Route>, Error> {
    let route = sql::fetch_commons_route(&state.pool, route_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let patch = changes.derive_patch(&route);

    if patch.is_empty() {
        return Ok(Json(route));
    }

    let original = route.clone().into();
    let mut patched = route.clone();
    patch.clone().apply(&mut patched);

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_route(&mut transaction, route_id, changes).await?;
    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::RouteUpdate { original, patch }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(patched))
}

pub(crate) async fn delete_route(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::DeleteRoute>,
    Path(route_id): Path<i32>,
) -> Result<(), Error> {
    let route = sql::fetch_commons_route(&state.pool, route_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::delete_route(&mut transaction, route_id).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::RouteDeletion { data: route.into() }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) async fn create_subroute(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::CreateRoute>,
    Path(route_id): Path<i32>,
    Json(subroute): Json<requests::ChangeSubroute>,
) -> Result<Json<routes::Subroute>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let subroute =
        sql::insert_subroute(&mut transaction, route_id, subroute).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::SubrouteCreation {
            data: subroute.clone().into(),
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;
    Ok(Json(subroute))
}

pub(crate) async fn patch_subroute(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<
        auth::perms::ModifyRouteSubroutes,
    >,
    Path(subroute_id): Path<i32>,
    Json(changes): Json<requests::ChangeSubroute>,
) -> Result<Json<routes::Subroute>, Error> {
    let subroute = sql::fetch_simple_subroute(&state.pool, subroute_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let patch = changes.derive_patch(&subroute);

    if patch.is_empty() {
        return Ok(Json(subroute));
    }

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::SubrouteUpdate {
            original: subroute.clone().into(),
            patch,
        }],
        None,
    )
    .await?;

    sql::update_subroute(&mut transaction, subroute_id, changes).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(subroute))
}

pub(crate) async fn delete_subroute(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<
        auth::perms::ModifyRouteSubroutes,
    >,
    Path(subroute_id): Path<i32>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let subroute = sql::fetch_simple_subroute(&state.pool, subroute_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let stops =
        sql::fetch_subroute_stops(&mut transaction, subroute_id).await?;
    let departures =
        sql::fetch_subroute_departures(&mut transaction, subroute_id).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::SubrouteDeletion {
            subroute: subroute.into(),
            stops: Some(stops),
            departures: Some(history::vec_into_vec(departures)),
        }],
        None,
    )
    .await?;

    sql::delete_subroute_stops(&mut transaction, subroute_id).await?;
    sql::delete_subroute_departures(&mut transaction, subroute_id).await?;
    sql::delete_subroute(&mut transaction, subroute_id).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) async fn create_subroute_departure(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<
        auth::perms::ModifyRouteDepartures,
    >,
    Path(subroute_id): Path<i32>,
    Json(departure): Json<requests::ChangeDeparture>,
) -> Result<Json<routes::Departure>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let departure =
        sql::insert_departure(&mut transaction, subroute_id, departure).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::DepartureCreation {
            data: departure.clone().into(),
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(departure))
}

pub(crate) async fn patch_subroute_departure(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<
        auth::perms::ModifyRouteDepartures,
    >,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
    Json(change): Json<requests::ChangeDeparture>,
) -> Result<Json<routes::Departure>, Error> {
    let departure = sql::fetch_departure(&state.pool, departure_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;
    let patch = change.derive_patch(&departure);

    if patch.is_empty() {
        return Ok(Json(departure));
    }

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_departure(&mut transaction, subroute_id, departure_id, change)
        .await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::DepartureUpdate {
            original: departure.clone().into(),
            patch,
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(departure))
}

pub(crate) async fn delete_subroute_departure(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<
        auth::perms::ModifyRouteDepartures,
    >,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    let departure = sql::fetch_departure(&state.pool, departure_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::delete_departure(&mut transaction, subroute_id, departure_id).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::DepartureDeletion {
            data: departure.into(),
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_subroute_stops(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<responses::SubrouteStops>>, Error> {
    Ok(Json(sql::fetch_route_stops(&state.pool, route_id).await?))
}

pub(crate) async fn patch_subroute_stops(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyRouteStops>,
    Path(subroute_id): Path<i32>,
    Json(request): Json<requests::ChangeSubrouteStops>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let sr_stops =
        sql::fetch_subroute_stops(&mut transaction, subroute_id).await?;

    if sr_stops != request.from {
        return Err(Error::ValidationFailure("Check mismatch".to_string()));
    }

    sql::update_subroute_stops(&mut transaction, subroute_id, &request.to)
        .await?;

    // TODO log
    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_schedule(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<responses::Departure>>, Error> {
    Ok(Json(sql::fetch_schedule(&state.pool, route_id).await?))
}

pub(crate) async fn post_replace_stop_across_routes(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyRouteStops>,
    Path((original_id, replacement_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    // TODO consider deprecating
    sql::migrate_stop(&state.pool, original_id, replacement_id).await?;
    Ok(())
}
