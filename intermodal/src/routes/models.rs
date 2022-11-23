/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022  Cláudio Pereira

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
use serde_repr::Serialize_repr;
use utoipa::Component;

use crate::calendar::Calendar;

#[repr(u8)]
#[derive(Debug, Serialize_repr)]
pub enum DepartureChangeType {
    New = 0,
    Change = 1,
    Cancel = 2,
}

#[derive(Debug, Serialize, Component)]
pub struct DepartureChange {
    pub id: i32,
    pub name: String,
    pub calendar: Calendar,
    pub departure_change_type: DepartureChangeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub(crate) id: i32,
    pub(crate) type_id: i32,
    pub(crate) operator: i32,
    pub(crate) code: Option<String>,
    pub(crate) name: String,
    pub(crate) circular: bool,
    pub(crate) main_subroute: Option<i32>,
    pub(crate) active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subroute {
    pub(crate) id: i32,
    pub(crate) route_id: i32,
    pub(crate) flag: String,
    pub(crate) circular: bool,
    pub(crate) polyline: Option<String>,
}

pub(crate) mod requests {
    use crate::calendar::Calendar;
    use serde::Deserialize;
    use utoipa::Component;

    #[derive(Deserialize, Component)]
    pub struct ChangeRoute {
        pub code: Option<String>,
        pub name: String,
        pub circular: bool,
        pub main_subroute: Option<i32>,
        pub operator: i32,
        pub active: bool,
        pub type_id: i32,
    }

    impl From<super::Route> for ChangeRoute {
        fn from(route: super::Route) -> Self {
            Self {
                code: route.code,
                name: route.name,
                circular: route.circular,
                main_subroute: route.main_subroute,
                operator: route.operator,
                active: route.active,
                type_id: route.type_id,
            }
        }
    }

    #[derive(Deserialize, Component)]
    pub struct ChangeSubroute {
        pub flag: String,
        pub circular: bool,
        pub polyline: Option<String>,
    }

    impl From<super::Subroute> for ChangeSubroute {
        fn from(subroute: super::Subroute) -> Self {
            Self {
                flag: subroute.flag,
                circular: subroute.circular,
                polyline: subroute.polyline,
            }
        }
    }

    #[derive(Deserialize, Component)]
    pub struct SubrouteStops {
        pub stops: Vec<i32>,
        pub diffs: Vec<Option<i32>>,
    }

    #[derive(Deserialize, Component)]
    pub struct ChangeSubrouteStops {
        pub from: SubrouteStops,
        pub to: SubrouteStops,
    }

    #[derive(Debug, Deserialize, Component)]
    pub struct ChangeDeparture {
        pub time: i16,
        pub calendar: Option<Calendar>,
        pub calendar_id: Option<i32>,
    }
}

pub(crate) mod responses {
    use crate::calendar::Calendar;
    use serde::Serialize;
    use utoipa::Component;

    #[derive(Serialize, Component)]
    pub struct Route {
        pub(crate) id: i32,
        pub(crate) type_id: i32,
        pub(crate) operator: i32,
        pub(crate) subroutes: Vec<Subroute>,
        #[component(example = "Azeitão (Circular)")]
        pub(crate) code: Option<String>,
        pub(crate) name: String,
        #[component(example = true)]
        pub(crate) circular: bool,
        pub(crate) main_subroute: Option<i32>,
        pub(crate) badge_text: String,
        pub(crate) badge_bg: String,
        pub(crate) active: bool,
    }

    #[derive(Debug, Serialize, Component)]
    pub struct Subroute {
        pub(crate) id: i32,
        #[component(example = "Azeitão (Circular)")]
        pub(crate) flag: String,
        pub(crate) circular: bool,
        pub(crate) polyline: Option<String>,
    }

    #[derive(Serialize, Component)]
    pub struct Departure {
        pub id: i32,
        pub subroute: i32,
        // Departure time in minutes starting at midnight
        #[component(example = 480)]
        pub time: i16,
        // TODO replace this
        pub calendar: Option<Calendar>,
        // With this
        pub calendar_id: Option<i32>,
    }

    #[derive(Serialize, Component)]
    pub struct DateDeparture {
        pub subroute: i32,
        #[component(example = 480)]
        pub time: i16,
    }

    #[derive(Serialize, Component)]
    pub struct SubrouteStops {
        pub subroute: i32,
        pub stops: Vec<i32>,
        pub diffs: Vec<Option<i32>>,
    }
}
