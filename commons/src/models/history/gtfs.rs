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

use crate::models::gtfs as current;

pub type StopId = String;
pub type TripId = String;
pub type RouteId = String;
pub type PatternId = String;

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

impl From<current::PatternCluster> for PatternCluster {
    fn from(cluster: current::PatternCluster) -> Self {
        Self {
            stops: cluster.stops,
            headsigns: cluster.headsigns,
            patterns: cluster.patterns,
            trips: cluster.trips,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubrouteValidation {
    pub gtfs_pattern_ids: Vec<PatternId>,
    pub gtfs_trip_ids: Vec<TripId>,
    pub iml_stops: Vec<i32>,
    pub gtfs_stops: Vec<StopId>,
}

impl From<current::SubrouteValidation> for SubrouteValidation {
    fn from(validation: current::SubrouteValidation) -> Self {
        Self {
            gtfs_pattern_ids: validation.gtfs_pattern_ids,
            gtfs_trip_ids: validation.gtfs_trip_ids,
            iml_stops: validation.iml_stops,
            gtfs_stops: validation.gtfs_stops,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteValidation {
    pub unmatched: Vec<PatternCluster>,
}

impl From<current::RouteValidation> for RouteValidation {
    fn from(validation: current::RouteValidation) -> Self {
        Self {
            unmatched: super::vec_into_vec(validation.unmatched),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorValidation {
    pub lints: Vec<Lint>,
}

impl From<current::OperatorValidation> for OperatorValidation {
    fn from(validation: current::OperatorValidation) -> Self {
        Self {
            lints: super::vec_into_vec(validation.lints),
        }
    }
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

impl From<current::Lint> for Lint {
    fn from(lint: current::Lint) -> Self {
        match lint {
            current::Lint::ServicelessTrip(trip_id) => {
                Self::ServicelessTrip(trip_id)
            }
            current::Lint::EmptyTrip(trip_id) => Self::EmptyTrip(trip_id),
            current::Lint::EmptyPattern(pattern_id) => {
                Self::EmptyPattern(pattern_id)
            }
            current::Lint::PatternlessRoute(route_id) => {
                Self::PatternlessRoute(route_id)
            }
            current::Lint::DuplicatedPattern(patterns) => {
                Self::DuplicatedPattern(patterns)
            }
            current::Lint::UnusedStop(stop_id) => Self::UnusedStop(stop_id),
            current::Lint::DanglingStopPointer(stop_id) => {
                Self::DanglingStopPointer(stop_id)
            }
        }
    }
}
