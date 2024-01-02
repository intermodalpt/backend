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

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::models::pics as current;

#[derive(Debug, Serialize)]
pub struct TaggedStopPic {
    pub id: i32,
    pub original_filename: String,
    pub sha1: String,
    pub public: bool,
    pub sensitive: bool,
    pub uploader: i32,
    pub upload_date: DateTime<Utc>,
    pub capture_date: Option<NaiveDateTime>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopPic {
    pub id: i32,
    pub original_filename: String,
    pub sha1: String,
    pub tagged: bool,
    pub uploader: i32,
    pub upload_date: DateTime<Utc>,
    pub capture_date: Option<NaiveDateTime>,
    pub updater: Option<i32>,
    pub update_date: Option<DateTime<Utc>>,
    pub width: i32,
    pub height: i32,
    pub camera_ref: Option<String>,
    #[serde(flatten)]
    pub dyn_meta: crate::models::pics::StopPicDynMeta,
}

impl From<current::StopPic> for StopPic {
    fn from(pic: current::StopPic) -> Self {
        Self {
            id: pic.id,
            original_filename: pic.original_filename,
            sha1: pic.sha1,
            tagged: false,
            uploader: pic.uploader,
            upload_date: pic.upload_date,
            capture_date: pic.capture_date,
            updater: pic.updater,
            update_date: pic.update_date,
            width: pic.width,
            height: pic.height,
            camera_ref: pic.camera_ref,
            dyn_meta: pic.dyn_meta,
        }
    }
}

impl TryFrom<StopPic> for current::StopPic {
    type Error = Error;

    fn try_from(pic: StopPic) -> Result<Self, Self::Error> {
        Ok(Self {
            id: pic.id,
            original_filename: pic.original_filename,
            sha1: pic.sha1,
            tagged: pic.tagged,
            uploader: pic.uploader,
            upload_date: pic.upload_date,
            capture_date: pic.capture_date,
            updater: pic.updater,
            update_date: pic.update_date,
            width: pic.width,
            height: pic.height,
            camera_ref: pic.camera_ref,
            dyn_meta: pic.dyn_meta,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopPicDynMeta {
    pub public: bool,
    pub sensitive: bool,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub quality: i16,
    pub tags: Vec<String>,
    #[serde(default)]
    pub attrs: Vec<String>,
    pub notes: Option<String>,
}

impl From<current::StopPicDynMeta> for StopPicDynMeta {
    fn from(meta: current::StopPicDynMeta) -> Self {
        Self {
            public: meta.public,
            sensitive: meta.sensitive,
            lon: meta.lon,
            lat: meta.lat,
            quality: meta.quality,
            tags: meta.tags,
            attrs: meta.attrs,
            notes: meta.notes,
        }
    }
}

impl TryFrom<StopPicDynMeta> for current::StopPicDynMeta {
    type Error = Error;

    fn try_from(meta: StopPicDynMeta) -> Result<Self, Self::Error> {
        Ok(Self {
            public: meta.public,
            sensitive: meta.sensitive,
            lon: meta.lon,
            lat: meta.lat,
            quality: meta.quality,
            tags: meta.tags,
            attrs: meta.attrs,
            notes: meta.notes,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopAttrs {
    pub id: i32,
    pub attrs: Vec<String>,
}

impl From<current::StopAttrs> for StopAttrs {
    fn from(attrs: current::StopAttrs) -> Self {
        Self {
            id: attrs.id,
            attrs: attrs.attrs,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanoPic {
    pub id: i32,
    pub original_filename: String,
    pub sha1: String,
    pub stop_id: Option<i32>,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub uploader: i32,
    pub upload_date: DateTime<Utc>,
    pub capture_date: Option<NaiveDateTime>,
    pub sensitive: bool,
}

impl From<current::PanoPic> for PanoPic {
    fn from(pic: current::PanoPic) -> Self {
        Self {
            id: pic.id,
            original_filename: pic.original_filename,
            sha1: pic.sha1,
            stop_id: pic.stop_id,
            lon: pic.lon,
            lat: pic.lat,
            uploader: pic.uploader,
            upload_date: pic.upload_date,
            capture_date: pic.capture_date,
            sensitive: pic.sensitive,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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
    pub attrs: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub notes: Option<Option<String>>,
}

impl StopPicturePatch {
    #[must_use]
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
    pub fn apply(self, pic: &mut current::StopPic) {
        if let Some(public) = self.public {
            pic.dyn_meta.public = public;
        }
        if let Some(sensitive) = self.sensitive {
            pic.dyn_meta.sensitive = sensitive;
        }
        if let Some(lon) = self.lon {
            pic.dyn_meta.lon = lon;
        }
        if let Some(lat) = self.lat {
            pic.dyn_meta.lat = lat;
        }
        if let Some(quality) = self.quality {
            pic.dyn_meta.quality = quality;
        }
        if let Some(tags) = self.tags {
            pic.dyn_meta.tags = tags;
        }
        if let Some(notes) = self.notes {
            pic.dyn_meta.notes = notes;
        }
    }
}
