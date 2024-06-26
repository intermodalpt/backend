/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

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
    use chrono::NaiveDate;
    use serde::Deserialize;

    use commons::models::{history, stops};

    #[derive(Debug, Deserialize)]
    pub struct NewStop {
        pub lon: f64,
        pub lat: f64,
        pub name: String,
        #[serde(default)]
        pub short_name: Option<String>,
        #[serde(default)]
        pub locality: Option<String>,
        #[serde(default)]
        pub street: Option<String>,
        #[serde(default)]
        pub door: Option<String>,
        #[serde(default)]
        pub notes: Option<String>,
        #[serde(default)]
        pub tags: Vec<String>,
        #[serde(default, flatten)]
        pub a11y: stops::A11yMeta,
        #[serde(default)]
        pub verification_level: u8,
        #[serde(default)]
        pub service_check_date: Option<NaiveDate>,
        #[serde(default)]
        pub infrastructure_check_date: Option<NaiveDate>,

        pub osm_id: Option<i64>,
        pub license: String,
        pub is_ghost: bool,
    }

    #[derive(Clone, Deserialize)]
    pub struct ChangeStop {
        pub lon: f64,
        pub lat: f64,

        pub name: String,
        pub short_name: Option<String>,
        pub locality: Option<String>,
        pub street: Option<String>,
        pub door: Option<String>,
        pub notes: Option<String>,
        pub tags: Vec<String>,

        pub a11y: stops::A11yMeta,
        pub verification_level: u8,
        pub service_check_date: Option<NaiveDate>,
        pub infrastructure_check_date: Option<NaiveDate>,

        pub license: String,
        pub is_ghost: bool,
    }

    impl From<stops::Stop> for ChangeStop {
        fn from(stop: stops::Stop) -> Self {
            ChangeStop {
                lon: stop.lon,
                lat: stop.lat,
                name: stop.name,
                short_name: stop.short_name,
                locality: stop.locality,
                street: stop.street,
                door: stop.door,
                notes: stop.notes,
                tags: stop.tags,
                a11y: stop.a11y,
                verification_level: stop.verification_level,
                service_check_date: stop.service_check_date,
                infrastructure_check_date: stop.infrastructure_check_date,
                license: stop.license,
                is_ghost: stop.is_ghost,
            }
        }
    }

    impl ChangeStop {
        #[allow(clippy::too_many_lines)]
        pub fn derive_patch(
            &self,
            stop: &stops::Stop,
        ) -> history::stops::StopPatch {
            let mut patch = history::stops::StopPatch::default();

            if self.name != stop.name {
                patch.name = Some(self.name.clone());
            }
            if self.short_name != stop.short_name {
                patch.short_name = Some(self.short_name.clone());
            }
            if self.locality != stop.locality {
                patch.locality = Some(self.locality.clone());
            }
            if self.street != stop.street {
                patch.street = Some(self.street.clone());
            }
            if self.door != stop.door {
                patch.door = Some(self.door.clone());
            }

            if self.a11y.schedules != stop.a11y.schedules {
                patch.schedules = Some(history::opt_vec_into_opt_vec(
                    self.a11y.schedules.clone(),
                ));
            }
            if self.a11y.flags != stop.a11y.flags {
                patch.flags = Some(history::opt_vec_into_opt_vec(
                    self.a11y.flags.clone(),
                ));
            }

            if self.a11y.has_sidewalk != stop.a11y.has_sidewalk {
                patch.has_sidewalk = Some(self.a11y.has_sidewalk);
            }
            if self.a11y.has_sidewalked_path != stop.a11y.has_sidewalked_path {
                patch.has_sidewalked_path = Some(self.a11y.has_sidewalked_path);
            }

            if self.a11y.has_shelter != stop.a11y.has_shelter {
                patch.has_shelter = Some(self.a11y.has_shelter);
            }
            if self.a11y.has_cover != stop.a11y.has_cover {
                patch.has_cover = Some(self.a11y.has_cover);
            }
            if self.a11y.has_bench != stop.a11y.has_bench {
                patch.has_bench = Some(self.a11y.has_bench);
            }
            if self.a11y.has_trash_can != stop.a11y.has_trash_can {
                patch.has_trash_can = Some(self.a11y.has_trash_can);
            }
            if self.a11y.has_waiting_times != stop.a11y.has_waiting_times {
                patch.has_waiting_times = Some(self.a11y.has_waiting_times);
            }
            if self.a11y.has_ticket_seller != stop.a11y.has_ticket_seller {
                patch.has_ticket_seller = Some(self.a11y.has_ticket_seller);
            }
            if self.a11y.has_costumer_support != stop.a11y.has_costumer_support
            {
                patch.has_costumer_support =
                    Some(self.a11y.has_costumer_support);
            }

            if self.a11y.advertisement_qty != stop.a11y.advertisement_qty {
                patch.advertisement_qty =
                    Some(self.a11y.advertisement_qty.map(Into::into));
            }

            if self.a11y.has_crossing != stop.a11y.has_crossing {
                patch.has_crossing = Some(self.a11y.has_crossing);
            }

            if self.a11y.has_wide_access != stop.a11y.has_wide_access {
                patch.has_wide_access = Some(self.a11y.has_wide_access);
            }
            if self.a11y.has_flat_access != stop.a11y.has_flat_access {
                patch.has_flat_access = Some(self.a11y.has_flat_access);
            }
            if self.a11y.has_tactile_access != stop.a11y.has_tactile_access {
                patch.has_tactile_access = Some(self.a11y.has_tactile_access);
            }

            if self.a11y.illumination_strength
                != stop.a11y.illumination_strength
            {
                patch.illumination_strength =
                    Some(self.a11y.illumination_strength.map(Into::into));
            }
            if self.a11y.illumination_position
                != stop.a11y.illumination_position
            {
                patch.illumination_position =
                    Some(self.a11y.illumination_position.map(Into::into));
            }
            if self.a11y.has_illuminated_path != stop.a11y.has_illuminated_path
            {
                patch.has_illuminated_path =
                    Some(self.a11y.has_illuminated_path);
            }
            if self.a11y.has_visibility_from_within
                != stop.a11y.has_visibility_from_within
            {
                patch.has_visibility_from_within =
                    Some(self.a11y.has_visibility_from_within);
            }
            if self.a11y.has_visibility_from_area
                != stop.a11y.has_visibility_from_area
            {
                patch.has_visibility_from_area =
                    Some(self.a11y.has_visibility_from_area);
            }
            if self.a11y.is_visible_from_outside
                != stop.a11y.is_visible_from_outside
            {
                patch.is_visible_from_outside =
                    Some(self.a11y.is_visible_from_outside);
            }

            if self.a11y.parking_visibility_impairment
                != stop.a11y.parking_visibility_impairment
            {
                patch.parking_visibility_impairment = Some(
                    self.a11y.parking_visibility_impairment.map(Into::into),
                );
            }

            if self.a11y.parking_local_access_impairment
                != stop.a11y.parking_local_access_impairment
            {
                patch.parking_local_access_impairment = Some(
                    self.a11y.parking_local_access_impairment.map(Into::into),
                );
            }

            if self.a11y.parking_area_access_impairment
                != stop.a11y.parking_area_access_impairment
            {
                patch.parking_area_access_impairment = Some(
                    self.a11y.parking_area_access_impairment.map(Into::into),
                );
            }

            if self.a11y.tmp_issues != stop.a11y.tmp_issues {
                patch.tmp_issues = Some(self.a11y.tmp_issues.clone());
            }

            if self.tags != stop.tags {
                patch.tags = Some(self.tags.clone());
            }
            if self.notes != stop.notes {
                patch.notes = Some(self.notes.clone());
            }

            if self.verification_level != stop.verification_level {
                patch.verification_level = Some(self.verification_level);
            }
            if self.service_check_date != stop.service_check_date {
                patch.service_check_date = Some(self.service_check_date);
            }
            if self.infrastructure_check_date != stop.infrastructure_check_date
            {
                patch.infrastructure_check_date =
                    Some(self.infrastructure_check_date);
            }

            if self.license != stop.license {
                patch.license = Some(self.license.clone());
            }

            if self.is_ghost != stop.is_ghost {
                patch.is_ghost = Some(self.is_ghost);
            }

            patch
        }
    }

    #[derive(Deserialize)]
    pub struct Position {
        pub lon: f64,
        pub lat: f64,
    }
}

pub(crate) mod responses {
    use chrono::{DateTime, NaiveDate, Utc};
    use serde::{Deserialize, Serialize};
    use sqlx::types::Json;
    use std::collections::HashMap;

    use commons::models::stops;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct OperatorStopRel {
        pub operator_id: i32,
        pub stop_ref: Option<String>,
        pub name: Option<String>,
        pub source: String,
    }

    /// Meant to be a minimal stop for the client to fill the UI with
    /// It should request `Stop` from then on
    #[derive(Debug, Clone, Serialize, PartialEq)]
    pub struct SimpleStop {
        pub id: i32,
        pub name: String,
        pub short_name: Option<String>,
        pub lat: f64,
        pub lon: f64,
    }

    /// Meant to be an information-rich stop for the client
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Stop {
        pub id: i32,
        pub name: String,
        pub short_name: Option<String>,
        pub locality: Option<String>,
        pub street: Option<String>,
        pub door: Option<String>,
        pub parish: Option<i32>,
        pub lat: f64,
        pub lon: f64,
        pub notes: Option<String>,
        pub tags: Vec<String>,
        pub a11y: Json<stops::A11yMeta>,
        // This is an 8 bit flag (u32 because of postgres::Decode) made of 4 duets.
        // The four binary duets are for: Position, Service, Infra and [reserved]
        // 0 => Not verified; 1 => Wrong; 2 => Likely; 3 => Verified
        #[serde(default)]
        pub verification_level: i16,

        #[serde(default)]
        pub service_check_date: Option<NaiveDate>,
        #[serde(default)]
        pub infrastructure_check_date: Option<NaiveDate>,

        pub osm_id: Option<i64>,
        pub license: String,
        pub is_ghost: bool,
    }

    impl From<Stop> for stops::Stop {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        fn from(stop: Stop) -> Self {
            let Json(ally) = stop.a11y;
            stops::Stop {
                id: stop.id,
                osm_id: stop.osm_id,
                name: stop.name,
                short_name: stop.short_name,
                locality: stop.locality,
                street: stop.street,
                door: stop.door,
                parish: stop.parish,
                lat: stop.lat,
                lon: stop.lon,
                notes: stop.notes,
                tags: stop.tags,
                a11y: ally,
                verification_level: stop.verification_level as u8,
                service_check_date: stop.service_check_date,
                infrastructure_check_date: stop.infrastructure_check_date,
                is_ghost: stop.is_ghost,
                license: stop.license,
            }
        }
    }

    /// Meant to be an information-rich stop for the editor
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct FullStop {
        #[serde(flatten)]
        pub stop: Stop,
        pub updater: i32,
        pub operators: Vec<OperatorStopRel>,
        pub verified_position: bool,
        pub update_date: DateTime<Utc>,
    }

    #[derive(Serialize)]
    pub struct SpiderRoute {
        pub code: Option<String>,
        pub name: String,
        pub circular: bool,
    }

    #[derive(Serialize)]
    pub struct SpiderSubroute {
        pub route: i32,
        pub flag: String,
        pub stop_sequence: Vec<i32>,
    }

    #[derive(Serialize)]
    pub struct SpiderStop {
        pub name: String,
        pub lat: f64,
        pub lon: f64,
    }

    #[derive(Serialize)]
    pub struct SpiderMap {
        pub routes: HashMap<i32, SpiderRoute>,
        pub subroutes: HashMap<i32, SpiderSubroute>,
        pub stops: HashMap<i32, SpiderStop>,
    }

    #[derive(Debug, Clone, Serialize, PartialEq)]
    pub struct StopTodos {
        pub id: i32,
        pub name: String,
        pub lat: f64,
        pub lon: f64,
        pub todo: Json<Vec<stops::StopTodo>>,
    }
}
