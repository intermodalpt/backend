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
use std::collections::HashMap;

use commons::models::osm;

use super::models::{requests, responses};
use super::sql;
use crate::osm::models::responses::FullOsmStop;
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
    Ok(Json(sql::fetch_osm_stop_history(&state.pool, id).await?))
}

pub(crate) async fn patch_osm_stops(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Json(mut osm_stops): Json<Vec<requests::OsmStop>>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut partial_updates = 0;

    for osm_stop in osm_stops.iter_mut() {
        if osm_stop.history.is_empty() {
            return Err(Error::ValidationFailure(
                "Received node with an empty history".to_string(),
            ));
        }
        osm_stop.history.sort_by_key(|h| h.version);

        let version_count = osm_stop.history.len();
        let last_version = osm_stop.history.last().unwrap();
        if version_count != last_version.version as usize {
            partial_updates += 1;
        }
    }

    if partial_updates > 0 {
        let histories = sql::fetch_osm_stop_histories(&state.pool).await?;
        for osm_stop in osm_stops.iter_mut() {
            osm_stop.history.sort_by_key(|h| h.version);
            let version_count = osm_stop.history.len();
            let last_version = osm_stop.history.last().unwrap();
            let is_partial = version_count != last_version.version as usize;

            if is_partial {
                if let Some(history) = histories.get(&osm_stop.id) {
                    let merged_history = history
                        .iter()
                        .chain(osm_stop.history.iter())
                        .sorted_by_key(|h| h.version)
                        .dedup_by(|a, b| a.version == b.version)
                        .cloned()
                        .collect();
                    osm_stop.history = sqlx::types::Json(merged_history);
                }
            }
        }
    }

    // Upsert in chunks to avoid exceeding the query param limit
    for chunk in osm_stops.chunks(5000) {
        sql::upsert_osm_stops(&state.pool, chunk).await?
    }

    Ok(())
}

pub(crate) async fn delete_osm_stop(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    claims: Option<auth::Claims>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

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
) -> Result<Json<Option<FullOsmStop>>, Error> {
    Ok(Json(
        sql::fetch_paired_osm_stop(&state.pool, iml_stop_id).await?,
    ))
}
