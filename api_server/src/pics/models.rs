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

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SimpleRichImg {
    pub id: Uuid,
    pub sha1: String,
    pub transcript: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ExternalNewsImg {
    pub id: i32,
    pub sha1: String,
    pub has_copyright_issues: Option<bool>,
    pub transcript: Option<String>,
}
pub(crate) mod requests {
    use serde::Deserialize;

    use commons::models::{history::pics as history, pics};

    use crate::utils::canonicalize_optional_string;

    #[derive(Debug, Deserialize)]
    pub struct ChangeStopPic {
        pub public: bool,
        pub sensitive: bool,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub tags: Vec<String>,
        pub attrs: Vec<String>,
        pub stops: Vec<pics::StopAttrs>,
        pub notes: Option<String>,
        pub quality: i16,
    }

    impl ChangeStopPic {
        pub(crate) fn derive_patch(
            &self,
            pic: &pics::StopPic,
        ) -> history::StopPicturePatch {
            let mut patch = history::StopPicturePatch::default();

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
                patch.tags = Some(self.tags.clone());
            }
            if self.attrs != pic.dyn_meta.attrs {
                patch.attrs = Some(self.attrs.clone());
            }
            if self.notes != pic.dyn_meta.notes {
                patch.notes = Some(self.notes.clone());
            }
            patch
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangeRichImgMeta {
        pub transcript: Option<String>,
        pub attribution: Option<String>,
        pub license: Option<String>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
    }

    impl ChangeRichImgMeta {
        pub(crate) fn tidy(&mut self) {
            canonicalize_optional_string(&mut self.transcript);
            canonicalize_optional_string(&mut self.attribution);
            canonicalize_optional_string(&mut self.license);
        }
    }
}

pub(crate) mod responses {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use commons::models::pics;
    use serde::Serialize;
    use uuid::Uuid;

    use crate::pics::{
        get_external_news_pic_path, get_rich_img_full_path,
        get_rich_img_medium_path, get_rich_img_thumb_path,
        get_stop_pic_medium_path, get_stop_pic_ori_named_path,
        get_stop_pic_ori_path, get_stop_pic_thumb_path,
    };

    #[derive(Serialize, Debug)]
    pub struct SimpleRichImg {
        pub id: Uuid,
        pub transcript: Option<String>,
        pub url_full: String,
        pub url_medium: String,
        pub url_thumb: String,
    }

    impl From<super::SimpleRichImg> for SimpleRichImg {
        fn from(value: super::SimpleRichImg) -> Self {
            Self {
                id: value.id,
                transcript: value.transcript,
                url_full: get_rich_img_full_path(&value.sha1),
                url_medium: get_rich_img_medium_path(&value.sha1),
                url_thumb: get_rich_img_thumb_path(&value.sha1),
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct RichImg {
        pub id: Uuid,
        pub transcript: Option<String>,
        pub attribution: Option<String>,
        pub license: Option<String>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub url_full: String,
        pub url_medium: String,
        pub url_thumb: String,
    }

    impl From<pics::RichImg> for RichImg {
        fn from(value: pics::RichImg) -> Self {
            Self {
                id: value.id,
                transcript: value.transcript,
                attribution: value.attribution,
                license: value.license,
                lat: value.lat,
                lon: value.lon,
                url_full: get_rich_img_full_path(&value.sha1),
                url_medium: get_rich_img_medium_path(&value.sha1),
                url_thumb: get_rich_img_thumb_path(&value.sha1),
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct FullRichImg {
        pub id: Uuid,
        pub transcript: Option<String>,
        pub attribution: Option<String>,
        pub license: Option<String>,
        pub lat: Option<f64>,
        pub lon: Option<f64>,
        pub filename: Option<String>,
        pub url_full: String,
        pub url_medium: String,
        pub url_thumb: String,
    }

    impl From<pics::RichImg> for FullRichImg {
        fn from(img: pics::RichImg) -> Self {
            Self {
                id: img.id,
                transcript: img.transcript,
                attribution: img.attribution,
                license: img.license,
                lat: img.lat,
                lon: img.lon,
                filename: img.filename,
                url_full: get_rich_img_full_path(&img.sha1),
                url_medium: get_rich_img_medium_path(&img.sha1),
                url_thumb: get_rich_img_thumb_path(&img.sha1),
            }
        }
    }

    #[derive(Debug, Serialize)]
    pub struct MinimalStopPic {
        pub id: i32,
        pub url_full: String,
        pub url_medium: String,
        pub url_thumb: String,
    }

    #[derive(Debug, Serialize)]
    pub struct PublicStopPic {
        pub id: i32,
        // TODO deprecate
        pub sha1: String,
        pub capture_date: Option<NaiveDateTime>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub quality: i16,
        pub tags: Vec<String>,
        pub attrs: Vec<String>,
        pub url_full: String,
        pub url_medium: String,
        pub url_thumb: String,
    }

    impl From<pics::StopPic> for PublicStopPic {
        fn from(value: pics::StopPic) -> Self {
            PublicStopPic {
                id: value.id,
                url_full: get_stop_pic_ori_path(&value.sha1),
                url_medium: get_stop_pic_medium_path(&value.sha1),
                url_thumb: get_stop_pic_thumb_path(&value.sha1),
                sha1: value.sha1,
                capture_date: value.capture_date,
                lon: value.dyn_meta.lon,
                lat: value.dyn_meta.lat,
                quality: value.dyn_meta.quality,
                tags: value.dyn_meta.tags,
                attrs: value.dyn_meta.attrs,
            }
        }
    }

    #[derive(Debug, Serialize)]
    pub struct PicWithStops {
        pub id: i32,
        pub original_filename: String,
        // TODO deprecate
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
        pub attrs: Vec<String>,
        pub notes: Option<String>,
        pub stops: Vec<pics::StopAttrs>,
        // TODO Consider this
        pub tagged: bool,
        pub url_full: String,
        pub url_medium: String,
        pub url_thumb: String,
    }

    impl From<(pics::StopPic, Vec<pics::StopAttrs>)> for PicWithStops {
        fn from(value: (pics::StopPic, Vec<pics::StopAttrs>)) -> Self {
            let (pic, stops) = value;
            Self {
                id: pic.id,
                url_full: get_stop_pic_ori_named_path(
                    &pic.sha1,
                    &pic.original_filename,
                ),
                url_medium: get_stop_pic_medium_path(&pic.sha1),
                url_thumb: get_stop_pic_thumb_path(&pic.sha1),
                original_filename: pic.original_filename,
                sha1: pic.sha1,
                public: pic.dyn_meta.public,
                sensitive: pic.dyn_meta.sensitive,
                uploader: pic.uploader,
                upload_date: pic.upload_date,
                capture_date: pic.capture_date,
                lon: pic.dyn_meta.lon,
                lat: pic.dyn_meta.lat,
                width: pic.width,
                height: pic.height,
                quality: pic.dyn_meta.quality,
                camera_ref: pic.camera_ref,
                tags: pic.dyn_meta.tags,
                attrs: pic.dyn_meta.attrs,
                notes: pic.dyn_meta.notes,
                tagged: pic.tagged,
                stops,
            }
        }
    }

    #[derive(Debug, Serialize)]
    pub struct UntaggedStopPic {
        pub id: i32,
        pub original_filename: String,
        pub sha1: String,
        pub public: bool,
        pub sensitive: bool,
        pub uploader: i32,
        pub upload_date: DateTime<Utc>,
        pub capture_date: Option<NaiveDateTime>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub width: i32,
        pub height: i32,
        pub camera_ref: Option<String>,
        pub tags: Vec<String>,
        pub attrs: Vec<String>,
        pub notes: Option<String>,
        pub url_full: String,
        pub url_medium: String,
        pub url_thumb: String,
    }

    #[derive(Debug, Serialize)]
    pub struct MinimalPicWithStops {
        pub id: i32,
        pub public: bool,
        pub sensitive: bool,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub stops: Vec<i32>,
        // TODO Consider this
        pub tagged: bool,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct FullPanoPic {
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

    #[derive(Debug, Clone, Serialize)]
    pub struct PanoPic {
        pub id: i32,
        pub sha1: String,
        pub stop_id: Option<i32>,
        pub lon: Option<f64>,
        pub lat: Option<f64>,
        pub capture_date: Option<NaiveDateTime>,
    }

    #[derive(Debug, Serialize)]
    pub struct PanoOnion {
        pub predecessors: Vec<MinimalPicWithStops>,
        pub successors: Vec<MinimalPicWithStops>,
    }

    #[derive(Serialize, Debug)]
    pub struct ExternalNewsImg {
        pub transcript: Option<String>,
        pub url: Option<String>,
    }

    impl From<pics::ExternalNewsImg> for ExternalNewsImg {
        fn from(value: pics::ExternalNewsImg) -> Self {
            Self {
                transcript: value.transcript,
                url: Some(get_external_news_pic_path(&value.sha1)),
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct FullExternalNewsImg {
        pub id: i32,
        pub has_copyright_issues: Option<bool>,
        pub transcript: Option<String>,
        pub url: String,
    }

    impl From<pics::ExternalNewsImg> for FullExternalNewsImg {
        fn from(value: pics::ExternalNewsImg) -> Self {
            FullExternalNewsImg {
                id: value.id,
                has_copyright_issues: value.has_copyright_issues,
                transcript: value.transcript,
                url: get_external_news_pic_path(&value.sha1),
            }
        }
    }

    impl From<crate::pics::models::ExternalNewsImg> for ExternalNewsImg {
        fn from(image: crate::pics::models::ExternalNewsImg) -> Self {
            ExternalNewsImg {
                transcript: image.transcript,
                url: Some(get_external_news_pic_path(&image.sha1)),
            }
        }
    }
    impl From<crate::pics::models::ExternalNewsImg> for FullExternalNewsImg {
        fn from(image: crate::pics::models::ExternalNewsImg) -> Self {
            FullExternalNewsImg {
                id: image.id,
                has_copyright_issues: image.has_copyright_issues,
                transcript: image.transcript,
                url: get_external_news_pic_path(&image.sha1),
            }
        }
    }
}

// Manual implementations of sqlx::Type due to
// https://github.com/rust-lang/rust/issues/82219
impl sqlx::Type<sqlx::Postgres> for SimpleRichImg {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("RichImg")
    }
}
impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for SimpleRichImg {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>>
    {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<Uuid>()?;
        let sha1 = decoder.try_decode::<String>()?;
        let transcript = decoder.try_decode::<Option<String>>()?;
        Ok(SimpleRichImg {
            id,
            sha1,
            transcript,
        })
    }
}

impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for ExternalNewsImg {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>>
    {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let sha1 = decoder.try_decode::<String>()?;
        let has_copyright_issues = decoder.try_decode::<Option<bool>>()?;
        let transcript = decoder.try_decode::<Option<String>>()?;
        Ok(ExternalNewsImg {
            id,
            sha1,
            has_copyright_issues,
            transcript,
        })
    }
}

impl sqlx::Type<sqlx::Postgres> for ExternalNewsImg {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("ExternalNewsPic")
    }
}
