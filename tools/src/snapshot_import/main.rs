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
use std::collections::HashMap;
use std::process::exit;

use itertools::Itertools;
use serde_derive::Deserialize;

use commons::models::osm;
use commons::models::osm::NodeVersion;

use crate::api::StopOsmMeta;

mod api;
mod models;

const FLOAT_TOLERANCE: f64 = 0.000_001;

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

    import().await.unwrap();
}

struct StopMetaPairing {
    stop: api::Stop,
    iml_osm_meta: Option<api::StopOsmMeta>,
    osm_stop: Option<models::OverpassStop>,
}

pub(crate) async fn import(
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let stops = api::fetch_all_iml_stops().await.unwrap();
    let iml_osm_meta = api::fetch_iml_stops_osm_meta().await.unwrap();
    let osm_stops = api::fetch_osm_stops().await.unwrap();

    let mut authors = HashMap::new();

    let mut stop_meta_pairing = stops
        .into_iter()
        .map(|stop| {
            (
                stop.id,
                StopMetaPairing {
                    stop,
                    iml_osm_meta: None,
                    osm_stop: None,
                },
            )
        })
        .collect::<HashMap<i32, StopMetaPairing>>();

    iml_osm_meta.into_iter().for_each(|(id, meta)| {
        if let Some(history) = &meta.osm_history {
            authors
                .entry(history.pos_author_uid)
                .or_insert(history.pos_author_uname.clone());
        }
        // Add meta to the pairing
        if let Some(pairing) = stop_meta_pairing.get_mut(&id) {
            pairing.iml_osm_meta = Some(meta);
        } else {
            panic!("Stop {} not returned by IML", id);
        }
    });

    let osm_to_iml_id = stop_meta_pairing
        .values()
        .map(|pairing| (pairing.stop.external_id.clone(), pairing.stop.id))
        .collect::<HashMap<String, i32>>();

    let mut unmatched_osm_stops = vec![];

    let mut osm_stop_ids = vec![];
    osm_stops
        .nodes
        .into_iter()
        .filter_map(|node| {
            if let models::XmlNodeTypes::Node(node) = node {
                Some(models::OverpassStop::from(node))
            } else {
                None
            }
        })
        .for_each(|osm_stop| {
            authors
                .entry(osm_stop.uid.parse::<i32>().unwrap())
                .or_insert(osm_stop.user.clone());
            osm_stop_ids.push(osm_stop.id.clone());
            if let Some(stop_id) = osm_to_iml_id.get(osm_stop.id.as_str()) {
                stop_meta_pairing.get_mut(stop_id).map(|stop| {
                    stop.osm_stop = Some(osm_stop);
                });
            } else {
                unmatched_osm_stops.push(osm_stop);
            }
        });

    let mut updated_stops = calc_updated_stops(stop_meta_pairing);
    updated_stops.sort_by_key(|(id, _, _, _)| *id);

    let mut updates = 0;
    let mut osm_calls = 0;

    let updated_count = updated_stops.len();
    for (stop_id, mut meta, mut changed, history_complete) in updated_stops {
        if !changed {
            if let Some(history) = meta.osm_history.as_mut() {
                let pos_author_uid = get_position_author(&history.versions);
                if history.pos_author_uid != pos_author_uid {
                    let author_uname = authors.get(&pos_author_uid).unwrap();
                    meta.osm_author = Some(author_uname.clone());
                    history.pos_author_uname = author_uname.clone();
                    history.pos_author_uid = pos_author_uid;
                    changed = true;
                } else {
                    continue;
                };
            } else {
                continue;
            }
        }
        if !history_complete {
            osm_calls += 1;
            let (node_versions, authors) =
                api::fetch_osm_stop_versions(&meta.external_id)
                    .await
                    .unwrap();
            // Sleep for 5s to be respectful of the OSM API
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            let last_version = node_versions.last().unwrap();
            let pos_author_uid = get_position_author(&node_versions);

            // Highly unlikely it is deleted. We just got it from overpass
            assert!(!last_version.deleted);
            meta.osm_history = Some(osm::StoredStopMeta {
                pos_author_uid,
                pos_author_uname: authors.get(&pos_author_uid).unwrap().clone(),
                deleted: false,
                last_version: last_version.version,
                versions: node_versions,
            });
        } else {
            assert!(meta.osm_history.is_some());
            updates += 1;
        }

        api::patch_iml_stop(stop_id, &meta).await?;
    }

    println!("{} call-less updates", updates);
    println!("{} updates requiring OSM calls", osm_calls);
    println!("New stops: {}", unmatched_osm_stops.len());
    let counts = (unmatched_osm_stops.len(), updated_count);

    Ok(counts)
}

fn get_position_author(versions: &Vec<NodeVersion>) -> i32 {
    let mut coord_author = -1;
    let mut lat = 0.0;
    let mut lon = 0.0;
    for version in versions.iter() {
        if version.lat != lat || version.lon != lon {
            coord_author = version.author;
            lat = version.lat;
            lon = version.lon;
        }
    }
    coord_author
}

fn calc_updated_stops(
    stop_meta_pairing: HashMap<i32, StopMetaPairing>,
) -> Vec<(i32, StopOsmMeta, bool, bool)> {
    let mut updated_stops = vec![];
    stop_meta_pairing
        .into_iter()
        .sorted_by_key(|(id, _)| *id)
        .for_each(|(id, pairing)| {
            let stop = pairing.stop;
            let mut iml_osm_meta = pairing.iml_osm_meta.unwrap();
            let mut changed;
            let mut coord_author_changed = false;
            let mut can_guess_history = true;

            if let Some(osm_stop) = pairing.osm_stop {
                // Check for a difference between OSM upstream and IML upstream coords
                if (stop.lat - osm_stop.lat).abs() > FLOAT_TOLERANCE
                    || (stop.lon - osm_stop.lon).abs() > FLOAT_TOLERANCE
                {
                    changed = !iml_osm_meta.osm_differs;
                    iml_osm_meta.osm_differs = true;
                } else {
                    changed = iml_osm_meta.osm_differs;
                    iml_osm_meta.osm_differs = false;
                }

                // Check for a difference between OSM upstream and the cached OSM coords
                if let (Some(prev_osm_lon), Some(prev_osm_lat)) =
                    (iml_osm_meta.osm_lon, iml_osm_meta.osm_lat)
                {
                    if (prev_osm_lat - osm_stop.lat).abs() > FLOAT_TOLERANCE
                        || (prev_osm_lon - osm_stop.lon).abs() > FLOAT_TOLERANCE
                    {
                        changed = true;
                        coord_author_changed = true;
                        iml_osm_meta.osm_lat = Some(osm_stop.lat);
                        iml_osm_meta.osm_lon = Some(osm_stop.lon);
                    }
                } else {
                    changed = true;
                    iml_osm_meta.osm_lat = Some(osm_stop.lat);
                    iml_osm_meta.osm_lon = Some(osm_stop.lon);
                }

                // Check for a difference between OSM upstream and the cached OSM name
                if iml_osm_meta.osm_name != osm_stop.name {
                    changed = true;
                    iml_osm_meta.osm_name = osm_stop.name;
                }

                if iml_osm_meta.osm_version != osm_stop.version {
                    if iml_osm_meta.osm_version + 1 == osm_stop.version {
                        // We can figure without hitting the OSM API
                        let last_version_uid =
                            osm_stop.uid.parse::<i32>().unwrap();
                        if let Some(history) = &mut iml_osm_meta.osm_history {
                            history.deleted = false;
                            history.last_version = osm_stop.version;
                            history.versions.push(NodeVersion {
                                version: osm_stop.version,
                                author: last_version_uid,
                                lat: osm_stop.lat,
                                lon: osm_stop.lon,
                                attributes: osm_stop.attributes,
                                deleted: false,
                            });

                            if coord_author_changed {
                                history.pos_author_uid =
                                    osm_stop.uid.parse::<i32>().expect(
                                        "Received a non-i32 uid from Overpass",
                                    );
                                history.pos_author_uname =
                                    osm_stop.user.clone();
                            }
                        } else {
                            can_guess_history = false;
                        }
                    } else {
                        can_guess_history = false;
                    }

                    changed = true;
                    iml_osm_meta.osm_version = osm_stop.version;
                    iml_osm_meta.osm_author = Some(osm_stop.user);
                }

                updated_stops.push((
                    stop.id,
                    iml_osm_meta,
                    changed,
                    can_guess_history,
                ));
            }
        });
    return updated_stops;
}
