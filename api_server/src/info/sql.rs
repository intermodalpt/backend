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

use chrono::Local;
use sqlx::PgPool;

use commons::models::info;

use super::models::{self, requests, responses};
use crate::pics::{get_external_news_img_path, get_external_news_ss_path};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<info::NewsItem>> {
    sqlx::query!(
        r#"
SELECT id, operator_id, title, summary,
    content as "content!: sqlx::types::Json<Vec<info::ContentBlock>>",
    publish_datetime, edit_datetime, visible
FROM news_items
LIMIT $1 OFFSET $2
"#,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(info::NewsItem {
            id: row.id,
            operator_id: row.operator_id,
            title: row.title,
            summary: row.summary,
            content: row.content.0,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row
                .edit_datetime
                .map(|datetime| datetime.with_timezone(&Local)),
            visible: row.visible,
        })
    })
    .collect()
}

pub(crate) async fn fetch_operator_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::OperatorNewsItem>> {
    sqlx::query!(
        r#"
SELECT id, title, operator_id, summary,
    content as "content!: sqlx::types::Json<Vec<info::ContentBlock>>",
    publish_datetime, edit_datetime, visible
FROM news_items
WHERE operator_id=$1
LIMIT $2 OFFSET $3
"#,
        operator_id,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(responses::OperatorNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            content: row.content.0,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row
                .edit_datetime
                .map(|datetime| datetime.with_timezone(&Local)),
            visible: row.visible,
        })
    })
    .collect()
}

pub(crate) async fn fetch_external_news_item(
    pool: &PgPool,
    item_id: i32,
) -> Result<Option<responses::ExternalNewsItem>> {
    Ok(sqlx::query!(
        r#"
SELECT external_news_items.id, operator_id, title, summary, author,
    COALESCE(content_md, prepro_content_md) as content_md,
    COALESCE(content_text, prepro_content_text) as content_text,
    edit_datetime, publish_datetime, source, url,
    is_partial, is_validated, is_relevant, is_sensitive,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(sha1, transcript, has_copyright_issues))
        ELSE array[]::record[]
    END as "imgs!: Vec<(String, Option<String>, Option<bool>)>"
FROM external_news_items
JOIN external_news_items_imgs ON external_news_items.id=external_news_items_imgs.item_id
JOIN external_news_imgs ON external_news_items_imgs.img_id=external_news_imgs.id
WHERE external_news_items.id=$1 AND has_copyright_issues=false
GROUP BY external_news_items.id
"#,
        item_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .map(|row| {

        responses::ExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            content_md: row.content_md,
            content_text: row.content_text,
            operator_id: row.operator_id,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            is_partial: row.is_partial,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            images: row.imgs
                .into_iter()
                .map(|(sha1, transcript, has_copyright_issues)| {
                    responses::ExternalNewsImage {
                        transcript,
                        url: if has_copyright_issues == Some(false) {
                            Some(get_external_news_img_path(sha1.as_ref()))
                        } else {
                            None
                        },
                    }
                })
                .collect::<Vec<_>>(),
        }
    }))
}

pub(crate) async fn fetch_full_external_news_item(
    pool: &PgPool,
    item_id: i32,
) -> Result<Option<responses::FullExternalNewsItem>> {
    Ok(sqlx::query!(
        r#"
SELECT external_news_items.id, operator_id, title, summary, author, content_md, prepro_content_md,
    content_text, prepro_content_text, edit_datetime, publish_datetime, source, url, is_partial,
    is_validated, is_relevant, is_sensitive, raw, ss_sha1,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(external_news_imgs.id, sha1, has_copyright_issues, transcript))
        ELSE array[]::record[]
    END as "imgs!: Vec<models::ExternalNewsImage>"
FROM external_news_items
JOIN external_news_items_imgs ON external_news_items.id=external_news_items_imgs.item_id
JOIN external_news_imgs ON external_news_items_imgs.img_id=external_news_imgs.id
WHERE external_news_items.id=$1
GROUP BY external_news_items.id
"#,
        item_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .map(|row| responses::FullExternalNewsItem {
        id: row.id,
        title: row.title,
        summary: row.summary,
        author: row.author,
        prepro_content_md: row.prepro_content_md,
        prepro_content_text: row.prepro_content_text,
        content_md: row.content_md,
        content_text: row.content_text,
        operator_id: row.operator_id,
        publish_datetime: row.publish_datetime.with_timezone(&Local),
        edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
        source: row.source,
        url: row.url,
        raw: row.raw,
        is_partial: row.is_partial,
        is_validated: row.is_validated,
        is_relevant: row.is_relevant,
        is_sensitive: row.is_sensitive,
        images: row.imgs.into_iter().map(Into::into).collect(),
        screenshot_url: row.ss_sha1.as_ref().map(|sha1| get_external_news_ss_path(sha1))
    }))
}

pub(crate) async fn fetch_external_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
    public_only: bool,
) -> Result<Vec<responses::ExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, operator_id, title, author, summary,
    COALESCE(content_md, prepro_content_md) as content_md,
    COALESCE(content_text, prepro_content_text) as content_text,
    publish_datetime, edit_datetime, source, url, is_partial, is_validated, is_relevant, is_sensitive,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(sha1, transcript, has_copyright_issues))
        ELSE array[]::record[]
    END as "imgs!: Vec<(String, Option<String>, Option<bool>)>"
FROM external_news_items
JOIN external_news_items_imgs ON external_news_items.id=external_news_items_imgs.item_id
JOIN external_news_imgs ON external_news_items_imgs.img_id=external_news_imgs.id
WHERE (($1 = true) OR (is_validated=true AND is_sensitive=false))
GROUP BY external_news_items.id
LIMIT $2 OFFSET $3
"#,
        public_only,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(responses::ExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            content_md: row.content_md,
            content_text: row.content_text,
            operator_id: row.operator_id,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            is_partial: row.is_partial,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            images: row
                .imgs
                .into_iter()
                .map(|(sha1, transcript, has_copyright_issues)| {
                    responses::ExternalNewsImage {
                        transcript,
                        url: if has_copyright_issues == Some(false)
                            || !public_only
                        {
                            Some(get_external_news_img_path(sha1.as_ref()))
                        } else {
                            None
                        },
                    }
                })
                .collect::<Vec<_>>(),
        })
    })
    .collect()
}

pub(crate) async fn fetch_operator_external_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
    public_only: bool,
) -> Result<Vec<responses::ExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, operator_id, title, author, summary,
    COALESCE(content_md, prepro_content_md) as content_md,
    COALESCE(content_text, prepro_content_text) as content_text,
    publish_datetime, edit_datetime, source, url,
    is_partial, is_validated, is_relevant, is_sensitive,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(sha1, transcript, has_copyright_issues))
        ELSE array[]::record[]
    END as "imgs!: Vec<(String, Option<String>, Option<bool>)>"
FROM external_news_items
JOIN external_news_items_imgs ON external_news_items.id=external_news_items_imgs.item_id
JOIN external_news_imgs ON external_news_items_imgs.img_id=external_news_imgs.id
WHERE operator_id=$1 
    AND (($2 = true) OR (is_validated=true AND is_sensitive=false))
GROUP BY external_news_items.id
LIMIT $3 OFFSET $4
"#,
        operator_id,
        public_only,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(responses::ExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            content_md: row.content_md,
            content_text: row.content_text,
            operator_id: row.operator_id,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            is_partial: row.is_partial,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            images: row.imgs
                .into_iter()
                .map(|(sha1, transcript, has_copyright_issues)| {
                    responses::ExternalNewsImage {
                        transcript,
                        url: if has_copyright_issues == Some(false) || !public_only {
                            Some(get_external_news_img_path(sha1.as_ref()))
                        } else {
                            None
                        },
                    }
                })
                .collect::<Vec<_>>(),
        })
    })
    .collect()
}

pub(crate) async fn fetch_pending_external_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::FullExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, operator_id, title, summary, author,
    content_md, prepro_content_md, content_text, prepro_content_text,
    publish_datetime, edit_datetime, source, url, is_partial, is_validated, is_relevant, is_sensitive, raw, ss_sha1,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(external_news_imgs.id, sha1, has_copyright_issues, transcript))
        ELSE array[]::record[]
    END as "imgs!: Vec<models::ExternalNewsImage>"
FROM external_news_items
JOIN external_news_items_imgs ON external_news_items.id=external_news_items_imgs.item_id
JOIN external_news_imgs ON external_news_items_imgs.img_id=external_news_imgs.id
WHERE is_validated=false
GROUP BY external_news_items.id
LIMIT $1 OFFSET $2
"#,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(responses::FullExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            prepro_content_md: row.prepro_content_md,
            prepro_content_text: row.prepro_content_text,
            content_md: row.content_md,
            content_text: row.content_text,
            operator_id: row.operator_id,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            raw: row.raw,
            is_partial: row.is_partial,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            images: row.imgs.into_iter().map(Into::into).collect(),
            screenshot_url: row.ss_sha1.as_ref().map(|sha1| get_external_news_ss_path(sha1))
        })
    })
    .collect()
}

pub(crate) async fn fetch_pending_operator_external_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::FullExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, operator_id, title, summary, author, content_md,
    prepro_content_md, content_text, prepro_content_text, publish_datetime, edit_datetime,
    source, url, is_partial, is_validated, is_relevant, is_sensitive, ss_sha1, raw,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(external_news_imgs.id, sha1, has_copyright_issues, transcript))
        ELSE array[]::record[]
    END as "imgs!: Vec<models::ExternalNewsImage>"
FROM external_news_items
JOIN external_news_items_imgs ON external_news_items.id=external_news_items_imgs.item_id
JOIN external_news_imgs ON external_news_items_imgs.img_id=external_news_imgs.id
WHERE operator_id=$1 AND is_validated=false
GROUP BY external_news_items.id
LIMIT $2 OFFSET $3
"#,
        operator_id,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(responses::FullExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            prepro_content_md: row.prepro_content_md,
            prepro_content_text: row.prepro_content_text,
            content_md: row.content_md,
            content_text: row.content_text,
            operator_id: row.operator_id,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            raw: row.raw,
            is_partial: row.is_partial,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            images: row.imgs.into_iter().map(Into::into).collect(),
            screenshot_url: row.ss_sha1.as_ref().map(|sha1| get_external_news_ss_path(sha1))
        })
    })
    .collect()
}

pub(crate) async fn insert_external_news(
    pool: &PgPool,
    change: requests::NewExternalNewsItem,
) -> Result<i32> {
    let row = sqlx::query!(
        r#"
INSERT INTO external_news_items (operator_id, title, summary, author,
    prepro_content_md, prepro_content_text, publish_datetime, edit_datetime,
    source, url, is_partial, raw)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
RETURNING id"#,
        change.operator_id,
        change.title,
        change.summary,
        change.author,
        change.prepro_content_md,
        change.prepro_content_text,
        change.publish_datetime,
        change.edit_datetime,
        change.source,
        change.url,
        change.is_partial,
        change.raw
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(row.id)
}

pub(crate) async fn delete_external_news_item(
    pool: &PgPool,
    id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM external_news_items
WHERE id=$1
"#,
        id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn fetch_external_news_source_urls(
    pool: &PgPool,
    source: &str,
) -> Result<Vec<String>> {
    Ok(sqlx::query!(
        r#"
SELECT url
FROM external_news_items
WHERE source=$1
"#,
        source,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .filter_map(|row| row.url)
    .collect())
}
