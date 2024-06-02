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

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use commons::models::osm::NodeVersion;

use crate::models::OverpassStop;
use crate::{api, MAX_OSM_CALLS};

#[derive(Debug, Default)]
pub(crate) struct ImportStats {
    new_stops: usize,
    updated_stops: usize,
    deleted_stops: usize,
    osm_calls: usize,
}
pub(crate) async fn import() -> Result<ImportStats, Box<dyn std::error::Error>>
{
    let cached_osm_stops = api::fetch_cached_osm_stops().await.unwrap();
    let cached_osm_stop_index = cached_osm_stops
        .iter()
        .map(|stop| (stop.id, stop))
        .collect::<HashMap<_, _>>();
    let overpass_stops = api::fetch_overpass_stops().await.unwrap();

    let mut new_stops = vec![];

    let unreturned_ids = cached_osm_stops
        .iter()
        .filter_map(|s| if s.deleted { None } else { Some(s.id) })
        .collect::<HashSet<_>>();

    let mut ids_pending_history = vec![];

    let mut stats = ImportStats::default();

    let mut predicted_osm_calls: usize =
        calc_predicted_osm_calls(&overpass_stops, &cached_osm_stop_index);

    predicted_osm_calls += unreturned_ids.len();

    if predicted_osm_calls > MAX_OSM_CALLS {
        eprintln!("Snapshot exceeds reasonable OSM calls ({} calls predicted). Skipping non-immediate additions.", predicted_osm_calls);

        perform_light_patch(overpass_stops, &cached_osm_stop_index, &mut stats)
            .await?;
        return Ok(stats);
    }

    // Calculate new stops and updates figuring which needs additional queries
    for overpass_stop in overpass_stops {
        if let Some(cached_stop) = cached_osm_stop_index.get(&overpass_stop.id)
        {
            merge_overpass_stop(
                overpass_stop,
                cached_stop,
                &mut ids_pending_history,
                &mut stats,
            )
            .await?;
        } else {
            // Postpone the addition of new stops so we can bulk
            new_stops.push(overpass_stop);
        }
    }

    // Update the old stops that need history-queries
    for (id, current_version) in ids_pending_history {
        deep_update_stop(id, current_version, &mut stats).await?;
    }

    // Add the new stops that need history-queries
    for stop in new_stops.iter().filter(|stop| stop.version > 1) {
        let history = api::fetch_osm_node_versions(stop.id).await.unwrap();

        api::patch_osm_stops_history(&[api::OsmHistoryPatch {
            id: stop.id,
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

    for chunk in patch.chunks(100) {
        api::patch_osm_stops_history(&chunk).await?;
        stats.new_stops += chunk.len();
    }

    // Update deleted stops
    for id in unreturned_ids {
        let cached_stop = cached_osm_stop_index.get(&id).unwrap();
        merge_unreturned_stop(id, cached_stop, &mut stats).await?;
    }

    Ok(stats)
}

fn calc_predicted_osm_calls(
    overpass_stops: &[OverpassStop],
    cached_osm_stop_index: &HashMap<i64, &api::OsmStop>,
) -> usize {
    overpass_stops
        .iter()
        .map(|overpass_stop| {
            let version_diff = if let Some(cached_stop) =
                cached_osm_stop_index.get(&overpass_stop.id)
            {
                overpass_stop.version - cached_stop.version
            } else {
                overpass_stop.version
            };

            if version_diff == 0 || version_diff == 1 {
                0
            } else {
                1
            }
        })
        .sum()
}

async fn perform_light_patch(
    overpass_stops: Vec<OverpassStop>,
    cached_osm_stop_index: &HashMap<i64, &api::OsmStop>,
    stats: &mut ImportStats,
) -> Result<(), Box<dyn std::error::Error>> {
    let patch = overpass_stops
        .into_iter()
        .filter_map(|overpass_stop| {
            if let Some(cached_stop) =
                cached_osm_stop_index.get(&overpass_stop.id)
            {
                if overpass_stop.version != cached_stop.version + 1 {
                    // Do not attempt to patch version gaps bigger than one
                    return None;
                }
            } else if overpass_stop.version != 1 {
                // Do not attempt to patch new stops beyond their first versions
                return None;
            }
            let id = overpass_stop.id.clone();
            let history = vec![NodeVersion::from(overpass_stop)];
            Some(api::OsmHistoryPatch { id, history })
        })
        .collect_vec();

    api::patch_osm_stops_history(&patch).await?;
    stats.updated_stops += patch.len();

    Ok(())
}

async fn merge_overpass_stop(
    overpass_stop: OverpassStop,
    cached_stop: &api::OsmStop,
    ids_pending_history: &mut Vec<(i64, i32)>,
    stats: &mut ImportStats,
) -> Result<(), Box<dyn std::error::Error>> {
    if overpass_stop.version == cached_stop.version + 1 {
        let id = overpass_stop.id;

        println!(
            "Patching stop {id}. Versions {}->{}",
            cached_stop.version, overpass_stop.version
        );

        let mut history =
            api::fetch_cached_osm_stop_history(overpass_stop.id).await?;

        history.push(NodeVersion::from(overpass_stop));

        let version_integrity = history
            .iter()
            .map(|node| node.version)
            .sorted()
            .enumerate()
            .all(|(i, version)| i + 1 == version as usize);

        if !version_integrity {
            eprintln!("Version integrity failed for {}", id);
        }

        api::patch_osm_stops_history(&[api::OsmHistoryPatch { id, history }])
            .await?;

        stats.updated_stops += 1;
    } else if overpass_stop.version > cached_stop.version {
        println!(
            "Stop {} needs history query. ({} -> {})",
            overpass_stop.id, cached_stop.version, overpass_stop.version
        );
        ids_pending_history.push((overpass_stop.id, cached_stop.version));
    }

    Ok(())
}

async fn deep_update_stop(
    id: i64,
    current_version: i32,
    stats: &mut ImportStats,
) -> Result<(), Box<dyn std::error::Error>> {
    let history = api::fetch_osm_node_versions(id).await?;

    let upstream_version =
        history.iter().map(|node| node.version).max().unwrap_or(0);

    println!(
        "Patching stop {id}. Versions {}->{}",
        current_version, upstream_version
    );

    api::patch_osm_stops_history(&[api::OsmHistoryPatch { id, history }])
        .await?;

    // Sleep for 5s to be respectful of the OSM API
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    stats.osm_calls += 1;
    stats.updated_stops += 1;

    Ok(())
}

async fn merge_unreturned_stop(
    id: i64,
    cached_stop: &api::OsmStop,
    stats: &mut ImportStats,
) -> Result<(), Box<dyn std::error::Error>> {
    if cached_stop.deleted {
        return Ok(());
    }

    println!("Fetching history for deleted stop {}", id);
    let history = api::fetch_osm_node_versions(id).await.unwrap();

    api::patch_osm_stops_history(&[api::OsmHistoryPatch { id, history }])
        .await?;

    // Sleep for 5s to be respectful of the OSM API
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    stats.osm_calls += 1;
    stats.deleted_stops += 1;

    Ok(())
}
