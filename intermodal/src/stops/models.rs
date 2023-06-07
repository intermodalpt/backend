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

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use utoipa::ToSchema;

#[repr(u8)]
#[derive(
    Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, sqlx::Type,
)]
pub enum IlluminationPos {
    Indirect = 0,
    Direct = 10,
    Own = 20,
}

#[repr(u8)]
#[derive(
    Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, sqlx::Type,
)]
pub enum IlluminationStrength {
    None = 0,
    Low = 1,
    Medium = 3,
    High = 5,
}

#[repr(u8)]
#[derive(
    Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, sqlx::Type,
)]
pub enum AdvertisementQuantification {
    None = 0,
    Few = 2,
    Many = 4,
    Intrusive = 6,
}

#[repr(u8)]
#[derive(
    Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, sqlx::Type,
)]
pub enum ParkingVisualLimitation {
    None = 0,
    Little = 2,
    Some = 4,
    Very = 6,
}

#[repr(u8)]
#[derive(
    Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, sqlx::Type,
)]
pub enum LocalParkingLimitation {
    None = 0,
    Low = 2,
    Medium = 4,
    High = 6,
}

#[repr(u8)]
#[derive(
    Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, sqlx::Type,
)]
pub enum AreaParkingLimitation {
    None = 0,
    Low = 2,
    Medium = 4,
    High = 6,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, sqlx::FromRow,
)]
pub struct Stop {
    pub id: i32,
    #[schema(example = "cmet")]
    pub source: String,
    #[schema(example = "Setúbal (ITS)")]
    pub name: Option<String>,
    #[schema(example = "Setúbal (ITS)")]
    pub official_name: Option<String>,
    #[schema(example = "Setúbal (ITS)")]
    pub osm_name: Option<String>,
    #[schema(example = "Setúbal")]
    pub short_name: Option<String>,
    #[schema(example = "Bairro das bairradas")]
    pub locality: Option<String>,
    #[schema(example = "Rua do Não Sei Decor")]
    pub street: Option<String>,
    #[schema(example = "123-A")]
    pub door: Option<String>,
    pub parish: Option<i32>,
    #[schema(example = 38.123_456)]
    pub lat: Option<f64>,
    #[schema(example = -9.654_321)]
    pub lon: Option<f64>,
    #[serde(default)]
    pub external_id: String,
    #[serde(default)]
    pub refs: Vec<String>,
    #[serde(default)]
    pub notes: Option<String>,
    pub updater: i32,
    pub update_date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub a11y: A11yMeta,
    // This is a bit flag made of 4 duets.
    // The four binary duets are for: Position, Service, Infra and [reserved]
    // 0 => Not verified; 1 => Wrong; 2 => Likely; 3 => Verified
    #[serde(default)]
    pub verification_level: u8,

    #[serde(default)]
    pub service_check_date: Option<NaiveDate>,
    #[serde(default)]
    pub infrastructure_check_date: Option<NaiveDate>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, ToSchema, Default, PartialEq,
)]
pub struct A11yMeta {
    #[serde(default)]
    pub schedules: Option<Vec<Schedule>>,
    #[serde(default)]
    pub flags: Option<Vec<Flag>>,

    // Amenities fields
    #[serde(default)]
    pub has_sidewalk: Option<bool>,
    #[serde(default)]
    pub has_sidewalked_path: Option<bool>,
    #[serde(default)]
    pub has_shelter: Option<bool>,
    #[serde(default)]
    pub has_cover: Option<bool>,
    #[serde(default)]
    pub has_bench: Option<bool>,
    #[serde(default)]
    pub has_trash_can: Option<bool>,
    #[serde(default)]
    pub has_waiting_times: Option<bool>,
    #[serde(default)]
    pub has_ticket_seller: Option<bool>,
    #[serde(default)]
    pub has_costumer_support: Option<bool>,
    #[serde(default)]
    pub advertisement_qty: Option<AdvertisementQuantification>,

    // Access fields
    #[serde(default)]
    pub has_crossing: Option<bool>,
    #[serde(default)]
    pub has_wide_access: Option<bool>,
    #[serde(default)]
    pub has_flat_access: Option<bool>,
    #[serde(default)]
    pub has_tactile_access: Option<bool>,

    // Visibility fields
    #[serde(default)]
    pub illumination_strength: Option<IlluminationStrength>,
    #[serde(default)]
    pub illumination_position: Option<IlluminationPos>,
    #[serde(default)]
    pub has_illuminated_path: Option<bool>,
    #[serde(default)]
    pub has_visibility_from_within: Option<bool>,
    #[serde(default)]
    pub has_visibility_from_area: Option<bool>,
    #[serde(default)]
    pub is_visible_from_outside: Option<bool>,

    // Parking fields
    #[serde(default)]
    pub parking_visibility_impairment: Option<ParkingVisualLimitation>,
    #[serde(default)]
    pub parking_local_access_impairment: Option<LocalParkingLimitation>,
    #[serde(default)]
    pub parking_area_access_impairment: Option<AreaParkingLimitation>,

    #[serde(default)]
    pub tmp_issues: Vec<String>,

    // FIXME Everything below is deprecated
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
    pub is_illumination_working: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct Flag {
    pub id: String,
    pub name: Option<String>,
    pub route_codes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleType {
    Origin,
    Prediction,
    Frequency,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct Schedule {
    pub code: Option<String>,
    pub discriminator: Option<String>,
    #[serde(rename = "type")]
    pub schedule_type: ScheduleType,
}

pub(crate) mod requests {
    use crate::contrib::models::StopPatch;
    use crate::stops::models::A11yMeta;
    use chrono::NaiveDate;
    use serde::Deserialize;
    use utoipa::ToSchema;

    use super::Stop;

    #[derive(Deserialize, ToSchema)]
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
        pub tags: Vec<String>,
        #[serde(default, flatten)]
        pub a11y: A11yMeta,
        pub verification_level: u8,
        #[serde(default)]
        pub service_check_date: Option<NaiveDate>,
        #[serde(default)]
        pub infrastructure_check_date: Option<NaiveDate>,
    }

    #[derive(Clone, Deserialize, ToSchema)]
    pub struct ChangeStop {
        // These two are not versioned
        pub lon: Option<f64>,
        pub lat: Option<f64>,

        // Neither this one, but we don't have a reason to change it
        // Consider dropping
        pub official_name: Option<String>,

        pub name: Option<String>,
        pub short_name: Option<String>,
        pub locality: Option<String>,
        pub street: Option<String>,
        pub door: Option<String>,
        #[serde(default)]
        pub notes: Option<String>,
        #[serde(default)]
        pub tags: Vec<String>,
        #[serde(flatten)]
        pub a11y: A11yMeta,
        pub verification_level: u8,
        #[serde(default)]
        pub service_check_date: Option<NaiveDate>,
        #[serde(default)]
        pub infrastructure_check_date: Option<NaiveDate>,
    }

    impl From<Stop> for ChangeStop {
        fn from(stop: Stop) -> Self {
            ChangeStop {
                lon: stop.lon,
                lat: stop.lat,
                name: stop.name,
                short_name: stop.short_name,
                official_name: stop.official_name,
                locality: stop.locality,
                street: stop.street,
                door: stop.door,
                notes: stop.notes,
                tags: stop.tags,
                a11y: stop.a11y,
                verification_level: stop.verification_level,
                service_check_date: stop.service_check_date,
                infrastructure_check_date: stop.infrastructure_check_date,
            }
        }
    }

    impl ChangeStop {
        pub fn derive_patch(&self, stop: &Stop) -> StopPatch {
            let mut patch = StopPatch::default();

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
                patch.schedules = Some(self.a11y.schedules.clone());
            }
            if self.a11y.flags != stop.a11y.flags {
                patch.flags = Some(self.a11y.flags.clone());
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
                patch.advertisement_qty = Some(self.a11y.advertisement_qty);
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
                    Some(self.a11y.illumination_strength);
            }
            if self.a11y.illumination_position
                != stop.a11y.illumination_position
            {
                patch.illumination_position =
                    Some(self.a11y.illumination_position);
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
                patch.parking_visibility_impairment =
                    Some(self.a11y.parking_visibility_impairment);
            }

            if self.a11y.parking_local_access_impairment
                != stop.a11y.parking_local_access_impairment
            {
                patch.parking_local_access_impairment =
                    Some(self.a11y.parking_local_access_impairment);
            }

            if self.a11y.parking_area_access_impairment
                != stop.a11y.parking_area_access_impairment
            {
                patch.parking_area_access_impairment =
                    Some(self.a11y.parking_area_access_impairment);
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

            patch
        }
    }
}

pub(crate) mod responses {
    use serde::Serialize;
    use std::collections::HashMap;
    use utoipa::ToSchema;

    #[derive(Serialize, ToSchema)]
    pub struct SpiderRoute {
        pub code: Option<String>,
        pub name: String,
        pub circular: bool,
    }

    #[derive(Serialize, ToSchema)]
    pub struct SpiderSubroute {
        pub route: i32,
        pub flag: String,
        pub stop_sequence: Vec<i32>,
    }

    #[derive(Serialize, ToSchema)]
    pub struct SpiderStop {
        pub name: Option<String>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
    }

    #[derive(Serialize, ToSchema)]
    pub struct SpiderMap {
        pub routes: HashMap<i32, SpiderRoute>,
        pub subroutes: HashMap<i32, SpiderSubroute>,
        pub stops: HashMap<i32, SpiderStop>,
    }
}

#[cfg(test)]
mod test {
    use super::{
        A11yMeta, AdvertisementQuantification, AreaParkingLimitation,
        IlluminationPos, IlluminationStrength, LocalParkingLimitation,
        ParkingVisualLimitation, ScheduleType, Stop,
    };
    use crate::stops::models::{Flag, Schedule};
    use chrono::NaiveDate;

    #[test]
    fn serialize_deserialize_a11y() {
        let a11y = A11yMeta {
            schedules: Some(vec![Schedule {
                code: Some("123".to_string()),
                discriminator: Some("321".to_string()),
                schedule_type: ScheduleType::Origin,
            }]),
            flags: Some(vec![Flag {
                id: "123".to_string(),
                name: Some("ABC".to_string()),
                route_codes: vec!["1234".to_string(), "4321".to_string()],
            }]),
            has_crossing: Some(false),
            has_wide_access: None,
            has_flat_access: Some(true),
            has_accessibility: Some(true),
            has_abusive_parking: Some(true),
            has_outdated_info: Some(true),
            is_damaged: Some(true),
            is_vandalized: Some(true),
            has_flag: Some(true),
            has_schedules: Some(true),
            has_sidewalk: Some(true),
            has_sidewalked_path: Some(true),
            has_shelter: Some(true),
            has_cover: Some(true),
            has_bench: Some(true),
            has_trash_can: Some(true),
            has_waiting_times: Some(true),
            has_ticket_seller: Some(true),
            has_costumer_support: Some(true),
            illumination_strength: Some(IlluminationStrength::High),
            illumination_position: Some(IlluminationPos::Own),
            is_illumination_working: Some(true),
            has_illuminated_path: Some(true),
            has_visibility_from_within: Some(true),
            has_visibility_from_area: Some(true),
            is_visible_from_outside: Some(true),
            parking_visibility_impairment: Some(
                ParkingVisualLimitation::Little,
            ),
            parking_local_access_impairment: Some(
                LocalParkingLimitation::Medium,
            ),
            parking_area_access_impairment: Some(AreaParkingLimitation::Medium),
            advertisement_qty: Some(AdvertisementQuantification::Many),
            has_tactile_access: Some(true),
            tmp_issues: vec!["foo".to_string(), "bar".to_string()],
        };
        let json = serde_json::to_string(&a11y).unwrap();

        let a11y2: A11yMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(a11y, a11y2);
    }

    #[test]
    fn deserialize_a11y() {
        let json = r#"{
            "schedules": [
                {
                    "code": "123",
                    "discriminator": "321",
                    "type": "origin"
                }
            ],
            "flags": [
                {
                    "id": "123",
                    "name": "ABC",
                    "route_codes": ["1234", "4321"]
                }
            ],
            "has_crossing": false,
            "has_sidewalk": true,
            "has_shelter": true,
            "has_bench": true,
            "has_trash_can": true,
            "illumination_strength": 5,
            "illumination_position": 20,
            "is_illumination_working": true,
            "has_illuminated_path": true,
            "has_visibility_from_within": true,
            "has_visibility_from_area": true,
            "is_visible_from_outside": true,
            "advertisement_qty": 4,
            "has_sidewalked_path": true,
            "has_cover": true,
            "has_waiting_times": true,
            "has_ticket_seller": true,
            "has_costumer_support": true,
            "has_wide_access": true,
            "has_flat_access": true,
            "has_tactile_access": true,
            "parking_visibility_impairment": 2,
            "parking_local_access_impairment": 4,
            "parking_area_access_impairment": 4
        }"#;
        let a11y: A11yMeta = serde_json::from_str(&json).unwrap();

        let a11y2 = A11yMeta {
            schedules: Some(vec![Schedule {
                code: Some("123".to_string()),
                discriminator: Some("321".to_string()),
                schedule_type: ScheduleType::Origin,
            }]),
            flags: Some(vec![Flag {
                id: "123".to_string(),
                name: Some("ABC".to_string()),
                route_codes: vec!["1234".to_string(), "4321".to_string()],
            }]),
            has_crossing: Some(false),
            has_wide_access: Some(true),
            has_flat_access: Some(true),
            has_sidewalk: Some(true),
            has_sidewalked_path: Some(true),
            has_shelter: Some(true),
            has_cover: Some(true),
            has_bench: Some(true),
            has_trash_can: Some(true),
            has_waiting_times: Some(true),
            has_ticket_seller: Some(true),
            has_costumer_support: Some(true),
            illumination_strength: Some(IlluminationStrength::High),
            illumination_position: Some(IlluminationPos::Own),
            is_illumination_working: Some(true),
            has_illuminated_path: Some(true),
            has_visibility_from_within: Some(true),
            has_visibility_from_area: Some(true),
            is_visible_from_outside: Some(true),
            parking_visibility_impairment: Some(
                ParkingVisualLimitation::Little,
            ),
            parking_local_access_impairment: Some(
                LocalParkingLimitation::Medium,
            ),
            parking_area_access_impairment: Some(AreaParkingLimitation::Medium),
            advertisement_qty: Some(AdvertisementQuantification::Many),
            has_tactile_access: Some(true),
            tmp_issues: vec![],
            // TODO Deprecated
            has_flag: None,
            has_schedules: None,
            has_accessibility: None,
            has_abusive_parking: None,
            has_outdated_info: None,
            is_damaged: None,
            is_vandalized: None,
        };

        assert_eq!(a11y, a11y2);
    }

    #[test]
    fn serialize_deserialize_stop() {
        let stop = Stop {
            id: 1,
            source: "".to_string(),
            name: Some("Test".to_string()),
            official_name: None,
            osm_name: None,
            short_name: None,
            locality: None,
            street: None,
            door: None,
            parish: None,
            lat: Some(1.0),
            lon: Some(2.0),
            external_id: "".to_string(),
            a11y: A11yMeta {
                schedules: Some(vec![Schedule {
                    code: Some("123".to_string()),
                    discriminator: Some("321".to_string()),
                    schedule_type: ScheduleType::Origin,
                }]),
                flags: Some(vec![Flag {
                    id: "123".to_string(),
                    name: Some("ABC".to_string()),
                    route_codes: vec!["1234".to_string(), "4321".to_string()],
                }]),
                has_crossing: Some(false),
                has_wide_access: None,
                has_flat_access: Some(true),
                has_accessibility: Some(true),
                has_abusive_parking: Some(true),
                has_outdated_info: Some(true),
                is_damaged: Some(true),
                is_vandalized: Some(true),
                has_flag: Some(true),
                has_schedules: Some(true),
                has_sidewalk: Some(true),
                has_sidewalked_path: Some(true),
                has_shelter: Some(true),
                has_cover: Some(true),
                has_bench: Some(true),
                has_trash_can: Some(true),
                has_waiting_times: Some(true),
                has_ticket_seller: Some(true),
                has_costumer_support: Some(true),
                illumination_strength: Some(IlluminationStrength::High),
                illumination_position: Some(IlluminationPos::Own),
                is_illumination_working: Some(true),
                has_illuminated_path: Some(true),
                has_visibility_from_within: Some(true),
                has_visibility_from_area: Some(true),
                is_visible_from_outside: Some(true),
                parking_visibility_impairment: Some(
                    ParkingVisualLimitation::Little,
                ),
                parking_local_access_impairment: Some(
                    LocalParkingLimitation::Medium,
                ),
                parking_area_access_impairment: Some(
                    AreaParkingLimitation::Medium,
                ),
                advertisement_qty: Some(AdvertisementQuantification::Many),
                has_tactile_access: Some(true),
                tmp_issues: vec!["foo".to_string(), "bar".to_string()],
            },
            verification_level: 4,
            service_check_date: Some(
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            ),
            infrastructure_check_date: Some(
                NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
            ),
            tags: vec!["test".to_string()],
            notes: Some("test".to_string()),
            updater: 0,
            update_date: "".to_string(),
            refs: vec!["aaaa".to_string()],
        };
        let json = serde_json::to_string(&stop).unwrap();

        let stop2: Stop = serde_json::from_str(&json).unwrap();
        assert_eq!(stop, stop2);
    }
}
