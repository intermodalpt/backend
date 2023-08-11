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

mod models;
mod sql;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::BufReader;

use config::Config;
use osmpbf::{Element, ElementReader};
use sqlx::postgres::PgPool;

use commons::models::osm;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("SETTINGS"))
        .build()
        .unwrap();

    let pool = PgPool::connect(&settings.get_string("db").expect("db not set"))
        .await
        .expect("Unable to connect to the database");

    let stops = sql::fetch_stops(&pool).await.unwrap();

    let id_set = stops
        .iter()
        .map(|s| s.external_id.parse::<i64>().unwrap())
        .collect::<HashSet<i64>>();

    let f = std::fs::File::open("history-230724.osm.pbf").unwrap();
    let reader = BufReader::new(f);

    // let (nodes, authors) = par_extract_changesets(id_set, reader)
    let (nodes, authors) = extract_changesets(id_set, reader)
        .expect("Unable to extract changesets");

    let author_names: HashMap<i32, String> =
        authors.into_iter().map(|a| (a.uid, a.username)).collect();

    let mut transaction = pool.begin().await.unwrap();

    let mut updated = 0;
    for node in nodes.into_iter() {
        if updated % 100 == 0 {
            println!("Updated {} stops", updated);
        }
        updated += 1;
        sql::update_stop_osm_versions(&mut *transaction, node, &author_names)
            .await
            .unwrap();
    }

    transaction.commit().await.unwrap();
}

fn extract_changesets(
    id_set: HashSet<i64>,
    reader: BufReader<std::fs::File>,
) -> Result<(Vec<osm::StopNode>, Vec<osm::OSMAuthor>)> {
    let reader = ElementReader::new(reader);

    let mut authors: HashMap<i32, osm::OSMAuthor> = HashMap::new();
    let mut nodes: HashMap<i64, osm::StopNode> = HashMap::new();

    reader.for_each(|element| match element {
        Element::DenseNode(n) => {
            let id = n.id();
            if id_set.contains(&id) {
                if let Some(info) = n.info() {
                    let author_uid = info.uid();
                    authors.entry(author_uid).or_insert(osm::OSMAuthor {
                        uid: author_uid,
                        username: info.user().unwrap_or("").to_string(),
                    });

                    let version = osm::NodeVersion {
                        version: info.version(),
                        author: author_uid,
                        lat: n.lat(),
                        lon: n.lon(),
                        attributes: n
                            .tags()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect(),
                        deleted: info.deleted(),
                    };

                    match nodes.entry(id) {
                        Entry::Occupied(e) => {
                            let node = e.into_mut();
                            node.versions.push(version);
                        }
                        Entry::Vacant(e) => {
                            e.insert(osm::StopNode {
                                id,
                                versions: vec![version],
                            });
                        }
                    }
                } else {
                    eprintln!("No info for node {}", id);
                }
            }
        }
        _ => {}
    })?;

    nodes.values_mut().for_each(|node| {
        node.versions.sort();
    });

    Ok((
        nodes.into_values().collect(),
        authors.into_values().collect(),
    ))
}

/*
This function requires an unmerged patched version of OSMPBF
fn par_extract_changesets(
    id_set: HashSet<i64>,
    reader: BufReader<std::fs::File>,
) -> Result<(Vec<osm::StopNode>, Vec<osm::OSMAuthor>)> {
    let reader = ElementReader::new(reader);

    let mut authors: HashMap<i32, osm::OSMAuthor> = HashMap::new();
    let mut nodes: HashMap<i64, osm::StopNode> = HashMap::new();

    struct RawNode {
        id: i64,
        lat: f64,
        lon: f64,
        tags: Vec<(String, String)>,
        version: i32,
        uid: i32,
        username: String,
        deleted: bool,
    }

    let raw_nodes = reader.par_filter_map(|e| match e {
        Element::DenseNode(n) => {
            if id_set.contains(&n.id()) {
                let info = n.info().unwrap();
                Some(RawNode {
                    id: n.id(),
                    lat: n.lat(),
                    lon: n.lon(),
                    tags: n
                        .tags()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                    version: info.version(),
                    uid: info.uid(),
                    username: info.user().unwrap().to_string(),
                    deleted: info.deleted(),
                })
            } else {
                None
            }
        }
        _ => None,
    });

    raw_nodes.into_iter().for_each(|node| {
        authors.entry(node.uid).or_insert(osm::OSMAuthor {
            uid: node.uid,
            username: node.username,
        });

        let version = osm::NodeVersion {
            version: node.version,
            author: node.uid,
            lat: node.lat,
            lon: node.lon,
            attributes: node.tags,
            deleted: node.deleted,
        };

        match nodes.entry(node.id) {
            Entry::Occupied(e) => {
                let node = e.into_mut();
                node.versions.push(version);
            }
            Entry::Vacant(e) => {
                e.insert(osm::StopNode {
                    id: node.id,
                    versions: vec![version],
                });
            }
        }
    });

    nodes.values_mut().for_each(|node| {
        node.versions.sort();
    });

    Ok((
        nodes.into_values().collect(),
        authors.into_values().collect(),
    ))
}
*/
