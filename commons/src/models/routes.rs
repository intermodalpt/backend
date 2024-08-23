/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023 - 2024  Cláudio Pereira

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
    pub badge_text_color: Option<String>,
    pub badge_bg_color: Option<String>,

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

// Proposal to redo the subroutes with sequence deduplication

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: i32,
    pub type_id: i32,
    pub operator_id: i32,

    pub code: Option<String>,
    pub name: String,
    pub circular: bool,
    pub active: bool,
    pub badge_text_color: Option<String>,
    pub badge_bg_color: Option<String>,

    pub directions: Vec<RouteDirection>,
}

/// Points that should be near the routed path.
/// These are used to override bad routing
pub struct NavigationPoint {
    // Stop ID
    pub from: Option<i32>,
    // Stop ID
    pub to: Option<i32>,
    // Coord
    pub lon: f64,
    pub lat: f64,
}

/// A `RouteDirection` is the main path in one direction
/// Actual instances (`Subroute`s) inherit from here and
/// apply 0..n of the available patches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDirection {
    pub from: String,
    pub to: String,
    pub stop_sequence: Vec<i32>,
    // The defined, available patches (unapplied)
    pub patches: Vec<SegmentPatch>,
    pub subroutes: Vec<Subroute>,
    pub main_stops: Vec<i32>,
    pub through: Vec<NavigationPoint>,
}

/// A `SegmentPatch` alters a stop sequence
/// To inject stops between stops X and Y:
/// - `from` should point to X
/// - `to` should point to Y
/// - `seq` points at ids (`[a, b, c]`)
/// - such that the sequence becomes `[..., X, a, b, c, Y, ...]`
/// To delete stops (`[a, b, c]`) in a sequence (`[..., X, a, b, c, Y, ...]`)
/// - `from` should point to X
/// - `to` should point to Y
/// - `seq` points to an empty sequence (`[]`)
/// - such that the sequence becomes `[..., X, Y, ...]`
/// Appending to the end requires ``
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentPatch {
    pub from: Option<i32>,
    pub to: Option<i32>,
    pub seq: Vec<i32>,
    pub via: Option<SubrouteVia>,
    pub through: Vec<NavigationPoint>,
    // The stops which are to be appended to the main stops
    pub add_main_stops: Vec<i32>,
    // The stops which are to be removed from the main stops
    pub rm_main_stops: Vec<i32>,
}

/// A `Subroute` is a bus path that is real and has services
/// It adds up to enough information to tell the stop sequence, precise way,
/// code, headsign and such attributes.
/// It is instantiated by adding `Departure`s to it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subroute {
    pub id: i32,
    // The applied patches (over the RouteDirection)
    pub patches: Vec<i32>,

    pub code_override: Option<String>,
    pub headsign_override: Option<String>,
    /// A pair identifier should be common between subroutes
    /// which are equivalent among different directions
    pub pair: i32,
    pub circular: bool,

    pub polyline: Option<String>,
    pub validation: Option<gtfs::SubrouteValidation>,
}
 */
