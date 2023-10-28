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
use std::path::{Path, PathBuf};

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
    pub stop_id: String,
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
