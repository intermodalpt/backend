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

    use commons::models::{history, pics, stops};

    #[derive(Deserialize)]
    pub struct NewStopMetaContribution {
        pub contribution: StopMetaContribution,
        pub comment: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct StopMetaContribution {
        pub locality: Option<String>,
        pub street: Option<String>,
        pub door: Option<String>,
        #[serde(flatten)]
        pub a11y: stops::A11yMeta,
        pub tags: Vec<String>,
        pub notes: Option<String>,
        #[serde(default)]
        pub service_check_date: Option<NaiveDate>,
        #[serde(default)]
        pub infrastructure_check_date: Option<NaiveDate>,
        pub comment: Option<String>,
    }

    impl StopMetaContribution {
        pub(crate) fn derive_patch(
            self,
            stop: &stops::Stop,
        ) -> history::StopPatch {
            let mut patch = history::StopPatch::default();

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
                patch.tags = Some(self.tags);
            }
            if self.notes != stop.notes {
                patch.notes = Some(self.notes);
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

    #[derive(Deserialize)]
    pub struct NewPictureContribution {
        pub contribution: pics::StopPicDynMeta,
        pub stops: Vec<pics::StopAttrs>,
        pub comment: Option<String>,
    }
}

pub(crate) mod responses {
    use chrono::{DateTime, Local};
    use serde::Serialize;

    use commons::models::history;

    #[derive(Serialize)]
    pub struct Contribution {
        #[serde(flatten)]
        pub contribution: history::Contribution,
        pub author_username: String,
        pub evaluator_username: Option<String>,
    }

    #[derive(Serialize)]
    pub struct Changeset {
        pub id: i64,
        pub author_id: i32,
        pub author_username: String,
        pub changes: Vec<history::Change>,
        pub datetime: DateTime<Local>,
        pub contribution_id: Option<i64>,
    }

    #[derive(Debug, Serialize)]
    pub struct Contributor {
        pub id: i32,
        pub username: String,
        pub works_for: Option<i32>,
    }
}
