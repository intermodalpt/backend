/*
    Intermodalis, transportation information aggregator
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

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct StopPic {
    pub id: i32,
    pub original_filename: String,
    pub sha1: String,
    pub public: bool,
    pub sensitive: bool,
    pub tagged: bool,
    pub uploader: i32,
    pub upload_date: String,
    pub capture_date: Option<String>,
    pub updater: Option<i32>,
    pub update_date: Option<String>,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub width: i32,
    pub height: i32,
    pub quality: i16,
    pub camera_ref: Option<String>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
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

    #[derive(Deserialize, Component)]
    pub struct ChangeRoute {
        pub code: String,
        pub name: String,
        pub circular: bool,
        pub main_subroute: Option<i32>,
        pub operator: i32,
        pub badge_text: String,
        pub badge_bg: String,
        pub active: bool,
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
    pub struct ChangeStopPic {
        pub public: bool,
        pub sensitive: bool,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub tags: Vec<String>,
        pub stops: Vec<i32>,
        pub notes: Option<String>,
        pub quality: i16,
    }
}

pub(crate) mod responses {

    use std::collections::HashMap;

    use crate::calendar::Calendar;
    use serde::Serialize;
    use utoipa::Component;

    #[derive(Serialize, Component)]
    pub struct Parish {
        pub id: i32,
        #[component(example = "Quinta do Conde")]
        pub name: String,
        #[component(example = "Sesimbra")]
        pub municipality: String,
        #[component(example = 3)]
        pub zone: i32,
        #[component(example = "GeoJSON polygon")]
        pub polygon: Option<String>,
    }

    #[derive(Serialize, Component)]
    pub struct Route {
        pub(crate) id: i32,
        pub(crate) subroutes: Vec<Subroute>,
        #[component(example = "Azeitão (Circular)")]
        pub(crate) code: String,
        pub(crate) name: String,
        #[component(example = true)]
        pub(crate) circular: bool,
        pub(crate) main_subroute: Option<i32>,
        pub(crate) badge_text: String,
        pub(crate) badge_bg: String,
        pub(crate) active: bool,
    }

    #[derive(Serialize, Component)]
    pub struct Subroute {
        pub(crate) id: i32,
        #[component(example = "Azeitão (Circular)")]
        pub(crate) flag: String,
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

    #[derive(Serialize, Component)]
    pub struct SpiderRoute {
        pub code: String,
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

    #[derive(Debug, Serialize, Component)]
    pub struct PublicStopPic {
        pub id: i32,
        pub sha1: String,
        pub capture_date: Option<String>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub quality: i16,
        pub tags: Vec<String>,
    }

    #[derive(Debug, Serialize, Component)]
    pub struct TaggedStopPic {
        pub id: i32,
        pub original_filename: String,
        pub sha1: String,
        pub public: bool,
        pub sensitive: bool,
        pub uploader: i32,
        pub upload_date: String,
        pub capture_date: Option<String>,
        // TODO if is tagged then this should not be optional.
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub width: i32,
        pub height: i32,
        pub quality: i16,
        pub camera_ref: Option<String>,
        pub tags: Vec<String>,
        pub notes: Option<String>,
    }

    #[derive(Debug, Serialize, Component)]
    pub struct UntaggedStopPic {
        pub id: i32,
        pub original_filename: String,
        pub sha1: String,
        pub public: bool,
        pub sensitive: bool,
        pub uploader: i32,
        pub upload_date: String,
        pub capture_date: Option<String>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub width: i32,
        pub height: i32,
        pub camera_ref: Option<String>,
        pub tags: Vec<String>,
        pub notes: Option<String>,
    }

    #[derive(Serialize, Clone, Component)]
    pub struct Stats {
        pub stop_count: i64,
        pub route_count: i64,
        pub subroute_count: i64,
        pub departure_count: i64,
        pub picture_count: i64,
    }
}
