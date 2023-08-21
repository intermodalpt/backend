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

use axum::extract::{Path, Query, State};
use axum::Json;

use commons::models::gtfs;
use commons::models::gtfs::GtfsFile;
use commons::utils::gtfs::{
    calculate_gtfs_stop_sequence, calculate_stop_sliding_windows,
};
use commons::utils::{git, gtfs as gtfs_utils};

use super::{loaders, models, sql};
use crate::operators::import::OperatorData;
use crate::operators::sql as operators_sql;
use crate::{auth, operators, AppState, Error};

pub(crate) async fn post_update_operator_gtfs(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    claims: Option<auth::Claims>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }

    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let operator =
        operators_sql::fetch_operator(&state.pool, operator_id).await?;
    if operator.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let operator = operator.unwrap();

    match operator.tag.as_str() {
        "cmet" => {
            operators::import::update_operator_meta(operator_id, |meta| {
                let path = format!("./data/operators/{}/gtfsrepo", operator_id);
                let url = "https://github.com/carrismetropolitana/gtfs";
                let remote_name = "origin";
                let remote_branch = "live";

                let version_date =
                    git::update_repo(url, &path, remote_name, remote_branch)
                        .map_err(|e| Error::Processing(e))?;

                meta.last_gtfs = Some(version_date);
                if meta.last_gtfs != Some(version_date) {
                    gtfs_utils::extract_gtfs(
                        &format!("./data/operators/{}/gtfsrepo/CarrisMetropolitana.zip", operator_id),
                        &format!("./data/operators/{}/gtfs", operator_id),
                    );
                    Ok(true)
                } else {
                    Ok(false)
                }
            })?;
        }
        _ => {
            return Err(Error::NotFoundUpstream);
        }
    }

    Ok(())
}

pub(crate) async fn get_gtfs_stops(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Arc<Vec<gtfs::GTFSStop>>>, Error> {
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

pub(crate) async fn tml_match_stop(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    params: Query<models::MatchVerification>,
    Path((stop_id, gtfs_id)): Path<(i64, String)>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    Ok(sql::gtfs_match(
        &state.pool,
        stop_id,
        gtfs_id,
        params.verified,
        &params.source.to_string(),
    )
    .await?)
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
) -> Result<Json<Vec<Vec<u32>>>, Error> {
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
    let stop_times_path = GtfsFile::StopTimes.prepend_root(&gtfs_root);

    let f = fs::File::open(stop_times_path).unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader);

    let gtfs_stop_times = rdr
        .deserialize()
        .into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<gtfs::GTFSStopTimes>>();

    let trips_stop_seq = calculate_gtfs_stop_sequence(&gtfs_stop_times);
    let sliding_windows = calculate_stop_sliding_windows(&trips_stop_seq);

    Ok(Json(sliding_windows))
}
