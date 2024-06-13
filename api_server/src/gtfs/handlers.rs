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
use tracing::log;

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
    let operator = operators_sql::fetch_operator(&state.pool, operator_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    update_operator_gtfs(operator.id, &operator.tag).await
}

pub(crate) async fn get_gtfs_stops(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Arc<Vec<gtfs::Stop>>>, Error> {
    let operator = operators_sql::fetch_operator(&state.pool, operator_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

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
    let operator = operators_sql::fetch_operator(&state.pool, operator_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

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
    let operator = operators_sql::fetch_operator(&state.pool, operator_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

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
    sql::fetch_operator_validation_data(&state.pool, operator)
        .await?
        .map(Json)
        .ok_or(Error::NotFoundUpstream)
}
pub(crate) async fn put_operator_validation_data(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(operator_id): Path<i32>,
    Json(validation): Json<gtfs::OperatorValidation>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_operator_validation_data(
        &mut transaction,
        operator_id,
        validation,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

/// Queries the validation data for a route and its subroutes
pub(crate) async fn get_route_validation_data(
    State(state): State<AppState>,
    Path(route): Path<i32>,
) -> Result<Json<responses::RouteValidation>, Error> {
    sql::fetch_route_validation_data(&state.pool, route)
        .await?
        .map(Json)
        .ok_or(Error::NotFoundUpstream)
}

/// Accepts the validation data for a route and its subroutes
/// and proceeds to replace the old data with the new one
pub(crate) async fn patch_route_validation_data(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(route): Path<i32>,
    Json(request): Json<requests::RouteSubroutesValidation>,
) -> Result<(), Error> {
    use crate::routes::sql as routes_sql;
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_route_validation_data(
        &mut transaction,
        route,
        request.validation,
    )
    .await?;

    for (subroute_id, validation) in request.subroutes {
        let correspondence_stops = validation.stops;
        let gtfs_cluster = validation.gtfs_cluster;

        sql::update_subroute_validation_data(
            &mut transaction,
            subroute_id,
            &correspondence_stops,
            &gtfs_cluster,
        )
        .await?;
    }

    routes_sql::regen_subroute_stops_cache(&mut transaction).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

/// Attach unpaired validation data in a route to a subroute
/// This means that a previously unrecognized pattern is attached to the subroute
pub(crate) async fn post_assign_subroute_validation(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(route): Path<i32>,
    Json(request): Json<requests::ValidateSubroute>,
) -> Result<(), Error> {
    use crate::routes::sql as routes_sql;

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let validation_data =
        sql::fetch_route_validation_data(&mut *transaction, route)
            .await?
            .ok_or(Error::NotFoundUpstream)?;

    // Check if there's an unassigned cluster with the referenced pattern
    let sqlx::types::Json(route_validation) =
        validation_data.validation.ok_or(Error::NotFoundUpstream)?;

    let (matched_validations, unmatched_validations): (
        Vec<gtfs::SubrouteValidation>,
        Vec<gtfs::SubrouteValidation>,
    ) = route_validation
        .unmatched
        .into_iter()
        .partition(|validation| {
            validation
                .gtfs_cluster
                .patterns
                .contains(&request.pattern_id)
        });

    // We naturally only want to match one single entry, no more
    if matched_validations.len() > 1 {
        log::error!(
            "Multiple subroutes matched the pattern id: {pattern_id}",
            pattern_id = request.pattern_id
        );
        return Err(Error::DependenciesNotMet);
    }
    // and no less
    let Some(subroute_validation) = matched_validations.first() else {
        log::warn!(
            "No subroutes matched the pattern id: {pattern_id}",
            pattern_id = request.pattern_id
        );
        return Err(Error::NotFoundUpstream);
    };

    // Remove the just-matched validation from the unmatched list
    sql::update_route_validation_data(
        &mut transaction,
        route,
        gtfs::RouteValidation {
            unmatched: unmatched_validations,
        },
    )
    .await?;

    // And attach it to the referenced subroute
    // TODO log this transition
    // let current_stops =
    //     routes_sql::fetch_subroute_stops(&mut transaction, request.subroute_id)
    //         .await?;
    let correspondence_stops = &subroute_validation.stops;
    let gtfs_cluster = &subroute_validation.gtfs_cluster;

    if request.sync {
        // Stores the extracted GTFS in the subroute validation_data
        // Also sets `validation_current` to match it
        sql::update_subroute_validation_data(
            &mut transaction,
            request.subroute_id,
            &correspondence_stops,
            gtfs_cluster,
        )
        .await?;
        // Sets stops to the GTFS (correspondence) stops
        routes_sql::update_subroute_stops(
            &mut transaction,
            request.subroute_id,
            correspondence_stops,
        )
        .await?;
        // Consider the GTFS stops (correspondence) acknowledged
        sql::update_subroute_correspondence_ack(
            &mut transaction,
            request.subroute_id,
            correspondence_stops,
        )
        .await?;
        // Do we also want to consider the IML stops acknowledged?
    } else {
        // Just bind them, without copying anything over or acknowledging
        sql::update_subroute_validation_data(
            &mut transaction,
            request.subroute_id,
            correspondence_stops,
            gtfs_cluster,
        )
        .await?;
    }

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

/// Acknowledges the current stops as the last validated against the GTFS data
pub(crate) async fn post_subroute_validation_current_ack(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(subroute_id): Path<i32>,
    Json(request): Json<requests::MatchedUpdateStopIds>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let sr_validation_data =
        sql::fetch_subroute_validation_data(&mut *transaction, subroute_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?;

    // Ensure that we're updating from the right value
    if request.from_stop_ids != sr_validation_data.current_ack {
        return Err(Error::DependenciesNotMet);
    }

    sql::update_subroute_validation_current_ack(
        &mut transaction,
        subroute_id,
        &request.to_stop_ids,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

/// Acknowledges the current GTFS correspondence data as the latest validated
pub(crate) async fn post_subroute_validation_correspondence_ack(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(subroute_id): Path<i32>,
    Json(request): Json<requests::MatchedUpdateStopIds>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let sr_validation_data =
        sql::fetch_subroute_validation_data(&mut *transaction, subroute_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?;

    // Ensure that we're updating from the right value
    if request.from_stop_ids != sr_validation_data.correspondence_ack {
        return Err(Error::DependenciesNotMet);
    }

    sql::update_subroute_correspondence_ack(
        &mut transaction,
        subroute_id,
        &request.to_stop_ids,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}
