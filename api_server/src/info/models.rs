/*
    Intermodal, transportation information aggregator
    Copyright (C) 2024  Cláudio Pereira

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

#[derive(Deserialize, Debug)]
pub struct ExternalNewsImage {
    pub id: i32,
    pub sha1: String,
    pub has_copyright_issues: Option<bool>,
    pub transcript: Option<String>,
}

pub(crate) mod responses {
    use chrono::{DateTime, Local};
    use serde::Serialize;
    use sqlx::types::JsonValue;

    use crate::pics::get_external_news_img_path;

    #[derive(Debug, Serialize)]
    pub struct OperatorNewsItem {
        pub id: i32,
        pub summary: String,
        pub content: String,
        pub datetime: DateTime<Local>,
        pub geojson: Option<JsonValue>,
        pub visible: bool,
    }

    #[derive(Debug, Serialize)]
    pub struct ExternalNewsItem {
        pub id: i32,

        pub content_md: Option<String>,
        pub content_text: Option<String>,

        pub operator_id: Option<i32>,
        pub images: Vec<ExternalNewsImage>,

        pub datetime: DateTime<Local>,

        pub source: String,
        pub url: Option<String>,
    }

    #[derive(Debug, Serialize)]
    pub struct FullExternalNewsItem {
        pub id: i32,

        pub prepro_content_md: Option<String>,
        pub prepro_content_text: Option<String>,
        pub content_md: Option<String>,
        pub content_text: Option<String>,

        pub operator_id: Option<i32>,

        pub datetime: DateTime<Local>,
        pub imgs: Vec<FullExternalNewsImage>,

        pub source: String,
        pub url: Option<String>,
        pub raw: JsonValue,
        pub is_validated: bool,
        pub is_relevant: bool,
        pub is_sensible: bool,
    }

    #[derive(Serialize, Debug)]
    pub struct ExternalNewsImage {
        pub transcript: Option<String>,
        pub url: Option<String>,
    }

    #[derive(Serialize, Debug)]
    pub struct FullExternalNewsImage {
        pub id: i32,
        pub has_copyright_issues: Option<bool>,
        pub transcript: Option<String>,
        pub url: String,
    }

    impl From<super::ExternalNewsImage> for FullExternalNewsImage {
        fn from(image: super::ExternalNewsImage) -> Self {
            FullExternalNewsImage {
                id: image.id,
                has_copyright_issues: image.has_copyright_issues,
                transcript: image.transcript,
                url: get_external_news_img_path(&image.sha1),
            }
        }
    }
}

pub(crate) mod requests {
    use chrono::{DateTime, Local};
    use serde::Deserialize;
    use sqlx::types::JsonValue;

    #[derive(Debug, Deserialize)]
    pub struct NewExternalNewsItem {
        pub operator_id: Option<i32>,
        pub prepro_content_md: String,
        pub prepro_content_text: String,
        pub datetime: Option<DateTime<Local>>,
        pub source: String,
        pub url: Option<String>,
        pub raw: JsonValue,
    }
}

// Manual implementations of sqlx::Type due to
// https://github.com/rust-lang/rust/issues/82219
impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for ExternalNewsImage {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>>
    {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let sha1 = decoder.try_decode::<String>()?;
        let has_copyright_issues = decoder.try_decode::<Option<bool>>()?;
        let transcript = decoder.try_decode::<Option<String>>()?;
        Ok(ExternalNewsImage {
            id,
            sha1,
            has_copyright_issues,
            transcript,
        })
    }
}

impl sqlx::Type<sqlx::Postgres> for ExternalNewsImage {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("ExternalNewsImage")
    }
}
