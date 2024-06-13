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

use config::Config;
use itertools::Itertools;
use serde_derive::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::exit;

use commons::models::gtfs as gtfs_commons;

use crate::gtfs::{load_gtfs, Data};
use crate::iml::{load_base_data, Route};
use crate::linter::lint_gtfs;
use crate::matcher::{match_gtfs_routes, RoutePairing, SubroutePatternPairing};

mod error;
mod gtfs;
mod iml;
mod linter;
mod matcher;
#[cfg(test)]
mod tests;
mod utils;

#[derive(Default, Deserialize)]
struct AppConfig {
    jwt: String,
}

#[derive(Debug)]
struct AppArgs {
    operator: i32,
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let args = AppArgs {
        operator: pargs.value_from_str("--op")?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Unknown args: {:?}.", remaining);
        exit(1);
    }

    Ok(args)
}

#[tokio::main]
async fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            exit(1);
        }
    };

    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("IML")
                .try_parsing(true)
                .separator("_"),
        )
        .build()
        .unwrap();

    if let Ok(config) = config.try_deserialize() {
        let config: AppConfig = config;
        iml::TOKEN.set(Box::leak(Box::new(config.jwt))).unwrap();
    } else {
        eprintln!("Token not found in the environment");
        exit(-1);
    }

    let gtfs_overrides = gtfs::load_overrides(args.operator).unwrap();
    gtfs::OVERRIDES.set(gtfs_overrides).unwrap();

    let gtfs = load_gtfs(&PathBuf::from(&format!(
        "./data/operators/{}/gtfs",
        args.operator
    )))
    .unwrap();
    let lints = lint_gtfs(&gtfs);

    iml::put_operator_validation(
        args.operator,
        iml::OperatorValidationData { gtfs_lints: lints },
    )
    .await
    .unwrap();

    let iml = load_base_data().await.unwrap();

    let mut matches =
        match_gtfs_routes(&gtfs, &iml, args.operator).await.unwrap();

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

    for route_pairing in matches {
        let route = iml.routes.get(&route_pairing.route_id).unwrap();
        println!("(#{}) - {:?} - {}", route.id, route.code, route.name);

        let route_validation_data = iml::RouteValidationData {
            validation: gtfs_commons::RouteValidation {
                unmatched: route_pairing
                    .unpaired_gtfs
                    .iter()
                    .cloned()
                    .map(|pattern| pattern.into())
                    .collect(),
            },
            subroutes: route_pairing
                .subroute_pairings
                .iter()
                .map(|pairing| (pairing.iml.subroute_id, pairing.into()))
                .collect::<HashMap<i32, gtfs_commons::SubrouteValidation>>(),
        };
        iml::patch_route_validation(
            route_pairing.route_id,
            route_validation_data,
        )
        .await
        .unwrap();

        let mut conflicts = false;

        {
            let mut used_gtfs_pattern_ids = HashSet::new();
            let mut used_iml_subroute_ids = HashSet::new();
            for subroute_pairing in route_pairing.subroute_pairings.iter() {
                for pattern_id in &subroute_pairing.gtfs.pattern_ids {
                    let new = used_gtfs_pattern_ids.insert(pattern_id);
                    if !new {
                        conflicts = true;
                        break;
                    }
                }

                let new = used_iml_subroute_ids
                    .insert(&subroute_pairing.iml.subroute_id);
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

        if !route_pairing.subroute_pairings.is_empty() {
            println!("\tMatches:");
            for subroute_pairing in route_pairing.subroute_pairings.iter() {
                let subroute = route
                    .subroutes
                    .iter()
                    .find(|subroute| {
                        subroute.id == subroute_pairing.iml.subroute_id
                    })
                    .unwrap();
                let trip_headsigns = subroute_pairing
                    .gtfs
                    .trip_ids
                    .iter()
                    .filter_map(|id| {
                        gtfs.trips.get(id).unwrap().trip_headsign.clone()
                    })
                    .unique()
                    .collect::<Vec<_>>()
                    .join(";");
                println!(
                    "\t\tIML#{} {} matched with GTFS#{};;{};HS:{}",
                    subroute_pairing.iml.subroute_id,
                    subroute.flag,
                    subroute_pairing.gtfs.route_id,
                    subroute_pairing.gtfs.pattern_ids.iter().join(";"),
                    trip_headsigns
                );

                // Check if the iml.stop_ids are equal to the gtfs.iml_stop_ids
                let stop_seq_equal = subroute_pairing.iml.stop_ids
                    == subroute_pairing.gtfs.iml_stop_ids;

                if stop_seq_equal {
                    print_matching_pattern(subroute_pairing);
                } else {
                    print_diverging_pattern(subroute_pairing);
                    every_match_perfect = false;
                }
            }
        }

        let no_unmatched = route_pairing.unpaired_iml.is_empty()
            || route_pairing.unpaired_gtfs.is_empty();

        if every_match_perfect && no_unmatched {
            good_cnt += 1;
        } else if no_unmatched {
            fixable_cnt += 1;
        } else {
            bad_cnt += 1;
            println!("\t\t### BAD MATCH ###")
        }

        if !route_pairing.unpaired_iml.is_empty() {
            print_unpaired_iml(route, &route_pairing);
        }
        // Show unmatched GTFS
        if !route_pairing.unpaired_gtfs.is_empty() {
            print_unpaired_gtfs(&gtfs, &route_pairing);
        }
    }

    println!("Good: {}", good_cnt);
    println!("Fixable: {}", fixable_cnt);
    println!("Bad: {}", bad_cnt);
    println!("Conflicts: {}", conflict_cnt);
}

fn print_matching_pattern(subroute_pairing: &SubroutePatternPairing) {
    println!(
        "\t\t{}",
        serde_json::to_string(&subroute_pairing.gtfs.iml_stop_ids).unwrap()
    );
    println!("\t\t--- Already upstream!");
}

fn print_diverging_pattern(subroute_pairing: &SubroutePatternPairing) {
    println!(
        "\t\tG{}",
        serde_json::to_string(&subroute_pairing.gtfs.stop_ids).unwrap()
    );
    println!(
        "\t\tG{}",
        serde_json::to_string(&subroute_pairing.gtfs.iml_stop_ids).unwrap()
    );
    println!(
        "\t\tI{}",
        serde_json::to_string(&subroute_pairing.iml.stop_ids).unwrap()
    );
    println!("\t\t---");
}

fn print_unpaired_iml(route: &Route, route_pairing: &RoutePairing) {
    println!("\tUnmatched IML:");
    for data in route_pairing.unpaired_iml.iter() {
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

fn print_unpaired_gtfs(gtfs: &Data, route_pairing: &RoutePairing) {
    println!("\tUnmatched GTFS:");
    for pattern_data in route_pairing.unpaired_gtfs.iter() {
        let trip_headsigns = pattern_data
            .trip_ids
            .iter()
            .filter_map(|id| gtfs.trips.get(id).unwrap().trip_headsign.clone())
            .unique()
            .collect::<Vec<_>>()
            .join(";");
        println!(
            "\t\tGTFS#{};;{};HS:{} - {:?}",
            pattern_data.route_id,
            pattern_data.pattern_ids.iter().join(";"),
            trip_headsigns,
            pattern_data.stop_ids
        );
        println!(
            "\t\t->IML {:?}",
            serde_json::to_string(&pattern_data.iml_stop_ids).unwrap()
        );
        println!("\t\t---");
    }
}
