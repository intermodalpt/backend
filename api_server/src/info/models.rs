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
    use chrono::{DateTime, Local, Utc};
    use serde::Serialize;
    use sqlx::types::JsonValue;

    use commons::models::info;

    use crate::pics::get_external_news_img_path;

    #[derive(Debug, Serialize)]
    pub struct NewsItem {
        pub id: i32,
        pub title: String,
        pub summary: String,
        pub content: Vec<info::ContentBlock>,
        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,
        pub visible: bool,

        pub operator_ids: Vec<i32>,
        pub regions_ids: Vec<i32>,
    }

    #[derive(Debug, Serialize)]
    pub struct ExternalNewsItem {
        pub id: i32,
        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,

        pub title: Option<String>,
        pub summary: Option<String>,
        pub author: Option<String>,

        pub content_md: Option<String>,
        pub content_text: Option<String>,

        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,

        pub source: String,
        pub url: Option<String>,

        pub is_complete: bool,
        pub is_validated: bool,
        pub is_relevant: Option<bool>,
        pub is_sensitive: bool,

        pub images: Vec<ExternalNewsImage>,
    }

    #[derive(Debug, Serialize)]
    pub struct FullExternalNewsItem {
        pub id: i32,

        pub title: Option<String>,
        pub summary: Option<String>,
        pub author: Option<String>,

        pub prepro_content_md: Option<String>,
        pub prepro_content_text: Option<String>,
        pub content_md: Option<String>,
        pub content_text: Option<String>,

        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,

        pub source: String,
        pub url: Option<String>,
        pub raw: JsonValue,

        pub is_complete: bool,
        pub is_validated: bool,
        pub is_relevant: Option<bool>,
        pub is_sensitive: bool,

        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,
        pub images: Vec<FullExternalNewsImage>,
        pub screenshot_url: Option<String>,
    }

    /// An item as sourced
    #[derive(Debug, Serialize)]
    pub struct SourceExternalNewsItem {
        pub id: i32,
        pub operator_ids: Vec<i32>,

        pub title: Option<String>,
        pub summary: Option<String>,
        pub author: Option<String>,

        pub prepro_content_md: Option<String>,
        pub prepro_content_text: Option<String>,

        pub publish_datetime: DateTime<Utc>,
        pub edit_datetime: Option<DateTime<Utc>>,

        pub source: String,
        pub url: Option<String>,
        pub raw: JsonValue,

        pub is_complete: bool,
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
    use commons::models::info::ContentBlock;
    use serde::Deserialize;
    use sqlx::types::JsonValue;

    #[derive(Debug, Deserialize)]
    pub struct NewNewsItem {
        pub title: Option<String>,
        pub summary: Option<String>,
        pub author_id: Option<i32>,
        pub author_override: Option<String>,

        pub content: Vec<ContentBlock>,

        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,

        pub visible: bool,

        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,
    }

    impl NewNewsItem {
        pub(crate) fn tidy(&mut self) {
            if let Some(title) = &self.title {
                if title.is_empty() {
                    self.title = None;
                }
            }
            if let Some(summary) = &self.summary {
                if summary.is_empty() {
                    self.summary = None;
                }
            }
            if let Some(author_override) = &self.author_override {
                if author_override.is_empty() {
                    self.author_override = None;
                }
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct NewExternalNewsItem {
        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,

        pub title: Option<String>,
        pub summary: Option<String>,
        pub author: Option<String>,

        pub prepro_content_md: Option<String>,
        pub prepro_content_text: Option<String>,

        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,

        pub source: String,
        pub url: Option<String>,
        pub is_complete: bool,
        pub raw: JsonValue,
    }

    impl NewExternalNewsItem {
        pub(crate) fn tidy(&mut self) {
            if let Some(title) = &self.title {
                if title.is_empty() {
                    self.title = None;
                }
            }
            if let Some(summary) = &self.summary {
                if summary.is_empty() {
                    self.summary = None;
                }
            }
            if let Some(author) = &self.author {
                if author.is_empty() {
                    self.author = None;
                }
            }
            if let Some(prepro_content_md) = &self.prepro_content_md {
                if prepro_content_md.is_empty() {
                    self.prepro_content_md = None;
                }
            }
            if let Some(prepro_content_text) = &self.prepro_content_text {
                if prepro_content_text.is_empty() {
                    self.prepro_content_text = None;
                }
            }
            if let Some(url) = &self.url {
                if url.is_empty() {
                    self.url = None;
                }
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangeExternalNewsItem {
        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,

        pub title: Option<String>,
        pub summary: Option<String>,
        pub author: Option<String>,

        pub content_md: Option<String>,

        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,

        pub url: Option<String>,

        pub is_complete: bool,
        pub is_relevant: bool,
        pub is_sensitive: bool,
        pub is_validated: bool,
    }

    impl ChangeExternalNewsItem {
        pub(crate) fn tidy(&mut self) {
            if let Some(title) = &self.title {
                if title.is_empty() {
                    self.title = None;
                }
            }
            if let Some(summary) = &self.summary {
                if summary.is_empty() {
                    self.summary = None;
                }
            }
            if let Some(author) = &self.author {
                if author.is_empty() {
                    self.author = None;
                }
            }
            if let Some(content_md) = &self.content_md {
                if content_md.is_empty() {
                    self.content_md = None;
                }
            }
            if let Some(url) = &self.url {
                if url.is_empty() {
                    self.url = None;
                }
            }
        }
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
