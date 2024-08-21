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

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalRel {
    pub id: i32,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub source: String,
    pub publish_datetime: DateTime<Local>,
}

pub(crate) mod responses {
    use chrono::{DateTime, Local, Utc};
    use serde::Serialize;
    use sqlx::types::JsonValue;
    use uuid::Uuid;

    use commons::models::content::RichContent;

    use crate::pics::models::responses as pic_responses;

    #[derive(Serialize)]
    pub struct NewsItemListing {
        pub id: i32,
        pub title: String,
        pub summary: String,
        pub content: RichContent,
        pub thumb_url: Option<String>,

        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,

        pub is_visible: bool,
        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,
    }

    #[derive(Serialize)]
    pub struct NewsItem {
        pub id: i32,
        pub title: String,
        pub summary: String,
        pub content: RichContent,
        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,
        pub is_visible: bool,

        pub thumb_url: Option<String>,
        pub external_rels: Vec<super::ExternalRel>,

        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,
    }

    #[derive(Serialize)]
    pub struct FullNewsItem {
        pub id: i32,
        pub title: String,
        pub summary: String,
        pub content: RichContent,
        pub publish_datetime: DateTime<Local>,
        pub edit_datetime: Option<DateTime<Local>>,
        pub is_visible: bool,

        pub thumb_id: Option<Uuid>,
        pub images: Vec<pic_responses::SimpleRichImg>,
        pub external_rels: Vec<super::ExternalRel>,

        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,
    }

    #[derive(Debug, Serialize)]
    pub struct ExternalNewsItem {
        pub id: i32,

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

        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,
        pub images: Vec<pic_responses::ExternalNewsImg>,
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
        pub images: Vec<pic_responses::FullExternalNewsImg>,
        pub screenshot_url: Option<String>,
    }

    /// An item as sourced
    #[derive(Debug, Serialize)]
    pub struct SourceExternalNewsItem {
        pub id: i32,
        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,

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
}

pub(crate) mod requests {
    use chrono::{DateTime, Local};
    use serde::Deserialize;
    use sqlx::types::JsonValue;
    use std::collections::HashSet;
    use uuid::Uuid;

    use commons::models::content::RichContent;

    use crate::utils::canonicalize_optional_string;

    #[derive(Debug, Deserialize)]
    pub struct ChangeNewsItem {
        pub title: String,
        pub summary: String,
        pub author_id: Option<i32>,
        pub author_override: Option<String>,

        pub content: RichContent,
        pub thumb_id: Option<Uuid>,

        pub publish_datetime: Option<DateTime<Local>>,
        pub edit_datetime: Option<DateTime<Local>>,

        pub is_visible: bool,

        pub operator_ids: Vec<i32>,
        pub region_ids: Vec<i32>,
        pub external_ids: Vec<i32>,
    }

    impl ChangeNewsItem {
        pub(crate) fn validate(&mut self) -> Result<(), &'static str> {
            canonicalize_optional_string(&mut self.author_override);

            if self.title.trim().is_empty() {
                return Err("Empty title");
            }
            if self.summary.trim().is_empty() {
                return Err("Empty summary");
            }
            self.content.validate()?;
            Ok(())
        }

        pub(crate) fn get_linked_images(&self) -> HashSet<Uuid> {
            let mut ids: HashSet<Uuid> =
                self.content.get_linked_images().into_iter().collect();
            if let Some(thumb_id) = self.thumb_id {
                ids.insert(thumb_id);
            }

            ids
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
            canonicalize_optional_string(&mut self.title);
            canonicalize_optional_string(&mut self.summary);
            canonicalize_optional_string(&mut self.author);
            canonicalize_optional_string(&mut self.prepro_content_md);
            canonicalize_optional_string(&mut self.prepro_content_text);
            canonicalize_optional_string(&mut self.url);
        }
    }

    #[allow(clippy::struct_excessive_bools)]
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
            canonicalize_optional_string(&mut self.title);
            canonicalize_optional_string(&mut self.summary);
            canonicalize_optional_string(&mut self.author);
            canonicalize_optional_string(&mut self.content_md);
            canonicalize_optional_string(&mut self.url);
        }
    }
}

// Manual implementations of sqlx::Type due to
// https://github.com/rust-lang/rust/issues/82219
impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for ExternalRel {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn ::std::error::Error + 'static + Send + Sync>>
    {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let title = decoder.try_decode::<Option<String>>()?;
        let summary = decoder.try_decode::<Option<String>>()?;
        let source = decoder.try_decode::<String>()?;
        let publish_datetime = decoder.try_decode::<DateTime<Local>>()?;
        Ok(ExternalRel {
            id,
            title,
            summary,
            source,
            publish_datetime,
        })
    }
}

impl sqlx::Type<sqlx::Postgres> for ExternalRel {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("ExternalRel")
    }
}
