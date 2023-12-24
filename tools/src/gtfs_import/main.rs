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

use crate::gtfs::load_gtfs;
use crate::iml::load_base_data;
use crate::linter::lint_gtfs;
use crate::matcher::match_gtfs_routes;
use std::collections::{HashMap, HashSet};

mod error;
mod gtfs;
mod iml;
mod linter;
mod matcher;
mod utils;

#[tokio::main]
async fn main() {
    let gtfs = load_gtfs().unwrap();
    let lints = lint_gtfs(&gtfs);
    for lint in lints {
        println!("{:?}", lint);
    }

    let iml = load_base_data().await.unwrap();

    let iml_to_gtfs_stops = iml
        .stops
        .iter()
        .filter_map(|(iml_id, iml_stop)| {
            let gtfs_id = iml_stop
                .operators
                .iter()
                .find(|rel| rel.operator_id == 1)
                .map(|rel| rel.stop_ref.as_ref().unwrap().clone());

            if let Some(id) = &gtfs_id {
                if !gtfs.stops.contains_key(id) {
                    println!("Missing GTFS stop {}", id);
                    // TODO add hint to unlink
                    return None;
                }
            }

            gtfs_id.map(|gtfs_id| (iml_id.clone(), gtfs_id))
        })
        .collect::<HashMap<i32, gtfs::StopId>>();

    let gtfs_to_iml_stops = iml_to_gtfs_stops
        .iter()
        .map(|(iml_id, gtfs_id)| (gtfs_id.clone(), *iml_id))
        .collect::<HashMap<gtfs::StopId, i32>>();

    let mut matches = match_gtfs_routes(&gtfs, &iml, &gtfs_to_iml_stops)
        .await
        .unwrap();

    // Sorting for determinism
    matches.sort_by(|m1, m2| {
        let r1 = iml.routes.get(&m1.route_id).unwrap();
        let r2 = iml.routes.get(&m2.route_id).unwrap();
        r1.code.cmp(&r2.code)
    });

    let mut good_cnt = 0;
    let mut fixable_cnt = 0;
    let mut bad_cnt = 0;
    let mut conflict_cnt = 0;

    for res in matches {
        let route = iml.routes.get(&res.route_id).unwrap();
        println!("(#{}) - {:?} - {}", route.id, route.code, route.name);

        let mut conflicts = false;

        {
            let mut used_gtfs_pattern_ids = HashSet::new();
            let mut used_iml_subroute_ids = HashSet::new();
            for m in res.matches.iter() {
                let new = used_gtfs_pattern_ids.insert(&m.gtfs.pattern_ids);
                if !new {
                    conflicts = true;
                    break;
                }
                let new = used_iml_subroute_ids.insert(&m.iml.subroute_id);
                if !new {
                    conflicts = true;
                    break;
                }
            }
        }

        if conflicts {
            println!("\t### THERE WERE CONFLICTING ASSIGNMENTS ###");
            conflict_cnt += 1;
        }

        let mut every_match_perfect = true;
        let mut missing_stop_rels = false;

        if !res.matches.is_empty() {
            println!("\tMatches:");
            for m in res.matches.iter() {
                let subroute = route
                    .subroutes
                    .iter()
                    .find(|subroute| subroute.id == m.iml.subroute_id)
                    .unwrap();
                let trip_headsigns = m
                    .gtfs
                    .trip_ids
                    .iter()
                    .map(|id| gtfs.trips.get(id).unwrap().trip_headsign.clone())
                    .unique()
                    .collect::<Vec<_>>()
                    .join(";");
                println!(
                    "\t\tIML#{} {} matched with GTFS#{};;{};HS:{}",
                    m.iml.subroute_id,
                    subroute.flag,
                    m.gtfs.route_id,
                    m.gtfs.pattern_ids.join(";"),
                    trip_headsigns
                );

                // Check if the iml.stop_ids are equal to the gtfs.iml_stop_ids
                let iml_ids_as_optional = m
                    .iml
                    .stop_ids
                    .iter()
                    .map(|id| Some(*id))
                    .collect::<Vec<_>>();
                let stop_seq_equal = iml_ids_as_optional == m.gtfs.iml_stop_ids;

                if stop_seq_equal {
                    println!(
                        "\t\t{}",
                        serde_json::to_string(&m.gtfs.iml_stop_ids).unwrap()
                    );
                    println!("\t\t--- Already upstream!");
                } else {
                    println!(
                        "\t\tG{}",
                        serde_json::to_string(&m.gtfs.stop_ids).unwrap()
                    );
                    println!(
                        "\t\tG{}",
                        serde_json::to_string(&m.gtfs.iml_stop_ids).unwrap()
                    );
                    println!(
                        "\t\tI{}",
                        serde_json::to_string(&m.iml.stop_ids).unwrap()
                    );
                    every_match_perfect = false;
                    println!("\t\t---");
                }

                if m.gtfs.iml_stop_ids.contains(&None) {
                    missing_stop_rels = true;
                }
            }
        }

        let no_unmatched =
            res.unmatched_iml.is_empty() || res.unmatched_gtfs.is_empty();

        if every_match_perfect && no_unmatched {
            good_cnt += 1;
        } else if no_unmatched && !missing_stop_rels {
            fixable_cnt += 1;
        } else {
            bad_cnt += 1;
            println!("\t\t### BAD MATCH ###")
        }

        if !res.unmatched_iml.is_empty() {
            println!("\tUnmatched IML:");
            for data in res.unmatched_iml.iter() {
                let subroute = route
                    .subroutes
                    .iter()
                    .find(|subroute| subroute.id == data.subroute_id)
                    .unwrap();
                println!("\t\tIML#{} - {}", data.subroute_id, subroute.flag);
                println!("\t\t\t{:?}", data.stop_ids);
                println!("\t\t---");
            }
        }
        if !res.unmatched_gtfs.is_empty() {
            println!("\tUnmatched GTFS:");
            for data in res.unmatched_gtfs.iter() {
                let trip_headsigns = data
                    .trip_ids
                    .iter()
                    .map(|id| gtfs.trips.get(id).unwrap().trip_headsign.clone())
                    .unique()
                    .collect::<Vec<_>>()
                    .join(";");
                println!(
                    "\t\tGTFS#{};;{};HS:{} - {:?}",
                    data.route_id,
                    data.pattern_ids.join(";"),
                    trip_headsigns,
                    data.stop_ids
                );
                println!(
                    "\t\t->IML {:?}",
                    serde_json::to_string(&data.iml_stop_ids).unwrap()
                );
                println!("\t\t---");
            }
        }
    }

    println!("Good: {}", good_cnt);
    println!("Fixable: {}", fixable_cnt);
    println!("Bad: {}", bad_cnt);
    println!("Conflicts: {}", conflict_cnt);
}
