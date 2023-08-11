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

use std::collections::HashSet;

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use super::operators;
use super::pics;
use super::routes;
use super::stops;

#[derive(Serialize, Deserialize)]
pub struct Contribution {
    pub id: i64,
    pub author_id: i32,
    pub change: Change,
    pub submission_date: DateTime<Local>,
    pub accepted: Option<bool>,
    pub evaluator_id: Option<i32>,
    pub evaluation_date: Option<DateTime<Local>>,
    pub comment: Option<String>,
}

#[derive(Serialize)]
pub struct Changeset {
    pub id: i64,
    pub author_id: i32,
    pub changes: Vec<Change>,
    pub datetime: DateTime<Local>,
    pub contribution_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Change {
    // Hint: Do not to change these names without a corresponding migration
    // as they'll be stored as strings in the database
    StopCreation {
        data: stops::Stop,
    },
    StopUpdate {
        original: stops::Stop,
        patch: StopPatch,
    },
    StopDeletion {
        data: stops::Stop,
    },
    RouteCreation {
        data: routes::Route,
    },
    RouteUpdate {
        original: routes::Route,
        patch: RoutePatch,
    },
    RouteDeletion {
        data: routes::Route,
    },
    SubrouteCreation {
        data: routes::Subroute,
    },
    SubrouteUpdate {
        original: routes::Subroute,
        patch: SubroutePatch,
    },
    SubrouteDeletion {
        data: routes::Subroute,
    },
    DepartureCreation {
        data: routes::Departure,
    },
    DepartureUpdate {
        original: routes::Departure,
        patch: DeparturePatch,
    },
    DepartureDeletion {
        data: routes::Departure,
    },
    StopPicUpload {
        pic: pics::StopPic,
        stops: Vec<i32>,
    },
    StopPicMetaUpdate {
        original_meta: pics::StopPicDynMeta,
        original_stops: Vec<i32>,
        meta_patch: StopPicturePatch,
        stops: Vec<i32>,
    },
    StopPicDeletion {
        pic: pics::StopPic,
        stops: Vec<i32>,
    },
    IssueCreation {
        data: operators::Issue,
    },
    IssueUpdate {
        original: operators::Issue,
        patch: IssuePatch,
    },
}

#[derive(Serialize, Deserialize, Default)]
pub struct StopPatch {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub name: Option<Option<String>>,
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
    pub schedules: Option<Option<Vec<stops::Schedule>>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub flags: Option<Option<Vec<stops::Flag>>>,

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
    pub advertisement_qty: Option<Option<stops::AdvertisementQuantification>>,

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
    pub illumination_strength: Option<Option<stops::IlluminationStrength>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub illumination_position: Option<Option<stops::IlluminationPos>>,
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
    pub parking_visibility_impairment:
        Option<Option<stops::ParkingVisualLimitation>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub parking_local_access_impairment:
        Option<Option<stops::LocalParkingLimitation>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub parking_area_access_impairment:
        Option<Option<stops::AreaParkingLimitation>>,

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
    pub verification_level: Option<u8>,

    // FIXME Deprecated
    pub has_schedules: Option<Option<bool>>,
    pub has_accessibility: Option<Option<bool>>,
    pub has_abusive_parking: Option<Option<bool>>,
    pub has_outdated_info: Option<Option<bool>>,
    pub is_damaged: Option<Option<bool>>,
    pub is_vandalized: Option<Option<bool>>,
    pub has_flag: Option<Option<bool>>,
    pub is_illumination_working: Option<Option<bool>>,
}

impl StopPatch {
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
            // TODO deprecate
            && self.has_flag.is_none()
            && self.has_schedules.is_none()
            && self.has_accessibility.is_none()
            && self.has_abusive_parking.is_none()
            && self.has_outdated_info.is_none()
            && self.is_damaged.is_none()
            && self.is_vandalized.is_none()
            && self.is_illumination_working.is_none()
    }

    pub fn apply(&self, stop: &mut stops::Stop) {
        if let Some(name) = self.name.clone() {
            stop.name = name
        }
        if let Some(short_name) = self.short_name.clone() {
            stop.short_name = short_name
        }
        if let Some(locality) = self.locality.clone() {
            stop.locality = locality
        }
        if let Some(street) = self.street.clone() {
            stop.street = street
        }
        if let Some(door) = self.door.clone() {
            stop.door = door
        }
        if let Some(schedules) = self.schedules.clone() {
            stop.a11y.schedules = schedules
        }
        if let Some(flags) = self.flags.clone() {
            stop.a11y.flags = flags
        }

        if let Some(has_sidewalk) = self.has_sidewalk {
            stop.a11y.has_sidewalk = has_sidewalk
        }
        if let Some(has_sidewalked_path) = self.has_sidewalked_path {
            stop.a11y.has_sidewalked_path = has_sidewalked_path
        }
        if let Some(has_shelter) = self.has_shelter {
            stop.a11y.has_shelter = has_shelter
        }
        if let Some(has_cover) = self.has_cover {
            stop.a11y.has_cover = has_cover
        }
        if let Some(has_bench) = self.has_bench {
            stop.a11y.has_bench = has_bench
        }
        if let Some(has_trash_can) = self.has_trash_can {
            stop.a11y.has_trash_can = has_trash_can
        }
        if let Some(has_waiting_times) = self.has_waiting_times {
            stop.a11y.has_waiting_times = has_waiting_times
        }
        if let Some(has_ticket_seller) = self.has_ticket_seller {
            stop.a11y.has_ticket_seller = has_ticket_seller
        }
        if let Some(has_costumer_support) = self.has_costumer_support {
            stop.a11y.has_costumer_support = has_costumer_support
        }
        if let Some(advertisement_qty) = self.advertisement_qty {
            stop.a11y.advertisement_qty = advertisement_qty
        }
        if let Some(has_crossing) = self.has_crossing {
            stop.a11y.has_crossing = has_crossing
        }
        if let Some(has_wide_access) = self.has_wide_access {
            stop.a11y.has_wide_access = has_wide_access
        }
        if let Some(has_flat_access) = self.has_flat_access {
            stop.a11y.has_flat_access = has_flat_access
        }
        if let Some(has_tactile_access) = self.has_tactile_access {
            stop.a11y.has_tactile_access = has_tactile_access
        }
        if let Some(illumination_strength) = self.illumination_strength {
            stop.a11y.illumination_strength = illumination_strength
        }
        if let Some(illumination_position) = self.illumination_position {
            stop.a11y.illumination_position = illumination_position
        }
        if let Some(is_illumination_working) = self.is_illumination_working {
            stop.a11y.is_illumination_working = is_illumination_working
        }
        if let Some(has_illuminated_path) = self.has_illuminated_path {
            stop.a11y.has_illuminated_path = has_illuminated_path
        }
        if let Some(has_visibility_from_within) =
            self.has_visibility_from_within
        {
            stop.a11y.has_visibility_from_within = has_visibility_from_within
        }
        if let Some(has_visibility_from_area) = self.has_visibility_from_area {
            stop.a11y.has_visibility_from_area = has_visibility_from_area
        }
        if let Some(is_visible_from_outside) = self.is_visible_from_outside {
            stop.a11y.is_visible_from_outside = is_visible_from_outside
        }
        if let Some(parking_visibility_impairment) =
            self.parking_visibility_impairment
        {
            stop.a11y.parking_visibility_impairment =
                parking_visibility_impairment
        }
        if let Some(parking_local_access_impairment) =
            self.parking_local_access_impairment
        {
            stop.a11y.parking_local_access_impairment =
                parking_local_access_impairment
        }
        if let Some(parking_area_access_impairment) =
            self.parking_area_access_impairment
        {
            stop.a11y.parking_area_access_impairment =
                parking_area_access_impairment
        }
        if let Some(tmp_issues) = self.tmp_issues.clone() {
            stop.a11y.tmp_issues = tmp_issues
        }
        if let Some(tags) = self.tags.clone() {
            stop.tags = tags
        }
        if let Some(notes) = self.notes.clone() {
            stop.notes = notes
        }
        if let Some(verification_level) = self.verification_level {
            stop.verification_level = verification_level
        }
        if let Some(service_check_date) = self.service_check_date {
            stop.service_check_date = service_check_date
        }
        if let Some(infrastructure_check_date) = self.infrastructure_check_date
        {
            stop.infrastructure_check_date = infrastructure_check_date
        }

        // FIXME deprecated
        if let Some(has_accessibility) = self.has_accessibility {
            stop.a11y.has_accessibility = has_accessibility
        }
        if let Some(has_abusive_parking) = self.has_abusive_parking {
            stop.a11y.has_abusive_parking = has_abusive_parking
        }
        if let Some(has_outdated_info) = self.has_outdated_info {
            stop.a11y.has_outdated_info = has_outdated_info
        }
        if let Some(is_damaged) = self.is_damaged {
            stop.a11y.is_damaged = is_damaged
        }
        if let Some(is_vandalized) = self.is_vandalized {
            stop.a11y.is_vandalized = is_vandalized
        }
        if let Some(has_flag) = self.has_flag {
            stop.a11y.has_flag = has_flag
        }
        if let Some(has_schedules) = self.has_schedules {
            stop.a11y.has_schedules = has_schedules
        }
    }

    pub fn drop_noops(&mut self, stop: &stops::Stop) {
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
            if flags == &stop.a11y.flags {
                self.flags = None;
            }
        }
        if let Some(schedules) = &self.schedules {
            if schedules == &stop.a11y.schedules {
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
            if advertisement_qty == &stop.a11y.advertisement_qty {
                self.advertisement_qty = None;
            }
        }
        if let Some(advertisement_qty) = &self.advertisement_qty {
            if advertisement_qty == &stop.a11y.advertisement_qty {
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
            if illumination_strength == &stop.a11y.illumination_strength {
                self.illumination_strength = None;
            }
        }
        if let Some(illumination_position) = &self.illumination_position {
            if illumination_position == &stop.a11y.illumination_position {
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
                == &stop.a11y.parking_visibility_impairment
            {
                self.parking_visibility_impairment = None;
            }
        }
        if let Some(parking_local_access_impairment) =
            &self.parking_local_access_impairment
        {
            if parking_local_access_impairment
                == &stop.a11y.parking_local_access_impairment
            {
                self.parking_local_access_impairment = None;
            }
        }
        if let Some(parking_area_access_impairment) =
            &self.parking_area_access_impairment
        {
            if parking_area_access_impairment
                == &stop.a11y.parking_area_access_impairment
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
    }

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
    }

    pub fn deverify(&mut self, original_verification: stops::StopVerification) {
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
            || self.parking_area_access_impairment.is_some();

        let mut new_verification = original_verification;

        if deverify_service {
            new_verification.service = stops::Verification::NotVerified;
        }
        if deverify_infra {
            new_verification.infrastructure = stops::Verification::NotVerified;
        }

        // We allow the patch to deverify by itself
        if let Some(patch_verification) = self.verification_level {
            let patch_verification =
                stops::StopVerification::from(patch_verification);

            if patch_verification.service == stops::Verification::NotVerified {
                new_verification.service = stops::Verification::NotVerified
            };

            if patch_verification.infrastructure
                == stops::Verification::NotVerified
            {
                new_verification.infrastructure =
                    stops::Verification::NotVerified
            };

            if patch_verification.position == stops::Verification::NotVerified {
                new_verification.position = stops::Verification::NotVerified
            };
        }

        self.verification_level = if new_verification == original_verification {
            None
        } else {
            Some(new_verification.into())
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
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
    pub fn apply(self, route: &mut routes::Route) {
        if let Some(type_id) = self.type_id {
            route.type_id = type_id
        }
        if let Some(operator) = self.operator_id {
            route.operator_id = operator
        }
        if let Some(code) = self.code {
            route.code = code
        }
        if let Some(name) = self.name {
            route.name = name
        }
        if let Some(main_subroute) = self.main_subroute {
            route.main_subroute = main_subroute
        }
        if let Some(active) = self.active {
            route.active = active
        }
        if let Some(circular) = self.circular {
            route.circular = circular
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SubroutePatch {
    pub flag: Option<String>,
    pub circular: Option<bool>,
    pub polyline: Option<String>,
}

impl SubroutePatch {
    pub fn is_empty(&self) -> bool {
        self.flag.is_none()
            && self.circular.is_none()
            && self.polyline.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, subroute: &mut routes::Subroute) {
        if let Some(flag) = self.flag {
            subroute.flag = flag
        }
        if let Some(circular) = self.circular {
            subroute.circular = circular
        }
        if let Some(polyline) = self.polyline {
            subroute.polyline = Some(polyline)
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct DeparturePatch {
    pub time: Option<i16>,
    pub subroute_id: Option<i32>,
    pub calendar_id: Option<i32>,
}

impl DeparturePatch {
    pub fn is_empty(&self) -> bool {
        self.time.is_none()
            && self.subroute_id.is_none()
            && self.calendar_id.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, departure: &mut routes::Departure) {
        if let Some(time) = self.time {
            departure.time = time
        }
        if let Some(subroute_id) = self.subroute_id {
            departure.subroute_id = subroute_id
        }
        if let Some(calendar_id) = self.calendar_id {
            departure.calendar_id = calendar_id
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct StopPicturePatch {
    pub public: Option<bool>,
    pub sensitive: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub lon: Option<Option<f64>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub lat: Option<Option<f64>>,
    pub quality: Option<i16>,
    pub tags: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub notes: Option<Option<String>>,
}

impl StopPicturePatch {
    pub fn is_empty(&self) -> bool {
        self.public.is_none()
            && self.sensitive.is_none()
            && self.lon.is_none()
            && self.lat.is_none()
            && self.quality.is_none()
            && self.tags.is_none()
            && self.notes.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, pic: &mut pics::StopPic) {
        if let Some(public) = self.public {
            pic.dyn_meta.public = public
        }
        if let Some(sensitive) = self.sensitive {
            pic.dyn_meta.sensitive = sensitive
        }
        if let Some(lon) = self.lon {
            pic.dyn_meta.lon = lon
        }
        if let Some(lat) = self.lat {
            pic.dyn_meta.lat = lat
        }
        if let Some(quality) = self.quality {
            pic.dyn_meta.quality = quality
        }
        if let Some(tags) = self.tags {
            pic.dyn_meta.tags = tags
        }
        if let Some(notes) = self.notes {
            pic.dyn_meta.notes = notes
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct IssuePatch {
    pub title: Option<String>,
    pub message: Option<String>,
    pub creation: Option<DateTime<Local>>,
    pub category: Option<operators::IssueCategory>,
    pub impact: Option<i32>,
    pub state: Option<operators::IssueState>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub state_justification: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub lat: Option<Option<f64>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub lon: Option<Option<f64>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub geojson: Option<Option<serde_json::Value>>,
    pub operator_ids: Option<Vec<i32>>,
    pub route_ids: Option<Vec<i32>>,
    pub stop_ids: Option<Vec<i32>>,
    pub pic_ids: Option<Vec<i32>>,
}

impl IssuePatch {
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.message.is_none()
            && self.creation.is_none()
            && self.impact.is_none()
            && self.category.is_none()
            && self.state.is_none()
            && self.state_justification.is_none()
            && self.lat.is_none()
            && self.lon.is_none()
            && self.geojson.is_none()
            && self.operator_ids.is_none()
            && self.route_ids.is_none()
            && self.stop_ids.is_none()
            && self.pic_ids.is_none()
    }

    #[allow(unused)]
    pub fn apply(self, issue: &mut operators::Issue) {
        if let Some(title) = self.title {
            issue.title = title
        }
        if let Some(message) = self.message {
            issue.message = message
        }
        if let Some(creation) = self.creation {
            issue.creation = creation
        }
        if let Some(category) = self.category {
            issue.category = category
        }
        if let Some(impact) = self.impact {
            issue.impact = impact
        }
        if let Some(state) = self.state {
            issue.state = state
        }
        if let Some(state_justification) = self.state_justification {
            issue.state_justification = state_justification
        }
        if let Some(lat) = self.lat {
            issue.lat = lat
        }
        if let Some(lon) = self.lon {
            issue.lon = lon
        }
        if let Some(geojson) = self.geojson {
            issue.geojson = geojson
        }
        if let Some(operator_ids) = self.operator_ids {
            issue.operator_ids = operator_ids
        }
        if let Some(route_ids) = self.route_ids {
            issue.route_ids = route_ids
        }
        if let Some(stop_ids) = self.stop_ids {
            issue.stop_ids = stop_ids
        }
        if let Some(pic_ids) = self.pic_ids {
            issue.pic_ids = pic_ids
        }
    }
}