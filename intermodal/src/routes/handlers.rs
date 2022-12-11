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

use axum::extract::Path;
use axum::{Extension, Json};
use chrono::NaiveDate;

use super::models::{self, requests, responses, sql};
use super::sql;
use crate::{auth, contrib, Error, State};

#[utoipa::path(
    get,
    path = "/v1/routes",
    responses(
        (status = 200, description = "List of routes", body = [Route]),
    )
)]
pub(crate) async fn get_routes(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Vec<responses::Route>>, Error> {
    let routes = sql::fetch_routes_with_subroutes(&state.pool).await?;
    Ok(Json(routes))
}

pub(crate) async fn create_route(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Json(route): Json<requests::ChangeRoute>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }
    let user_id = claims.uid;

    //TODO as a transaction

    let id = sql::insert_route(&state.pool, &route).await?;
    let route = models::Route {
        id,
        type_id: route.type_id,
        operator_id: route.operator_id,
        code: route.code,
        name: route.name,
        main_subroute: route.main_subroute,
        active: route.active,
    };

    contrib::sql::insert_changeset_log(
        &state.pool,
        user_id,
        &[contrib::models::Change::RouteCreation { data: route }],
        None,
    )
    .await?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn get_route(
    Extension(state): Extension<Arc<State>>,
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

pub(crate) async fn patch_route(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path(route_id): Path<i32>,
    Json(changes): Json<requests::ChangeRoute>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    //TODO as a transaction
    let route = sql::fetch_route(&state.pool, route_id).await?;
    if route.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let route = route.unwrap();

    let patch = changes.derive_patch(&route);

    if patch.is_empty() {
        return Err(Error::ValidationFailure(
            "No changes were made".to_string(),
        ));
    }

    contrib::sql::insert_changeset_log(
        &state.pool,
        user_id,
        &[contrib::models::Change::RouteUpdate {
            original: route,
            patch,
        }],
        None,
    )
    .await?;

    sql::update_route(&state.pool, route_id, changes).await
}

pub(crate) async fn delete_route(
    Extension(state): Extension<Arc<State>>,
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

    let route = sql::fetch_route(&state.pool, route_id).await?;
    if route.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let route = route.unwrap();

    contrib::sql::insert_changeset_log(
        &state.pool,
        user_id,
        &[contrib::models::Change::RouteDeletion { data: route }],
        None,
    )
    .await?;
    sql::delete_route(&state.pool, route_id).await
}

pub(crate) async fn create_subroute(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path(route_id): Path<i32>,
    Json(subroute): Json<requests::ChangeSubroute>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    //TODO as a transaction
    let subroute =
        sql::insert_subroute(&state.pool, route_id, subroute).await?;
    let id = subroute.id;

    contrib::sql::insert_changeset_log(
        &state.pool,
        user_id,
        &[contrib::models::Change::SubrouteCreation { data: subroute }],
        None,
    )
    .await?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn patch_subroute(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path((route_id, subroute_id)): Path<(i32, i32)>,
    Json(changes): Json<requests::ChangeSubroute>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    //TODO as a transaction
    let subroute = sql::fetch_subroute(&state.pool, subroute_id).await?;
    if subroute.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let subroute = subroute.unwrap();

    let patch = changes.derive_patch(&subroute);

    if patch.is_empty() {
        return Err(Error::ValidationFailure(
            "No changes were made".to_string(),
        ));
    }

    contrib::sql::insert_changeset_log(
        &state.pool,
        user_id,
        &[contrib::models::Change::SubrouteUpdate {
            original: subroute,
            patch,
        }],
        None,
    )
    .await?;

    sql::update_subroute(&state.pool, route_id, subroute_id, changes).await
}

pub(crate) async fn delete_subroute(
    Extension(state): Extension<Arc<State>>,
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

    //TODO as a transaction
    let subroute = sql::fetch_subroute(&state.pool, subroute_id).await?;
    if subroute.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let subroute = subroute.unwrap();

    let stop_count =
        sql::fetch_subroute_stop_count(&state.pool, subroute_id).await?;
    let departure_count =
        sql::fetch_subroute_departure_count(&state.pool, subroute_id).await?;

    if stop_count > 0 || departure_count > 0 {
        return Err(Error::DependenciesNotMet);
    }

    contrib::sql::insert_changeset_log(
        &state.pool,
        user_id,
        &[contrib::models::Change::SubrouteDeletion { data: subroute }],
        None,
    )
    .await?;

    sql::delete_subroute(&state.pool, route_id, subroute_id).await
}

pub(crate) async fn create_subroute_departure(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Json(departure): Json<requests::ChangeDeparture>,
    Path(subroute_id): Path<i32>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let id = sql::insert_departure(&state.pool, subroute_id, departure).await?;
    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn patch_subroute_departure(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Json(departure): Json<requests::ChangeDeparture>,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    sql::update_departure(&state.pool, subroute_id, departure_id, departure)
        .await?;
    Ok(())
}

pub(crate) async fn delete_subroute_departure(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path((subroute_id, departure_id)): Path<(i32, i32)>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

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
) -> Result<Json<Vec<responses::SubrouteStops>>, Error> {
    Ok(Json(sql::fetch_route_stops(&state.pool, route_id).await?))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub(crate) async fn patch_subroute_stops(
    Extension(state): Extension<Arc<State>>,
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

    Ok(
        sql::update_subroute_stops(&state.pool, route_id, subroute_id, request)
            .await?,
    )
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
) -> Result<Json<Vec<responses::Departure>>, Error> {
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
) -> Result<Json<Vec<responses::DateDeparture>>, Error> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    Ok(Json(
        sql::fetch_schedule_for_date(&state.pool, route_id, date).await?,
    ))
}
