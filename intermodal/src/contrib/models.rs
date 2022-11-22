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

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::stops::models::{IlluminationPos, IlluminationStrength, Stop};

#[derive(Serialize, Deserialize)]
pub struct Contribution {
    pub id: i64,
    pub author_id: i32,
    pub changeset: Changeset,
    pub submission_date: DateTime<Local>,
    pub accepted: Option<bool>,
    pub evaluator_id: Option<i32>,
    pub evaluation_date: Option<DateTime<Local>>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Changeset {
    StopDataChange { original: Stop, patch: StopPatch },
    StopPics(StopPictureContribution),
}

#[derive(Serialize, Deserialize, Default)]
pub struct StopPatch {
    pub locality: Option<Option<String>>,
    pub street: Option<Option<String>>,
    pub door: Option<Option<String>>,
    pub has_crossing: Option<Option<bool>>,
    pub has_accessibility: Option<Option<bool>>,
    pub has_abusive_parking: Option<Option<bool>>,
    pub has_outdated_info: Option<Option<bool>>,
    pub is_damaged: Option<Option<bool>>,
    pub is_vandalized: Option<Option<bool>>,
    pub has_flag: Option<Option<bool>>,
    pub has_schedules: Option<Option<bool>>,
    pub has_sidewalk: Option<Option<bool>>,
    pub has_shelter: Option<Option<bool>>,
    pub has_bench: Option<Option<bool>>,
    pub has_trash_can: Option<Option<bool>>,
    pub illumination_strength: Option<Option<IlluminationStrength>>,
    pub illumination_position: Option<Option<IlluminationPos>>,
    pub is_illumination_working: Option<Option<bool>>,
    pub has_illuminated_path: Option<Option<bool>>,
    pub has_visibility_from_within: Option<Option<bool>>,
    pub has_visibility_from_area: Option<Option<bool>>,
    pub is_visible_from_outside: Option<Option<bool>>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<Option<String>>,
}

impl StopPatch {
    pub(crate) fn is_empty(&self) -> bool {
        self.locality.is_none()
            && self.street.is_none()
            && self.door.is_none()
            && self.has_crossing.is_none()
            && self.has_accessibility.is_none()
            && self.has_abusive_parking.is_none()
            && self.has_outdated_info.is_none()
            && self.is_damaged.is_none()
            && self.is_vandalized.is_none()
            && self.has_flag.is_none()
            && self.has_schedules.is_none()
            && self.has_sidewalk.is_none()
            && self.has_shelter.is_none()
            && self.has_bench.is_none()
            && self.has_trash_can.is_none()
            && self.illumination_strength.is_none()
            && self.illumination_position.is_none()
            && self.is_illumination_working.is_none()
            && self.has_illuminated_path.is_none()
            && self.has_visibility_from_within.is_none()
            && self.has_visibility_from_area.is_none()
            && self.is_visible_from_outside.is_none()
            && self.tags.is_none()
            && self.notes.is_none()
    }

    pub(crate) fn apply(self, stop: &mut Stop) {
        if let Some(locality) = self.locality {
            stop.locality = locality
        }
        if let Some(street) = self.street {
            stop.street = street
        }
        if let Some(door) = self.door {
            stop.door = door
        }
        if let Some(has_crossing) = self.has_crossing {
            stop.accessibility_meta.has_crossing = has_crossing
        }
        if let Some(has_accessibility) = self.has_accessibility {
            stop.accessibility_meta.has_accessibility = has_accessibility
        }
        if let Some(has_abusive_parking) = self.has_abusive_parking {
            stop.accessibility_meta.has_abusive_parking = has_abusive_parking
        }
        if let Some(has_outdated_info) = self.has_outdated_info {
            stop.accessibility_meta.has_outdated_info = has_outdated_info
        }
        if let Some(is_damaged) = self.is_damaged {
            stop.accessibility_meta.is_damaged = is_damaged
        }
        if let Some(is_vandalized) = self.is_vandalized {
            stop.accessibility_meta.is_vandalized = is_vandalized
        }
        if let Some(has_flag) = self.has_flag {
            stop.accessibility_meta.has_flag = has_flag
        }
        if let Some(has_schedules) = self.has_schedules {
            stop.accessibility_meta.has_schedules = has_schedules
        }
        if let Some(has_sidewalk) = self.has_sidewalk {
            stop.accessibility_meta.has_sidewalk = has_sidewalk
        }
        if let Some(has_shelter) = self.has_shelter {
            stop.accessibility_meta.has_shelter = has_shelter
        }
        if let Some(has_bench) = self.has_bench {
            stop.accessibility_meta.has_bench = has_bench
        }
        if let Some(has_trash_can) = self.has_trash_can {
            stop.accessibility_meta.has_trash_can = has_trash_can
        }
        if let Some(illumination_strength) = self.illumination_strength {
            stop.accessibility_meta.illumination_strength =
                illumination_strength
        }
        if let Some(illumination_position) = self.illumination_position {
            stop.accessibility_meta.illumination_position =
                illumination_position
        }
        if let Some(is_illumination_working) = self.is_illumination_working {
            stop.accessibility_meta.is_illumination_working =
                is_illumination_working
        }
        if let Some(has_illuminated_path) = self.has_illuminated_path {
            stop.accessibility_meta.has_illuminated_path = has_illuminated_path
        }
        if let Some(has_visibility_from_within) =
            self.has_visibility_from_within
        {
            stop.accessibility_meta.has_visibility_from_within =
                has_visibility_from_within
        }
        if let Some(has_visibility_from_area) = self.has_visibility_from_area {
            stop.accessibility_meta.has_visibility_from_area =
                has_visibility_from_area
        }
        if let Some(is_visible_from_outside) = self.is_visible_from_outside {
            stop.accessibility_meta.is_visible_from_outside =
                is_visible_from_outside
        }
        if let Some(tags) = self.tags {
            stop.tags = tags
        }
        if let Some(notes) = self.notes {
            stop.notes = notes
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StopPictureContribution {
    pub pic_id: i32,
    pub stops: Vec<i32>,
}

pub(crate) mod requests {
    use super::{IlluminationPos, IlluminationStrength, Stop, StopPatch};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct NewStopMetaContribution {
        pub contribution: StopMetaContribution,
        pub comment: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StopMetaContribution {
        pub locality: Option<String>,
        pub street: Option<String>,
        pub door: Option<String>,
        pub has_crossing: Option<bool>,
        pub has_accessibility: Option<bool>,
        pub has_abusive_parking: Option<bool>,
        pub has_outdated_info: Option<bool>,
        pub is_damaged: Option<bool>,
        pub is_vandalized: Option<bool>,
        pub has_flag: Option<bool>,
        pub has_schedules: Option<bool>,
        pub has_sidewalk: Option<bool>,
        pub has_shelter: Option<bool>,
        pub has_bench: Option<bool>,
        pub has_trash_can: Option<bool>,
        pub illumination_strength: Option<IlluminationStrength>,
        pub illumination_position: Option<IlluminationPos>,
        pub is_illumination_working: Option<bool>,
        pub has_illuminated_path: Option<bool>,
        pub has_visibility_from_within: Option<bool>,
        pub has_visibility_from_area: Option<bool>,
        pub is_visible_from_outside: Option<bool>,
        pub tags: Vec<String>,
        pub notes: Option<String>,
        pub comment: Option<String>,
    }

    impl StopMetaContribution {
        pub(crate) fn derive_patch(self, stop: &Stop) -> StopPatch {
            let mut patch = StopPatch::default();

            if self.locality != stop.locality {
                patch.locality = Some(self.locality)
            }
            if self.street != stop.street {
                patch.street = Some(self.street)
            }
            if self.door != stop.door {
                patch.door = Some(self.door)
            }
            if self.has_crossing != stop.accessibility_meta.has_crossing {
                patch.has_crossing = Some(self.has_crossing)
            }
            if self.has_accessibility
                != stop.accessibility_meta.has_accessibility
            {
                patch.has_accessibility = Some(self.has_accessibility)
            }
            if self.has_abusive_parking
                != stop.accessibility_meta.has_abusive_parking
            {
                patch.has_abusive_parking = Some(self.has_abusive_parking)
            }
            if self.has_outdated_info
                != stop.accessibility_meta.has_outdated_info
            {
                patch.has_outdated_info = Some(self.has_outdated_info)
            }
            if self.is_damaged != stop.accessibility_meta.is_damaged {
                patch.is_damaged = Some(self.is_damaged)
            }
            if self.is_vandalized != stop.accessibility_meta.is_vandalized {
                patch.is_vandalized = Some(self.is_vandalized)
            }
            if self.has_flag != stop.accessibility_meta.has_flag {
                patch.has_flag = Some(self.has_flag)
            }
            if self.has_schedules != stop.accessibility_meta.has_schedules {
                patch.has_schedules = Some(self.has_schedules)
            }
            if self.has_sidewalk != stop.accessibility_meta.has_sidewalk {
                patch.has_sidewalk = Some(self.has_sidewalk)
            }
            if self.has_shelter != stop.accessibility_meta.has_shelter {
                patch.has_shelter = Some(self.has_shelter)
            }
            if self.has_bench != stop.accessibility_meta.has_bench {
                patch.has_bench = Some(self.has_bench)
            }
            if self.has_trash_can != stop.accessibility_meta.has_trash_can {
                patch.has_trash_can = Some(self.has_trash_can)
            }
            if self.illumination_strength
                != stop.accessibility_meta.illumination_strength
            {
                patch.illumination_strength = Some(self.illumination_strength)
            }
            if self.illumination_position
                != stop.accessibility_meta.illumination_position
            {
                patch.illumination_position = Some(self.illumination_position)
            }
            if self.is_illumination_working
                != stop.accessibility_meta.is_illumination_working
            {
                patch.is_illumination_working =
                    Some(self.is_illumination_working)
            }
            if self.has_illuminated_path
                != stop.accessibility_meta.has_illuminated_path
            {
                patch.has_illuminated_path = Some(self.has_illuminated_path)
            }
            if self.has_visibility_from_within
                != stop.accessibility_meta.has_visibility_from_within
            {
                patch.has_visibility_from_within =
                    Some(self.has_visibility_from_within)
            }
            if self.has_visibility_from_area
                != stop.accessibility_meta.has_visibility_from_area
            {
                patch.has_visibility_from_area =
                    Some(self.has_visibility_from_area)
            }
            if self.is_visible_from_outside
                != stop.accessibility_meta.is_visible_from_outside
            {
                patch.is_visible_from_outside =
                    Some(self.is_visible_from_outside)
            }
            if self.tags != stop.tags {
                patch.tags = Some(self.tags)
            }
            if self.notes != stop.notes {
                patch.notes = Some(self.notes)
            }

            patch
        }
    }
}
