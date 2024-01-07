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

use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::Json;

use commons::models::{history, routes, stops};

use super::{models, sql};
use crate::stops::models::responses;
use crate::{auth, contrib, AppState, Error};

pub(crate) async fn get_stops(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::SimpleStop>>, Error> {
    Ok(Json(sql::fetch_simple_stops(&state.pool, region_id).await?))
}

pub(crate) async fn get_detailed_stops(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    Ok(Json(
        sql::fetch_detailed_stops(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn get_full_stops(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::FullStop>>, Error> {
    Ok(Json(sql::fetch_full_stops(&state.pool, region_id).await?))
}

pub(crate) async fn get_all_stops(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    Ok(Json(sql::fetch_all_detailed_stops(&state.pool).await?))
}

#[utoipa::path(
    get,
    path = "/v1/stop/{id}",
    responses(
        (status = 200, description = "Stop information", body = Stop)
    )
)]
pub(crate) async fn get_stop(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<responses::Stop>, Error> {
    let stop = sql::fetch_stop(&state.pool, stop_id).await?;

    if let Some(stop) = stop {
        Ok(Json(stop))
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn create_stop(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Json(stop): Json<models::requests::NewStop>,
) -> Result<Json<HashMap<String, i32>>, Error> {
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

    let stop = sql::insert_stop(&mut transaction, stop, user_id).await?;
    let id = stop.id;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::StopCreation { data: stop.into() }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn patch_stop(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(stop_id): Path<i32>,
    Json(changes): Json<models::requests::ChangeStop>,
) -> Result<Json<stops::Stop>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    let stop = sql::fetch_stop(&state.pool, stop_id).await?;
    if stop.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let mut stop = stops::Stop::from(stop.unwrap());

    let patch = changes.derive_patch(&stop);

    if patch.is_empty() {
        return Ok(Json(stop));
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::StopUpdate {
            original: stop.clone().into(),
            patch: patch.clone(),
        }],
        None,
    )
    .await?;

    sql::update_stop(&mut transaction, stop_id, changes, user_id).await?;

    // If this fails then proceed find a better suited job in a fast food chain.
    // The patch was just made, must be valid.
    assert!(patch.apply(&mut stop).is_ok());

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(stop))
}

pub(crate) async fn get_bounded_stops(
    State(state): State<AppState>,
    Path(boundary): Path<(f64, f64, f64, f64)>,
) -> Result<Json<Vec<responses::Stop>>, Error> {
    Ok(Json(sql::fetch_bounded_stops(&state.pool, boundary).await?))
}

pub(crate) async fn get_stop_routes(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<Vec<routes::Route>>, Error> {
    Ok(Json(sql::fetch_stop_routes(&state.pool, stop_id).await?))
}

#[utoipa::path(get, path = "/v1/stops/{stop_id}/spider")]
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
