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

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::stops::models as stops;

#[derive(Debug, Serialize, Deserialize)]
pub struct GTFSStop {
    pub stop_id: String,
    pub stop_name: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GTFSStopTimes {
    pub(crate) trip_id: String,
    pub(crate) stop_id: u32,
    pub(crate) stop_sequence: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GTFSTrips {
    pub(crate) route_id: String,
    service_id: String,
    pub(crate) trip_id: String,
    pub(crate) trip_headsign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMLTrip {
    pub(crate) id: String,
    pub(crate) headsign: String,
    pub(crate) stops: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMLRoute {
    pub(crate) id: String,
    pub(crate) trips: Vec<TMLTrip>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct TMLStop {
    #[serde(flatten)]
    pub stop: stops::Stop,
    pub tml_id_verified: bool,
    pub tml_id: Option<String>,
    pub tml_id_source: String,
    pub deleted_upstream: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MatchVerification {
    #[serde(default)]
    pub(crate) verified: bool,
    pub(crate) source: MatchSource,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MatchSource {
    Unknown,
    Manual,
    OSM,
    Flags,
}

impl fmt::Display for MatchSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchSource::Unknown => write!(f, "unknown"),
            MatchSource::Manual => write!(f, "manual"),
            MatchSource::OSM => write!(f, "osm"),
            MatchSource::Flags => write!(f, "flags"),
        }
    }
}
