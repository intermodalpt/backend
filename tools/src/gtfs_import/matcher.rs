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

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use strsim::normalized_levenshtein;

use commons::models::gtfs as gtfs_commons;

use crate::error::Error;
use crate::utils::stop_seq_error;
use crate::{gtfs, iml};

/// The intersection between an IML route and GTFS data
pub(crate) struct ImlGtfsRouteIntersection<'iml, 'gtfs> {
    // Some of these fields are unnecessary since the subroute has a ref
    pub(crate) iml_route: &'iml iml::Route,
    pub(crate) iml_route_id: iml::RouteId,
    pub(crate) subroutes: Vec<SubrouteSummary<'iml>>,
    pub(crate) patterns_cluster: Vec<PatternCluster<'gtfs>>,
}

#[derive(Clone)]
pub(crate) struct SubrouteSummary<'iml> {
    // Some of these fields are unnecessary since the subroute has a ref
    pub(crate) subroute: &'iml iml::Subroute,
    pub(crate) subroute_id: iml::SubrouteId,
    pub(crate) gtfs_patterns: HashSet<gtfs::PatternId>,
    pub(crate) stop_ids: &'iml [iml::StopId],
}

#[derive(Clone)]
pub(crate) struct PatternCluster<'gtfs> {
    pub(crate) gtfs_stop_ids: &'gtfs Vec<gtfs::StopId>,
    pub(crate) iml_stop_ids: Vec<iml::StopId>,
    pub(crate) route_id: &'gtfs gtfs::RouteId,
    pub(crate) patterns: &'gtfs HashSet<gtfs::PatternId>,
    pub(crate) trips: &'gtfs HashSet<gtfs::TripId>,
    pub(crate) headsigns: &'gtfs HashSet<String>,
}

#[derive(Debug)]
pub(crate) struct RoutePairing<'iml, 'gtfs> {
    pub(crate) route_id: iml::RouteId,
    pub(crate) subroute_pairings: Vec<SubroutePatternPairing<'iml, 'gtfs>>,
    pub(crate) unpaired_gtfs: Vec<GtfsPatternData<'gtfs>>,
    pub(crate) unpaired_iml: Vec<ImlSubrouteData<'iml>>,
}

#[derive(Debug)]
pub(crate) struct SubroutePatternPairing<'iml, 'gtfs> {
    pub(crate) gtfs: GtfsPatternData<'gtfs>,
    pub(crate) iml: ImlSubrouteData<'iml>,
    pub(crate) stop_matches: usize,
    pub(crate) stop_mismatches: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct GtfsPatternData<'gtfs> {
    pub(crate) stop_ids: &'gtfs [gtfs::StopId],
    pub(crate) route_id: &'gtfs gtfs::RouteId,
    pub(crate) pattern_ids: &'gtfs HashSet<gtfs::PatternId>,
    pub(crate) headsigns: &'gtfs HashSet<String>,
    pub(crate) trip_ids: &'gtfs HashSet<gtfs::TripId>,
    pub(crate) iml_stop_ids: Vec<iml::StopId>,
}

impl From<GtfsPatternData<'_>> for gtfs_commons::SubrouteValidation {
    fn from(cluster: GtfsPatternData<'_>) -> Self {
        gtfs_commons::SubrouteValidation {
            gtfs_cluster: gtfs_commons::PatternCluster {
                stops: cluster.stop_ids.to_vec(),
                headsigns: cluster.headsigns.clone(),
                patterns: cluster.pattern_ids.clone(),
                trips: cluster.trip_ids.clone(),
            },
            stops: cluster.iml_stop_ids.to_vec(),
        }
    }
}

impl From<&SubroutePatternPairing<'_, '_>>
    for gtfs_commons::SubrouteValidation
{
    fn from(pairing: &SubroutePatternPairing<'_, '_>) -> Self {
        gtfs_commons::SubrouteValidation {
            gtfs_cluster: gtfs_commons::PatternCluster {
                stops: pairing.gtfs.stop_ids.to_vec(),
                headsigns: pairing.gtfs.headsigns.clone(),
                patterns: pairing.gtfs.pattern_ids.clone(),
                trips: pairing.gtfs.trip_ids.clone(),
            },
            stops: pairing.gtfs.iml_stop_ids.to_vec(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ImlSubrouteData<'iml> {
    pub(crate) subroute_id: iml::SubrouteId,
    pub(crate) stop_ids: &'iml [iml::StopId],
}

/// Produces reasonable guesses of which IML subroutes correspond to which
/// GTFS routes/patterns. Also points what can be dangling data.
/// It has a relatively high minimum pairing threshold.
/// It will never pair silly data
pub(crate) async fn cross_reference_routes<'iml, 'gtfs>(
    gtfs: &'gtfs gtfs::Data,
    iml: &'iml iml::Data,
    operator_id: i32,
) -> Result<Vec<RoutePairing<'iml, 'gtfs>>, Error> {
    let gtfs_remaps = &gtfs::OVERRIDES.get().unwrap().remaps;

    // Create a dictionary of IML stops to GTFS stops
    let iml_to_gtfs_stops = iml
        .stops
        .iter()
        .map(|(iml_id, iml_stop)| {
            let gtfs_id = iml_stop
                .operators
                .iter()
                .find(|rel| rel.operator_id == operator_id)
                .map(|rel| rel.stop_ref.as_ref().unwrap().clone());
            (iml_id, gtfs_id)
        })
        .filter_map(|(iml_id, gtfs_id)| {
            if let Some(id) = &gtfs_id {
                if !gtfs.stops.contains_key(id) {
                    println!(
                        "Stop {} is linked against an unknown GTFS stop {}",
                        iml_id, id
                    );
                    // TODO add hint to unlink
                    None
                } else {
                    Some((*iml_id, gtfs_id.unwrap()))
                }
            } else {
                None
            }
        })
        .collect::<HashMap<i32, gtfs::StopId>>();

    // Create a dictionary of GTFS stops to IML stops
    let mut gtfs_to_iml_stops = iml_to_gtfs_stops
        .iter()
        .map(|(iml_id, gtfs_id)| (gtfs_id.clone(), *iml_id))
        .collect::<HashMap<gtfs::StopId, i32>>();

    // Change GTFS->IML links based on the configured overrides
    gtfs_remaps.iter().for_each(|(gtfs_id, iml_id)| {
        if !iml.stops.contains_key(iml_id) {
            panic!("IML stop {} does not exist", iml_id);
        }
        gtfs_to_iml_stops
            .entry(gtfs_id.to_string())
            .or_insert(*iml_id);
    });

    // Dictionary of GTFS routes by their code (that's usually the route number)
    let gtfs_routes_by_code = gtfs
        .routes
        .values()
        .into_group_map_by(|route| route.route_short_name.clone())
        .into_iter()
        .collect::<HashMap<String, Vec<&gtfs::Route>>>();

    // The sorting step is not necessary for the algorithm
    // but determinism is good for debugging.
    // TODO put behind debug-build flag
    let iml_routes = iml
        .routes
        .values()
        .filter(|r| r.operator == operator_id)
        .sorted_by_key(|r| r.id);

    let mut paired_routes = vec![];
    for iml_route in iml_routes {
        // Intersect the IML route with GTFS data
        let route_intersection = cross_intersect_route(
            gtfs,
            iml_route,
            &gtfs_to_iml_stops,
            &gtfs_routes_by_code,
        )
        .await?;

        paired_routes.push(pair_route_intersection(route_intersection));
    }
    Ok(paired_routes)
}

/// Aggregates every bit of IML and GTFS data for a given route
/// It intersects an IML route with GTFS routes/patterns
/// (convenient to match IML subroutes and GTFS patterns later on)
async fn cross_intersect_route<'iml, 'gtfs>(
    gtfs: &'gtfs gtfs::Data,
    iml_route: &'iml iml::Route,
    gtfs_to_iml_stops: &HashMap<gtfs::StopId, iml::StopId>,
    gtfs_routes_by_code: &HashMap<String, Vec<&'gtfs gtfs::Route>>,
) -> Result<ImlGtfsRouteIntersection<'iml, 'gtfs>, Error> {
    let suppressions = &gtfs::OVERRIDES.get().unwrap().suppressions;
    let mut iml_subroute_data = vec![];
    let mut gtfs_patterns_data = vec![];

    for iml_subroute in iml_route.subroutes.iter() {
        iml_subroute_data.push(SubrouteSummary {
            subroute_id: iml_subroute.id,
            subroute: iml_subroute,
            stop_ids: &iml_subroute.stops,
            gtfs_patterns: iml_subroute.validation.gtfs.as_ref().map_or_else(
                || HashSet::new(),
                |v| v.patterns.iter().cloned().collect::<HashSet<_>>(),
            ),
        });
    }

    // Get the GTFS routes that match the IML route code
    let gtfs_route_cluster =
        gtfs_routes_by_code.get(iml_route.code.as_ref().unwrap());

    let Some(gtfs_route_cluster) = gtfs_route_cluster else {
        println!(
            "No GTFS route matches IML route {}",
            iml_route.code.as_ref().unwrap()
        );

        return Ok(ImlGtfsRouteIntersection {
            iml_route,
            iml_route_id: iml_route.id,
            subroutes: iml_subroute_data,
            patterns_cluster: gtfs_patterns_data,
        });
    };

    // Get the unique patterns for that route
    for gtfs_route in gtfs_route_cluster.iter() {
        if let Some(gtfs_pattern_cluster) =
            gtfs.route_pattern_clusters.get(&gtfs_route.route_id)
        {
            for cluster in gtfs_pattern_cluster {
                let mut iml_stop_ids = cluster
                    .stops
                    .iter()
                    .filter_map(|gtfs_stop_id| {
                        if suppressions.contains(&gtfs_stop_id) {
                            println!("Supressing GTFS stop {}", gtfs_stop_id);
                            return None;
                        }

                        let iml_stop_id =
                            gtfs_to_iml_stops.get(gtfs_stop_id).cloned();

                        let Some(stop_id) = iml_stop_id else {
                            return Some(Err(Error::MissingData(format!(
                                "Missing GTFS stop {}",
                                gtfs_stop_id
                            ))));
                        };

                        Some(Ok(stop_id))
                    })
                    .collect::<Result<Vec<iml::StopId>, Error>>()?;

                iml_stop_ids.dedup();

                gtfs_patterns_data.push(PatternCluster {
                    route_id: &gtfs_route.route_id,
                    gtfs_stop_ids: &cluster.stops,
                    iml_stop_ids,
                    patterns: &cluster.patterns,
                    trips: &cluster.trips,
                    headsigns: &cluster.headsigns,
                });
            }
        }
    }

    Ok(ImlGtfsRouteIntersection {
        iml_route,
        iml_route_id: iml_route.id,
        subroutes: iml_subroute_data,
        patterns_cluster: gtfs_patterns_data,
    })
}

/// This is the matcher workhorse
/// It takes the `ImlGtfsRouteIntersection` and takes a series of steps
/// to pair IML subroutes with GTFS patterns, from the most certain matches
/// to the longer shots.
/// It has a relatively high minimum threshold. It will never pair silly data
pub(crate) fn pair_route_intersection<'iml, 'gtfs>(
    route_intersection: ImlGtfsRouteIntersection<'iml, 'gtfs>,
) -> RoutePairing<'iml, 'gtfs> {
    let pattern_clusters = &route_intersection.patterns_cluster;
    let subroutes = &route_intersection.subroutes;

    // Check the results of past mappings and confirm that they're still valid
    let prematched_patterns = subroutes
        .iter()
        .flat_map(|subroute| &subroute.gtfs_patterns)
        .collect::<HashSet<_>>();
    let available_patterns = pattern_clusters
        .iter()
        .flat_map(|cluster| cluster.patterns)
        .collect::<HashSet<_>>();
    let valid_prematched_patterns = prematched_patterns
        .intersection(&available_patterns)
        .collect::<HashSet<_>>();
    let no_longer_available_patterns = prematched_patterns
        .difference(&available_patterns)
        .collect::<HashSet<_>>();

    // Matrix of matches and mismatches between IML subroutes and GTFS patterns
    // [IML index][GTFS index]
    let mut matches =
        vec![
            vec![(usize::MAX, usize::MAX); pattern_clusters.len()];
            subroutes.len()
        ];

    for (iml_idx, iml_subroute) in subroutes.iter().enumerate() {
        for (gtfs_idx, gtfs_pattern) in pattern_clusters.iter().enumerate() {
            matches[iml_idx][gtfs_idx] = stop_seq_error(
                &gtfs_pattern.iml_stop_ids,
                iml_subroute.stop_ids,
            );
        }
    }

    // Waiting to be promoted in a given stage (If no conflicts ensue)
    let mut strong_match_candidates = vec![];
    // Already promoted. Every match that hits this will be returned
    let mut strong_matches: Vec<_>;
    // Assignments from previous runs which are not all that strong anymore
    // (because data has changed elsewhere)
    let mut weak_past_assignments = vec![];

    // ### STAGE 1 ###
    // Raise the past-run matches to strong-candidate status
    for (iml_idx, iml_subroute) in subroutes.iter().enumerate() {
        let subroute = &subroutes[iml_idx];

        if !subroute.gtfs_patterns.is_empty() {
            let matched_clusters = pattern_clusters
                .iter()
                .enumerate()
                .filter(|(_, p)| {
                    !p.patterns.is_disjoint(&subroute.gtfs_patterns)
                })
                .collect::<Vec<(usize, &PatternCluster)>>();

            if matched_clusters.len() != 1 {
                // We could do better here...
                // but hey, better an explosion than wrong data
                panic!("GTFS integrity issue")
            }

            let (gtfs_idx, pattern_cluster) = matched_clusters[0];

            let pattern_stops_len = pattern_cluster.gtfs_stop_ids.len();
            let subroute_stops_len = iml_subroute.stop_ids.len();
            let (matches, mismatches) = matches[iml_idx][gtfs_idx];

            if is_strong_sequence_match(
                (matches, mismatches),
                subroute_stops_len,
                pattern_stops_len,
            ) {
                strong_match_candidates.push((iml_idx, gtfs_idx));
            } else {
                weak_past_assignments.push((iml_idx, gtfs_idx));
            }
        }
    }

    // Promote them to strong matches if the conditions make up for it
    strong_matches = strong_match_candidates
        .iter()
        .filter(|(iml_idx, gtfs_idx)| {
            let conflicts = has_conflicting_pair(
                &strong_match_candidates,
                *iml_idx,
                *gtfs_idx,
            );

            if conflicts {
                eprintln!(
                    "{} - {:?} had a conflict",
                    &subroutes[*iml_idx].subroute_id,
                    &pattern_clusters[*gtfs_idx].patterns
                );
            }

            !conflicts
        })
        .map(|(iml_idx, gtfs_idx)| (*iml_idx, *gtfs_idx))
        .collect();
    strong_match_candidates = vec![];

    // Have the already-strong match indexes in a convenient set
    let already_strong_iml_idxs = strong_matches
        .iter()
        .map(|(strong_iml_idx, _strong_gtfs_idx)| *strong_iml_idx)
        .collect::<HashSet<_>>();
    let already_strong_gtfs_idxs = strong_matches
        .iter()
        .map(|(_strong_iml_idx, strong_gtfs_idx)| *strong_gtfs_idx)
        .collect::<HashSet<_>>();

    // ### STAGE 2 ###
    // Raise the previously unmatched matches to strong-candidate status
    for (iml_idx, iml_subroute) in subroutes.iter().enumerate() {
        let subroute = &subroutes[iml_idx];
        if already_strong_iml_idxs.contains(&iml_idx) {
            continue;
        }

        // Do not attempt to pair against subroutes that are weakly paired
        if weak_past_assignments
            .iter()
            .any(|(weak_iml_idx, _)| weak_iml_idx == &iml_idx)
        {
            continue;
        }

        let subroute_matches = &matches[iml_idx];

        let strong_enough_pairs = subroute_matches
            .iter()
            .enumerate()
            // We're not going to attempt to match against already-strong matches
            .filter(|(gtfx_idx, _matches)| {
                // Still unsure between
                !already_strong_gtfs_idxs.contains(gtfx_idx)
                // And
                // let pattern_cluster = &pattern_clusters[*gtfx_idx];
                // !available_patterns.is_disjoint(pattern_cluster.patterns)
            })
            .filter_map(|(gtfx_idx, matches)| {
                let pattern = &pattern_clusters[gtfx_idx];
                is_strong_sequence_match(
                    *matches,
                    iml_subroute.stop_ids.len(),
                    pattern.gtfs_stop_ids.len(),
                )
                .then_some((iml_idx, gtfx_idx))
            })
            .collect_vec();

        // Calculate the most promising pair
        let best_hypothesis =
            strong_enough_pairs
                .iter()
                .min_by_key(|(iml_idx, gtfx_idx)| {
                    let (matches, mismatches) = matches[*iml_idx][*gtfx_idx];
                    // Penalize for both mismatches and length changes
                    mismatches as u32
                        + i32::abs_diff(
                            subroute.stop_ids.len() as i32,
                            (matches + mismatches) as i32,
                        )
                });
        if let Some(best_hypothesis) = best_hypothesis {
            strong_match_candidates.push(*best_hypothesis);
        }
    }

    // Promote candidates again (if the conditions make up for it)
    strong_match_candidates
        .iter()
        .for_each(|(iml_idx, gtfs_idx)| {
            // This is an impossible condition, but let's have it for now just in case
            if already_strong_gtfs_idxs.contains(gtfs_idx)
                || already_strong_iml_idxs.contains(iml_idx)
            {
                return;
            }

            if !has_conflicting_pair(
                &strong_match_candidates,
                *iml_idx,
                *gtfs_idx,
            ) {
                strong_matches.push((*iml_idx, *gtfs_idx));
            }
        });

    // ### STAGE 3 ###
    // Deal with the weirdos
    // Notable cases:
    // Weak assignments from the past pose no conflict -> Add them all
    // Only one pattern and subroute -> Match them
    // Two and the headsigns are very close to the subroute's -> Match them
    // One strong and one weak -> Match the strong

    weak_past_assignments
        .iter()
        .for_each(|(weak_iml_idx, weak_gtfs_idx)| {
            // Check if there are conflicts with any of the strong matches or other weak matches
            if !has_conflicting_pair(
                &strong_matches,
                *weak_iml_idx,
                *weak_gtfs_idx,
            ) && !has_conflicting_pair(
                &weak_past_assignments,
                *weak_iml_idx,
                *weak_gtfs_idx,
            ) {
                strong_matches.push((*weak_iml_idx, *weak_gtfs_idx))
            }
        });

    if strong_matches.is_empty() {
        if pattern_clusters.len() == 1 && subroutes.len() == 1 {
            strong_matches.push((0, 0));
        } else if pattern_clusters.len() == 2 && subroutes.len() == 2 {
            if let (Some(sr0_hs), Some(sr1_hs)) = (
                &subroutes[0].subroute.headsign,
                &subroutes[1].subroute.headsign,
            ) {
                const MIN_SIMILARITY: f64 = 0.9;
                let sr0_matches_p0 =
                    pattern_clusters[0].headsigns.iter().any(|hs| {
                        normalized_levenshtein(hs, sr0_hs) > MIN_SIMILARITY
                    });
                let sr0_matches_p1 =
                    pattern_clusters[1].headsigns.iter().any(|hs| {
                        normalized_levenshtein(hs, sr0_hs) > MIN_SIMILARITY
                    });
                let sr1_matches_p0 =
                    pattern_clusters[0].headsigns.iter().any(|hs| {
                        normalized_levenshtein(hs, sr1_hs) > MIN_SIMILARITY
                    });
                let sr1_matches_p1 =
                    pattern_clusters[1].headsigns.iter().any(|hs| {
                        normalized_levenshtein(hs, sr1_hs) > MIN_SIMILARITY
                    });

                if sr0_matches_p0
                    && sr1_matches_p1
                    && !sr0_matches_p1
                    && !sr1_matches_p0
                {
                    strong_matches.push((0, 0));
                    strong_matches.push((1, 1));
                } else if sr0_matches_p1
                    && sr1_matches_p0
                    && !sr0_matches_p0
                    && !sr1_matches_p1
                {
                    strong_matches.push((0, 1));
                    strong_matches.push((1, 0));
                }
            }
        }
    }

    let used_iml_idxs = strong_matches
        .iter()
        .map(|(iml_idx, _)| *iml_idx)
        .collect::<HashSet<_>>();
    let used_gtfs_idxs = strong_matches
        .iter()
        .map(|(_, gtfs_idx)| *gtfs_idx)
        .collect::<HashSet<_>>();

    let matches = strong_matches
        .into_iter()
        .map(|(iml_idx, gtfs_idx)| SubroutePatternPairing {
            gtfs: GtfsPatternData {
                stop_ids: pattern_clusters[gtfs_idx].gtfs_stop_ids,
                route_id: pattern_clusters[gtfs_idx].route_id,
                pattern_ids: pattern_clusters[gtfs_idx].patterns,
                headsigns: pattern_clusters[gtfs_idx].headsigns,
                trip_ids: pattern_clusters[gtfs_idx].trips,
                iml_stop_ids: pattern_clusters[gtfs_idx].iml_stop_ids.clone(),
            },
            iml: ImlSubrouteData {
                subroute_id: subroutes[iml_idx].subroute_id,
                stop_ids: subroutes[iml_idx].stop_ids,
            },
            stop_matches: matches[iml_idx][gtfs_idx].0,
            stop_mismatches: matches[iml_idx][gtfs_idx].1,
        })
        .collect();

    let unmatched_gtfs = pattern_clusters
        .iter()
        .enumerate()
        .filter(|(idx, _)| !used_gtfs_idxs.contains(idx))
        .map(|(_, pattern)| GtfsPatternData {
            stop_ids: pattern.gtfs_stop_ids,
            route_id: pattern.route_id,
            pattern_ids: pattern.patterns,
            headsigns: pattern.headsigns,
            trip_ids: pattern.trips,
            iml_stop_ids: pattern.iml_stop_ids.clone(),
        })
        .collect();

    let unmatched_iml = subroutes
        .iter()
        .enumerate()
        .filter(|(idx, _)| !used_iml_idxs.contains(idx))
        .map(|(_, subroute)| ImlSubrouteData {
            subroute_id: subroute.subroute_id,
            stop_ids: subroute.stop_ids,
        })
        .collect();

    RoutePairing {
        route_id: route_intersection.iml_route_id,
        subroute_pairings: matches,
        unpaired_gtfs: unmatched_gtfs,
        unpaired_iml: unmatched_iml,
    }
}

fn has_conflicting_pair(
    iml_gtfs_pairs: &[(usize, usize)],
    iml_idx: usize,
    gtfs_idx: usize,
) -> bool {
    iml_gtfs_pairs
        .iter()
        .any(|(other_iml_idx, other_gtfs_idx)| {
            iml_idx == *other_iml_idx && gtfs_idx != *other_gtfs_idx
                || iml_idx != *other_iml_idx && gtfs_idx == *other_gtfs_idx
        })
}

// A strong match:
// - Is not competing with another would-be strong match
// - Has a minimum length of 5 stops
// - Has a minium of 12 matches per 15 stops
// - Has a maximum of 2 mismatches per 15 stops for long sequences
// - Has a maximum of 10 mismatches total
// (This means at most 3 new stops or 2 removed stops per 15 stops)
fn is_strong_sequence_match(
    (matches, mismatches): (usize, usize),
    subroute_stops_len: usize,
    pattern_stops_len: usize,
) -> bool {
    const MIN_STOP_LEN: usize = 4;
    const MIN_MATCH_RATIO: f32 = 12.0 / 15.0;
    const MAX_MISMATCHES: usize = 10;

    if subroute_stops_len < MIN_STOP_LEN || pattern_stops_len < MIN_STOP_LEN {
        return false;
    }

    let max_stop_len = subroute_stops_len.max(pattern_stops_len);
    // An asymptote at 2/15 with some slack in smaller sequences
    // (Accepts a ratio of 23/30 in a 4 stop sequence 1/3 in a 10-stop sequence)
    let max_mismatch_ratio: f32 = 2.0 / 15.0 + (2.0 / max_stop_len as f32);

    if mismatches > MAX_MISMATCHES {
        return false;
    }
    if (matches as f32 / max_stop_len as f32) <= MIN_MATCH_RATIO {
        return false;
    }
    if (mismatches as f32 / max_stop_len as f32) >= max_mismatch_ratio {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_strong_sequence_match;

    #[test]
    fn test_is_strong_sequence_match() {
        // Null sequence
        assert!(!is_strong_sequence_match((0, 0), 0, 0));
        // Too short
        assert!(!is_strong_sequence_match((1, 0), 1, 1));
        assert!(!is_strong_sequence_match((0, 1), 1, 1));
        assert!(!is_strong_sequence_match((1, 1), 2, 2));
        assert!(!is_strong_sequence_match((2, 0), 2, 2));
        assert!(!is_strong_sequence_match((0, 2), 2, 2));
        assert!(!is_strong_sequence_match((3, 0), 3, 3));
        // Impossible but still bellow minium size
        assert!(!is_strong_sequence_match((10, 0), 4, 3));
        assert!(!is_strong_sequence_match((10, 0), 3, 4));

        // Minimum viable match
        assert!(is_strong_sequence_match((4, 0), 4, 4));
        assert!(is_strong_sequence_match((4, 0), 4, 4));
        // Slightly worse
        assert!(!is_strong_sequence_match((3, 1), 4, 4));
        assert!(!is_strong_sequence_match((3, 1), 4, 4));
        // Even worse
        assert!(!is_strong_sequence_match((2, 2), 4, 4));
        assert!(!is_strong_sequence_match((2, 2), 4, 4));

        // Half match
        assert!(!is_strong_sequence_match((5, 5), 10, 10));
        assert!(!is_strong_sequence_match((500, 500), 1000, 1000));

        // Relatively small sequence matches
        assert!(!is_strong_sequence_match((5, 15), 20, 20));
        assert!(!is_strong_sequence_match((100, 500), 600, 600));

        // A small loop causes an insignificant match
        assert!(!is_strong_sequence_match((3, 97), 100, 100));

        // Long-ish match with a relatively big sequence size difference
        assert!(!is_strong_sequence_match((10, 10), 10, 20));
        assert!(!is_strong_sequence_match((8, 12), 10, 20));
        // Long-ish match with an absolutely big size difference
        assert!(!is_strong_sequence_match((45, 55), 50, 100));

        // Barely good enough matches
        assert!(!is_strong_sequence_match((10, 5), 15, 15));
        assert!(!is_strong_sequence_match((40, 15), 55, 55));
        assert!(!is_strong_sequence_match((40, 15), 55, 40));
        assert!(!is_strong_sequence_match((40, 15), 40, 55));

        // Good enough matches
        assert!(is_strong_sequence_match((23, 4), 23, 27));
        assert!(is_strong_sequence_match((45, 5), 45, 50));
        assert!(is_strong_sequence_match((45, 5), 50, 45));
    }
}
