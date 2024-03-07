/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2024  Cláudio Pereira

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

use chrono::{DateTime, Utc};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urlencoding::encode as urlencode;

use commons::models::osm;

use crate::models;

// const API_URL: &str = "https://api.intermodal.pt";
const API_URL: &str = "http://localhost:1893";

pub(crate) static TOKEN: OnceCell<&'static str> = OnceCell::new();

#[derive(Deserialize)]
pub struct OsmStop {
    pub id: i64,
    pub version: i32,
}

#[derive(Serialize, Deserialize)]
pub struct OsmHistoryPatch {
    pub id: i64,
    pub history: osm::NodeHistory,
}

pub(crate) async fn fetch_cached_osm_stop_versions(
) -> Result<HashMap<i64, i32>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/osm/stops", API_URL);
    println!("Fetching {}", url);
    let res = reqwest::Client::new()
        .get(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await?;

    if res.status().is_success() {
        let stops: Vec<OsmStop> = res.json().await?;
        Ok(stops
            .into_iter()
            .map(|stop| (stop.id, stop.version))
            .collect())
    } else {
        eprintln!("API error");
        eprintln!("Status: {}", res.status());
        std::process::exit(1);
    }
}

pub(crate) async fn fetch_cached_osm_stop_history(
    stop_id: i64,
) -> Result<osm::NodeHistory, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/osm/stop/{}", API_URL, stop_id);
    println!("Fetching {}", url);
    let res = reqwest::Client::new()
        .get(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await?;

    if res.status().is_success() {
        let history: osm::NodeHistory = res.json().await?;
        Ok(history)
    } else {
        eprintln!("API error");
        eprintln!("Status: {}", res.status());
        std::process::exit(1);
    }
}

pub(crate) async fn patch_iml_stop(
    stop_id: i32,
    meta: &StopOsmMeta,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/stops/{}/osm_meta", API_URL, stop_id);
    println!("Patching {}", url);
    let res = reqwest::Client::new()
        .patch(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .json(meta)
        .send()
        .await?;
    if res.status().is_success() {
        Ok(())
    } else {
        eprintln!("API error");
        eprintln!("Status: {}", res.status());
        eprintln!("Error: {}", res.text().await.unwrap());
        std::process::exit(1);
    }
}

pub(crate) async fn fetch_osm_stops(
) -> Result<models::XmlOsm, Box<dyn std::error::Error>> {
    let query = r#"
    area[name="Portugal"][admin_level=2];
        node["highway"="bus_stop"](area)->.a;
    area[name="Portugal"][admin_level=2];
        node["station"="light_rail"](area)->.b;
    (.a;.b;);
    out meta;"#;

    let osm_query_url = format!(
        "https://overpass-api.de/api/interpreter?data={}",
        urlencode(query)
    );
    dbg!(&osm_query_url);

    // TODO wrong errors
    let xml = reqwest::get(&osm_query_url).await?.text().await?;
    Ok(serde_xml_rs::from_str(&xml)?)
}

pub(crate) async fn fetch_osm_node_versions(
    osm_node_id: &str,
) -> Result<Vec<osm::NodeVersion>, Box<dyn std::error::Error>> {
    let osm_query_url = format!(
        "https://www.openstreetmap.org/api/0.6/node/{}/history",
        osm_node_id
    );
    println!("Fetching {}", osm_query_url);

    let res = reqwest::Client::new()
        .get(&osm_query_url)
        // Compliance with the OSM API policy
        .header("User-Agent", "Intermodal-utils (https://intermodal.pt)")
        .send()
        .await?;
    let xml = res.text().await?;
    let osm_xml = serde_xml_rs::from_str::<models::XmlOsm>(&xml)?;

    let mut history: Vec<osm::NodeVersion> = osm_xml
        .nodes
        .into_iter()
        .map(|node| match node {
            models::XmlNodeTypes::Node(node) => osm::NodeVersion {
                version: node.version,
                author: node.uid,
                author_uname: node.user,
                lat: node.lat.unwrap_or(-0.0),
                lon: node.lon.unwrap_or(-0.0),
                attributes: node
                    .tags
                    .into_iter()
                    .map(|tag| (tag.k, tag.v))
                    .collect(),
                timestamp: Default::default(),
                deleted: !node.visible.unwrap(),
            },
            _ => {
                panic!("Unexpected node type")
            }
        })
        .sorted_by_key(|v| v.version)
        .collect();

    // Add missing coords to deleted versions
    let mut last_pos = (0.0, 0.0);
    history.iter_mut().for_each(|node_version| {
        if node_version.deleted {
            node_version.lon = last_pos.0;
            node_version.lat = last_pos.1;
        } else {
            last_pos = (node_version.lon, node_version.lat);
        }
    });

    Ok(history)
}

#[cfg(test)]
mod test {
    use crate::models;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_deserialization() {
        let data = r#"
<osm>
 <node id="1111" visible="true" version="1" changeset="444" timestamp="2011-02-16T14:22:25Z" user="Foo1" uid="123" lat="38.0" lon="-8.0"/>
 <node id="2222" visible="true" version="2" changeset="555" timestamp="2011-10-03T19:13:43Z" user="Foo2" uid="456" lat="38.1" lon="-8.1"/>
 <node id="3333" visible="true" version="3" changeset="666" timestamp="2022-08-12T05:47:09Z" user="Foo2" uid="789" lat="38.2" lon="-8.2">
  <tag k="bus" v="yes"/>
 </node>
</osm>
    "#;
        let xml_root: models::XmlOsm = serde_xml_rs::from_str(data).unwrap();
        let proper_parse = vec![
            models::XmlNodeTypes::Node(models::XmlNode {
                id: 1111,
                visible: Some(true),
                version: 1,
                user: "Foo1".to_string(),
                uid: 123,
                lat: Some(38.0),
                lon: Some(-8.0),
                tags: vec![],
                timestamp: "2011-02-16T14:22:25Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap(),
            }),
            models::XmlNodeTypes::Node(models::XmlNode {
                id: 2222,
                visible: Some(true),
                version: 2,
                user: "Foo2".to_string(),
                uid: 456,
                lat: Some(38.1),
                lon: Some(-8.1),
                tags: vec![],
                timestamp: "2011-10-03T19:13:43Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap(),
            }),
            models::XmlNodeTypes::Node(models::XmlNode {
                id: 3333,
                visible: Some(true),
                version: 3,
                user: "Foo2".to_string(),
                uid: 789,
                lat: Some(38.2),
                lon: Some(-8.2),
                tags: vec![models::XMLTag {
                    k: "bus".to_string(),
                    v: "yes".to_string(),
                }],
                timestamp: "2022-08-12T05:47:09Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap(),
            }),
        ];
        assert_eq!(xml_root.nodes, proper_parse);
    }
}
