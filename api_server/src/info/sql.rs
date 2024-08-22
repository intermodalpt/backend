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
use itertools::Itertools;
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

use commons::models::content::RichContent;

use super::models::{self, requests, responses};
use crate::pics::{
    get_external_news_pic_path, get_external_news_ss_path,
    get_rich_img_thumb_path, models as pic_models,
};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::NewsItemListing>> {
    sqlx::query!(
        r#"
SELECT id, title, summary,
    content as "content!: sqlx::types::Json<RichContent>",
    publish_datetime, edit_datetime, is_visible, thumb_url,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>"
FROM news_items
LEFT JOIN news_items_operators ON news_items.id=news_items_operators.item_id
LEFT JOIN news_items_regions ON news_items.id=news_items_regions.item_id
GROUP BY news_items.id
LIMIT $1 OFFSET $2
"#,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::NewsItemListing {
            id: row.id,
            title: row.title,
            summary: row.summary,
            content: row.content.0,
            thumb_url: row.thumb_url,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row
                .edit_datetime
                .map(|datetime| datetime.with_timezone(&Local)),
            is_visible: row.is_visible,
            operator_ids: row.operator_ids,
            region_ids: row.region_ids,
        })
    })
    .collect()
}

pub(crate) async fn count_news(pool: &PgPool) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as "cnt!: i64"
FROM news_items
"#,
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .map_or(0, |row| row.cnt))
}

pub(crate) async fn fetch_operator_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::NewsItemListing>> {
    sqlx::query!(
        r#"
SELECT id, title, summary,
    content as "content!: sqlx::types::Json<RichContent>",
    publish_datetime, edit_datetime, is_visible, thumb_url,
    array_agg(distinct news_items_operators.operator_id) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>"
FROM news_items
JOIN news_items_operators as rel ON news_items.id=rel.item_id
JOIN news_items_operators ON news_items.id=news_items_operators.item_id
LEFT JOIN news_items_regions ON news_items.id=news_items_regions.item_id
WHERE rel.operator_id=$1
GROUP BY news_items.id
LIMIT $2 OFFSET $3
"#,
        operator_id,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id, take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::NewsItemListing {
            id: row.id,
            title: row.title,
            summary: row.summary,
            content: row.content.0,
            thumb_url: row.thumb_url,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row
                .edit_datetime
                .map(|datetime| datetime.with_timezone(&Local)),
            is_visible: row.is_visible,
            operator_ids: row.operator_ids,
            region_ids: row.region_ids,
        })
    })
    .collect()
}

pub(crate) async fn count_operator_news(
    pool: &PgPool,
    operator_id: i32,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as "cnt!: i64"
FROM news_items
LEFT JOIN news_items_operators ON news_items.id=news_items_operators.item_id
WHERE operator_id=$1
"#,
        operator_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })?
    .map_or(0, |row| row.cnt))
}

pub(crate) async fn fetch_region_news(
    pool: &PgPool,
    region_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::NewsItemListing>> {
    sqlx::query!(
        r#"
SELECT id, title, summary,
    content as "content!: sqlx::types::Json<RichContent>",
    publish_datetime, edit_datetime, is_visible, thumb_url,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_agg(distinct news_items_regions.region_id) as "region_ids!: Vec<i32>"
FROM news_items
LEFT JOIN news_items_operators ON news_items.id=news_items_operators.item_id
JOIN news_items_regions as rel ON news_items.id=rel.item_id
JOIN news_items_regions ON news_items.id=news_items_regions.item_id
WHERE rel.region_id=$1
GROUP BY news_items.id
LIMIT $2 OFFSET $3
"#,
        region_id,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id, take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::NewsItemListing {
            id: row.id,
            title: row.title,
            summary: row.summary,
            content: row.content.0,
            thumb_url: row.thumb_url,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row
                .edit_datetime
                .map(|datetime| datetime.with_timezone(&Local)),
            is_visible: row.is_visible,
            operator_ids: row.operator_ids,
            region_ids: row.region_ids,
        })
    })
    .collect()
}

pub(crate) async fn count_region_news(
    pool: &PgPool,
    region_id: i32,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as "cnt!: i64"
FROM news_items
LEFT JOIN news_items_regions ON news_items.id=news_items_regions.item_id
WHERE region_id=$1
"#,
        region_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id);
        Error::DatabaseExecution
    })?
    .map_or(0, |row| row.cnt))
}

pub(crate) async fn fetch_news_item(
    pool: &PgPool,
    item_id: i32,
) -> Result<Option<responses::NewsItem>> {
    Ok(sqlx::query!(
        r#"
SELECT news_items.id, news_items.title, news_items.summary,
    content as "content!: sqlx::types::Json<RichContent>",
    news_items.publish_datetime, news_items.edit_datetime, is_visible, thumb_url,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    CASE
        WHEN count(news_items_external_news_items.item_id) > 0
        THEN array_agg(ROW(
            external_news_items.id,
            external_news_items.title,
            external_news_items.summary,
            external_news_items.source,
            external_news_items.publish_datetime
            ))
        ELSE array[]::record[]
    END as "external_rels!: Vec<models::ExternalRel>"
FROM news_items
LEFT JOIN news_items_operators ON news_items.id=news_items_operators.item_id
LEFT JOIN news_items_regions ON news_items.id=news_items_regions.item_id
LEFT JOIN news_items_external_news_items
    ON news_items.id=news_items_external_news_items.item_id
LEFT JOIN external_news_items
    ON news_items_external_news_items.external_item_id=external_news_items.id
WHERE news_items.id=$1
GROUP BY news_items.id
"#,
        item_id,
    )
        .fetch_optional(pool)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), item_id);
            Error::DatabaseExecution
        })?
        .map(|row| {
            responses::NewsItem {
                id: row.id,
                title: row.title,
                summary: row.summary,
                content: row.content.0,
                publish_datetime: row.publish_datetime.with_timezone(&Local),
                edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
                is_visible: row.is_visible,
                thumb_url: row.thumb_url,
                external_rels: row.external_rels,
                operator_ids: row.operator_ids,
                region_ids: row.region_ids,
            }
        }))
}

pub(crate) async fn fetch_full_news_item<'c, E>(
    executor: E,
    item_id: i32,
) -> Result<Option<responses::FullNewsItem>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    Ok(sqlx::query!(
        r#"
SELECT news_items.id, news_items.title, news_items.summary,
    content as "content!: sqlx::types::Json<RichContent>",
    news_items.publish_datetime, news_items.edit_datetime, is_visible, thumb_id,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    CASE
        WHEN count(rich_imgs.id) > 0
        THEN array_agg(ROW(rich_imgs.id, sha1, transcript))
        ELSE array[]::record[]
    END as "imgs!: Vec<pic_models::SimpleRichImg>",
    CASE
        WHEN count(news_items_external_news_items.item_id) > 0
        THEN array_agg(ROW(
            external_news_items.id,
            external_news_items.title,
            external_news_items.summary,
            external_news_items.source,
            external_news_items.publish_datetime
            ))
        ELSE array[]::record[]
    END as "external_rels!: Vec<models::ExternalRel>"
FROM news_items
LEFT JOIN news_items_operators ON news_items.id=news_items_operators.item_id
LEFT JOIN news_items_regions ON news_items.id=news_items_regions.item_id
LEFT JOIN news_items_imgs ON news_items.id=news_items_imgs.item_id
LEFT JOIN rich_imgs ON news_items_imgs.img_id=rich_imgs.id
LEFT JOIN news_items_external_news_items
    ON news_items.id=news_items_external_news_items.item_id
LEFT JOIN external_news_items
    ON news_items_external_news_items.external_item_id=external_news_items.id
WHERE news_items.id=$1
GROUP BY news_items.id
"#,
        item_id,
    )
        .fetch_optional(executor)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), item_id);
            Error::DatabaseExecution
        })?
        .map(|row| {
            responses::FullNewsItem {
                id: row.id,
                title: row.title,
                summary: row.summary,
                content: row.content.0,
                publish_datetime: row.publish_datetime.with_timezone(&Local),
                edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
                is_visible: row.is_visible,
                thumb_id: row.thumb_id,
                images: row.imgs.into_iter().map(Into::into).collect(),
                external_rels: row.external_rels,
                operator_ids: row.operator_ids,
                region_ids: row.region_ids,
            }
        }))
}

pub(crate) async fn insert_news(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    change: requests::ChangeNewsItem,
) -> Result<i32> {
    let publish_datetime = change.publish_datetime.unwrap_or(Local::now());

    let thumb_url: Option<String> = if let Some(thumb_id) = change.thumb_id {
        Some(get_item_thumb_url(transaction, thumb_id).await?)
    } else {
        None
    };

    let row = sqlx::query!(
        r#"
INSERT INTO news_items (title, summary, author_id, author_override, content,
    publish_datetime, edit_datetime, is_visible, thumb_id, thumb_url)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
RETURNING id"#,
        change.title,
        change.summary,
        change.author_id,
        change.author_override,
        json!(&change.content),
        publish_datetime,
        change.edit_datetime,
        change.is_visible,
        change.thumb_id,
        thumb_url
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), change = ?change);
        Error::DatabaseExecution
    })?;

    let id = row.id;

    // These functions can be optimized to do bulk inserts but big fat whatever
    for operator_id in &change.operator_ids {
        insert_news_item_operator(transaction, id, *operator_id).await?;
    }

    for region_id in &change.region_ids {
        insert_news_item_region(transaction, id, *region_id).await?;
    }

    for external_id in &change.external_ids {
        insert_news_item_external_news_item(transaction, id, *external_id)
            .await?;
    }

    for img_id in change.get_linked_images() {
        insert_news_item_img(transaction, id, img_id).await?;
    }

    Ok(id)
}

pub(crate) async fn update_news_item(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    change: &requests::ChangeNewsItem,
) -> Result<()> {
    // This function has quite some slack for optimization
    let current = fetch_full_news_item(&mut **transaction, item_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let thumb_url: Option<String> = if let Some(thumb_id) = change.thumb_id {
        Some(get_item_thumb_url(transaction, thumb_id).await?)
    } else {
        None
    };

    sqlx::query!(
        r#"
UPDATE news_items
SET title=$1, summary=$2, author_id=$3, author_override=$4, content=$5,
    publish_datetime=$6, edit_datetime=$7, is_visible=$8, thumb_id=$9,
    thumb_url=$10
WHERE id=$11"#,
        change.title,
        change.summary,
        change.author_id,
        change.author_override,
        json!(change.content),
        change.publish_datetime,
        change.edit_datetime,
        change.is_visible,
        change.thumb_id,
        thumb_url,
        item_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, change = ?change);
        Error::DatabaseExecution
    })?;

    update_news_item_operators(transaction, item_id, &current, change).await?;
    update_news_item_regions(transaction, item_id, &current, change).await?;
    update_news_item_external(transaction, item_id, &current, change).await?;
    update_news_item_imgs(transaction, item_id, &current, change).await?;

    Ok(())
}

pub(crate) async fn update_news_item_operators(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    current: &responses::FullNewsItem,
    change: &requests::ChangeNewsItem,
) -> Result<()> {
    let deleted_operator_ids = current
        .operator_ids
        .iter()
        .filter(|operator_id| !change.operator_ids.contains(operator_id))
        .copied()
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
DELETE FROM news_items_operators
WHERE item_id=$1 AND operator_id = ANY($2)"#,
        item_id,
        &deleted_operator_ids[..]
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            item_id,
            deleted_operator_ids = format!("{:?}", deleted_operator_ids)
        );
        Error::DatabaseExecution
    })?;

    for operator_id in &change.operator_ids {
        if !current.operator_ids.contains(operator_id) {
            insert_news_item_operator(transaction, item_id, *operator_id)
                .await?;
        }
    }

    Ok(())
}

pub(crate) async fn update_news_item_regions(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    current: &responses::FullNewsItem,
    change: &requests::ChangeNewsItem,
) -> Result<()> {
    let deleted_region_ids = current
        .region_ids
        .iter()
        .filter(|region_id| !change.region_ids.contains(region_id))
        .copied()
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
DELETE FROM news_items_regions
WHERE item_id=$1 AND region_id = ANY($2)"#,
        item_id,
        &deleted_region_ids[..]
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            item_id,
            deleted_region_ids = format!("{:?}", deleted_region_ids)
        );
        Error::DatabaseExecution
    })?;

    for region_id in &change.region_ids {
        if !current.region_ids.contains(region_id) {
            insert_news_item_region(transaction, item_id, *region_id).await?;
        }
    }

    Ok(())
}

pub(crate) async fn update_news_item_external(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    current: &responses::FullNewsItem,
    change: &requests::ChangeNewsItem,
) -> Result<()> {
    let (common_external_ids, deleted_external_ids): (Vec<i32>, Vec<i32>) =
        current
            .external_rels
            .iter()
            .map(|rel| rel.id)
            .partition(|id| change.external_ids.contains(id));

    sqlx::query!(
        r#"
DELETE FROM news_items_external_news_items
WHERE item_id=$1 AND external_item_id = ANY($2)"#,
        item_id,
        &deleted_external_ids[..]
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            item_id,
            deleted_external_ids = format!("{:?}", deleted_external_ids)
        );
        Error::DatabaseExecution
    })?;

    for external_id in &change.external_ids {
        if !common_external_ids.contains(external_id) {
            insert_news_item_external_news_item(
                transaction,
                item_id,
                *external_id,
            )
            .await?;
        }
    }

    Ok(())
}

pub(crate) async fn update_news_item_imgs(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    current: &responses::FullNewsItem,
    change: &requests::ChangeNewsItem,
) -> Result<()> {
    let change_imgs = change.get_linked_images();
    let curr_imgs = current
        .images
        .iter()
        .map(|img| img.id)
        .collect::<HashSet<_>>();
    let new_imgs = change_imgs.difference(&curr_imgs);
    let deleted_imgs =
        curr_imgs.difference(&change_imgs).copied().collect_vec();

    sqlx::query!(
        r#"
DELETE FROM news_items_imgs
WHERE item_id=$1 AND img_id = ANY($2)"#,
        item_id,
        &deleted_imgs[..]
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            item_id,
            deleted_imgs = format!("{:?}", deleted_imgs)
        );
        Error::DatabaseExecution
    })?;

    for img_id in new_imgs {
        insert_news_item_img(transaction, item_id, *img_id).await?;
    }

    Ok(())
}

async fn get_item_thumb_url(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    thumb_id: Uuid,
) -> Result<String> {
    fetch_rich_img_sha1(transaction, thumb_id)
        .await?
        .ok_or(Error::ValidationFailure(
            "The referenced thumb_id does not exist".to_string(),
        ))
        .map(|sha1| get_rich_img_thumb_path(&sha1))
}

async fn fetch_rich_img_sha1(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    img_id: Uuid,
) -> Result<Option<String>> {
    // TODO move to pics/sql.rs
    sqlx::query!(r#"SELECT sha1 FROM rich_imgs WHERE id=$1"#, img_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), img_id = ?img_id);
            Error::DatabaseExecution
        })
        .map(|row| row.map(|row| row.sha1))
}

async fn insert_news_item_operator(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    operator_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO news_items_operators (item_id, operator_id)
VALUES ($1, $2)"#,
        item_id,
        operator_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, operator_id);
        Error::DatabaseExecution
    })?;
    Ok(())
}

async fn insert_news_item_region(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    region_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO news_items_regions (item_id, region_id)
VALUES ($1, $2)"#,
        item_id,
        region_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, region_id);
        Error::DatabaseExecution
    })?;
    Ok(())
}

async fn insert_news_item_external_news_item(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    external_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO news_items_external_news_items (item_id, external_item_id)
VALUES ($1, $2)"#,
        item_id,
        external_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, external_id);
        Error::DatabaseExecution
    })?;
    Ok(())
}

async fn insert_news_item_img(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    img_id: Uuid,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO news_items_imgs (item_id, img_id)
VALUES ($1, $2)"#,
        item_id,
        img_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, img_id = ?img_id);
        Error::DatabaseExecution
    })?;
    Ok(())
}

pub(crate) async fn fetch_external_news_item(
    pool: &PgPool,
    item_id: i32,
    incl_private: bool,
) -> Result<Option<responses::ExternalNewsItem>> {
    Ok(sqlx::query!(
        r#"
SELECT external_news_items.id, title, summary, author,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    COALESCE(content_md, prepro_content_md) as content_md,
    COALESCE(content_text, prepro_content_text) as content_text,
    edit_datetime, publish_datetime, source, url,
    is_complete, is_validated, is_relevant, is_sensitive,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(sha1, transcript, has_copyright_issues))
        ELSE array[]::record[]
    END as "imgs!: Vec<(String, Option<String>, Option<bool>)>"
FROM external_news_items
LEFT JOIN external_news_items_imgs
    ON external_news_items.id=external_news_items_imgs.item_id
LEFT JOIN external_news_imgs
    ON external_news_items_imgs.img_id=external_news_imgs.id
LEFT JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
LEFT JOIN external_news_items_regions
    ON external_news_items.id=external_news_items_regions.item_id
WHERE external_news_items.id=$1
    AND (NOT has_copyright_issues OR $2)
GROUP BY external_news_items.id
"#,
        item_id,
        incl_private,
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id);
        Error::DatabaseExecution
    })?
    .map(|row| responses::ExternalNewsItem {
        id: row.id,
        title: row.title,
        summary: row.summary,
        author: row.author,
        content_md: row.content_md,
        content_text: row.content_text,
        publish_datetime: row.publish_datetime.with_timezone(&Local),
        edit_datetime: row
            .edit_datetime
            .map(|datetime| datetime.with_timezone(&Local)),
        source: row.source,
        url: row.url,
        is_complete: row.is_complete,
        is_validated: row.is_validated,
        is_relevant: row.is_relevant,
        is_sensitive: row.is_sensitive,
        operator_ids: row.operator_ids,
        region_ids: row.region_ids,
        images: row
            .imgs
            .into_iter()
            .map(|(sha1, transcript, has_copyright_issues)| {
                pic_models::responses::ExternalNewsImg {
                    transcript,
                    url: if has_copyright_issues == Some(false) || incl_private {
                        Some(get_external_news_pic_path(sha1.as_ref()))
                    } else {
                        None
                    },
                }
            })
            .collect::<Vec<_>>(),
    }))
}

pub(crate) async fn fetch_full_external_news_item(
    pool: &PgPool,
    item_id: i32,
) -> Result<Option<responses::FullExternalNewsItem>> {
    Ok(sqlx::query!(
        r#"
SELECT external_news_items.id, title, summary, author,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    content_md, prepro_content_md, content_text, prepro_content_text,
    edit_datetime, publish_datetime, source, url, is_complete,
    is_validated, is_relevant, is_sensitive, raw, ss_sha1,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(external_news_imgs.id, sha1, has_copyright_issues, transcript))
        ELSE array[]::record[]
    END as "imgs!: Vec<pic_models::ExternalNewsImg>"
FROM external_news_items
LEFT JOIN external_news_items_imgs
    ON external_news_items.id=external_news_items_imgs.item_id
LEFT JOIN external_news_imgs
    ON external_news_items_imgs.img_id=external_news_imgs.id
LEFT JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
LEFT JOIN external_news_items_regions
    ON external_news_items.id=external_news_items_regions.item_id
WHERE external_news_items.id=$1
GROUP BY external_news_items.id
"#,
        item_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id);
        Error::DatabaseExecution
    })?
    .map(|row| responses::FullExternalNewsItem {
        id: row.id,
        title: row.title,
        summary: row.summary,
        author: row.author,
        prepro_content_md: row.prepro_content_md,
        prepro_content_text: row.prepro_content_text,
        content_md: row.content_md,
        content_text: row.content_text,
        publish_datetime: row.publish_datetime.with_timezone(&Local),
        edit_datetime: row
            .edit_datetime
            .map(|datetime| datetime.with_timezone(&Local)),
        source: row.source,
        url: row.url,
        raw: row.raw,
        is_complete: row.is_complete,
        is_validated: row.is_validated,
        is_relevant: row.is_relevant,
        is_sensitive: row.is_sensitive,
        images: row.imgs.into_iter().map(Into::into).collect(),
        operator_ids: row.operator_ids,
        region_ids: row.region_ids,
        screenshot_url: row
            .ss_sha1
            .as_ref()
            .map(|sha1| get_external_news_ss_path(sha1)),
    }))
}

pub(crate) async fn fetch_external_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
    incl_private: bool,
) -> Result<Vec<responses::ExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, title, author, summary,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    COALESCE(content_md, prepro_content_md) as content_md,
    COALESCE(content_text, prepro_content_text) as content_text,
    publish_datetime, edit_datetime, source, url,
    is_complete, is_validated, is_relevant, is_sensitive,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(sha1, transcript, has_copyright_issues))
        ELSE array[]::record[]
    END as "imgs!: Vec<(String, Option<String>, Option<bool>)>"
FROM external_news_items
LEFT JOIN external_news_items_imgs ON external_news_items.id=external_news_items_imgs.item_id
LEFT JOIN external_news_imgs ON external_news_items_imgs.img_id=external_news_imgs.id
LEFT JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
LEFT JOIN external_news_items_regions
    ON external_news_items.id=external_news_items_regions.item_id
WHERE ($1 OR (is_validated AND NOT is_sensitive))
GROUP BY external_news_items.id
LIMIT $2 OFFSET $3
"#,
        incl_private,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), incl_private, take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::ExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            content_md: row.content_md,
            content_text: row.content_text,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            is_complete: row.is_complete,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            operator_ids: row.operator_ids,
            region_ids: row.region_ids,
            images: row
                .imgs
                .into_iter()
                .map(|(sha1, transcript, has_copyright_issues)| {
                    pic_models::responses::ExternalNewsImg {
                        transcript,
                        url: if has_copyright_issues == Some(false)
                            || incl_private
                        {
                            Some(get_external_news_pic_path(sha1.as_ref()))
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

pub(crate) async fn count_external_news(
    pool: &PgPool,
    incl_private: bool,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as "cnt!: i64"
FROM external_news_items
WHERE ($1 OR (is_validated AND NOT is_sensitive))
"#,
        incl_private
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), incl_private);
        Error::DatabaseExecution
    })?
    .map_or(0, |row| row.cnt))
}

pub(crate) async fn fetch_operator_external_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
    incl_private: bool,
) -> Result<Vec<responses::ExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, title, author, summary,
    array_agg(distinct external_news_items_operators.operator_id) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    COALESCE(content_md, prepro_content_md) as content_md,
    COALESCE(content_text, prepro_content_text) as content_text,
    publish_datetime, edit_datetime, source, url,
    is_complete, is_validated, is_relevant, is_sensitive,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(sha1, transcript, has_copyright_issues))
        ELSE array[]::record[]
    END as "imgs!: Vec<(String, Option<String>, Option<bool>)>"
FROM external_news_items
LEFT JOIN external_news_items_imgs
    ON external_news_items.id=external_news_items_imgs.item_id
LEFT JOIN external_news_imgs
    ON external_news_items_imgs.img_id=external_news_imgs.id
JOIN external_news_items_operators AS rel ON external_news_items.id=rel.item_id
JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
LEFT JOIN external_news_items_regions
    ON external_news_items.id=external_news_items_regions.item_id
WHERE rel.operator_id=$1
    AND ($2 OR (is_validated AND NOT is_sensitive))
GROUP BY external_news_items.id
LIMIT $3 OFFSET $4
"#,
        operator_id,
        incl_private,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            operator_id,
            incl_private,
            take,
            skip
        );
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::ExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            content_md: row.content_md,
            content_text: row.content_text,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row
                .edit_datetime
                .map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            is_complete: row.is_complete,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            operator_ids: row.operator_ids,
            region_ids: row.region_ids,
            images: row
                .imgs
                .into_iter()
                .map(|(sha1, transcript, has_copyright_issues)| {
                    pic_models::responses::ExternalNewsImg {
                        transcript,
                        url: if has_copyright_issues == Some(false)
                            || incl_private
                        {
                            Some(get_external_news_pic_path(sha1.as_ref()))
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

pub(crate) async fn count_operator_external_news(
    pool: &PgPool,
    operator_id: i32,
    incl_private: bool,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as "cnt!: i64"
FROM external_news_items
LEFT JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
WHERE operator_id=$1
    AND ($2 OR (is_validated AND NOT is_sensitive))
"#,
        operator_id,
        incl_private,
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id, incl_private);
        Error::DatabaseExecution
    })?
    .map_or(0, |row| row.cnt))
}

pub(crate) async fn fetch_pending_external_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::ExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, title, summary, author,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    content_md, content_text, publish_datetime, edit_datetime, source, url,
    is_complete, is_validated, is_relevant, is_sensitive,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(external_news_imgs.id, sha1, has_copyright_issues, transcript))
        ELSE array[]::record[]
    END as "imgs!: Vec<pic_models::ExternalNewsImg>"
FROM external_news_items
LEFT JOIN external_news_items_imgs
    ON external_news_items.id=external_news_items_imgs.item_id
LEFT JOIN external_news_imgs
    ON external_news_items_imgs.img_id=external_news_imgs.id
LEFT JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
LEFT JOIN external_news_items_regions
    ON external_news_items.id=external_news_items_regions.item_id
WHERE NOT is_validated
GROUP BY external_news_items.id
LIMIT $1 OFFSET $2
"#,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::ExternalNewsItem {
            id: row.id,
            title: row.title,
            summary: row.summary,
            author: row.author,
            content_md: row.content_md,
            content_text: row.content_text,
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row
                .edit_datetime
                .map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            is_complete: row.is_complete,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            operator_ids: row.operator_ids,
            region_ids: row.region_ids,
            images: row.imgs.into_iter().map(Into::into).collect(),
        })
    })
    .collect()
}

pub(crate) async fn count_pending_external_news(pool: &PgPool) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as "cnt!: i64"
FROM external_news_items
WHERE NOT is_validated
"#,
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .map_or(0, |row| row.cnt))
}

pub(crate) async fn fetch_pending_operator_external_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::FullExternalNewsItem>> {
    sqlx::query!(
        r#"
SELECT external_news_items.id, title, summary, author, content_md,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    prepro_content_md, content_text, prepro_content_text, publish_datetime, edit_datetime,
    source, url, is_complete, is_validated, is_relevant, is_sensitive, ss_sha1, raw,
    CASE
        WHEN count(external_news_imgs.id) > 0
        THEN array_agg(
            ROW(external_news_imgs.id, sha1, has_copyright_issues, transcript))
        ELSE array[]::record[]
    END as "imgs!: Vec<pic_models::ExternalNewsImg>"
FROM external_news_items
LEFT JOIN external_news_items_imgs
    ON external_news_items.id=external_news_items_imgs.item_id
LEFT JOIN external_news_imgs
    ON external_news_items_imgs.img_id=external_news_imgs.id
JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
LEFT JOIN external_news_items_regions
    ON external_news_items.id=external_news_items_regions.item_id
WHERE operator_id=$1 AND NOT is_validated
GROUP BY external_news_items.id
LIMIT $2 OFFSET $3
"#,
        operator_id,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), operator_id, take, skip);
        Error::DatabaseExecution
    })?
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
            publish_datetime: row.publish_datetime.with_timezone(&Local),
            edit_datetime: row.edit_datetime.map(|datetime| datetime.with_timezone(&Local)),
            source: row.source,
            url: row.url,
            raw: row.raw,
            is_complete: row.is_complete,
            is_validated: row.is_validated,
            is_relevant: row.is_relevant,
            is_sensitive: row.is_sensitive,
            operator_ids: row.operator_ids,
            region_ids: row.region_ids,
            images: row.imgs.into_iter().map(Into::into).collect(),
            screenshot_url: row.ss_sha1.as_ref().map(|sha1| get_external_news_ss_path(sha1)),
        })
    })
    .collect()
}

pub(crate) async fn count_pending_operator_external_news(
    pool: &PgPool,
    operator_id: i32,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as "cnt!: i64"
FROM external_news_items
LEFT JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
WHERE operator_id=$1 AND NOT is_validated
"#,
        operator_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })?
    .map_or(0, |row| row.cnt))
}

pub(crate) async fn insert_external_news(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    change: requests::NewExternalNewsItem,
) -> Result<i32> {
    let row = sqlx::query!(
        r#"
INSERT INTO external_news_items (title, summary, author,
    prepro_content_md, prepro_content_text, publish_datetime, edit_datetime,
    source, url, is_complete, raw)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
RETURNING id"#,
        change.title,
        change.summary,
        change.author,
        change.prepro_content_md,
        change.prepro_content_text,
        change.publish_datetime,
        change.edit_datetime,
        change.source,
        change.url,
        change.is_complete,
        change.raw
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), change = ?change);
        Error::DatabaseExecution
    })?;

    let id = row.id;

    for operator_id in change.operator_ids {
        sqlx::query!(
            r#"
INSERT INTO external_news_items_operators (item_id, operator_id)
VALUES ($1, $2)"#,
            id,
            operator_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), id, operator_id);
            Error::DatabaseExecution
        })?;
    }

    for region_id in change.region_ids {
        sqlx::query!(
            r#"
INSERT INTO external_news_items_regions (item_id, region_id)
VALUES ($1, $2)"#,
            id,
            region_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), id, region_id);
            Error::DatabaseExecution
        })?;
    }

    Ok(id)
}

pub(crate) async fn update_external_news_item(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    change: requests::ChangeExternalNewsItem,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE external_news_items
SET title=$1, summary=$2, author=$3, content_md=$4,
    publish_datetime=$5, edit_datetime=$6, url=$7,
    is_complete=$8, is_relevant=$9, is_sensitive=$10, is_validated=$11
WHERE id=$12"#,
        change.title,
        change.summary,
        change.author,
        change.content_md,
        change.publish_datetime,
        change.edit_datetime,
        change.url,
        change.is_complete,
        change.is_relevant,
        change.is_sensitive,
        change.is_validated,
        item_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), change = ?change);
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        r#"
DELETE FROM external_news_items_operators
WHERE item_id=$1"#,
        item_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id);
        Error::DatabaseExecution
    })?;

    for operator_id in change.operator_ids {
        sqlx::query!(
            r#"
INSERT INTO external_news_items_operators (item_id, operator_id)
VALUES ($1, $2)"#,
            item_id,
            operator_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), item_id, operator_id);
            Error::DatabaseExecution
        })?;
    }

    sqlx::query!(
        r#"
DELETE FROM external_news_items_regions
WHERE item_id=$1"#,
        item_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id);
        Error::DatabaseExecution
    })?;

    for region_id in change.region_ids {
        sqlx::query!(
            r#"
INSERT INTO external_news_items_regions (item_id, region_id)
VALUES ($1, $2)"#,
            item_id,
            region_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), item_id, region_id);
            Error::DatabaseExecution
        })?;
    }

    Ok(())
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
    .map_err(|err| {
        tracing::error!(error = err.to_string(), id);
        Error::DatabaseExecution
    })?;
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
    .map_err(|err| {
        tracing::error!(error = err.to_string(), source);
        Error::DatabaseExecution
    })?
    .into_iter()
    .filter_map(|row| row.url)
    .collect())
}

pub(crate) async fn fetch_external_news_source_dump(
    pool: &PgPool,
    source: &str,
) -> Result<Vec<responses::SourceExternalNewsItem>> {
    sqlx::query_as!(
        responses::SourceExternalNewsItem,
        r#"
SELECT id, title, summary, author,
    array_remove(array_agg(distinct operator_id), NULL) as "operator_ids!: Vec<i32>",
    array_remove(array_agg(distinct region_id), NULL) as "region_ids!: Vec<i32>",
    prepro_content_md, prepro_content_text,
    publish_datetime, edit_datetime, source, url, is_complete, raw
FROM external_news_items
JOIN external_news_items_operators
    ON external_news_items.id=external_news_items_operators.item_id
JOIN external_news_items_regions
    ON external_news_items.id=external_news_items_regions.item_id
WHERE source=$1
GROUP BY external_news_items.id
"#,
        source,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), source);
        Error::DatabaseExecution
    })
}
