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
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct GTFSStop {
    pub stop_id: String,
    pub stop_name: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GTFSStopTimes {
    pub trip_id: String,
    pub stop_id: u32,
    pub stop_sequence: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GTFSRoute {
    pub route_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GTFSTrips {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: String,
}

const GTFS_FILES: [&'static str; 13] = [
    "agency.txt",
    "calendar_dates.txt",
    "facilities.txt",
    "fare_attributes.txt",
    "fare_rules.txt",
    "feed_info.txt",
    "helpdesks.txt",
    "municipalities.txt",
    "routes.txt",
    "shapes.txt",
    "stops.txt",
    "stop_times.txt",
    "trips.txt",
];
pub enum GtfsFile {
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

impl GtfsFile {
    pub fn filename(&self) -> &'static str {
        match self {
            GtfsFile::Agency => "agency.txt",
            GtfsFile::CalendarDates => "calendar_dates.txt",
            GtfsFile::Facilities => "facilities.txt",
            GtfsFile::FareAttributes => "fare_attributes.txt",
            GtfsFile::FareRules => "fare_rules.txt",
            GtfsFile::FeedInfo => "feed_info.txt",
            GtfsFile::Helpdesks => "helpdesks.txt",
            GtfsFile::Municipalities => "municipalities.txt",
            GtfsFile::Routes => "routes.txt",
            GtfsFile::Shapes => "shapes.txt",
            GtfsFile::Stops => "stops.txt",
            GtfsFile::StopTimes => "stop_times.txt",
            GtfsFile::Trips => "trips.txt",
        }
    }

    pub fn prepend_root(&self, root: &PathBuf) -> PathBuf {
        root.join(self.filename())
    }
}
