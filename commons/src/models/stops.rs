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

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use utoipa::ToSchema;

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum IlluminationPos {
    Indirect = 0,
    Direct = 10,
    Own = 20,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum IlluminationStrength {
    None = 0,
    Low = 1,
    Medium = 3,
    High = 5,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum AdvertisementQuantification {
    None = 0,
    Few = 2,
    Many = 4,
    Intrusive = 6,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum ParkingVisualLimitation {
    None = 0,
    Little = 2,
    Some = 4,
    Very = 6,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum LocalParkingLimitation {
    None = 0,
    Low = 2,
    Medium = 4,
    High = 6,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum AreaParkingLimitation {
    None = 0,
    Low = 2,
    Medium = 4,
    High = 6,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct Stop {
    pub id: i32,
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
    pub notes: Option<String>,
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

impl Stop {
    pub fn verification(&self) -> StopVerification {
        StopVerification::from(self.verification_level)
    }
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

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq, Hash,
)]
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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Verification {
    NotVerified = 0,
    Wrong = 1,
    Likely = 2,
    Verified = 3,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct StopVerification {
    pub position: Verification,
    pub service: Verification,
    pub infrastructure: Verification,
}

impl StopVerification {
    pub fn is_fully_verified(&self) -> bool {
        self.position == Verification::Verified
            && self.service == Verification::Verified
            && self.infrastructure == Verification::Verified
    }

    pub fn verified() -> Self {
        StopVerification {
            position: Verification::Verified,
            service: Verification::Verified,
            infrastructure: Verification::Verified,
        }
    }

    pub fn unverified() -> Self {
        StopVerification {
            position: Verification::NotVerified,
            service: Verification::NotVerified,
            infrastructure: Verification::NotVerified,
        }
    }
}

impl From<u8> for StopVerification {
    fn from(value: u8) -> Self {
        StopVerification {
            position: match value & 0b11 {
                0 => Verification::NotVerified,
                1 => Verification::Wrong,
                2 => Verification::Likely,
                3 => Verification::Verified,
                _ => unreachable!(),
            },
            service: match (value >> 2) & 0b11 {
                0 => Verification::NotVerified,
                1 => Verification::Wrong,
                2 => Verification::Likely,
                3 => Verification::Verified,
                _ => unreachable!(),
            },
            infrastructure: match (value >> 4) & 0b11 {
                0 => Verification::NotVerified,
                1 => Verification::Wrong,
                2 => Verification::Likely,
                3 => Verification::Verified,
                _ => unreachable!(),
            },
        }
    }
}

impl From<StopVerification> for u8 {
    fn from(value: StopVerification) -> Self {
        let mut result = 0;
        result |= match value.position {
            Verification::NotVerified => 0,
            Verification::Wrong => 1,
            Verification::Likely => 2,
            Verification::Verified => 3,
        };
        result |= match value.service {
            Verification::NotVerified => 0,
            Verification::Wrong => 1,
            Verification::Likely => 2,
            Verification::Verified => 3,
        } << 2;
        result |= match value.infrastructure {
            Verification::NotVerified => 0,
            Verification::Wrong => 1,
            Verification::Likely => 2,
            Verification::Verified => 3,
        } << 4;
        result
    }
}

#[cfg(test)]
mod test {
    use super::{
        A11yMeta, AdvertisementQuantification, AreaParkingLimitation,
        IlluminationPos, IlluminationStrength, LocalParkingLimitation,
        ParkingVisualLimitation, ScheduleType, Stop,
    };
    use super::{Flag, Schedule};
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
