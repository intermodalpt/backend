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

#[utoipa::path(
    get,
    path = "/v1/routes",
    responses(
        (status = 200, description = "List of routes", body = [Route]),
    )
)]
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

pub(crate) async fn post_route(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Json(route): Json<requests::ChangeRoute>,
) -> Result<Json<routes::Route>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }
    let user_id = claims.uid;

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let route = sql::insert_route(&mut transaction, route).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::RouteCreation {
            data: route.clone().into(),
        }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(route))
}

pub(crate) async fn get_route(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<responses::Route>, Error> {
    if let Some(route) =
        sql::fetch_route_with_subroutes(&state.pool, route_id).await?
    {
        Ok(Json(route))
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn get_route_full(
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<responses::FullRoute>, Error> {
    if let Some(route) =
        sql::fetch_full_route_with_subroutes(&state.pool, route_id).await?
    {
        Ok(Json(route))
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn patch_route(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(route_id): Path<i32>,
    Json(changes): Json<requests::ChangeRoute>,
) -> Result<Json<routes::Route>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    let route = sql::fetch_commons_route(&state.pool, route_id).await?;
    if route.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let route = route.unwrap();

    let patch = changes.derive_patch(&route);

    if patch.is_empty() {
        return Ok(Json(route));
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_route(&mut transaction, route_id, changes).await?;
    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::RouteUpdate {
            original: route.clone().into(),
            patch,
        }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(route))
}

pub(crate) async fn delete_route(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(route_id): Path<i32>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    let route = sql::fetch_commons_route(&state.pool, route_id).await?;
    if route.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let route = route.unwrap();

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::delete_route(&mut transaction, route_id).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::RouteDeletion { data: route.into() }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn create_subroute(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(route_id): Path<i32>,
    Json(subroute): Json<requests::ChangeSubroute>,
) -> Result<Json<routes::Subroute>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let subroute =
        sql::insert_subroute(&mut transaction, route_id, subroute).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::SubrouteCreation {
            data: subroute.clone().into(),
        }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(Json(subroute))
}

pub(crate) async fn patch_subroute(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path((route_id, subroute_id)): Path<(i32, i32)>,
    Json(changes): Json<requests::ChangeSubroute>,
) -> Result<Json<routes::Subroute>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    let subroute = sql::fetch_simple_subroute(&state.pool, subroute_id).await?;
    if subroute.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let subroute = subroute.unwrap();

    let patch = changes.derive_patch(&subroute);

    if patch.is_empty() {
        return Ok(Json(subroute));
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::SubrouteUpdate {
            original: subroute.clone().into(),
            patch,
        }],
        None,
    )
    .await?;

    sql::update_subroute(&mut transaction, route_id, subroute_id, changes)
        .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(subroute))
}

pub(crate) async fn delete_subroute(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path((route_id, subroute_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let subroute = sql::fetch_simple_subroute(&state.pool, subroute_id).await?;
    if subroute.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let subroute = subroute.unwrap();

    let stops =
        sql::fetch_subroute_stops(&mut transaction, subroute_id).await?;
    let departures =
        sql::fetch_subroute_departures(&mut transaction, subroute_id).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
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
    sql::delete_subroute(&mut transaction, route_id, subroute_id).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn create_subroute_departure(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(subroute_id): Path<i32>,
    Json(departure): Json<requests::ChangeDeparture>,
) -> Result<Json<routes::Departure>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !(claims.permissions.is_admin || claims.permissions.is_trusted) {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(departure))
}

pub(crate) async fn patch_subroute_departure(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
    Json(change): Json<requests::ChangeDeparture>,
) -> Result<Json<routes::Departure>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !(claims.permissions.is_admin || claims.permissions.is_trusted) {
        return Err(Error::Forbidden);
    }

    let departure = sql::fetch_departure(&state.pool, departure_id).await?;

    if departure.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let departure = departure.unwrap();

    let patch = change.derive_patch(&departure);

    if patch.is_empty() {
        return Ok(Json(departure));
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(departure))
}

pub(crate) async fn delete_subroute_departure(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !(claims.permissions.is_admin || claims.permissions.is_trusted) {
        return Err(Error::Forbidden);
    }

    let departure = sql::fetch_departure(&state.pool, departure_id).await?;

    if departure.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let departure = departure.unwrap();

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/v1/routes/{route_id}/stops",
    params(
        (
            "route_id",
            Path,
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
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<responses::SubrouteStops>>, Error> {
    Ok(Json(sql::fetch_route_stops(&state.pool, route_id).await?))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub(crate) async fn patch_subroute_stops(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path((route_id, subroute_id)): Path<(i32, i32)>,
    Json(request): Json<requests::ChangeSubrouteStops>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_subroute_stops(
        &mut transaction,
        route_id,
        subroute_id,
        request,
    )
    .await?;

    // TODO log
    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/v1/routes/{route_id}/schedule",
    params(
        (
            "route_id",
            Path,
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
    State(state): State<AppState>,
    Path(route_id): Path<i32>,
) -> Result<Json<Vec<responses::Departure>>, Error> {
    Ok(Json(sql::fetch_schedule(&state.pool, route_id).await?))
}

pub(crate) async fn post_replace_stop_across_routes(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path((original_id, replacement_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    sql::migrate_stop(&state.pool, original_id, replacement_id).await?;
    Ok(())
}
