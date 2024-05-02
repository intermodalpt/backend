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

#[derive(Debug, PartialEq)]
pub enum Resource {
    StopPic(StopPic),
    PanoPic(PanoPic),
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub dyn_meta: StopPicDynMeta,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopAttrs {
    pub id: i32,
    pub attrs: Vec<String>,
}

impl From<(i32, Vec<String>)> for StopAttrs {
    fn from((id, attrs): (i32, Vec<String>)) -> Self {
        StopAttrs { id, attrs }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsPic {
    pub id: i32,
    pub sha1: String,
    pub filename: Option<String>,
    pub transcript: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalNewsPic {
    pub id: i32,
    pub sha1: String,
    pub has_copyright_issues: Option<bool>,
    pub transcript: Option<String>,
}
