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
use serde::{Deserialize, Serialize};

use commons::models::osm;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct XmlOsm {
    #[serde(rename = "$value", default)]
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
    pub(crate) id: i64,
    pub(crate) visible: Option<bool>,
    pub(crate) lon: Option<f64>,
    pub(crate) lat: Option<f64>,
    pub(crate) version: i32,
    pub(crate) user: String,
    pub(crate) uid: i32,
    pub(crate) timestamp: DateTime<Utc>,
    #[serde(rename = "$value", default)]
    pub(crate) tags: Vec<XMLTag>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "tag")]
pub(crate) struct XMLTag {
    pub(crate) k: String,
    pub(crate) v: String,
}

#[derive(Debug)]
pub(crate) struct OverpassStop {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub version: i32,
    pub uid: i32,
    pub user: String,
    pub attributes: Vec<(String, String)>,
    pub timestamp: DateTime<Utc>,
}

impl From<XmlNode> for OverpassStop {
    fn from(node: XmlNode) -> Self {
        Self {
            lat: node.lat.expect("Overpass returned a node without a lat"),
            lon: node.lon.expect("Overpass returned a node without a lon"),
            version: node.version,
            id: node.id.to_string(),
            user: node.user,
            uid: node.uid,
            attributes: node
                .tags
                .into_iter()
                .map(|tag| (tag.k, tag.v))
                .collect_vec(),
            timestamp: node.timestamp,
        }
    }
}
impl From<OverpassStop> for osm::NodeVersion {
    fn from(overpass_stop: OverpassStop) -> Self {
        osm::NodeVersion {
            version: overpass_stop.version,
            author: overpass_stop.uid,
            author_uname: overpass_stop.user,
            lat: overpass_stop.lat,
            lon: overpass_stop.lon,
            attributes: overpass_stop.attributes,
            timestamp: overpass_stop.timestamp,
            deleted: false,
        }
    }
}
