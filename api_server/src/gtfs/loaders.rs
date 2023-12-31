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
use std::{fs, io};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use commons::models::gtfs::File;
use commons::models::{gtfs, operators};
use commons::utils::gtfs::calculate_gtfs_stop_sequence;

use super::models;
use crate::gtfs::models::TMLRoute;
use crate::operators::import::OperatorData;
use crate::Error;

pub(crate) fn gtfs_stops(
    operator: &operators::Operator,
) -> Result<Vec<gtfs::Stop>, Error> {
    let gtfs_root = operator.get_gtfs_root();
    let stops_path = File::Stops.prepend_root(&gtfs_root);

    if !stops_path.exists() {
        return Err(Error::NotFoundUpstream);
    }

    let f = fs::File::open(stops_path).unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        // .trim(csv::Trim::All)
        .from_reader(reader);

    Ok(rdr
        .deserialize()
        .map(Result::unwrap)
        .collect::<Vec<gtfs::Stop>>())
}

// Read trips from GTFS tile
pub(crate) fn gtfs_routes(
    operator: &operators::Operator,
) -> Result<Vec<gtfs::Route>, Error> {
    let gtfs_root = operator.get_gtfs_root();
    let routes_path = File::Routes.prepend_root(&gtfs_root);

    if !routes_path.exists() {
        return Err(Error::NotFoundUpstream);
    }
    let f = fs::File::open(routes_path).unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new().from_reader(reader);

    Ok(rdr
        .deserialize()
        .map(Result::unwrap)
        .collect::<Vec<gtfs::Route>>())
}

// Read trips from GTFS tile
pub(crate) fn gtfs_trips(
    operator: &operators::Operator,
) -> Result<Vec<gtfs::Trip>, Error> {
    let gtfs_root = operator.get_gtfs_root();
    let trips_path = File::Trips.prepend_root(&gtfs_root);

    if !trips_path.exists() {
        return Err(Error::NotFoundUpstream);
    }

    let f = fs::File::open(trips_path).unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        // .trim(csv::Trim::All)
        .from_reader(reader);

    Ok(rdr
        .deserialize()
        .map(Result::unwrap)
        .collect::<Vec<gtfs::Trip>>())
}

// Read stop times from GTFS tile
pub(crate) fn gtfs_stop_times(
    operator: &operators::Operator,
) -> Result<Vec<gtfs::StopTime>, Error> {
    let gtfs_root = operator.get_gtfs_root();
    let stop_times_path = File::StopTimes.prepend_root(&gtfs_root);

    if !stop_times_path.exists() {
        return Err(Error::NotFoundUpstream);
    }

    let f = fs::File::open(stop_times_path).unwrap();
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        // .trim(csv::Trim::All)
        .from_reader(reader);

    Ok(rdr
        .deserialize()
        .map(Result::unwrap)
        .collect::<Vec<gtfs::StopTime>>())
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

pub(crate) fn simplified_gtfs_routes(
    operator: &operators::Operator,
) -> Result<Vec<TMLRoute>, Error> {
    let gtfs_stop_times = gtfs_stop_times(operator)?;
    let gtfs_trips = gtfs_trips(operator)?;
    let gtfs_routes = gtfs_routes(operator)?;

    let gtfs_route_names = gtfs_routes
        .into_iter()
        .map(|route| (route.route_id, route.route_long_name))
        .collect::<HashMap<String, String>>();

    let trips_stop_seq = calculate_gtfs_stop_sequence(&gtfs_stop_times);

    let routes = gtfs_trips
        .into_iter()
        .into_group_map_by(|trip| trip.route_id.clone())
        .into_iter()
        .map(|(route_id, trips)| models::TMLRoute {
            id: route_id.clone(),
            name: gtfs_route_names.get(&route_id).cloned().unwrap_or_default(),
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

    Ok(routes)
}
