/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

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

use chrono::Local;
use serde::{Deserialize, Serialize};

use commons::models::stops;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct XmlOsm {
    #[serde(rename = "$value")]
    pub(crate) nodes: Vec<XmlNodeTypes>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum XmlNodeTypes {
    Meta(XmlMeta),
    Note(XmlNote),
    Node(XmlNode),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "meta")]
pub(crate) struct XmlMeta {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "note")]
pub(crate) struct XmlNote {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "node")]
pub(crate) struct XmlNode {
    id: i64,
    lon: f64,
    lat: f64,
    #[serde(rename = "$value")]
    tags: Vec<XMLTag>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "tag")]
pub(crate) struct XMLTag {
    k: String,
    v: String,
}

impl From<XmlNode> for stops::Stop {
    fn from(node: XmlNode) -> Self {
        let mut res = Self {
            id: -1,
            source: "osm".to_string(),
            name: None,
            official_name: None,
            osm_name: None,
            short_name: None,
            locality: None,
            street: None,
            door: None,
            parish: None,
            lat: Some(node.lat),
            lon: Some(node.lon),
            external_id: node.id.to_string(),
            refs: vec![],
            notes: None,
            a11y: stops::A11yMeta::default(),
            updater: -1,
            update_date: Local::now().to_string(),
            tags: vec![],
            verification_level: 0,
            service_check_date: None,
            infrastructure_check_date: None,
        };

        for tag in node.tags {
            match tag.k.as_str() {
                "name" => res.osm_name = Some(tag.v),
                "official_name" => res.official_name = Some(tag.v),
                "ref" => {
                    res.refs = tag
                        .v
                        .split(';')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>()
                }
                _ => {}
            }
        }

        res
    }
}
