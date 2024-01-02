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

use super::gtfs;
use crate::errors::Error;
use crate::models::routes as current;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: i32,
    pub type_id: i32,
    pub operator_id: i32,
    pub code: Option<String>,
    pub name: String,
    #[serde(default)]
    pub circular: Option<bool>,
    pub active: bool,

    // --- TODO Maybe deprecate. Keep for historical data
    pub main_subroute: Option<i32>,
}

impl From<current::Route> for Route {
    fn from(route: current::Route) -> Self {
        Self {
            id: route.id,
            type_id: route.type_id,
            operator_id: route.operator_id,
            code: route.code,
            name: route.name,
            circular: Some(route.circular),
            active: route.active,
            main_subroute: route.main_subroute,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subroute {
    pub id: i32,
    pub route_id: i32,
    pub group: Option<i32>,
    pub origin: Option<String>,
    pub destination: Option<String>,
    pub headsign: Option<String>,
    pub via: Option<Vec<SubrouteVia>>,
    pub circular: bool,
    pub validation: Option<gtfs::SubrouteValidation>,

    // Deprecated
    pub flag: Option<String>,
    // TODO Delete this from history if it ever becomes an annoyance
    pub polyline: Option<String>,
}

impl From<current::Subroute> for Subroute {
    fn from(subroute: current::Subroute) -> Self {
        Self {
            id: subroute.id,
            route_id: subroute.route_id,
            group: Some(subroute.group),
            origin: Some(subroute.origin),
            destination: Some(subroute.destination),
            headsign: Some(subroute.headsign),
            via: Some(super::vec_into_vec(subroute.via)),
            circular: subroute.circular,
            validation: subroute.validation.map(Into::into),
            flag: Some(subroute.flag),
            polyline: subroute.polyline,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubrouteVia {
    pub name: String,
    pub stops: Option<[i32; 2]>,
}

impl From<current::SubrouteVia> for SubrouteVia {
    fn from(via: current::SubrouteVia) -> Self {
        Self {
            name: via.name,
            stops: via.stops,
        }
    }
}

impl From<SubrouteVia> for current::SubrouteVia {
    fn from(via: SubrouteVia) -> Self {
        Self {
            name: via.name,
            stops: via.stops,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Departure {
    pub id: i32,
    pub subroute_id: i32,
    pub time: i16,
    pub calendar_id: i32,
}

impl From<current::Departure> for Departure {
    fn from(departure: current::Departure) -> Self {
        Self {
            id: departure.id,
            subroute_id: departure.subroute_id,
            time: departure.time,
            calendar_id: departure.calendar_id,
        }
    }
}

impl TryFrom<Departure> for current::Departure {
    type Error = Error;

    fn try_from(departure: Departure) -> Result<Self, Self::Error> {
        Ok(Self {
            id: departure.id,
            subroute_id: departure.subroute_id,
            time: departure.time,
            calendar_id: departure.calendar_id,
        })
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RoutePatch {
    pub type_id: Option<i32>,
    pub operator_id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub code: Option<Option<String>>,
    pub name: Option<String>,
    pub circular: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub main_subroute: Option<Option<i32>>,
    pub active: Option<bool>,
}

impl RoutePatch {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.type_id.is_none()
            && self.operator_id.is_none()
            && self.code.is_none()
            && self.name.is_none()
            && self.circular.is_none()
            && self.main_subroute.is_none()
            && self.active.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, route: &mut current::Route) {
        if let Some(type_id) = self.type_id {
            route.type_id = type_id;
        }
        if let Some(operator) = self.operator_id {
            route.operator_id = operator;
        }
        if let Some(code) = self.code {
            route.code = code;
        }
        if let Some(name) = self.name {
            route.name = name;
        }
        if let Some(main_subroute) = self.main_subroute {
            route.main_subroute = main_subroute;
        }
        if let Some(active) = self.active {
            route.active = active;
        }
        if let Some(circular) = self.circular {
            route.circular = circular;
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SubroutePatch {
    pub group: Option<i32>,
    pub flag: Option<String>,
    pub headsign: Option<String>,
    pub origin: Option<String>,
    pub destination: Option<String>,
    pub via: Option<Vec<SubrouteVia>>,
    pub circular: Option<bool>,

    // TODO This ended up here by mistake. Drop it
    pub polyline: Option<String>,
}

impl SubroutePatch {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.group.is_none()
            && self.flag.is_none()
            && self.headsign.is_none()
            && self.origin.is_none()
            && self.destination.is_none()
            && self.via.is_none()
            && self.circular.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, subroute: &mut current::Subroute) -> Result<(), Error> {
        if let Some(group) = self.group {
            subroute.group = group;
        }
        if let Some(flag) = self.flag {
            subroute.flag = flag;
        }
        if let Some(headsign) = self.headsign {
            subroute.headsign = headsign;
        }
        if let Some(origin) = self.origin {
            subroute.origin = origin;
        }
        if let Some(destination) = self.destination {
            subroute.destination = destination;
        }
        if let Some(via) = self.via {
            subroute.via = via.into_iter().map(Into::into).collect::<Vec<_>>();
        }
        if let Some(circular) = self.circular {
            subroute.circular = circular;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeparturePatch {
    pub time: Option<i16>,
    pub subroute_id: Option<i32>,
    pub calendar_id: Option<i32>,
}

impl DeparturePatch {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.time.is_none()
            && self.subroute_id.is_none()
            && self.calendar_id.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, departure: &mut current::Departure) {
        if let Some(time) = self.time {
            departure.time = time;
        }
        if let Some(subroute_id) = self.subroute_id {
            departure.subroute_id = subroute_id;
        }
        if let Some(calendar_id) = self.calendar_id {
            departure.calendar_id = calendar_id;
        }
    }
}
