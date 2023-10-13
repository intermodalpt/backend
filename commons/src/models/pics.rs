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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
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
    pub stops: Vec<i32>,
    // TODO Consider this
    pub tagged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StopPic {
    pub id: i32,
    pub original_filename: String,
    pub sha1: String,
    pub tagged: bool,
    pub uploader: i32,
    pub upload_date: String,
    pub capture_date: Option<String>,
    pub updater: Option<i32>,
    pub update_date: Option<String>,
    pub width: i32,
    pub height: i32,
    pub camera_ref: Option<String>,
    #[serde(flatten)]
    pub dyn_meta: StopPicDynMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StopPicDynMeta {
    pub public: bool,
    pub sensitive: bool,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub quality: i16,
    pub tags: Vec<String>,
    pub notes: Option<String>,
}

impl StopPicDynMeta {
    pub(crate) fn derive_patch(
        self,
        pic: &StopPic,
    ) -> crate::models::history::StopPicturePatch {
        let mut patch = crate::models::history::StopPicturePatch::default();

        if self.public != pic.dyn_meta.public {
            patch.public = Some(self.public);
        }
        if self.sensitive != pic.dyn_meta.sensitive {
            patch.sensitive = Some(self.sensitive);
        }
        if self.lon != pic.dyn_meta.lon {
            patch.lon = Some(self.lon);
        }
        if self.lat != pic.dyn_meta.lat {
            patch.lat = Some(self.lat);
        }
        if self.quality != pic.dyn_meta.quality {
            patch.quality = Some(self.quality);
        }
        if self.tags != pic.dyn_meta.tags {
            patch.tags = Some(self.tags);
        }
        if self.notes != pic.dyn_meta.notes {
            patch.notes = Some(self.notes);
        }
        patch
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PanoPic {
    pub id: i32,
    pub original_filename: String,
    pub sha1: String,
    pub stop_id: Option<i32>,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub uploader: i32,
    pub upload_date: DateTime<Utc>,
    pub capture_date: Option<DateTime<Utc>>,
    pub sensitive: bool,
}
