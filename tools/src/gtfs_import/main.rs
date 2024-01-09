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

use crate::gtfs::load_gtfs;
use crate::iml::load_base_data;
use crate::linter::lint_gtfs;
use crate::matcher::match_gtfs_routes;
use commons::models::gtfs as gtfs_commons;

mod error;
mod gtfs;
mod iml;
mod linter;
mod matcher;
#[cfg(test)]
mod tests;
mod utils;

pub(crate) static GTFS_TMP_SUPRESS: [&'static str; 8] = [
    "040151", // Póvoa da Galega, WTF
    "081003", "081031", "081032", "140288", "160425",
    // Marquesa, Penalva
    "130813", "130814",
];
pub(crate) static GTFS_TMP_STICK_TO_ORIGINALS: [&'static str; 46] = [
    // Campo grande
    "060301", "060303", "060155", "060306", "060302", "060311", "060226",
    "060308", "060305", "060316", "060341", "060339", "060156", "060312",
    "060259", "060314", "060171", "060337", "060304", "060310", "060369",
    "060286", "061200", // Oriente
    "060001", "060002", "060009", "060011", "060321", "060322", "060253",
    "060207", "060327", "060323", "060325", "060361",
    // Terminal de Mafra
    "080207", "082320", "082321", "080208", "082322",
    // Terminal da Pontinha
    "110695", "110859", // Avelar Brotero
    "110401", "110402", "110109", // TF Seixal
    "140073",
];
pub(crate) static GTFS_TMP_OVERRIDES: [(&'static str, iml::StopId); 100] = [
    ("020006", 25584),
    ("020027", 348),
    ("020219", 7693),
    ("020363", 7696),
    ("021009", 7693),
    ("030819", 17102),
    ("030857", 16528),
    ("040025", 12959),
    ("040142", 1066),
    ("060368", 11432),
    ("060061", 11432),
    ("060156", 10514),
    ("060345", 23648),
    ("060347", 25412),
    ("060351", 23645),
    ("070363", 23325),
    ("070483", 23206),
    ("070523", 23207),
    ("070553", 23183),
    ("071005", 22460),
    ("071006", 22461),
    ("071080", 22808),
    ("071084", 22804),
    ("071049", 22779),
    ("071050", 22780),
    ("071100", 22778),
    ("071101", 22780),
    ("071102", 22779),
    ("071106", 22783),
    ("071107", 22784),
    ("071144", 22752),
    ("071427", 23207),
    ("080269", 22036),
    ("080273", 22107),
    ("080274", 22119),
    ("080275", 22119),
    ("080276", 22107),
    ("080351", 22036),
    ("080405", 21342),
    ("080406", 21342),
    ("080485", 21910),
    ("080583", 21910),
    ("080901", 21570),
    ("080913", 21570),
    ("080943", 21534),
    ("080986", 21426),
    ("082202", 21342),
    ("090275", 1444),
    ("100166", 14993),
    ("100407", 13080),
    ("100408", 13080),
    ("110783", 20775),
    ("110554", 20890),
    ("130157", 14234),
    ("130158", 14333),
    ("130230", 13707),
    ("130276", 14333),
    ("130277", 14333),
    ("130278", 14333),
    ("130716", 25207),
    ("140075", 1429),
    ("140273", 976),
    ("140469", 940),
    ("140470", 939),
    ("140554", 14502),
    ("140557", 1002),
    ("140561", 620),
    ("140788", 7685),
    ("150028", 12010),
    ("150051", 13814),
    ("150089", 902),
    ("160052", 118),
    ("160071", 118),
    ("160246", 14671),
    ("160296", 13345),
    ("160306", 13345),
    ("160327", 14717),
    ("160349", 95),
    ("160501", 14623),
    ("160566", 1521),
    ("160983", 15192),
    ("171290", 17765),
    ("171299", 17570),
    ("171301", 17569),
    ("171302", 17568),
    ("171543", 15406),
    ("171544", 16000),
    ("171307", 15406),
    ("171308", 16000),
    ("171313", 16006),
    ("172625", 17765),
    ("180473", 20087),
    ("180515", 20087),
    ("180711", 19827),
    ("180847", 19827),
    ("180997", 20250),
    ("180998", 20165),
    // THESE ARE WROOOONG
    ("110402", 21048),
    ("110695", 20722),
    ("140073", 854),
];

#[derive(Default, Deserialize)]
struct AppConfig {
    jwt: String,
}

#[tokio::main]
async fn main() {
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

    let gtfs = load_gtfs(&PathBuf::from("./data/operators/1/gtfs")).unwrap();
    let lints = lint_gtfs(&gtfs);

    iml::put_operator_validation(
        1,
        iml::OperatorValidationData { gtfs_lints: lints },
    )
    .await
    .unwrap();

    let iml = load_base_data().await.unwrap();

    let mut matches = match_gtfs_routes(&gtfs, &iml).await.unwrap();

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

        let route_validation_data = iml::RouteValidationData {
            validation: gtfs_commons::RouteValidation {
                unmatched: res
                    .unpaired_gtfs
                    .iter()
                    .map(|pattern| pattern.into())
                    .collect(),
            },
            subroutes: res
                .pairings
                .iter()
                .map(|pairing| (pairing.iml.subroute_id, pairing.into()))
                .collect::<HashMap<i32, gtfs_commons::SubrouteValidation>>(),
        };
        iml::put_route_validation(res.route_id, route_validation_data)
            .await
            .unwrap();

        let mut conflicts = false;

        {
            let mut used_gtfs_pattern_ids = HashSet::new();
            let mut used_iml_subroute_ids = HashSet::new();
            for m in res.pairings.iter() {
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

        if !res.pairings.is_empty() {
            println!("\tMatches:");
            for m in res.pairings.iter() {
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
            res.unpaired_iml.is_empty() || res.unpaired_gtfs.is_empty();

        if every_match_perfect && no_unmatched {
            good_cnt += 1;
        } else if no_unmatched && !missing_stop_rels {
            fixable_cnt += 1;
        } else {
            bad_cnt += 1;
            println!("\t\t### BAD MATCH ###")
        }

        if !res.unpaired_iml.is_empty() {
            println!("\tUnmatched IML:");
            for data in res.unpaired_iml.iter() {
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
        if !res.unpaired_gtfs.is_empty() {
            println!("\tUnmatched GTFS:");
            for data in res.unpaired_gtfs.iter() {
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
