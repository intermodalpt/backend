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

pub(crate) struct RouteSummary<'iml, 'gtfs> {
    // Some of these fields are unnecessary since the subroute has a ref
    pub(crate) iml_route: &'iml iml::Route,
    pub(crate) iml_route_id: iml::RouteId,
    pub(crate) subroutes: Vec<SubrouteSummary<'iml>>,
    pub(crate) patterns: Vec<PatternSummary<'gtfs>>,
}

#[derive(Clone)]
pub(crate) struct SubrouteSummary<'iml> {
    // Some of these fields are unnecessary since the subroute has a ref
    pub(crate) subroute: &'iml iml::Subroute,
    pub(crate) subroute_id: iml::SubrouteId,
    pub(crate) prematched_gtfs_pattern: Option<gtfs::PatternId>,
    pub(crate) stop_ids: &'iml [iml::StopId],
}

#[derive(Clone)]
pub(crate) struct PatternSummary<'gtfs> {
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
    pub(crate) pattern_ids: HashSet<gtfs::PatternId>,
    pub(crate) headsigns: HashSet<String>,
    pub(crate) trip_ids: HashSet<gtfs::TripId>,
    pub(crate) iml_stop_ids: Vec<iml::StopId>,
}

impl From<GtfsPatternData<'_>> for gtfs_commons::SubrouteValidation {
    fn from(cluster: GtfsPatternData<'_>) -> Self {
        gtfs_commons::SubrouteValidation {
            gtfs_cluster: gtfs_commons::PatternCluster {
                stops: cluster.stop_ids.to_vec(),
                headsigns: cluster.headsigns,
                patterns: cluster.pattern_ids,
                trips: cluster.trip_ids,
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

pub(crate) async fn match_gtfs_routes<'iml, 'gtfs>(
    gtfs: &'gtfs gtfs::Data,
    iml: &'iml iml::Data,
    operator_id: i32,
) -> Result<Vec<RoutePairing<'iml, 'gtfs>>, Error> {
    let gtfs_remaps = &gtfs::OVERRIDES.get().unwrap().remaps;

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

    let mut gtfs_to_iml_stops = iml_to_gtfs_stops
        .iter()
        .map(|(iml_id, gtfs_id)| (gtfs_id.clone(), *iml_id))
        .collect::<HashMap<gtfs::StopId, i32>>();

    gtfs_remaps.iter().for_each(|(gtfs_id, iml_id)| {
        if !iml.stops.contains_key(iml_id) {
            panic!("IML stop {} does not exist", iml_id);
        }
        gtfs_to_iml_stops
            .entry(gtfs_id.to_string())
            .or_insert(*iml_id);
    });

    let gtfs_routes_by_code = gtfs
        .routes
        .values()
        .into_group_map_by(|route| route.route_short_name.clone())
        .into_iter()
        .collect::<HashMap<String, Vec<&gtfs::Route>>>();

    // Optional sorting, for determinism
    let iml_routes = iml
        .routes
        .values()
        .filter(|r| r.operator == 1)
        .sorted_by_key(|r| r.id);

    let mut route_summaries = vec![];
    for iml_route in iml_routes {
        let res = link_gtfs_to_iml_route(
            gtfs,
            iml_route,
            &gtfs_to_iml_stops,
            &gtfs_routes_by_code,
        )
        .await?;
        let matched_res = pair_patterns_with_subroutes(res);
        route_summaries.push(matched_res);
    }
    Ok(route_summaries)
}

async fn link_gtfs_to_iml_route<'iml, 'gtfs>(
    gtfs: &'gtfs gtfs::Data,
    iml_route: &'iml iml::Route,
    gtfs_to_iml_stops: &HashMap<gtfs::StopId, iml::StopId>,
    gtfs_routes_by_code: &HashMap<String, Vec<&'gtfs gtfs::Route>>,
) -> Result<RouteSummary<'iml, 'gtfs>, Error> {
    let suppressions = &gtfs::OVERRIDES.get().unwrap().suppressions;
    let mut iml_subroute_data = vec![];
    let mut gtfs_patterns_data = vec![];

    for iml_subroute in iml_route.subroutes.iter() {
        iml_subroute_data.push(SubrouteSummary {
            subroute_id: iml_subroute.id,
            subroute: iml_subroute,
            stop_ids: &iml_subroute.stops,
            prematched_gtfs_pattern: iml_subroute
                .prematched_gtfs_pattern
                .clone(),
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

        return Ok(RouteSummary {
            iml_route,
            iml_route_id: iml_route.id,
            subroutes: iml_subroute_data.clone(),
            patterns: vec![],
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

                gtfs_patterns_data.push(PatternSummary {
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

    Ok(RouteSummary {
        iml_route,
        iml_route_id: iml_route.id,
        subroutes: iml_subroute_data.clone(),
        patterns: gtfs_patterns_data.clone(),
    })
}

pub(crate) fn pair_patterns_with_subroutes<'iml, 'gtfs>(
    summary: RouteSummary<'iml, 'gtfs>,
) -> RoutePairing<'iml, 'gtfs> {
    let patterns = &summary.patterns;
    let subroutes = &summary.subroutes;

    // Check the results of past mappings and confirm that they're still valid
    let prematched_patterns = subroutes
        .iter()
        .flat_map(|subroute| &subroute.prematched_gtfs_pattern)
        .collect::<HashSet<_>>();
    let available_patterns = patterns
        .iter()
        .flat_map(|pattern| pattern.patterns.iter())
        .collect::<HashSet<_>>();
    let _valid_prematched_patterns = prematched_patterns
        .intersection(&available_patterns)
        .collect::<HashSet<_>>();
    let disappeared_patterns = prematched_patterns
        .difference(&available_patterns)
        .collect::<HashSet<_>>();

    // Matrix of matches and mismatches between IML subroutes and GTFS patterns
    // [IML index][GTFS index]
    let mut matches =
        vec![
            vec![(usize::MAX, usize::MAX); summary.patterns.len()];
            summary.subroutes.len()
        ];

    for (iml_idx, iml_subroute) in summary.subroutes.iter().enumerate() {
        for (gtfs_idx, gtfs_pattern) in summary.patterns.iter().enumerate() {
            matches[iml_idx][gtfs_idx] = stop_seq_error(
                &gtfs_pattern.iml_stop_ids,
                iml_subroute.stop_ids,
            );
        }
    }

    // A strong match:
    // - Is not competing with another would-be strong match
    // - Has a minimum length of 5 stops
    // - Has a minium of 12 matches per 15 stops
    // - Has a maximum of 2 mismatches per 15 stops
    // - Has a maximum of 10 mismatches total
    // (This means at most 3 new stops or 2 removed stops per 15 stops)
    let mut strong_match_candidates = vec![];
    let mut strong_matches: Vec<_>;
    let mut weak_past_assignments = vec![];

    fn meets_strong_match_criteria(
        (matches, mismatches): (usize, usize),
        subroute_stops_len: usize,
        pattern_stops_len: usize,
    ) -> bool {
        const MIN_STOP_LEN: usize = 4;
        const MIN_MATCH_RATIO: f32 = 12.0 / 15.0;
        const MAX_MISMATCH_RATIO: f32 = 2.0 / 15.0;
        const MAX_MISMATCHES: usize = 10;

        if subroute_stops_len < MIN_STOP_LEN || pattern_stops_len < MIN_STOP_LEN
        {
            return false;
        }
        if pattern_stops_len < MIN_STOP_LEN {
            return false;
        }

        let max_stop_len = subroute_stops_len.max(pattern_stops_len);

        if mismatches > MAX_MISMATCHES {
            return false;
        }
        if (matches as f32 / max_stop_len as f32) <= MIN_MATCH_RATIO {
            return false;
        }
        if (mismatches as f32 / max_stop_len as f32) >= MAX_MISMATCH_RATIO {
            return false;
        }

        true
    }

    // Raise the past-run matches to strong-candidate status
    for (iml_idx, iml_subroute) in summary.subroutes.iter().enumerate() {
        let subroute = &subroutes[iml_idx];

        if let Some(prematched_pattern) = &subroute.prematched_gtfs_pattern {
            let matched_pattern_cnt = patterns
                .iter()
                .filter(|p| p.patterns.contains(prematched_pattern))
                .count();

            match matched_pattern_cnt {
                1 => {
                    let (gtfs_idx, pattern) = patterns
                        .iter()
                        .enumerate()
                        .find(|(_gtfs_idx, p)| {
                            p.patterns.contains(prematched_pattern)
                        })
                        .unwrap();

                    let pattern_stops_len = pattern.gtfs_stop_ids.len();
                    let subroute_stops_len = iml_subroute.stop_ids.len();
                    let (matches, mismatches) = matches[iml_idx][gtfs_idx];

                    if meets_strong_match_criteria(
                        (matches, mismatches),
                        subroute_stops_len,
                        pattern_stops_len,
                    ) {
                        strong_match_candidates.push((iml_idx, gtfs_idx));
                    } else {
                        weak_past_assignments.push((iml_idx, gtfs_idx));
                    }
                }
                _ => {
                    panic!("GTFS integrity issue")
                }
            }
        }
    }

    // Promote them to strong matches if the conditions make up for it
    strong_matches = strong_match_candidates
        .iter()
        .filter(|(iml_idx, gtfs_idx)| {
            let conflicts = strong_match_candidates
                .iter()
                .find_position(|(other_iml_idx, other_gtfs_idx)| {
                    iml_idx == other_iml_idx && gtfs_idx != other_gtfs_idx
                })
                .is_some();

            let subroute = &subroutes[*iml_idx];
            let pattern = &patterns[*gtfs_idx];
            eprintln!(
                "{} - {:?} had a conflict",
                subroute.subroute_id, pattern.patterns
            );
            !conflicts
        })
        .map(|(iml_idx, gtfs_idx)| (*iml_idx, *gtfs_idx))
        .collect();
    strong_match_candidates = vec![];

    let already_strong_iml_idxs = strong_matches
        .iter()
        .map(|(strong_iml_idx, _strong_gtfs_idx)| *strong_iml_idx)
        .collect::<HashSet<_>>();
    let already_strong_gtfs_idxs = strong_matches
        .iter()
        .map(|(_strong_iml_idx, strong_gtfs_idx)| *strong_gtfs_idx)
        .collect::<HashSet<_>>();

    // Raise the previously unmatched matches to strong-candidate status
    for (iml_idx, iml_subroute) in summary.subroutes.iter().enumerate() {
        let subroute = &subroutes[iml_idx];
        if already_strong_iml_idxs.contains(&iml_idx) {
            continue;
        }

        if let Some(prematched_gtfs_pattern) = &subroute.prematched_gtfs_pattern
        {
            if !disappeared_patterns.contains(&prematched_gtfs_pattern) {
                continue;
            }
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
                // let pattern = &patterns[*gtfx_idx];
                // available_patterns.contains(&pattern.patterns)
            })
            // Within the remaining matches, we're going to filter out the ones that
            .filter_map(|(gtfx_idx, matches)| {
                let pattern = &patterns[gtfx_idx];
                let subroute_stops_len = iml_subroute.stop_ids.len();
                let pattern_stops_len = pattern.gtfs_stop_ids.len();
                if meets_strong_match_criteria(
                    *matches,
                    subroute_stops_len,
                    pattern_stops_len,
                ) {
                    Some((iml_idx, gtfx_idx))
                } else {
                    None
                }
            })
            .collect_vec();

        // Calculate the best hypothesis
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

            let conflicts = strong_match_candidates
                .iter()
                .find_position(|(other_iml_idx, other_gtfs_idx)| {
                    iml_idx == other_iml_idx && gtfs_idx != other_gtfs_idx
                        || iml_idx != other_iml_idx
                            && gtfs_idx == other_gtfs_idx
                })
                .is_some();

            if !conflicts {
                strong_matches.push((*iml_idx, *gtfs_idx));
            }
        });

    // Notable cases:
    // Only one pattern and subroute -> Match them
    // Two and the headsigns are very close to the subroute's -> Match them
    // One strong and one weak -> Match the strong
    // Weak assignments from the past pose no conflict -> Add them all
    if strong_matches.is_empty() {
        if patterns.len() == 1 && subroutes.len() == 1 {
            strong_matches.push((0, 0));
        } else if !weak_past_assignments.is_empty() {
            weak_past_assignments.iter().for_each(
                |(weak_iml_idx, weak_gtfs_idx)| {
                    // Check if there are conflicts with any of the strong matches or other weak matches
                    let conflicts = strong_matches
                        .iter()
                        .find_position(|(strong_iml_idx, strong_gtfs_idx)| {
                            weak_iml_idx == strong_iml_idx
                                && weak_gtfs_idx != strong_gtfs_idx
                        })
                        .is_some()
                        || weak_past_assignments
                            .iter()
                            .find_position(|(other_iml_idx, other_gtfs_idx)| {
                                weak_iml_idx == other_iml_idx
                                    && weak_gtfs_idx != other_gtfs_idx
                            })
                            .is_some();
                    if !conflicts {
                        strong_matches.push((*weak_iml_idx, *weak_gtfs_idx))
                    }
                },
            );
        } else if patterns.len() == 2 && subroutes.len() == 2 {
            if let (Some(sr0_hs), Some(sr1_hs)) = (
                &subroutes[0].subroute.headsign,
                &subroutes[1].subroute.headsign,
            ) {
                const MIN_SIMILARITY: f64 = 0.9;
                let sr0_matches_p0 = patterns[0].headsigns.iter().any(|hs| {
                    normalized_levenshtein(hs, sr0_hs) > MIN_SIMILARITY
                });
                let sr0_matches_p1 = patterns[1].headsigns.iter().any(|hs| {
                    normalized_levenshtein(hs, sr0_hs) > MIN_SIMILARITY
                });
                let sr1_matches_p0 = patterns[0].headsigns.iter().any(|hs| {
                    normalized_levenshtein(hs, sr1_hs) > MIN_SIMILARITY
                });
                let sr1_matches_p1 = patterns[1].headsigns.iter().any(|hs| {
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
                stop_ids: patterns[gtfs_idx].gtfs_stop_ids,
                route_id: patterns[gtfs_idx].route_id,
                pattern_ids: patterns[gtfs_idx]
                    .patterns
                    .iter()
                    .cloned()
                    .collect(),
                headsigns: patterns[gtfs_idx]
                    .headsigns
                    .iter()
                    .cloned()
                    .collect(),
                trip_ids: patterns[gtfs_idx].trips.iter().cloned().collect(),
                iml_stop_ids: patterns[gtfs_idx].iml_stop_ids.clone(),
            },
            iml: ImlSubrouteData {
                subroute_id: subroutes[iml_idx].subroute_id,
                stop_ids: subroutes[iml_idx].stop_ids,
            },
            stop_matches: matches[iml_idx][gtfs_idx].0,
            stop_mismatches: matches[iml_idx][gtfs_idx].1,
        })
        .collect();

    let unmatched_gtfs = patterns
        .iter()
        .enumerate()
        .filter(|(idx, _)| !used_gtfs_idxs.contains(idx))
        .map(|(_, pattern)| GtfsPatternData {
            stop_ids: pattern.gtfs_stop_ids,
            route_id: pattern.route_id,
            pattern_ids: pattern.patterns.iter().cloned().collect(),
            headsigns: pattern.headsigns.iter().cloned().collect(),
            trip_ids: pattern.trips.iter().cloned().collect(),
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
        route_id: summary.iml_route_id,
        subroute_pairings: matches,
        unpaired_gtfs: unmatched_gtfs,
        unpaired_iml: unmatched_iml,
    }
}
