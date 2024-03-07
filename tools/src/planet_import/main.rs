/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023 - 2024  Cláudio Pereira

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

mod api;

use chrono::{DateTime, Utc};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use std::process::exit;

use config::Config;
use osmpbf::{Element, ElementReader};
use serde_derive::Deserialize;

use commons::models::osm;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
        api::TOKEN.set(Box::leak(Box::new(config.jwt))).unwrap();
    } else {
        eprintln!("Token not found in the environment");
        exit(-1);
    }

    let cached_versions = api::fetch_cached_osm_stop_versions().await.unwrap();

    let node_set = cached_versions.keys().copied().collect::<HashSet<i64>>();

    let f = std::fs::File::open("history-230724.osm.pbf").unwrap();
    let reader = BufReader::new(f);

    let nodes_versions = extract_node_versions(node_set, reader)
        .expect("Unable to extract changesets");
    // let nodes_versions = par_extract_node_versions(node_set, reader)
    //     .expect("Unable to extract changesets");

    let patch = nodes_versions
        .into_iter()
        .filter(|(id, history)| {
            let current_cnt = history.len();
            let last_current = history.last().unwrap();

            let cached_versions = cached_versions.get(&id).unwrap();
            let cached_cnt = cached_versions.len();
            let last_cached = history.last().unwrap();

            last_cached.version as usize != cached_cnt + 1
                || cached_cnt != current_cnt
                || last_current.version != last_cached.version
        })
        .map(|(id, history)| api::OsmHistoryPatch { id, history })
        .collect::<Vec<api::OsmHistoryPatch>>();

    api::patch_osm_stops_history(&patch).await.unwrap();
}

fn extract_node_versions(
    id_set: HashSet<i64>,
    reader: BufReader<std::fs::File>,
) -> Result<HashMap<i64, osm::NodeHistory>> {
    let reader = ElementReader::new(reader);

    let mut node_versions: HashMap<i64, osm::NodeHistory> = HashMap::new();

    reader.for_each(|element| match element {
        Element::DenseNode(n) => {
            let id = n.id();
            if id_set.contains(&id) {
                if let Some(info) = n.info() {
                    let version = osm::NodeVersion {
                        version: info.version(),
                        author: info.uid(),
                        author_uname: info.user().unwrap_or("?").to_string(),
                        lat: n.lat(),
                        lon: n.lon(),
                        attributes: n
                            .tags()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                        timestamp: millis_to_datetime(info.milli_timestamp()),
                        deleted: info.deleted(),
                    };

                    match node_versions.entry(id) {
                        Entry::Occupied(e) => {
                            e.into_mut().push(version);
                        }
                        Entry::Vacant(e) => {
                            e.insert(vec![version]);
                        }
                    }
                } else {
                    eprintln!("No info for node {}", id);
                }
            }
        }
        _ => {}
    })?;

    node_versions.values_mut().for_each(|versions| {
        versions.sort_by_key(|v| v.version);
    });

    Ok(node_versions)
}

fn millis_to_datetime(millis: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(millis).expect("invalid timestamp")
}

// This function requires an unmerged patched version of OSMPBF
fn par_extract_node_versions(
    id_set: HashSet<i64>,
    reader: BufReader<std::fs::File>,
) -> Result<HashMap<i64, osm::NodeHistory>> {
    let reader = ElementReader::new(reader);

    let mut node_versions: HashMap<i64, osm::NodeHistory> = HashMap::new();

    struct RawNode {
        id: i64,
        lat: f64,
        lon: f64,
        tags: Vec<(String, String)>,
        version: i32,
        author: i32,
        author_uname: String,
        timestamp: DateTime<Utc>,
        deleted: bool,
    }

    let raw_nodes = reader.par_filter_map(|e| match e {
        Element::DenseNode(n) => {
            let id = n.id();
            if id_set.contains(&id) {
                let info = n.info().unwrap();
                Some(RawNode {
                    id,
                    lat: n.lat(),
                    lon: n.lon(),
                    tags: n
                        .tags()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                    version: info.version(),
                    author: info.uid(),
                    author_uname: info.user().unwrap().to_string(),
                    timestamp: millis_to_datetime(info.milli_timestamp()),
                    deleted: info.deleted(),
                })
            } else {
                None
            }
        }
        _ => None,
    });

    raw_nodes.into_iter().for_each(|node| {
        let version = osm::NodeVersion {
            version: node.version,
            author: node.author,
            author_uname: node.author_uname,
            lat: node.lat,
            lon: node.lon,
            attributes: node.tags,
            timestamp: node.timestamp,
            deleted: node.deleted,
        };

        match node_versions.entry(node.id) {
            Entry::Occupied(e) => {
                e.into_mut().push(version);
            }
            Entry::Vacant(e) => {
                e.insert(vec![version]);
            }
        }
    });

    node_versions.values_mut().for_each(|versions| {
        versions.sort();
    });

    Ok(node_versions)
}
