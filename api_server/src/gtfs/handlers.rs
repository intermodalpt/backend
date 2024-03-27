/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023  Cláudio Pereira

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
use std::{fs, io};

use axum::extract::{Path, State};
use axum::Json;

use commons::models::gtfs::{self, File};
use commons::utils::gtfs::{
    calculate_gtfs_stop_sequence, calculate_stop_sliding_windows,
};

use super::models::{requests, responses};
use super::sql;
use super::{loaders, models};
use crate::operators::import::{update_operator_gtfs, OperatorData};
use crate::operators::sql as operators_sql;
use crate::{auth, AppState, Error};

pub(crate) async fn post_update_operator_gtfs(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
) -> Result<(), Error> {
    let operator =
        operators_sql::fetch_operator(&state.pool, operator_id).await?;
    if operator.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let operator = operator.unwrap();

    update_operator_gtfs(operator.id, &operator.tag).await
}

pub(crate) async fn get_gtfs_stops(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Arc<Vec<gtfs::Stop>>>, Error> {
    let operator =
        operators_sql::fetch_operator(&state.pool, operator_id).await?;
    if operator.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let operator = operator.unwrap();

    let meta = operator.get_storage_meta()?;

    if meta.last_gtfs.is_none() {
        return Err(Error::NotFoundUpstream);
    }

    {
        let gtfs_stops_read_guard = state.cached.gtfs_stops.read().unwrap();
        if let Some(gtfs_stops) = gtfs_stops_read_guard.get(&operator_id) {
            return Ok(Json(gtfs_stops.clone()));
        }
    }

    // Calc data
    let gtfs_stops = loaders::gtfs_stops(&operator)?;
    let gtfs_stops = Arc::new(gtfs_stops);
    // Cache it
    let mut gtfs_stops_write_guard = state.cached.gtfs_stops.write().unwrap();
    gtfs_stops_write_guard.insert(operator_id, gtfs_stops.clone());

    Ok(Json(gtfs_stops))
}

pub(crate) async fn get_gtfs_route_trips(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Arc<Vec<models::TMLRoute>>>, Error> {
    let operator =
        operators_sql::fetch_operator(&state.pool, operator_id).await?;
    if operator.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let operator = operator.unwrap();

    let meta = operator.get_storage_meta()?;

    if meta.last_gtfs.is_none() {
        return Err(Error::NotFoundUpstream);
    }

    {
        let routes_read_guard = state.cached.tml_routes.read().unwrap();
        if let Some(routes) = routes_read_guard.get(&operator_id) {
            return Ok(Json(routes.clone()));
        }
    }

    // Calc data
    let routes = loaders::simplified_gtfs_routes(&operator)?;
    let tml_routes = Arc::new(routes);
    // Cache it
    let mut routes_write_guard = state.cached.tml_routes.write().unwrap();
    routes_write_guard.insert(operator_id, tml_routes.clone());

    Ok(Json(tml_routes))
}

pub(crate) async fn get_gtfs_stop_sliding_windows(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<Vec<String>>>, Error> {
    let operator =
        operators_sql::fetch_operator(&state.pool, operator_id).await?;
    if operator.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let operator = operator.unwrap();

    let meta = operator.get_storage_meta()?;

    if meta.last_gtfs.is_none() {
        return Err(Error::NotFoundUpstream);
    }

    let gtfs_root = operator.get_gtfs_root();
    let stop_times_path = File::StopTimes.prepend_root(&gtfs_root);

    let f = fs::File::open(stop_times_path).unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader);

    let gtfs_stop_times = rdr
        .deserialize()
        .map(Result::unwrap)
        .collect::<Vec<gtfs::StopTime>>();

    let trips_stop_seq = calculate_gtfs_stop_sequence(&gtfs_stop_times);
    let sliding_windows = calculate_stop_sliding_windows(&trips_stop_seq);

    Ok(Json(sliding_windows))
}

pub(crate) async fn get_operator_validation_data(
    State(state): State<AppState>,
    Path(operator): Path<i32>,
) -> Result<Json<Option<gtfs::OperatorValidation>>, Error> {
    let data =
        sql::fetch_operator_validation_data(&state.pool, operator).await?;

    if let Some(data) = data {
        Ok(Json(data))
    } else {
        Err(Error::NotFoundUpstream)
    }
}
pub(crate) async fn put_operator_validation_data(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(operator_id): Path<i32>,
    Json(validation): Json<gtfs::OperatorValidation>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_operator_validation_data(
        &mut transaction,
        operator_id,
        validation,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn get_route_validation_data(
    State(state): State<AppState>,
    Path(route): Path<i32>,
) -> Result<Json<responses::RouteValidation>, Error> {
    let data = sql::fetch_route_validation_data(&state.pool, route).await?;

    if let Some(data) = data {
        Ok(Json(data))
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn put_route_validation_data(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(route): Path<i32>,
    Json(request): Json<requests::RouteSubroutesValidation>,
) -> Result<(), Error> {
    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_route_validation_data(
        &mut transaction,
        route,
        request.validation,
    )
    .await?;

    for (subroute, validation) in request.subroutes {
        sql::update_subroute_validation_data(
            &mut transaction,
            subroute,
            validation,
        )
        .await?;
    }

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn post_assign_subroute_validation(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(route): Path<i32>,
    Json(request): Json<requests::ValidateSubroute>,
) -> Result<(), Error> {
    use crate::routes::sql as routes_sql;

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let validation_data =
        sql::fetch_route_validation_data(&mut *transaction, route)
            .await?
            .ok_or(Error::NotFoundUpstream)?;

    // Check if there's an unassigned cluster with the referenced pattern
    let sqlx::types::Json(route_validation) =
        validation_data.validation.ok_or(Error::NotFoundUpstream)?;

    let unassigned_cluster = route_validation
        .unmatched
        .iter()
        .find(|cluster| cluster.patterns.contains(&request.pattern_id))
        .ok_or(Error::NotFoundUpstream)?;

    if let Some(sr_validation) =
        validation_data.subroutes.get(&request.subroute_id)
    {
        // Ensure that the subroute has no assigned cluster
        if sr_validation.is_some() {
            return Err(Error::DependenciesNotMet);
        }
    }

    let sr_stops =
        routes_sql::fetch_subroute_stops(&mut transaction, request.subroute_id)
            .await?;

    sql::update_route_validation_data(
        &mut transaction,
        route,
        gtfs::RouteValidation {
            unmatched: route_validation
                .unmatched
                .iter()
                .filter(|cluster| {
                    cluster.patterns != unassigned_cluster.patterns
                })
                .cloned()
                .collect(),
        },
    )
    .await?;

    sql::update_subroute_validation_data(
        &mut transaction,
        request.subroute_id,
        gtfs::SubrouteValidation {
            gtfs_pattern_ids: unassigned_cluster
                .patterns
                .iter()
                .cloned()
                .collect(),
            gtfs_trip_ids: unassigned_cluster.trips.iter().cloned().collect(),
            gtfs_headsigns: unassigned_cluster
                .headsigns
                .iter()
                .cloned()
                .collect(),
            gtfs_stops: unassigned_cluster.stops.clone(),
            iml_stops: sr_stops,
        },
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn patch_subroute_validation_stops(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(subroute_id): Path<i32>,
    Json(request): Json<requests::UpdateSubrouteValidationStops>,
) -> Result<(), Error> {
    use crate::routes::sql as routes_sql;

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let sr_validation_data =
        sql::fetch_subroute_validation_data(&mut *transaction, subroute_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?;

    sr_validation_data
        .gtfs_pattern_ids
        .contains(&request.pattern_id)
        .then(|| ())
        .ok_or(Error::ValidationFailure(format!(
            "Unrecognized pattern '{}' for subroute {}",
            request.pattern_id, subroute_id
        )))?;

    let sr_stops =
        routes_sql::fetch_subroute_stops(&mut transaction, subroute_id).await?;

    if request.from_stop_ids != sr_stops {
        return Err(Error::DependenciesNotMet);
    }

    if request.from_stop_ids != request.to_stop_ids {
        routes_sql::update_subroute_stops(
            &mut transaction,
            subroute_id,
            &request.to_stop_ids,
            &request.from_stop_ids,
        )
        .await?;
    }

    sql::update_subroute_validation_data(
        &mut transaction,
        subroute_id,
        gtfs::SubrouteValidation {
            iml_stops: request.to_stop_ids,
            ..sr_validation_data
        },
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}
