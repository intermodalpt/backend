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

use crate::gtfs;
use std::collections::HashSet;

#[derive(Debug)]
pub(crate) enum GtfsLint {
    ServicelessTrip(gtfs::TripId),
    EmptyTrip(gtfs::TripId),
    EmptyPattern(gtfs::PatternId),
    PatternlessRoute(gtfs::RouteId),
    DuplicatedPattern(HashSet<gtfs::PatternId>),
    UnusedStop(gtfs::StopId),
    DanglingStopPointer(gtfs::StopId),
}

pub(crate) fn lint_gtfs(data: &gtfs::Data) -> Vec<GtfsLint> {
    let mut lints = vec![];

    let mut used_stops = HashSet::new();
    data.stop_times.iter().for_each(|time| {
        used_stops.insert(&time.stop_id);
    });

    let declared_stops: HashSet<&gtfs::StopId> = data.stops.keys().collect();
    let unused_stops = declared_stops.difference(&used_stops);
    let dangling_stops = used_stops.difference(&declared_stops);
    unused_stops.for_each(|stop_id| {
        lints.push(GtfsLint::UnusedStop((*stop_id).clone()));
    });
    dangling_stops.for_each(|stop_id| {
        lints.push(GtfsLint::DanglingStopPointer((*stop_id).clone()));
    });

    data.trips.values().for_each(|trip| {
        if let Some(stops) = data.trip_stops.get(&trip.trip_id) {
            if stops.is_empty() {
                lints.push(GtfsLint::EmptyTrip(trip.trip_id.clone()));
            }
        } else {
            lints.push(GtfsLint::ServicelessTrip(trip.trip_id.clone()));
        }
    });

    data.route_pattern_clusters
        .iter()
        .for_each(|(_, route_clusters)| {
            route_clusters.iter().for_each(|cluster| {
                assert!(cluster.trips.len() > 0);
                if cluster.stops.len() == 0 {
                    for pattern_id in cluster.patterns.iter() {
                        lints.push(GtfsLint::EmptyPattern(pattern_id.clone()));
                    }
                }
                if cluster.patterns.len() > 1 {
                    lints.push(GtfsLint::DuplicatedPattern(
                        cluster.patterns.clone(),
                    ))
                }
            });
        });

    data.routes.values().for_each(|route| {
        if let Some(cluster) = data.route_pattern_clusters.get(&route.route_id)
        {
            let contains_non_empty_patterns =
                cluster.iter().any(|cluster| cluster.stops.len() > 0);
            if !contains_non_empty_patterns || cluster.len() == 0 {
                lints.push(GtfsLint::PatternlessRoute(route.route_id.clone()));
            }
        } else {
            lints.push(GtfsLint::PatternlessRoute(route.route_id.clone()));
        }
    });
    lints
}
