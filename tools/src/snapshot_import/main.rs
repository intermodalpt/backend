/*
    Intermodal, transportation information aggregator
    Copyright (C) 2024  Cláudio Pereira

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
use std::collections::HashSet;
use std::process::exit;

use itertools::Itertools;
use serde_derive::Deserialize;

use commons::models::osm::NodeVersion;

mod api;
mod models;

const MAX_OSM_CALLS: usize = 30;

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

    let stats = import().await.unwrap();
    dbg!(stats);
}

#[derive(Debug, Default)]
struct ImportStats {
    new_stops: usize,
    updated_stops: usize,
    deleted_stops: usize,
    osm_calls: usize,
}
pub(crate) async fn import() -> Result<ImportStats, Box<dyn std::error::Error>>
{
    let stop_versions = api::fetch_cached_osm_stop_versions().await.unwrap();
    let osm_stops = api::fetch_osm_stops().await.unwrap();

    let mut new_stops = vec![];

    let mut unreturned_ids = stop_versions.keys().collect::<HashSet<&String>>();
    let mut ids_pending_history = vec![];

    let mut stats = ImportStats::default();

    let overpass_stops = osm_stops
        .nodes
        .into_iter()
        .filter_map(|node| {
            if let models::XmlNodeTypes::Node(node) = node {
                Some(models::OverpassStop::from(node))
            } else {
                None
            }
        })
        .collect_vec();

    let mut predicted_osm_calls: usize = overpass_stops
        .iter()
        .map(|overpass_stop| {
            if let Some(cached_version) = stop_versions.get(&overpass_stop.id) {
                unreturned_ids.remove(&overpass_stop.id);
                if overpass_stop.version > *cached_version {
                    if overpass_stop.version == cached_version + 1 {
                        0
                    } else {
                        1
                    }
                } else {
                    0
                }
            } else {
                if overpass_stop.version == 1 {
                    0
                } else {
                    1
                }
            }
        })
        .sum();

    predicted_osm_calls += unreturned_ids.len();

    if predicted_osm_calls > MAX_OSM_CALLS {
        println!("Snapshot exceeds the maximum number of OSM calls ({} calls predicted)", predicted_osm_calls);

        let patch = overpass_stops
            .into_iter()
            .map(|overpass_stop| {
                let id = overpass_stop.id.clone();
                let history = vec![NodeVersion::from(overpass_stop)];
                api::OsmHistoryPatch { id, history }
            })
            .collect_vec();

        api::patch_osm_stops_history(&patch).await?;
        stats.updated_stops += patch.len();
        return Ok(stats);
    }

    // Calculate new stops and updates figuring which needs additional queries
    for overpass_stop in overpass_stops {
        if let Some(cached_version) = stop_versions.get(&overpass_stop.id) {
            unreturned_ids.remove(&overpass_stop.id);

            if overpass_stop.version > *cached_version {
                if overpass_stop.version == cached_version + 1 {
                    let mut history =
                        api::fetch_cached_osm_stop_history(&overpass_stop.id)
                            .await?;

                    let id = overpass_stop.id.clone();
                    history.push(NodeVersion::from(overpass_stop));

                    let version_integrity = history
                        .iter()
                        .map(|version| version.version)
                        .sorted()
                        .enumerate()
                        .all(|(i, version)| i + 1 == version as usize);

                    if !version_integrity {
                        eprintln!("Version integrity failed for {}", id);
                    }

                    api::patch_osm_stops_history(&[api::OsmHistoryPatch {
                        id,
                        history,
                    }])
                    .await?;
                } else {
                    ids_pending_history.push(overpass_stop.id);
                }
            }
        } else {
            // Postpone the addition of new stops so we can bulk
            new_stops.push(overpass_stop);
        }
    }

    // Update the old stops that need history-queries
    for id in ids_pending_history {
        let history = api::fetch_osm_node_versions(&id).await.unwrap();

        api::patch_osm_stops_history(&[api::OsmHistoryPatch { id, history }])
            .await?;

        // Sleep for 5s to be respectful of the OSM API
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        stats.osm_calls += 1;
        stats.updated_stops += 1;
    }

    // Add the new stops that need history-queries
    for stop in new_stops.iter().filter(|stop| stop.version > 1) {
        let history = api::fetch_osm_node_versions(&stop.id).await.unwrap();

        api::patch_osm_stops_history(&[api::OsmHistoryPatch {
            id: stop.id.clone(),
            history,
        }])
        .await?;

        // Sleep for 5s to be respectful of the OSM API
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        stats.osm_calls += 1;
        stats.new_stops += 1;
    }

    // Add the new stops that do **not** need history-queries
    let patch = new_stops
        .into_iter()
        .filter(|stop| stop.version == 1)
        .map(|overpass_stop| {
            let history = vec![NodeVersion {
                version: overpass_stop.version,
                author: overpass_stop.uid,
                author_uname: overpass_stop.user,
                lat: overpass_stop.lat,
                lon: overpass_stop.lon,
                attributes: overpass_stop.attributes,
                timestamp: overpass_stop.timestamp,
                deleted: false,
            }];
            api::OsmHistoryPatch {
                id: overpass_stop.id,
                history,
            }
        })
        .collect_vec();
    api::patch_osm_stops_history(&patch).await?;
    stats.new_stops += patch.len();

    // Update deleted stops
    for id in unreturned_ids {
        let history = api::fetch_osm_node_versions(&id).await.unwrap();

        api::patch_osm_stops_history(&[api::OsmHistoryPatch {
            id: id.to_string(),
            history,
        }])
        .await?;

        // Sleep for 5s to be respectful of the OSM API
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        stats.osm_calls += 1;
        stats.deleted_stops += 1;
    }

    Ok(stats)
}
