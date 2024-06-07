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
use std::collections::HashSet;

use crate::errors::Error;
use crate::models::stops as current;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum IlluminationPos {
    Indirect = 0,
    Direct = 10,
    Own = 20,
}

impl From<current::IlluminationPos> for IlluminationPos {
    fn from(pos: current::IlluminationPos) -> Self {
        match pos {
            current::IlluminationPos::Indirect => IlluminationPos::Indirect,
            current::IlluminationPos::Direct => IlluminationPos::Direct,
            current::IlluminationPos::Own => IlluminationPos::Own,
        }
    }
}

impl TryFrom<IlluminationPos> for current::IlluminationPos {
    type Error = Error;

    fn try_from(pos: IlluminationPos) -> Result<Self, Self::Error> {
        match pos {
            IlluminationPos::Indirect => Ok(current::IlluminationPos::Indirect),
            IlluminationPos::Direct => Ok(current::IlluminationPos::Direct),
            IlluminationPos::Own => Ok(current::IlluminationPos::Own),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum IlluminationStrength {
    None = 0,
    Low = 1,
    Medium = 3,
    High = 5,
}

impl From<current::IlluminationStrength> for IlluminationStrength {
    fn from(strength: current::IlluminationStrength) -> Self {
        match strength {
            current::IlluminationStrength::None => IlluminationStrength::None,
            current::IlluminationStrength::Low => IlluminationStrength::Low,
            current::IlluminationStrength::Medium => {
                IlluminationStrength::Medium
            }
            current::IlluminationStrength::High => IlluminationStrength::High,
        }
    }
}

impl TryFrom<IlluminationStrength> for current::IlluminationStrength {
    type Error = Error;

    fn try_from(strength: IlluminationStrength) -> Result<Self, Self::Error> {
        match strength {
            IlluminationStrength::None => {
                Ok(current::IlluminationStrength::None)
            }
            IlluminationStrength::Low => Ok(current::IlluminationStrength::Low),
            IlluminationStrength::Medium => {
                Ok(current::IlluminationStrength::Medium)
            }
            IlluminationStrength::High => {
                Ok(current::IlluminationStrength::High)
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum AdvertisementQuantification {
    None = 0,
    Few = 2,
    Many = 4,
    Intrusive = 6,
}

impl From<current::AdvertisementQuantification>
    for AdvertisementQuantification
{
    fn from(quantification: current::AdvertisementQuantification) -> Self {
        match quantification {
            current::AdvertisementQuantification::None => {
                AdvertisementQuantification::None
            }
            current::AdvertisementQuantification::Few => {
                AdvertisementQuantification::Few
            }
            current::AdvertisementQuantification::Many => {
                AdvertisementQuantification::Many
            }
            current::AdvertisementQuantification::Intrusive => {
                AdvertisementQuantification::Intrusive
            }
        }
    }
}

impl TryFrom<AdvertisementQuantification>
    for current::AdvertisementQuantification
{
    type Error = Error;

    fn try_from(
        quantification: AdvertisementQuantification,
    ) -> Result<Self, Self::Error> {
        match quantification {
            AdvertisementQuantification::None => {
                Ok(current::AdvertisementQuantification::None)
            }
            AdvertisementQuantification::Few => {
                Ok(current::AdvertisementQuantification::Few)
            }
            AdvertisementQuantification::Many => {
                Ok(current::AdvertisementQuantification::Many)
            }
            AdvertisementQuantification::Intrusive => {
                Ok(current::AdvertisementQuantification::Intrusive)
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum ParkingVisualLimitation {
    None = 0,
    Little = 2,
    Some = 4,
    Very = 6,
}

impl From<current::ParkingVisualLimitation> for ParkingVisualLimitation {
    fn from(limitation: current::ParkingVisualLimitation) -> Self {
        match limitation {
            current::ParkingVisualLimitation::None => {
                ParkingVisualLimitation::None
            }
            current::ParkingVisualLimitation::Little => {
                ParkingVisualLimitation::Little
            }
            current::ParkingVisualLimitation::Some => {
                ParkingVisualLimitation::Some
            }
            current::ParkingVisualLimitation::Very => {
                ParkingVisualLimitation::Very
            }
        }
    }
}

impl TryFrom<ParkingVisualLimitation> for current::ParkingVisualLimitation {
    type Error = Error;

    fn try_from(
        limitation: ParkingVisualLimitation,
    ) -> Result<Self, Self::Error> {
        match limitation {
            ParkingVisualLimitation::None => {
                Ok(current::ParkingVisualLimitation::None)
            }
            ParkingVisualLimitation::Little => {
                Ok(current::ParkingVisualLimitation::Little)
            }
            ParkingVisualLimitation::Some => {
                Ok(current::ParkingVisualLimitation::Some)
            }
            ParkingVisualLimitation::Very => {
                Ok(current::ParkingVisualLimitation::Very)
            }
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum LocalParkingLimitation {
    None = 0,
    Low = 2,
    Medium = 4,
    High = 6,
}

impl From<current::LocalParkingLimitation> for LocalParkingLimitation {
    fn from(limitation: current::LocalParkingLimitation) -> Self {
        match limitation {
            current::LocalParkingLimitation::None => {
                LocalParkingLimitation::None
            }
            current::LocalParkingLimitation::Low => LocalParkingLimitation::Low,
            current::LocalParkingLimitation::Medium => {
                LocalParkingLimitation::Medium
            }
            current::LocalParkingLimitation::High => {
                LocalParkingLimitation::High
            }
        }
    }
}

impl TryFrom<LocalParkingLimitation> for current::LocalParkingLimitation {
    type Error = Error;

    fn try_from(
        limitation: LocalParkingLimitation,
    ) -> Result<Self, Self::Error> {
        match limitation {
            LocalParkingLimitation::None => {
                Ok(current::LocalParkingLimitation::None)
            }
            LocalParkingLimitation::Low => {
                Ok(current::LocalParkingLimitation::Low)
            }
            LocalParkingLimitation::Medium => {
                Ok(current::LocalParkingLimitation::Medium)
            }
            LocalParkingLimitation::High => {
                Ok(current::LocalParkingLimitation::High)
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
pub enum AreaParkingLimitation {
    None = 0,
    Low = 2,
    Medium = 4,
    High = 6,
}

impl From<current::AreaParkingLimitation> for AreaParkingLimitation {
    fn from(limitation: current::AreaParkingLimitation) -> Self {
        match limitation {
            current::AreaParkingLimitation::None => AreaParkingLimitation::None,
            current::AreaParkingLimitation::Low => AreaParkingLimitation::Low,
            current::AreaParkingLimitation::Medium => {
                AreaParkingLimitation::Medium
            }
            current::AreaParkingLimitation::High => AreaParkingLimitation::High,
        }
    }
}

impl TryFrom<AreaParkingLimitation> for current::AreaParkingLimitation {
    type Error = Error;

    fn try_from(
        limitation: AreaParkingLimitation,
    ) -> Result<Self, Self::Error> {
        match limitation {
            AreaParkingLimitation::None => {
                Ok(current::AreaParkingLimitation::None)
            }
            AreaParkingLimitation::Low => {
                Ok(current::AreaParkingLimitation::Low)
            }
            AreaParkingLimitation::Medium => {
                Ok(current::AreaParkingLimitation::Medium)
            }
            AreaParkingLimitation::High => {
                Ok(current::AreaParkingLimitation::High)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stop {
    pub id: i32,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub locality: Option<String>,
    pub street: Option<String>,
    pub door: Option<String>,
    pub parish: Option<i32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub a11y: A11yMeta,
    #[serde(default)]
    pub verification_level: u8,

    pub service_check_date: Option<NaiveDate>,
    pub infrastructure_check_date: Option<NaiveDate>,

    #[serde(default)]
    pub osm_id: Option<i64>,
    pub license: Option<String>,
    pub is_ghost: Option<bool>,
}

impl From<current::Stop> for Stop {
    fn from(stop: current::Stop) -> Self {
        Stop {
            id: stop.id,
            name: Some(stop.name),
            short_name: stop.short_name,
            locality: stop.locality,
            street: stop.street,
            door: stop.door,
            parish: stop.parish,
            lat: Some(stop.lat),
            lon: Some(stop.lon),
            notes: stop.notes,
            tags: stop.tags,
            a11y: stop.a11y.into(),
            verification_level: stop.verification_level,
            service_check_date: stop.service_check_date,
            infrastructure_check_date: stop.infrastructure_check_date,

            osm_id: stop.osm_id,
            license: Some(stop.license),
            is_ghost: Some(stop.is_ghost),
        }
    }
}

impl TryFrom<Stop> for current::Stop {
    type Error = Error;

    fn try_from(stop: Stop) -> Result<Self, Self::Error> {
        Ok(current::Stop {
            id: stop.id,
            name: stop.name.ok_or(Error::Conversion)?,
            short_name: stop.short_name,
            locality: stop.locality,
            street: stop.street,
            door: stop.door,
            parish: stop.parish,
            lat: stop.lat.ok_or_else(|| Error::Patching {
                field: "lat",
                value: "None".to_string(),
            })?,
            lon: stop.lon.ok_or_else(|| Error::Patching {
                field: "lon",
                value: "None".to_string(),
            })?,
            notes: stop.notes,
            tags: stop.tags,
            a11y: stop.a11y.try_into()?,
            verification_level: stop.verification_level,
            service_check_date: stop.service_check_date,
            infrastructure_check_date: stop.infrastructure_check_date,

            osm_id: stop.osm_id,
            license: stop.license.unwrap_or("?".to_string()),
            is_ghost: stop.is_ghost.unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A11yMeta {
    pub schedules: Option<Vec<Schedule>>,
    pub flags: Option<Vec<Flag>>,

    // Amenities fields
    pub has_sidewalk: Option<bool>,
    pub has_sidewalked_path: Option<bool>,
    pub has_shelter: Option<bool>,
    pub has_cover: Option<bool>,
    pub has_bench: Option<bool>,
    pub has_trash_can: Option<bool>,
    pub has_waiting_times: Option<bool>,
    pub has_ticket_seller: Option<bool>,
    pub has_costumer_support: Option<bool>,
    pub advertisement_qty: Option<AdvertisementQuantification>,

    // Access fields
    pub has_crossing: Option<bool>,
    pub has_wide_access: Option<bool>,
    pub has_flat_access: Option<bool>,
    pub has_tactile_access: Option<bool>,

    // Visibility fields
    pub illumination_strength: Option<IlluminationStrength>,
    pub illumination_position: Option<IlluminationPos>,
    pub has_illuminated_path: Option<bool>,
    pub has_visibility_from_within: Option<bool>,
    pub has_visibility_from_area: Option<bool>,
    pub is_visible_from_outside: Option<bool>,

    // Parking fields
    pub parking_visibility_impairment: Option<ParkingVisualLimitation>,
    pub parking_local_access_impairment: Option<LocalParkingLimitation>,
    pub parking_area_access_impairment: Option<AreaParkingLimitation>,

    #[serde(default)]
    pub tmp_issues: Vec<String>,
}

impl From<current::A11yMeta> for A11yMeta {
    fn from(a11y: current::A11yMeta) -> Self {
        A11yMeta {
            schedules: a11y.schedules.map(|schedules| {
                schedules.into_iter().map(Into::into).collect::<Vec<_>>()
            }),
            flags: a11y.flags.map(|flags| {
                flags.into_iter().map(Into::into).collect::<Vec<_>>()
            }),
            has_sidewalk: a11y.has_sidewalk,
            has_sidewalked_path: a11y.has_sidewalked_path,
            has_shelter: a11y.has_shelter,
            has_cover: a11y.has_cover,
            has_bench: a11y.has_bench,
            has_trash_can: a11y.has_trash_can,
            has_waiting_times: a11y.has_waiting_times,
            has_ticket_seller: a11y.has_ticket_seller,
            has_costumer_support: a11y.has_costumer_support,
            advertisement_qty: a11y.advertisement_qty.map(Into::into),
            has_crossing: a11y.has_crossing,
            has_wide_access: a11y.has_wide_access,
            has_flat_access: a11y.has_flat_access,
            has_tactile_access: a11y.has_tactile_access,
            illumination_strength: a11y.illumination_strength.map(Into::into),
            illumination_position: a11y.illumination_position.map(Into::into),
            has_illuminated_path: a11y.has_illuminated_path,
            has_visibility_from_within: a11y.has_visibility_from_within,
            has_visibility_from_area: a11y.has_visibility_from_area,
            is_visible_from_outside: a11y.is_visible_from_outside,
            parking_visibility_impairment: a11y
                .parking_visibility_impairment
                .map(Into::into),
            parking_local_access_impairment: a11y
                .parking_local_access_impairment
                .map(Into::into),
            parking_area_access_impairment: a11y
                .parking_area_access_impairment
                .map(Into::into),
            tmp_issues: a11y.tmp_issues,
        }
    }
}

impl TryFrom<A11yMeta> for current::A11yMeta {
    type Error = Error;

    fn try_from(a11y: A11yMeta) -> Result<Self, Self::Error> {
        Ok(current::A11yMeta {
            schedules: super::opt_vec_try_into(a11y.schedules)?,
            flags: super::opt_vec_try_into(a11y.flags)?,
            has_sidewalk: a11y.has_sidewalk,
            has_sidewalked_path: a11y.has_sidewalked_path,
            has_shelter: a11y.has_shelter,
            has_cover: a11y.has_cover,
            has_bench: a11y.has_bench,
            has_trash_can: a11y.has_trash_can,
            has_waiting_times: a11y.has_waiting_times,
            has_ticket_seller: a11y.has_ticket_seller,
            has_costumer_support: a11y.has_costumer_support,
            advertisement_qty: a11y
                .advertisement_qty
                .map(TryInto::try_into)
                .transpose()?,
            has_crossing: a11y.has_crossing,
            has_wide_access: a11y.has_wide_access,
            has_flat_access: a11y.has_flat_access,
            has_tactile_access: a11y.has_tactile_access,
            illumination_strength: a11y
                .illumination_strength
                .map(TryInto::try_into)
                .transpose()?,
            illumination_position: a11y
                .illumination_position
                .map(TryInto::try_into)
                .transpose()?,
            has_illuminated_path: a11y.has_illuminated_path,
            has_visibility_from_within: a11y.has_visibility_from_within,
            has_visibility_from_area: a11y.has_visibility_from_area,
            is_visible_from_outside: a11y.is_visible_from_outside,
            parking_visibility_impairment: a11y
                .parking_visibility_impairment
                .map(TryInto::try_into)
                .transpose()?,
            parking_local_access_impairment: a11y
                .parking_local_access_impairment
                .map(TryInto::try_into)
                .transpose()?,
            parking_area_access_impairment: a11y
                .parking_area_access_impairment
                .map(TryInto::try_into)
                .transpose()?,
            tmp_issues: a11y.tmp_issues,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    pub id: String,
    pub name: Option<String>,
    pub route_codes: Vec<String>,
}

impl From<current::Flag> for Flag {
    fn from(flag: current::Flag) -> Self {
        Flag {
            id: flag.id,
            name: flag.name,
            route_codes: flag.route_codes,
        }
    }
}

impl TryFrom<Flag> for current::Flag {
    type Error = Error;

    fn try_from(flag: Flag) -> Result<Self, Self::Error> {
        Ok(current::Flag {
            id: flag.id,
            name: flag.name,
            route_codes: flag.route_codes,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleType {
    Origin,
    Prediction,
    Frequency,
}

impl From<current::ScheduleType> for ScheduleType {
    fn from(schedule_type: current::ScheduleType) -> Self {
        match schedule_type {
            current::ScheduleType::Origin => ScheduleType::Origin,
            current::ScheduleType::Prediction => ScheduleType::Prediction,
            current::ScheduleType::Frequency => ScheduleType::Frequency,
        }
    }
}

impl TryFrom<ScheduleType> for current::ScheduleType {
    type Error = Error;

    fn try_from(schedule_type: ScheduleType) -> Result<Self, Self::Error> {
        match schedule_type {
            ScheduleType::Origin => Ok(current::ScheduleType::Origin),
            ScheduleType::Prediction => Ok(current::ScheduleType::Prediction),
            ScheduleType::Frequency => Ok(current::ScheduleType::Frequency),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub code: Option<String>,
    pub discriminator: Option<String>,
    #[serde(rename = "type")]
    pub schedule_type: ScheduleType,
}

impl From<current::Schedule> for Schedule {
    fn from(schedule: current::Schedule) -> Self {
        Schedule {
            code: schedule.code,
            discriminator: schedule.discriminator,
            schedule_type: schedule.schedule_type.into(),
        }
    }
}

impl TryFrom<Schedule> for current::Schedule {
    type Error = Error;

    fn try_from(schedule: Schedule) -> Result<Self, Self::Error> {
        Ok(current::Schedule {
            code: schedule.code,
            discriminator: schedule.discriminator,
            schedule_type: schedule.schedule_type.try_into()?,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Verification {
    NotVerified = 0,
    Wrong = 1,
    Likely = 2,
    Verified = 3,
}

impl From<current::Verification> for Verification {
    fn from(verification: current::Verification) -> Self {
        match verification {
            current::Verification::NotVerified => Verification::NotVerified,
            current::Verification::Wrong => Verification::Wrong,
            current::Verification::Likely => Verification::Likely,
            current::Verification::Verified => Verification::Verified,
        }
    }
}

impl TryFrom<Verification> for current::Verification {
    type Error = Error;

    fn try_from(verification: Verification) -> Result<Self, Self::Error> {
        match verification {
            Verification::NotVerified => Ok(current::Verification::NotVerified),
            Verification::Wrong => Ok(current::Verification::Wrong),
            Verification::Likely => Ok(current::Verification::Likely),
            Verification::Verified => Ok(current::Verification::Verified),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct StopVerification {
    pub position: Verification,
    pub service: Verification,
    pub infrastructure: Verification,
}

#[allow(clippy::option_option)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StopPatch {
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub short_name: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub locality: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub street: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub door: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub schedules: Option<Option<Vec<Schedule>>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub flags: Option<Option<Vec<Flag>>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_sidewalk: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_sidewalked_path: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_shelter: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_cover: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_bench: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_trash_can: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_waiting_times: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_ticket_seller: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_costumer_support: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub advertisement_qty: Option<Option<AdvertisementQuantification>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_crossing: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_wide_access: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_flat_access: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_tactile_access: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub illumination_strength: Option<Option<IlluminationStrength>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub illumination_position: Option<Option<IlluminationPos>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_illuminated_path: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_visibility_from_within: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub has_visibility_from_area: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub is_visible_from_outside: Option<Option<bool>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub parking_visibility_impairment: Option<Option<ParkingVisualLimitation>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub parking_local_access_impairment: Option<Option<LocalParkingLimitation>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub parking_area_access_impairment: Option<Option<AreaParkingLimitation>>,

    pub tmp_issues: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub notes: Option<Option<String>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub service_check_date: Option<Option<NaiveDate>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub infrastructure_check_date: Option<Option<NaiveDate>>,
    #[serde(default)]
    pub verification_level: Option<u8>,

    pub license: Option<String>,
    pub is_ghost: Option<bool>,
}

impl StopPatch {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.short_name.is_none()
            && self.locality.is_none()
            && self.street.is_none()
            && self.door.is_none()
            && self.flags.is_none()
            && self.schedules.is_none()
            && self.has_sidewalk.is_none()
            && self.has_sidewalked_path.is_none()
            && self.has_shelter.is_none()
            && self.has_cover.is_none()
            && self.has_bench.is_none()
            && self.has_trash_can.is_none()
            && self.has_waiting_times.is_none()
            && self.has_ticket_seller.is_none()
            && self.has_costumer_support.is_none()
            && self.advertisement_qty.is_none()
            && self.has_crossing.is_none()
            && self.has_wide_access.is_none()
            && self.has_flat_access.is_none()
            && self.has_tactile_access.is_none()
            && self.illumination_strength.is_none()
            && self.illumination_position.is_none()
            && self.has_illuminated_path.is_none()
            && self.has_visibility_from_within.is_none()
            && self.has_visibility_from_area.is_none()
            && self.is_visible_from_outside.is_none()
            && self.parking_visibility_impairment.is_none()
            && self.parking_local_access_impairment.is_none()
            && self.parking_area_access_impairment.is_none()
            && self.tmp_issues.is_none()
            && self.tags.is_none()
            && self.notes.is_none()
            && self.service_check_date.is_none()
            && self.infrastructure_check_date.is_none()
            && self.verification_level.is_none()
            && self.license.is_none()
            && self.is_ghost.is_none()
    }

    #[allow(clippy::too_many_lines)]
    pub fn apply(&self, stop: &mut current::Stop) -> Result<(), Error> {
        if let Some(name) = self.name.clone() {
            stop.name = name;
        }
        if let Some(short_name) = self.short_name.clone() {
            stop.short_name = short_name;
        }
        if let Some(locality) = self.locality.clone() {
            stop.locality = locality;
        }
        if let Some(street) = self.street.clone() {
            stop.street = street;
        }
        if let Some(door) = self.door.clone() {
            stop.door = door;
        }
        if let Some(schedules) = self.schedules.clone() {
            stop.a11y.schedules = schedules
                .map(|schedules| {
                    schedules
                        .into_iter()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<_>, Error>>()
                })
                .transpose()?;
        }
        if let Some(flags) = self.flags.clone() {
            stop.a11y.flags = super::opt_vec_try_into(flags)?;
        }

        if let Some(has_sidewalk) = self.has_sidewalk {
            stop.a11y.has_sidewalk = has_sidewalk;
        }
        if let Some(has_sidewalked_path) = self.has_sidewalked_path {
            stop.a11y.has_sidewalked_path = has_sidewalked_path;
        }
        if let Some(has_shelter) = self.has_shelter {
            stop.a11y.has_shelter = has_shelter;
        }
        if let Some(has_cover) = self.has_cover {
            stop.a11y.has_cover = has_cover;
        }
        if let Some(has_bench) = self.has_bench {
            stop.a11y.has_bench = has_bench;
        }
        if let Some(has_trash_can) = self.has_trash_can {
            stop.a11y.has_trash_can = has_trash_can;
        }
        if let Some(has_waiting_times) = self.has_waiting_times {
            stop.a11y.has_waiting_times = has_waiting_times;
        }
        if let Some(has_ticket_seller) = self.has_ticket_seller {
            stop.a11y.has_ticket_seller = has_ticket_seller;
        }
        if let Some(has_costumer_support) = self.has_costumer_support {
            stop.a11y.has_costumer_support = has_costumer_support;
        }
        if let Some(advertisement_qty) = self.advertisement_qty {
            stop.a11y.advertisement_qty =
                advertisement_qty.map(TryInto::try_into).transpose()?;
        }
        if let Some(has_crossing) = self.has_crossing {
            stop.a11y.has_crossing = has_crossing;
        }
        if let Some(has_wide_access) = self.has_wide_access {
            stop.a11y.has_wide_access = has_wide_access;
        }
        if let Some(has_flat_access) = self.has_flat_access {
            stop.a11y.has_flat_access = has_flat_access;
        }
        if let Some(has_tactile_access) = self.has_tactile_access {
            stop.a11y.has_tactile_access = has_tactile_access;
        }
        if let Some(illumination_strength) = self.illumination_strength {
            stop.a11y.illumination_strength =
                illumination_strength.map(TryInto::try_into).transpose()?;
        }
        if let Some(illumination_position) = self.illumination_position {
            stop.a11y.illumination_position =
                illumination_position.map(TryInto::try_into).transpose()?;
        }
        if let Some(has_illuminated_path) = self.has_illuminated_path {
            stop.a11y.has_illuminated_path = has_illuminated_path;
        }
        if let Some(has_visibility_from_within) =
            self.has_visibility_from_within
        {
            stop.a11y.has_visibility_from_within = has_visibility_from_within;
        }
        if let Some(has_visibility_from_area) = self.has_visibility_from_area {
            stop.a11y.has_visibility_from_area = has_visibility_from_area;
        }
        if let Some(is_visible_from_outside) = self.is_visible_from_outside {
            stop.a11y.is_visible_from_outside = is_visible_from_outside;
        }
        if let Some(parking_visibility_impairment) =
            self.parking_visibility_impairment
        {
            stop.a11y.parking_visibility_impairment =
                parking_visibility_impairment
                    .map(TryInto::try_into)
                    .transpose()?;
        }
        if let Some(parking_local_access_impairment) =
            self.parking_local_access_impairment
        {
            stop.a11y.parking_local_access_impairment =
                parking_local_access_impairment
                    .map(TryInto::try_into)
                    .transpose()?;
        }
        if let Some(parking_area_access_impairment) =
            self.parking_area_access_impairment
        {
            stop.a11y.parking_area_access_impairment =
                parking_area_access_impairment
                    .map(TryInto::try_into)
                    .transpose()?;
        }
        if let Some(tmp_issues) = self.tmp_issues.clone() {
            stop.a11y.tmp_issues = tmp_issues;
        }
        if let Some(tags) = self.tags.clone() {
            stop.tags = tags;
        }
        if let Some(notes) = self.notes.clone() {
            stop.notes = notes;
        }
        if let Some(verification_level) = self.verification_level {
            stop.verification_level = verification_level;
        }
        if let Some(service_check_date) = self.service_check_date {
            stop.service_check_date = service_check_date;
        }
        if let Some(infrastructure_check_date) = self.infrastructure_check_date
        {
            stop.infrastructure_check_date = infrastructure_check_date;
        }
        if let Some(license) = self.license.clone() {
            stop.license = license;
        }
        if let Some(is_ghost) = self.is_ghost {
            stop.is_ghost = is_ghost;
        }
        Ok(())
    }
    #[allow(clippy::too_many_lines)]
    pub fn drop_noops(&mut self, stop: &current::Stop) -> Result<(), Error> {
        if let Some(name) = &self.name {
            if name == &stop.name {
                self.name = None;
            }
        }
        if let Some(short_name) = &self.short_name {
            if short_name == &stop.short_name {
                self.short_name = None;
            }
        }
        if let Some(locality) = &self.locality {
            if locality == &stop.locality {
                self.locality = None;
            }
        }
        if let Some(street) = &self.street {
            if street == &stop.street {
                self.street = None;
            }
        }
        if let Some(door) = &self.door {
            if door == &stop.door {
                self.door = None;
            }
        }
        if let Some(flags) = &self.flags {
            let flags_to_historical =
                &stop.a11y.flags.as_ref().map(|flags| {
                    flags.iter().cloned().map(Into::into).collect()
                });
            if flags == flags_to_historical {
                self.flags = None;
            }
        }
        if let Some(schedules) = &self.schedules {
            let schedules_to_historical =
                &stop.a11y.schedules.as_ref().map(|schedules| {
                    schedules.iter().cloned().map(Into::into).collect()
                });
            if schedules == schedules_to_historical {
                self.schedules = None;
            }
        }
        if let Some(has_sidewalk) = &self.has_sidewalk {
            if has_sidewalk == &stop.a11y.has_sidewalk {
                self.has_sidewalk = None;
            }
        }
        if let Some(has_sidewalked_path) = &self.has_sidewalked_path {
            if has_sidewalked_path == &stop.a11y.has_sidewalked_path {
                self.has_sidewalked_path = None;
            }
        }
        if let Some(has_shelter) = &self.has_shelter {
            if has_shelter == &stop.a11y.has_shelter {
                self.has_shelter = None;
            }
        }
        if let Some(has_cover) = &self.has_cover {
            if has_cover == &stop.a11y.has_cover {
                self.has_cover = None;
            }
        }
        if let Some(has_bench) = &self.has_bench {
            if has_bench == &stop.a11y.has_bench {
                self.has_bench = None;
            }
        }
        if let Some(has_trash_can) = &self.has_trash_can {
            if has_trash_can == &stop.a11y.has_trash_can {
                self.has_trash_can = None;
            }
        }
        if let Some(has_waiting_times) = &self.has_waiting_times {
            if has_waiting_times == &stop.a11y.has_waiting_times {
                self.has_waiting_times = None;
            }
        }
        if let Some(has_ticket_seller) = &self.has_ticket_seller {
            if has_ticket_seller == &stop.a11y.has_ticket_seller {
                self.has_ticket_seller = None;
            }
        }
        if let Some(has_costumer_support) = &self.has_costumer_support {
            if has_costumer_support == &stop.a11y.has_costumer_support {
                self.has_costumer_support = None;
            }
        }
        if let Some(advertisement_qty) = &self.advertisement_qty {
            if advertisement_qty.map(TryInto::try_into).transpose()?
                == stop.a11y.advertisement_qty
            {
                self.advertisement_qty = None;
            }
        }
        if let Some(has_crossing) = &self.has_crossing {
            if has_crossing == &stop.a11y.has_crossing {
                self.has_crossing = None;
            }
        }
        if let Some(has_wide_access) = &self.has_wide_access {
            if has_wide_access == &stop.a11y.has_wide_access {
                self.has_wide_access = None;
            }
        }
        if let Some(has_flat_access) = &self.has_flat_access {
            if has_flat_access == &stop.a11y.has_flat_access {
                self.has_flat_access = None;
            }
        }
        if let Some(has_tactile_access) = &self.has_tactile_access {
            if has_tactile_access == &stop.a11y.has_tactile_access {
                self.has_tactile_access = None;
            }
        }
        if let Some(illumination_strength) = &self.illumination_strength {
            if illumination_strength.map(TryInto::try_into).transpose()?
                == stop.a11y.illumination_strength
            {
                self.illumination_strength = None;
            }
        }
        if let Some(illumination_position) = &self.illumination_position {
            if illumination_position.map(TryInto::try_into).transpose()?
                == stop.a11y.illumination_position
            {
                self.illumination_position = None;
            }
        }
        if let Some(has_illuminated_path) = &self.has_illuminated_path {
            if has_illuminated_path == &stop.a11y.has_illuminated_path {
                self.has_illuminated_path = None;
            }
        }
        if let Some(has_visibility_from_within) =
            &self.has_visibility_from_within
        {
            if has_visibility_from_within
                == &stop.a11y.has_visibility_from_within
            {
                self.has_visibility_from_within = None;
            }
        }
        if let Some(has_visibility_from_area) = &self.has_visibility_from_area {
            if has_visibility_from_area == &stop.a11y.has_visibility_from_area {
                self.has_visibility_from_area = None;
            }
        }
        if let Some(is_visible_from_outside) = &self.is_visible_from_outside {
            if is_visible_from_outside == &stop.a11y.is_visible_from_outside {
                self.is_visible_from_outside = None;
            }
        }
        if let Some(parking_visibility_impairment) =
            &self.parking_visibility_impairment
        {
            if parking_visibility_impairment
                .map(TryInto::try_into)
                .transpose()?
                == stop.a11y.parking_visibility_impairment
            {
                self.parking_visibility_impairment = None;
            }
        }
        if let Some(parking_local_access_impairment) =
            &self.parking_local_access_impairment
        {
            if parking_local_access_impairment
                .map(TryInto::try_into)
                .transpose()?
                == stop.a11y.parking_local_access_impairment
            {
                self.parking_local_access_impairment = None;
            }
        }
        if let Some(parking_area_access_impairment) =
            &self.parking_area_access_impairment
        {
            if parking_area_access_impairment
                .map(TryInto::try_into)
                .transpose()?
                == stop.a11y.parking_area_access_impairment
            {
                self.parking_area_access_impairment = None;
            }
        }
        if let Some(tmp_issues) = &self.tmp_issues {
            if tmp_issues == &stop.a11y.tmp_issues {
                self.tmp_issues = None;
            }
        }
        if let Some(tags) = &self.tags {
            if tags == &stop.tags {
                self.tags = None;
            }
        }
        if let Some(notes) = &self.notes {
            if notes == &stop.notes {
                self.notes = None;
            }
        }
        if let Some(service_check_date) = &self.service_check_date {
            if service_check_date == &stop.service_check_date {
                self.service_check_date = None;
            }
        }
        if let Some(infrastructure_check_date) = &self.infrastructure_check_date
        {
            if infrastructure_check_date == &stop.infrastructure_check_date {
                self.infrastructure_check_date = None;
            }
        }
        if let Some(verification_level) = &self.verification_level {
            if verification_level == &stop.verification_level {
                self.verification_level = None;
            }
        }
        if let Some(license) = &self.license {
            if license == &stop.license {
                self.license = None;
            }
        }
        if let Some(is_ghost) = &self.is_ghost {
            if is_ghost == &stop.is_ghost {
                self.is_ghost = None;
            }
        }
        Ok(())
    }
    #[allow(clippy::too_many_lines)]
    pub fn drop_fields(&mut self, fields: &HashSet<&str>) {
        if fields.contains(&"name") {
            self.name = None;
        }
        if fields.contains(&"short_name") {
            self.short_name = None;
        }
        if fields.contains(&"locality") {
            self.locality = None;
        }
        if fields.contains(&"street") {
            self.street = None;
        }
        if fields.contains(&"door") {
            self.door = None;
        }
        if fields.contains(&"flags") {
            self.flags = None;
        }
        if fields.contains(&"schedules") {
            self.schedules = None;
        }
        if fields.contains(&"has_sidewalk") {
            self.has_sidewalk = None;
        }
        if fields.contains(&"has_sidewalked_path") {
            self.has_sidewalked_path = None;
        }
        if fields.contains(&"has_shelter") {
            self.has_shelter = None;
        }
        if fields.contains(&"has_cover") {
            self.has_cover = None;
        }
        if fields.contains(&"has_bench") {
            self.has_bench = None;
        }
        if fields.contains(&"has_trash_can") {
            self.has_trash_can = None;
        }
        if fields.contains(&"has_waiting_times") {
            self.has_waiting_times = None;
        }
        if fields.contains(&"has_ticket_seller") {
            self.has_ticket_seller = None;
        }
        if fields.contains(&"has_costumer_support") {
            self.has_costumer_support = None;
        }
        if fields.contains(&"advertisement_qty") {
            self.advertisement_qty = None;
        }
        if fields.contains(&"has_crossing") {
            self.has_crossing = None;
        }
        if fields.contains(&"has_wide_access") {
            self.has_wide_access = None;
        }
        if fields.contains(&"has_flat_access") {
            self.has_flat_access = None;
        }
        if fields.contains(&"has_tactile_access") {
            self.has_tactile_access = None;
        }
        if fields.contains(&"illumination_strength") {
            self.illumination_strength = None;
        }
        if fields.contains(&"illumination_position") {
            self.illumination_position = None;
        }
        if fields.contains(&"has_illuminated_path") {
            self.has_illuminated_path = None;
        }
        if fields.contains(&"has_visibility_from_within") {
            self.has_visibility_from_within = None;
        }
        if fields.contains(&"has_visibility_from_area") {
            self.has_visibility_from_area = None;
        }
        if fields.contains(&"is_visible_from_outside") {
            self.is_visible_from_outside = None;
        }
        if fields.contains(&"parking_visibility_impairment") {
            self.parking_visibility_impairment = None;
        }
        if fields.contains(&"parking_local_access_impairment") {
            self.parking_local_access_impairment = None;
        }
        if fields.contains(&"parking_area_access_impairment") {
            self.parking_area_access_impairment = None;
        }
        if fields.contains(&"tmp_issues") {
            self.tmp_issues = None;
        }
        if fields.contains(&"tags") {
            self.tags = None;
        }
        if fields.contains(&"notes") {
            self.notes = None;
        }
        if fields.contains(&"service_check_date") {
            self.service_check_date = None;
        }
        if fields.contains(&"infrastructure_check_date") {
            self.infrastructure_check_date = None;
        }
        if fields.contains(&"verification_level") {
            self.verification_level = None;
        }
        if fields.contains(&"license") {
            self.license = None;
        }
        if fields.contains(&"is_ghost") {
            self.is_ghost = None;
        }
    }

    pub fn deverify(
        &mut self,
        original_verification: current::StopVerification,
    ) {
        let deverify_service = self.flags.is_some() || self.schedules.is_some();
        let deverify_infra = self.has_sidewalk.is_some()
            || self.has_sidewalked_path.is_some()
            || self.has_shelter.is_some()
            || self.has_cover.is_some()
            || self.has_bench.is_some()
            || self.has_trash_can.is_some()
            || self.has_waiting_times.is_some()
            || self.has_ticket_seller.is_some()
            || self.has_costumer_support.is_some()
            || self.advertisement_qty.is_some()
            || self.has_crossing.is_some()
            || self.has_wide_access.is_some()
            || self.has_flat_access.is_some()
            || self.has_tactile_access.is_some()
            || self.illumination_strength.is_some()
            || self.illumination_position.is_some()
            || self.has_illuminated_path.is_some()
            || self.has_visibility_from_within.is_some()
            || self.has_visibility_from_area.is_some()
            || self.is_visible_from_outside.is_some()
            || self.parking_visibility_impairment.is_some()
            || self.parking_local_access_impairment.is_some()
            || self.parking_area_access_impairment.is_some()
            || self.is_ghost.is_some();

        let mut new_verification = original_verification;

        if deverify_service {
            new_verification.service = current::Verification::NotVerified;
        }
        if deverify_infra {
            new_verification.infrastructure =
                current::Verification::NotVerified;
        }

        // We allow the patch to deverify by itself
        if let Some(patch_verification) = self.verification_level {
            let patch_verification =
                current::StopVerification::from(patch_verification);

            if patch_verification.service == current::Verification::NotVerified
            {
                new_verification.service = current::Verification::NotVerified;
            };

            if patch_verification.infrastructure
                == current::Verification::NotVerified
            {
                new_verification.infrastructure =
                    current::Verification::NotVerified;
            };

            if patch_verification.position == current::Verification::NotVerified
            {
                new_verification.position = current::Verification::NotVerified;
            };
        }

        self.verification_level = if new_verification == original_verification {
            None
        } else {
            Some(new_verification.into())
        }
    }
}
