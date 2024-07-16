/*
    Intermodal, transportation information aggregator
    Copyright (C) 2024  Cláudio Pereira

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
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use commons::models::osm;

use super::models::{requests, responses};
use super::sql;
use crate::{auth, AppState, Error};

pub(crate) async fn get_osm_stops(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::OsmStop>>, Error> {
    Ok(Json(sql::fetch_osm_stops(&state.pool).await?))
}

pub(crate) async fn get_osm_stop_history(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<osm::NodeHistory>, Error> {
    Ok(Json(
        sql::fetch_osm_stop_history(&state.pool, id)
            .await?
            .ok_or(Error::NotFoundUpstream)?,
    ))
}

#[allow(clippy::cast_sign_loss)]
pub(crate) async fn patch_osm_stops(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Json(mut newer_stops): Json<Vec<requests::OsmStop>>,
) -> Result<(), Error> {
    let mut partial_updates = 0;

    for newer_stop in &mut newer_stops {
        if newer_stop.history.is_empty() {
            return Err(Error::ValidationFailure(
                "Received node with an empty history".to_string(),
            ));
        }
        newer_stop.history.sort_by_key(|h| h.version);

        let version_count = newer_stop.history.len();
        let last_version = newer_stop.history.last().unwrap();
        if version_count != last_version.version as usize {
            partial_updates += 1;
        }
    }

    if partial_updates > 0 {
        let histories = sql::fetch_osm_stop_histories(&state.pool).await?;
        for newer_stop in &mut newer_stops {
            let version_count = newer_stop.history.len();
            let last_version = newer_stop.history.last().unwrap();
            let is_partial = version_count != last_version.version as usize;

            if is_partial {
                let newer_versions = newer_stop
                    .history
                    .iter()
                    .map(|node| node.version)
                    .collect::<HashSet<i32>>();
                if let Some(old_history) = histories.get(&newer_stop.id) {
                    let merged_history = old_history
                        .iter()
                        // Ensure that the newer versions prevail over the older
                        .filter(|h| !newer_versions.contains(&h.version))
                        .chain(newer_stop.history.iter())
                        .sorted_by_key(|h| h.version)
                        .cloned()
                        .collect();
                    newer_stop.history = merged_history;
                }
            }
        }
    }

    sql::upsert_osm_stops(&state.pool, &newer_stops).await?;

    Ok(())
}

pub(crate) async fn delete_osm_stop(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
) -> Result<(), Error> {
    sql::delete_osm_stop(&state.pool, id).await
}

pub(crate) async fn get_osm_stop_versions(
    State(state): State<AppState>,
) -> Result<Json<HashMap<i64, Vec<i32>>>, Error> {
    Ok(Json(sql::fetch_osm_stop_versions(&state.pool).await?))
}

pub(crate) async fn get_paired_osm_stop(
    State(state): State<AppState>,
    Path(iml_stop_id): Path<i32>,
) -> Result<Json<Option<responses::FullOsmStop>>, Error> {
    Ok(Json(
        sql::fetch_paired_osm_stop(&state.pool, iml_stop_id).await?,
    ))
}

pub(crate) async fn get_stops_map_features(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::StopMapFeatures>>, Error> {
    Ok(Json(sql::fetch_stops_map_features(&state.pool).await?))
}

pub(crate) async fn get_region_stops_map_features(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::StopMapFeatures>>, Error> {
    Ok(Json(
        sql::fetch_region_stops_map_features(&state.pool, region_id).await?,
    ))
}

pub(crate) async fn put_stop_map_features(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Json(change): Json<requests::OsmFeaturesChange>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_stop_map_features(&mut transaction, stop_id, change).await?;
    // TODO changelog

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}
