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

use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Debug, Serialize, Deserialize, Component)]
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

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct StopPicDynMeta {
    pub public: bool,
    pub sensitive: bool,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub quality: i16,
    pub tags: Vec<String>,
    pub notes: Option<String>,
}

pub(crate) mod requests {
    use serde::Deserialize;
    use utoipa::Component;

    #[derive(Debug, Deserialize, Component)]
    pub struct ChangeStopPic {
        pub public: bool,
        pub sensitive: bool,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub tags: Vec<String>,
        pub stops: Vec<i32>,
        pub notes: Option<String>,
        pub quality: i16,
    }
}

pub(crate) mod responses {
    use serde::Serialize;
    use utoipa::Component;

    #[derive(Debug, Serialize, Component)]
    pub struct PublicStopPic {
        pub id: i32,
        pub sha1: String,
        pub capture_date: Option<String>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub quality: i16,
        pub tags: Vec<String>,
    }

    #[derive(Debug, Serialize, Component)]
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
    }

    #[derive(Debug, Serialize, Component)]
    pub struct UntaggedStopPic {
        pub id: i32,
        pub original_filename: String,
        pub sha1: String,
        pub public: bool,
        pub sensitive: bool,
        pub uploader: i32,
        pub upload_date: String,
        pub capture_date: Option<String>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub width: i32,
        pub height: i32,
        pub camera_ref: Option<String>,
        pub tags: Vec<String>,
        pub notes: Option<String>,
    }
}
