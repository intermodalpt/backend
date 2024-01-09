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
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub type StopId = String;
pub type TripId = String;
pub type RouteId = String;
pub type PatternId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    pub stop_id: StopId,
    pub stop_name: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTime {
    pub trip_id: TripId,
    pub stop_id: StopId,
    pub stop_sequence: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub route_id: RouteId,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_type: Option<String>,
    pub circular: Option<u8>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trip {
    pub route_id: RouteId,
    pub pattern_id: String,
    pub service_id: String,
    pub trip_id: TripId,
    pub trip_headsign: String,
}

pub enum File {
    Agency,
    CalendarDates,
    Facilities,
    FareAttributes,
    FareRules,
    FeedInfo,
    Helpdesks,
    Municipalities,
    Routes,
    Shapes,
    Stops,
    StopTimes,
    Trips,
}

impl File {
    #[must_use]
    pub fn filename(&self) -> &'static str {
        match self {
            File::Agency => "agency.txt",
            File::CalendarDates => "calendar_dates.txt",
            File::Facilities => "facilities.txt",
            File::FareAttributes => "fare_attributes.txt",
            File::FareRules => "fare_rules.txt",
            File::FeedInfo => "feed_info.txt",
            File::Helpdesks => "helpdesks.txt",
            File::Municipalities => "municipalities.txt",
            File::Routes => "routes.txt",
            File::Shapes => "shapes.txt",
            File::Stops => "stops.txt",
            File::StopTimes => "stop_times.txt",
            File::Trips => "trips.txt",
        }
    }

    #[must_use]
    pub fn prepend_root(&self, root: &Path) -> PathBuf {
        root.join(self.filename())
    }
}

// Validation structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCluster {
    // This field is the unique identifier
    pub stops: Vec<StopId>,
    // And these are just agglomerates
    pub headsigns: HashSet<PatternId>,
    pub patterns: HashSet<PatternId>,
    pub trips: HashSet<TripId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubrouteValidation {
    pub gtfs_pattern_ids: Vec<PatternId>,
    pub gtfs_trip_ids: Vec<TripId>,
    pub gtfs_headsigns: Vec<String>,
    pub iml_stops: Vec<i32>,
    pub gtfs_stops: Vec<StopId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteValidation {
    pub unmatched: Vec<PatternCluster>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorValidation {
    pub gtfs_lints: Vec<Lint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Lint {
    ServicelessTrip(TripId),
    EmptyTrip(TripId),
    EmptyPattern(PatternId),
    PatternlessRoute(RouteId),
    DuplicatedPattern(HashSet<PatternId>),
    UnusedStop(StopId),
    DanglingStopPointer(StopId),
}
