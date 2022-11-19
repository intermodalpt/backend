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

pub(crate) mod requests {
    use crate::calendar::models::Calendar;
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
        pub service_type: i32,
    }

    #[derive(Deserialize, Component)]
    pub struct ChangeSubroute {
        pub flag: String,
        pub circular: bool,
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
        pub calendar: Calendar,
    }
}

pub(crate) mod responses {
    use crate::calendar::models::Calendar;
    use serde::Serialize;
    use utoipa::Component;

    #[derive(Serialize, Component)]
    pub struct Route {
        pub(crate) id: i32,
        pub(crate) service_type: i32,
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

    #[derive(Debug, Serialize, Component, sqlx::Type)]
    pub struct Subroute {
        pub(crate) id: i32,
        #[component(example = "Azeitão (Circular)")]
        pub(crate) flag: String,
        pub(crate) circular: bool,
        // #[component(example = 123)]
        // pub(crate) cached_from: Option<i32>,
        // #[component(example = 123)]
        // pub(crate) cached_to: Option<i32>,
    }

    #[derive(Serialize, Component)]
    pub struct Departure {
        pub id: i32,
        pub subroute: i32,
        // Departure time in minutes starting at midnight
        #[component(example = 480)]
        pub time: i16,
        pub calendar: Calendar,
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
