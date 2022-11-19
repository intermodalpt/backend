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
use utoipa::Component;

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Stop {
    pub id: i32,
    #[component(example = "cmet")]
    pub source: String,
    #[component(example = "Setúbal (ITS)")]
    pub name: Option<String>,
    #[component(example = "Setúbal (ITS)")]
    pub official_name: Option<String>,
    #[component(example = "Setúbal (ITS)")]
    pub osm_name: Option<String>,
    #[component(example = "Setúbal")]
    pub short_name: Option<String>,
    #[component(example = "Bairro das bairradas")]
    pub locality: Option<String>,
    #[component(example = "Rua do Não Sei Decor")]
    pub street: Option<String>,
    #[component(example = "123-A")]
    pub door: Option<String>,
    pub parish: Option<i32>,
    #[component(example = 38.123_456)]
    pub lat: Option<f64>,
    #[component(example = -9.654_321)]
    pub lon: Option<f64>,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub succeeded_by: Option<i32>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub has_crossing: Option<bool>,
    #[serde(default)]
    pub has_accessibility: Option<bool>,
    #[serde(default)]
    pub has_abusive_parking: Option<bool>,
    #[serde(default)]
    pub has_outdated_info: Option<bool>,
    #[serde(default)]
    pub is_damaged: Option<bool>,
    #[serde(default)]
    pub is_vandalized: Option<bool>,
    #[serde(default)]
    pub has_flag: Option<bool>,
    #[serde(default)]
    pub has_schedules: Option<bool>,
    #[serde(default)]
    pub has_sidewalk: Option<bool>,
    #[serde(default)]
    pub has_shelter: Option<bool>,
    #[serde(default)]
    pub has_bench: Option<bool>,
    #[serde(default)]
    pub has_trash_can: Option<bool>,
    #[serde(default)]
    pub is_illuminated: Option<bool>,
    #[serde(default)]
    pub has_illuminated_path: Option<bool>,
    #[serde(default)]
    pub has_visibility_from_within: Option<bool>,
    #[serde(default)]
    pub has_visibility_from_area: Option<bool>,
    #[serde(default)]
    pub is_visible_from_outside: Option<bool>,
    pub updater: i32,
    pub update_date: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub(crate) mod requests {
    use serde::Deserialize;
    use utoipa::Component;

    #[derive(Deserialize, Component)]
    pub struct NewStop {
        pub source: String,
        pub lon: f64,
        pub lat: f64,
        pub name: Option<String>,
        pub short_name: Option<String>,
        pub official_name: Option<String>,
        pub locality: Option<String>,
        pub street: Option<String>,
        pub door: Option<String>,
        #[serde(default)]
        pub notes: Option<String>,
        #[serde(default)]
        pub has_crossing: Option<bool>,
        #[serde(default)]
        pub has_accessibility: Option<bool>,
        #[serde(default)]
        pub has_abusive_parking: Option<bool>,
        #[serde(default)]
        pub has_outdated_info: Option<bool>,
        #[serde(default)]
        pub is_damaged: Option<bool>,
        #[serde(default)]
        pub is_vandalized: Option<bool>,
        #[serde(default)]
        pub has_flag: Option<bool>,
        #[serde(default)]
        pub has_schedules: Option<bool>,
        #[serde(default)]
        pub has_sidewalk: Option<bool>,
        #[serde(default)]
        pub has_shelter: Option<bool>,
        #[serde(default)]
        pub has_bench: Option<bool>,
        #[serde(default)]
        pub has_trash_can: Option<bool>,
        #[serde(default)]
        pub is_illuminated: Option<bool>,
        #[serde(default)]
        pub has_illuminated_path: Option<bool>,
        #[serde(default)]
        pub has_visibility_from_within: Option<bool>,
        #[serde(default)]
        pub has_visibility_from_area: Option<bool>,
        #[serde(default)]
        pub is_visible_from_outside: Option<bool>,
        #[serde(default)]
        pub tags: Vec<String>,
    }

    #[derive(Deserialize, Component)]
    pub struct ChangeStop {
        pub lon: f64,
        pub lat: f64,
        pub name: Option<String>,
        pub short_name: Option<String>,
        pub official_name: Option<String>,
        pub locality: Option<String>,
        pub street: Option<String>,
        pub door: Option<String>,
        #[serde(default)]
        pub notes: Option<String>,
        #[serde(default)]
        pub has_crossing: Option<bool>,
        #[serde(default)]
        pub has_accessibility: Option<bool>,
        #[serde(default)]
        pub has_abusive_parking: Option<bool>,
        #[serde(default)]
        pub has_outdated_info: Option<bool>,
        #[serde(default)]
        pub is_damaged: Option<bool>,
        #[serde(default)]
        pub is_vandalized: Option<bool>,
        #[serde(default)]
        pub has_flag: Option<bool>,
        #[serde(default)]
        pub has_schedules: Option<bool>,
        #[serde(default)]
        pub has_sidewalk: Option<bool>,
        #[serde(default)]
        pub has_shelter: Option<bool>,
        #[serde(default)]
        pub has_bench: Option<bool>,
        #[serde(default)]
        pub has_trash_can: Option<bool>,
        #[serde(default)]
        pub is_illuminated: Option<bool>,
        #[serde(default)]
        pub has_illuminated_path: Option<bool>,
        #[serde(default)]
        pub has_visibility_from_within: Option<bool>,
        #[serde(default)]
        pub has_visibility_from_area: Option<bool>,
        #[serde(default)]
        pub is_visible_from_outside: Option<bool>,
        #[serde(default)]
        pub tags: Vec<String>,
    }
}

pub(crate) mod responses {
    use serde::Serialize;
    use std::collections::HashMap;
    use utoipa::Component;

    #[derive(Serialize, Component)]
    pub struct SpiderRoute {
        pub code: Option<String>,
        pub name: String,
        pub circular: bool,
    }

    #[derive(Serialize, Component)]
    pub struct SpiderSubroute {
        pub route: i32,
        pub flag: String,
        pub stop_sequence: Vec<i32>,
    }

    #[derive(Serialize, Component)]
    pub struct SpiderStop {
        pub name: Option<String>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
    }

    #[derive(Serialize, Component)]
    pub struct SpiderMap {
        pub routes: HashMap<i32, SpiderRoute>,
        pub subroutes: HashMap<i32, SpiderSubroute>,
        pub stops: HashMap<i32, SpiderStop>,
    }
}
