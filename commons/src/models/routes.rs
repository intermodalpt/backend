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

use crate::models::gtfs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: i32,
    pub type_id: i32,
    pub operator_id: i32,
    pub code: Option<String>,
    pub name: String,
    // FIXME this default is temporary while we have change logs without it
    #[serde(default)]
    pub circular: bool,
    pub active: bool,

    // --- TODO Maybe deprecate. Keep for historical data
    pub main_subroute: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subroute {
    pub id: i32,
    pub route_id: i32,
    #[serde(default)]
    pub group: i32,
    pub origin: String,
    pub destination: String,
    pub headsign: String,
    pub via: Vec<SubrouteVia>,
    pub circular: bool,
    pub polyline: Option<String>,
    pub validation: Option<gtfs::SubrouteValidation>,

    // --- Deprecated. Needed for historical data
    pub flag: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubrouteVia {
    pub name: String,
    pub stops: Option<[i32; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Departure {
    pub id: i32,
    pub subroute_id: i32,
    pub time: i16,
    pub calendar_id: i32,
}
