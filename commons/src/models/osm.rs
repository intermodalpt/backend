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

use chrono::{DateTime, Utc};
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OSMAuthor {
    pub uid: i32,
    pub username: String,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct StoredStopMeta {
    pub deleted: bool,
    pub versions: Vec<NodeVersion>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct StopNode {
    pub id: i64,
    pub versions: Vec<NodeVersion>,
}

pub type NodeHistory = Vec<NodeVersion>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeVersion {
    pub version: i32,
    pub author: i32,
    pub author_uname: String,
    pub lat: f64,
    pub lon: f64,
    pub attributes: Vec<(String, String)>,
    pub timestamp: DateTime<Utc>,
    pub deleted: bool,
}

impl PartialEq for NodeVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl Eq for NodeVersion {}

impl PartialOrd for NodeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapFeatures {
    pub highways: bool,
    pub addresses: bool,
    pub buildings: bool,
    pub terrain: bool,
    pub amenities: bool,
}
