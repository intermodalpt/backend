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

use serde::{Deserialize, Serialize};

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

pub(crate) struct Stop {
    pub id: i32,
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,

    pub osm_name: Option<String>,
    pub external_id: String,
    pub refs: Vec<String>,
}

impl From<XmlNode> for Stop {
    fn from(node: XmlNode) -> Self {
        let mut res = Self {
            id: -1,
            name: None,
            osm_name: None,
            lat: Some(node.lat),
            lon: Some(node.lon),
            external_id: node.id.to_string(),
            refs: vec![],
        };

        for tag in node.tags {
            match tag.k.as_str() {
                "name" => res.osm_name = Some(tag.v),
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
