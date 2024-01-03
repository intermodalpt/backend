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

#![allow(dead_code)]

use itertools::Itertools;
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::OnceLock;

pub(crate) use commons::models::gtfs::{
    self, Lint, PatternId, Route, RouteId, Stop, StopId, StopTime, Trip, TripId,
};

use crate::error::Error;

pub(crate) struct Data {
    pub(crate) stops: HashMap<StopId, Stop>,
    pub(crate) routes: HashMap<RouteId, Route>,
    pub(crate) trips: HashMap<TripId, Trip>,
    pub(crate) stop_times: Vec<StopTime>,
    // Calculated data
    pub(crate) trip_stops: HashMap<TripId, Vec<StopId>>,
    pub(crate) route_pattern_clusters: HashMap<RouteId, Vec<PatternCluster>>,
}
pub(crate) struct PatternCluster {
    pub(crate) stops: Vec<StopId>,
    pub(crate) patterns: HashSet<PatternId>,
    pub(crate) trips: HashSet<TripId>,
    pub(crate) headsigns: HashSet<String>,
}

pub(crate) fn load_gtfs(root: &PathBuf) -> Result<Data, Error> {
    let (gtfs_stops, gtfs_routes, gtfs_trips, gtfs_times) =
        load_gtfs_files(root)?;

    let trip_stops = gtfs_times
        .iter()
        .into_group_map_by(|time| time.trip_id.clone())
        .into_iter()
        .map(|(trip_id, times)| {
            (
                trip_id,
                times
                    .into_iter()
                    .sorted_by_key(|time| time.stop_sequence)
                    .map(|time| time.stop_id.clone())
                    .collect::<Vec<StopId>>(),
            )
        })
        .collect::<HashMap<TripId, Vec<StopId>>>();

    let route_pattern_clusters = gtfs_trips
        .iter()
        // Group trips by route
        .into_group_map_by(|trip| &trip.route_id)
        .into_iter()
        .map(|(route_id, trips)| {
            // Cluster patterns per trip
            let mut clusters = vec![];

            let mut trips_by_stops = HashMap::new();
            trips.into_iter().for_each(|trip| {
                let trip_stops =
                    trip_stops.get(&trip.trip_id).unwrap_or(&vec![]).clone();
                if !trip_stops.is_empty() {
                    trips_by_stops
                        .entry(trip_stops)
                        .or_insert(vec![])
                        .push(trip);
                }
            });
            trips_by_stops.into_iter().for_each(|(stops, trips)| {
                let mut pattern_ids = HashSet::new();
                let mut trip_ids = HashSet::new();
                let mut headsigns = HashSet::new();

                trips.into_iter().for_each(|trip| {
                    pattern_ids.insert(trip.pattern_id.clone());
                    trip_ids.insert(trip.trip_id.clone());
                    headsigns.insert(trip.trip_headsign.clone());
                });

                let cluster = PatternCluster {
                    stops,
                    patterns: pattern_ids,
                    trips: trip_ids,
                    headsigns,
                };
                clusters.push(cluster);
            });

            (route_id.clone(), clusters)
        })
        .collect::<HashMap<RouteId, _>>();

    let gtfs = Data {
        stops: gtfs_stops
            .into_iter()
            .map(|stop| (stop.stop_id.clone(), stop))
            .collect(),
        routes: gtfs_routes
            .into_iter()
            .map(|route| (route.route_id.clone(), route))
            .collect(),
        trips: gtfs_trips
            .into_iter()
            .map(|trip| (trip.trip_id.clone(), trip))
            .collect(),
        stop_times: gtfs_times,

        trip_stops,
        route_pattern_clusters,
    };
    Ok(gtfs)
}
fn load_gtfs_files(
    root: &PathBuf,
) -> Result<(Vec<Stop>, Vec<Route>, Vec<Trip>, Vec<StopTime>), Error> {
    let gtfs_stops: OnceLock<Result<Vec<Stop>, Error>> = OnceLock::new();
    let gtfs_times: OnceLock<Result<Vec<StopTime>, Error>> = OnceLock::new();
    let gtfs_routes: OnceLock<Result<Vec<Route>, Error>> = OnceLock::new();
    let gtfs_trips: OnceLock<Result<Vec<Trip>, Error>> = OnceLock::new();

    rayon::scope(|s| {
        s.spawn(|_| {
            let _ = gtfs_stops.set(deserialize_gtfs_entity(
                &gtfs::File::Stops.prepend_root(root),
            ));
        });
        s.spawn(|_| {
            let _ = gtfs_times.set(deserialize_gtfs_entity(
                &gtfs::File::StopTimes.prepend_root(root),
            ));
        });
        s.spawn(|_| {
            let _ = gtfs_routes.set(deserialize_gtfs_entity(
                &gtfs::File::Routes.prepend_root(root),
            ));
        });
        s.spawn(|_| {
            let _ = gtfs_trips.set(deserialize_gtfs_entity(
                &gtfs::File::Trips.prepend_root(root),
            ));
        });
    });
    let gtfs_stops = gtfs_stops.into_inner().unwrap()?;
    let gtfs_times = gtfs_times.into_inner().unwrap()?;
    let gtfs_routes = gtfs_routes.into_inner().unwrap()?;
    let gtfs_trips = gtfs_trips.into_inner().unwrap()?;

    Ok((gtfs_stops, gtfs_routes, gtfs_trips, gtfs_times))
}

fn deserialize_gtfs_entity<E: DeserializeOwned>(
    path: &PathBuf,
) -> Result<Vec<E>, Error> {
    let res = csv::Reader::from_path(path)
        .map(|mut stops| {
            stops
                .deserialize::<E>()
                .map(|stop| stop.map_err(|err| Error::Deserialization(err)))
                .collect::<Result<Vec<E>, Error>>()
        })
        .map_err(|err| Error::Files(err.to_string()))?;

    println!("Done deserializing {}", path.display());
    res
}
