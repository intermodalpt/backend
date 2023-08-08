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

use std::collections::HashMap;
use std::sync::Arc;
use std::{fs, io};

use axum::extract::{Path, Query, State};
use axum::Json;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use super::{logic, models, sql};
use crate::{auth, AppState, Error};

pub(crate) async fn tml_get_stops(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::TMLStop>>, Error> {
    Ok(Json(sql::fetch_gtfs_stops(&state.pool).await?))
}

pub(crate) async fn tml_get_gtfs_stops(
    State(state): State<AppState>,
) -> Result<Json<Arc<Vec<models::GTFSStop>>>, Error> {
    let gtfs_stops = state
        .cached
        .gtfs_stops
        .get_or_init(|| {
            let f = fs::File::open("gtfs/stops.txt").unwrap();
            let reader = io::BufReader::new(f);

            let csv_reader = csv::ReaderBuilder::new()
                // .trim(csv::Trim::All)
                .from_reader(reader);
            let mut rdr = csv_reader;

            let gtfs_stops = rdr
                .deserialize()
                .into_iter()
                .map(|result| result.unwrap())
                .collect();

            Arc::new(gtfs_stops)
        })
        .clone();

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

// Read trips from GTFS tile
fn tml_gtfs_routes() -> Vec<models::GTFSRoute> {
    let f = fs::File::open("gtfs/routes.txt").unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        // .trim(csv::Trim::All)
        .from_reader(reader);

    rdr.deserialize()
        .into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<models::GTFSRoute>>()
}

// Read trips from GTFS tile
fn tml_gtfs_trips() -> Vec<models::GTFSTrips> {
    let f = fs::File::open("gtfs/trips.txt").unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        // .trim(csv::Trim::All)
        .from_reader(reader);

    rdr.deserialize()
        .into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<models::GTFSTrips>>()
}

// Read stop times from GTFS tile
fn load_gtfs_stop_times() -> Vec<models::GTFSStopTimes> {
    let f = fs::File::open("gtfs/stop_times.txt").unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        // .trim(csv::Trim::All)
        .from_reader(reader);

    rdr.deserialize()
        .into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<models::GTFSStopTimes>>()
}

// Have regex as a static
static SUBROUTE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[\w\d]*_(?P<subroute>\d{4}_\d_\d)_").unwrap());

fn simplified_trip_id(trip_id: &str) -> String {
    if let Some(caps) = SUBROUTE_RE.captures(trip_id) {
        caps.name("subroute").unwrap().as_str().to_string()
    } else {
        trip_id.to_string()
    }
}

pub(crate) async fn tml_gtfs_route_trips(
    State(state): State<AppState>,
) -> Json<Arc<Vec<models::TMLRoute>>> {
    let tml_routes = state
        .cached
        .tml_routes
        .get_or_init(|| {
            let gtfs_stop_times = load_gtfs_stop_times();
            let gtfs_trips = tml_gtfs_trips();
            let gtfs_routes = tml_gtfs_routes();

            let gtfs_route_names = gtfs_routes
                .into_iter()
                .map(|route| (route.route_id, route.route_long_name))
                .collect::<HashMap<String, String>>();

            let trips_stop_seq =
                logic::calculate_gtfs_stop_sequence(&gtfs_stop_times);

            let routes = gtfs_trips
                .into_iter()
                .into_group_map_by(|trip| trip.route_id.clone())
                .into_iter()
                .map(|(route_id, trips)| models::TMLRoute {
                    id: route_id.clone(),
                    name: gtfs_route_names
                        .get(&route_id)
                        .cloned()
                        .unwrap_or_default(),
                    trips: trips
                        .into_iter()
                        .map(|trip| models::TMLTrip {
                            headsign: trip.trip_headsign,
                            stops: trips_stop_seq
                                .get(&trip.trip_id)
                                .cloned()
                                .unwrap_or_default(),
                            id: simplified_trip_id(&trip.trip_id),
                        })
                        .unique()
                        .collect(),
                })
                .collect::<Vec<models::TMLRoute>>();

            Arc::new(routes)
        })
        .clone();

    Json(tml_routes)
}

pub(crate) async fn tml_gtfs_stop_sliding_windows(
) -> Result<Json<Vec<Vec<u32>>>, Error> {
    let f = fs::File::open("gtfs/stop_times.txt").unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader);

    let gtfs_stop_times = rdr
        .deserialize()
        .into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<models::GTFSStopTimes>>();

    let trips_stop_seq = logic::calculate_gtfs_stop_sequence(&gtfs_stop_times);
    let sliding_windows =
        logic::calculate_stop_sliding_windows(&trips_stop_seq);

    Ok(Json(sliding_windows))
}
