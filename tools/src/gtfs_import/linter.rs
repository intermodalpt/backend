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

use std::collections::HashSet;

use crate::gtfs;

pub(crate) fn lint_gtfs(data: &gtfs::Data) -> Vec<gtfs::Lint> {
    let mut lints = vec![];

    let mut used_stops = HashSet::new();
    data.stop_times.iter().for_each(|time| {
        used_stops.insert(&time.stop_id);
    });

    let declared_stops: HashSet<&gtfs::StopId> = data.stops.keys().collect();
    let unused_stops = declared_stops.difference(&used_stops);
    let dangling_stops = used_stops.difference(&declared_stops);
    unused_stops.for_each(|stop_id| {
        lints.push(gtfs::Lint::UnusedStop((*stop_id).clone()));
    });
    dangling_stops.for_each(|stop_id| {
        lints.push(gtfs::Lint::DanglingStopPointer((*stop_id).clone()));
    });

    data.trips.values().for_each(|trip| {
        if let Some(stops) = data.trip_stops.get(&trip.trip_id) {
            if stops.is_empty() {
                lints.push(gtfs::Lint::EmptyTrip(trip.trip_id.clone()));
            }
        } else {
            lints.push(gtfs::Lint::ServicelessTrip(trip.trip_id.clone()));
        }
    });

    data.route_pattern_clusters
        .iter()
        .for_each(|(_, route_clusters)| {
            route_clusters.iter().for_each(|cluster| {
                // assert!(!cluster.trips.is_empty());
                if cluster.stops.is_empty() {
                    for pattern_id in cluster.patterns.iter() {
                        lints
                            .push(gtfs::Lint::EmptyPattern(pattern_id.clone()));
                    }
                }
                if cluster.patterns.len() > 1 {
                    lints.push(gtfs::Lint::DuplicatedPattern(
                        cluster.patterns.clone(),
                    ))
                }
            });
        });

    data.routes.values().for_each(|route| {
        if let Some(cluster) = data.route_pattern_clusters.get(&route.route_id)
        {
            let contains_non_empty_patterns =
                cluster.iter().any(|cluster| !cluster.stops.is_empty());
            if !contains_non_empty_patterns || cluster.is_empty() {
                lints
                    .push(gtfs::Lint::PatternlessRoute(route.route_id.clone()));
            }
        } else {
            lints.push(gtfs::Lint::PatternlessRoute(route.route_id.clone()));
        }
    });
    lints
}
