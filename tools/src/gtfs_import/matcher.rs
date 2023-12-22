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

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use crate::error::Error;
use crate::utils::stop_seq_error2;
use crate::{gtfs, iml};
use itertools::Itertools;

pub(crate) struct RouteMatchResult {
    pub(crate) route_id: iml::RouteId,
    pub(crate) matches: Vec<RouteMatch>,
    pub(crate) unmatched_gtfs: Vec<GtfsPatternData>,
    pub(crate) unmatched_iml: Vec<ImlSubrouteData>,
}

pub(crate) struct RouteMatch {
    pub(crate) gtfs: GtfsPatternData,
    pub(crate) iml: ImlSubrouteData,
    pub(crate) matches: usize,
    pub(crate) mismatches: usize,
}

pub(crate) struct GtfsPatternData {
    pub(crate) stop_ids: Vec<gtfs::StopId>,
    pub(crate) route_id: gtfs::RouteId,
    pub(crate) pattern_ids: Vec<gtfs::PatternId>,
    pub(crate) trip_ids: Vec<gtfs::TripId>,
    pub(crate) iml_stop_ids: Vec<Option<iml::StopId>>,
}

pub(crate) struct ImlSubrouteData {
    pub(crate) route_id: iml::RouteId,
    pub(crate) subroute_id: iml::SubrouteId,
    pub(crate) stop_ids: Vec<iml::StopId>,
}

pub(crate) async fn match_gtfs_routes(
    gtfs: &gtfs::Data,
    iml: &iml::Data,
    gtfs_to_iml_stops: &HashMap<gtfs::StopId, iml::StopId>,
) -> Result<Vec<RouteMatchResult>, Error> {
    let gtfs_routes_by_code = gtfs
        .routes
        .values()
        .into_group_map_by(|route| route.route_short_name.clone())
        .into_iter()
        .map(|(code, routes)| (code, routes))
        .collect::<HashMap<String, Vec<&gtfs::Route>>>();

    // Optional sorting, for determinism
    let iml_routes = iml
        .routes
        .values()
        .filter(|r| r.operator == 1)
        .sorted_by_key(|r| r.id);

    let mut matches = vec![];
    for iml_route in iml_routes {
        let res = match_gtfs_route(
            gtfs,
            iml_route,
            &gtfs_to_iml_stops,
            &gtfs_routes_by_code,
        )
        .await?;
        matches.push(res);
    }
    Ok(matches)
}

async fn match_gtfs_route(
    gtfs: &gtfs::Data,
    iml_route: &iml::Route,
    gtfs_to_iml_stops: &HashMap<gtfs::StopId, iml::StopId>,
    gtfs_routes_by_code: &HashMap<String, Vec<&gtfs::Route>>,
) -> Result<RouteMatchResult, Error> {
    struct SubrouteSummary {
        subroute_id: iml::SubrouteId,
        stop_ids: Vec<iml::StopId>,
        // Cached field to aid the comparisons
        stop_ids_as_option: Vec<Option<iml::StopId>>,
    }

    struct PatternSummary<'a> {
        route_id: &'a gtfs::RouteId,
        gtfs_stop_ids: &'a Vec<gtfs::StopId>,
        iml_stop_ids: Vec<Option<iml::StopId>>,
        patterns: &'a HashSet<gtfs::PatternId>,
        trips: &'a HashSet<gtfs::TripId>,
    }

    impl Debug for SubrouteSummary {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SubrouteData")
                .field("subroute_id", &self.subroute_id)
                //.field("iml_stop_ids", &self.iml_stop_ids)
                //.field("gtfs_stop_ids", &self.gtfs_stop_ids)
                .finish()
        }
    }

    impl<'a> Debug for PatternSummary<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PatternData")
                .field("route_id", &self.route_id)
                //.field("gtfs_stop_ids", &self.gtfs_stop_ids)
                //.field("gtfs_stop_ids_as_option", &self.gtfs_stop_ids_as_option)
                .field("patterns", &self.patterns)
                //.field("trips", &self.trips)
                .finish()
        }
    }

    let iml_subroute_stops = iml::fetch_subroute_stops(iml_route.id).await?;

    let mut iml_subroute_data = vec![];
    let mut gtfs_patterns_data = vec![];

    for iml_subroute in iml_route.subroutes.iter() {
        let iml_stops = iml_subroute_stops
            .get(&iml_subroute.id)
            .unwrap_or(&vec![])
            .clone();
        iml_subroute_data.push(SubrouteSummary {
            subroute_id: iml_subroute.id,
            stop_ids_as_option: iml_stops.iter().map(|s| Some(*s)).collect(),
            stop_ids: iml_stops,
        });
    }

    // Get the GTFS routes that match the IML route code
    let gtfs_route_cluster =
        gtfs_routes_by_code.get(iml_route.code.as_ref().unwrap());

    if gtfs_route_cluster.is_none() {
        println!(
            "No GTFS route matches IML route {}",
            iml_route.code.as_ref().unwrap()
        );

        let unmatched_iml = iml_subroute_data
            .into_iter()
            .map(|subroute| ImlSubrouteData {
                route_id: iml_route.id,
                subroute_id: subroute.subroute_id,
                stop_ids: subroute.stop_ids.clone(),
            })
            .collect();

        return Ok(RouteMatchResult {
            route_id: iml_route.id,
            matches: vec![],
            unmatched_gtfs: vec![],
            unmatched_iml,
        });
    }
    let gtfs_route_cluster = gtfs_route_cluster.unwrap();

    // Get the unique patterns for that route
    for gtfs_route in gtfs_route_cluster.iter() {
        if let Some(gtfs_pattern_cluster) =
            gtfs.route_pattern_clusters.get(&gtfs_route.route_id)
        {
            for cluster in gtfs_pattern_cluster {
                println!(
                    "\t\tGTFS#{:?} - {:?}",
                    // First pattern or if none first trip
                    cluster
                        .patterns
                        .iter()
                        .next()
                        .or(cluster.trips.iter().next())
                        .map(|id| id.as_ref())
                        .unwrap_or("???"),
                    cluster.stops
                );

                let iml_stop_ids = cluster
                    .stops
                    .iter()
                    .map(|gtfs_stop_id| {
                        let iml_stop_id =
                            gtfs_to_iml_stops.get(gtfs_stop_id).cloned();
                        if iml_stop_id.is_none() {
                            println!("Missing GTFS stop {}", gtfs_stop_id);
                        }
                        iml_stop_id
                    })
                    .collect::<Vec<Option<iml::StopId>>>();

                gtfs_patterns_data.push(PatternSummary {
                    route_id: &gtfs_route.route_id,
                    gtfs_stop_ids: &cluster.stops,
                    iml_stop_ids,
                    patterns: &cluster.patterns,
                    trips: &cluster.trips,
                });
            }
        }
    }

    // [IML index][GTFS index]
    let mut matches =
        vec![
            vec![(usize::MAX, usize::MAX); gtfs_patterns_data.len()];
            iml_subroute_data.len()
        ];

    for (iml_idx, iml_subroute) in iml_subroute_data.iter().enumerate() {
        for (gtfs_idx, gtfs_pattern) in gtfs_patterns_data.iter().enumerate() {
            matches[iml_idx][gtfs_idx] = stop_seq_error2(
                &gtfs_pattern.iml_stop_ids,
                &iml_subroute.stop_ids_as_option,
            );
        }
    }

    let mut matched_indices = vec![];

    for (iml_idx, iml_subroute) in matches.iter().enumerate() {
        let mut min_idx = -1;
        let mut min_mismatches = usize::MAX;

        for (iml_idx, (_, mismatches)) in iml_subroute.iter().enumerate() {
            if *mismatches < min_mismatches {
                min_mismatches = *mismatches;
                min_idx = iml_idx as i32;
            }
        }

        if min_idx == -1 {
            // FIXME THIS IS WRONG
            continue;
        }

        matched_indices.push((iml_idx, min_idx as usize));
    }

    let produced_matches = matched_indices.len();
    let used_iml_idxs = matched_indices
        .iter()
        .map(|(iml_idx, _)| *iml_idx)
        .collect::<HashSet<_>>();
    let used_gtfs_idxs = matched_indices
        .iter()
        .map(|(_, gtfs_idx)| *gtfs_idx)
        .collect::<HashSet<_>>();

    if used_gtfs_idxs.len() != produced_matches
        || used_iml_idxs.len() != produced_matches
    {
        println!(
            "Route {} has {} subroutes, but only {} were matched",
            iml_route.code.as_ref().unwrap(),
            iml_subroute_data.len(),
            produced_matches
        );
    }

    let matches = matched_indices
        .into_iter()
        .map(|(iml_idx, gtfs_idx)| RouteMatch {
            gtfs: GtfsPatternData {
                stop_ids: gtfs_patterns_data[gtfs_idx].gtfs_stop_ids.clone(),
                route_id: gtfs_patterns_data[gtfs_idx].route_id.clone(),
                pattern_ids: gtfs_patterns_data[gtfs_idx]
                    .patterns
                    .iter()
                    .cloned()
                    .collect(),
                trip_ids: gtfs_patterns_data[gtfs_idx]
                    .trips
                    .iter()
                    .cloned()
                    .collect(),
                iml_stop_ids: gtfs_patterns_data[gtfs_idx].iml_stop_ids.clone(),
            },
            iml: ImlSubrouteData {
                route_id: iml_route.id,
                subroute_id: iml_subroute_data[iml_idx].subroute_id,
                stop_ids: iml_subroute_data[iml_idx].stop_ids.clone(),
            },
            matches: matches[iml_idx][gtfs_idx].0,
            mismatches: matches[iml_idx][gtfs_idx].1,
        })
        .collect();

    let unmatched_gtfs = gtfs_patterns_data
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| !used_gtfs_idxs.contains(idx))
        .map(|(_, pattern)| GtfsPatternData {
            stop_ids: pattern.gtfs_stop_ids.clone(),
            route_id: pattern.route_id.clone(),
            pattern_ids: pattern.patterns.iter().cloned().collect(),
            trip_ids: pattern.trips.iter().cloned().collect(),
            iml_stop_ids: pattern.iml_stop_ids.clone(),
        })
        .collect();

    let unmatched_iml = iml_subroute_data
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| !used_iml_idxs.contains(idx))
        .map(|(_, subroute)| ImlSubrouteData {
            route_id: iml_route.id,
            subroute_id: subroute.subroute_id,
            stop_ids: subroute.stop_ids.clone(),
        })
        .collect();

    Ok(RouteMatchResult {
        route_id: iml_route.id,
        matches,
        unmatched_gtfs,
        unmatched_iml,
    })
}
