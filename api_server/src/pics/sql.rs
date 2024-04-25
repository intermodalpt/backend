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

use std::collections::HashMap;

use chrono::Local;
use itertools::Itertools;
use sqlx::PgPool;

use commons::models::pics;

use super::{models::requests, models::responses};
use crate::pics::{get_full_path, get_medium_path, get_thumb_path};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

/// Fetches a picture by its id.
pub(crate) async fn fetch_picture<'c, E>(
    executor: E,
    pic_id: i32,
) -> Result<Option<pics::StopPic>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query!(
        r#"
SELECT id, original_filename, sha1, tagged, public, sensitive, uploader,
    upload_date, capture_date, lon, lat, quality, width,
    height, camera_ref, tags, attrs, notes
FROM stop_pics
WHERE id = $1
"#,
        pic_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pic_id);
        Error::DatabaseExecution
    })
    .map(|row| {
        row.map(|row| pics::StopPic {
            id: row.id,
            original_filename: row.original_filename,
            sha1: row.sha1,
            tagged: row.tagged,
            uploader: row.uploader,
            upload_date: row.upload_date,
            capture_date: row.capture_date,
            updater: None,
            update_date: None,
            width: row.width,
            height: row.height,
            camera_ref: row.camera_ref,
            dyn_meta: pics::StopPicDynMeta {
                public: row.public,
                sensitive: row.sensitive,
                lon: row.lon,
                lat: row.lat,
                quality: row.quality,
                tags: row.tags,
                attrs: row.attrs,
                notes: row.notes,
            },
        })
    })
}

/// Fetches a picture by its hash.
pub(crate) async fn fetch_picture_by_hash(
    pool: &PgPool,
    pic_hash: &str,
) -> Result<Option<pics::StopPic>> {
    sqlx::query!(
        r#"
SELECT id, original_filename, sha1, tagged, public, sensitive, uploader,
    upload_date, capture_date, lon, lat, quality, width,
    height, camera_ref, tags, attrs, notes
FROM stop_pics
WHERE sha1 = $1
"#,
        pic_hash
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pic_hash);
        Error::DatabaseExecution
    })
    .map(|row| {
        row.map(|row| pics::StopPic {
            id: row.id,
            original_filename: row.original_filename,
            sha1: row.sha1,
            tagged: row.tagged,
            uploader: row.uploader,
            upload_date: row.upload_date,
            capture_date: row.capture_date,
            updater: None,
            update_date: None,
            width: row.width,
            height: row.height,
            camera_ref: row.camera_ref,
            dyn_meta: pics::StopPicDynMeta {
                public: row.public,
                sensitive: row.sensitive,
                lon: row.lon,
                lat: row.lat,
                quality: row.quality,
                tags: row.tags,
                attrs: row.attrs,
                notes: row.notes,
            },
        })
    })
}

/// A specific picture and its stops
pub(crate) async fn fetch_picture_with_stops(
    pool: &PgPool,
    pic_id: i32,
) -> Result<Option<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.lon, stop_pics.lat,
    stop_pics.quality, stop_pics.width, stop_pics.height, stop_pics.camera_ref,
    stop_pics.tags, stop_pics.attrs, stop_pics.notes, stop_pics.tagged,
    CASE
        WHEN count(stop_pic_stops.stop) > 0
        THEN array_agg(ROW(stop_pic_stops.stop, stop_pic_stops.attrs))
        ELSE array[]::record[]
    END as "rels!: Vec<(i32, Vec<String>)>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.id = $1
GROUP BY stop_pics.id
"#,
        pic_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pic_id);
        Error::DatabaseExecution
    })?
    .map(|r| responses::PicWithStops {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
        original_filename: r.original_filename,
        sha1: r.sha1,
        tagged: r.tagged,
        uploader: r.uploader,
        upload_date: r.upload_date,
        capture_date: r.capture_date,
        width: r.width,
        height: r.height,
        camera_ref: r.camera_ref,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        quality: r.quality,
        tags: r.tags,
        attrs: r.attrs,
        notes: r.notes,
        stops: r.rels.into_iter().map(Into::into).collect(),
    }))
}

/// Every picture and its stops
pub(crate) async fn fetch_pictures_with_stops(
    pool: &PgPool,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.lon, stop_pics.lat,
    stop_pics.quality, stop_pics.width, stop_pics.height, stop_pics.camera_ref,
    stop_pics.tags, stop_pics.attrs, stop_pics.notes, stop_pics.tagged,
    CASE
        WHEN count(stop_pic_stops.stop) > 0
        THEN array_agg(ROW(stop_pic_stops.stop, stop_pic_stops.attrs))
        ELSE array[]::record[]
    END as "rels!: Vec<(i32, Vec<String>)>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
GROUP BY stop_pics.id
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| responses::PicWithStops {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
        original_filename: r.original_filename,
        sha1: r.sha1,
        tagged: r.tagged,
        uploader: r.uploader,
        upload_date: r.upload_date,
        capture_date: r.capture_date,
        width: r.width,
        height: r.height,
        camera_ref: r.camera_ref,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        quality: r.quality,
        tags: r.tags,
        attrs: r.attrs,
        notes: r.notes,
        stops: r.rels.into_iter().map(Into::into).collect(),
    })
    .collect())
}

/// Ids, URLs and stops for every picture
pub(crate) async fn fetch_minimal_pictures_with_stops(
    pool: &PgPool,
    trusted: bool,
    uid: Option<i32>,
) -> Result<Vec<responses::MinimalPicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.public, stop_pics.sensitive,
    stop_pics.lon, stop_pics.lat, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.uploader = $1
    OR (stop_pics.public AND NOT stop_pics.sensitive)
    OR $2
GROUP BY stop_pics.id
"#,
        uid,
        trusted
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), uid, trusted);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| responses::MinimalPicWithStops {
        id: r.id,
        tagged: r.tagged,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        stops: r.stops,
    })
    .collect())
}

/// All the stops that are linked to a picture
pub(crate) async fn fetch_picture_stops_rel_attrs(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    pic_id: i32,
) -> Result<Vec<pics::StopAttrs>> {
    Ok(sqlx::query!(
        r#"
SELECT stop, attrs
FROM stop_pic_stops
WHERE pic = $1
ORDER BY stop ASC
"#,
        pic_id
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pic_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| pics::StopAttrs {
        id: r.stop,
        attrs: r.attrs,
    })
    .collect())
}

/// All the pictures that are attached to a stop and meant to be public
pub(crate) async fn fetch_public_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<responses::PublicStopPic>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.sha1, stop_pics.capture_date, stop_pics.lon, stop_pics.lat,
    stop_pics.tags, stop_pics.attrs, stop_pics.quality
FROM stop_pics
JOIN stop_pic_stops on stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.tagged AND NOT stop_pics.sensitive
    AND stop_pics.public AND stop_pic_stops.stop=$1
ORDER BY stop_pics.capture_date DESC
    "#,
        stop_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
        tracing::error!(error=err.to_string(), stop_id);
        Error::DatabaseExecution})?
        .into_iter()
        .map(|r| responses::PublicStopPic {
            id: r.id,
            url_full: get_full_path(&r.sha1),
            url_medium: get_medium_path(&r.sha1),
            url_thumb: get_thumb_path(&r.sha1),
            sha1: r.sha1,
            capture_date: r.capture_date,
            lon: r.lon,
            lat: r.lat,
            tags: r.tags,
            attrs: r.attrs,
            quality: r.quality,
        }).collect()
    )
}

/// All the pictures that are attached to a stop and visible to the user
pub(crate) async fn fetch_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
    trusted: bool,
    uid: Option<i32>,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.attrs, stop_pics.notes, stop_pics.tagged,
    CASE
        WHEN count(stop_pic_stops.stop) > 0
        THEN array_agg(ROW(stop_pic_stops.stop, stop_pic_stops.attrs))
        ELSE array[]::record[]
    END as "rels!: Vec<(i32, Vec<String>)>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pic_stops.stop=$1
    AND (stop_pics.uploader = $2
        OR (stop_pics.public AND NOT stop_pics.sensitive)
        OR $3)
GROUP BY stop_pics.id
ORDER BY quality DESC
    "#,
        stop_id,
        uid,
        trusted
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), stop_id, uid, trusted);
        Error::DatabaseExecution})?
    .into_iter()
    .map(|r| responses::PicWithStops {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
        original_filename: r.original_filename,
        sha1: r.sha1,
        tagged: r.tagged,
        uploader: r.uploader,
        upload_date: r.upload_date,
        capture_date: r.capture_date,
        width: r.width,
        height: r.height,
        camera_ref: r.camera_ref,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        quality: r.quality,
        tags: r.tags,
        attrs: r.attrs,
        notes: r.notes,
        stops: r.rels.into_iter().map(Into::into).collect()
    })
    .collect())
}

/// A range of pictures that have been uploaded by a user
pub(crate) async fn fetch_user_pictures(
    pool: &PgPool,
    user_id: i32,
    include_private: bool,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.attrs, stop_pics.notes, stop_pics.tagged,
    CASE
        WHEN count(stop_pic_stops.stop) > 0
        THEN array_agg(ROW(stop_pic_stops.stop, stop_pic_stops.attrs))
        ELSE array[]::record[]
    END as "rels!: Vec<(i32, Vec<String>)>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE uploader=$1
    AND ((stop_pics.public AND NOT stop_pics.sensitive) OR $4)
GROUP BY stop_pics.id
ORDER BY capture_date DESC, upload_date DESC
LIMIT $2 OFFSET $3
    "#,
        user_id,
        take,
        skip,
        include_private
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(
            error=err.to_string(),
            user_id,
            take,
            skip,
            include_private
        );
        Error::DatabaseExecution})?
    .into_iter()
    .map(|r| responses::PicWithStops {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
        original_filename: r.original_filename,
        sha1: r.sha1,
        tagged: r.tagged,
        uploader: r.uploader,
        upload_date: r.upload_date,
        capture_date: r.capture_date,
        width: r.width,
        height: r.height,
        camera_ref: r.camera_ref,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        quality: r.quality,
        tags: r.tags,
        attrs: r.attrs,
        notes: r.notes,
        stops: r.rels.into_iter().map(Into::into).collect()
    })
    .collect())
}

/// A range of pictures that are visible to the user
pub(crate) async fn fetch_latest_pictures(
    pool: &PgPool,
    trusted: bool,
    uid: Option<i32>,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.attrs, stop_pics.notes, stop_pics.tagged,
    CASE
        WHEN count(stop_pic_stops.stop) > 0
        THEN array_agg(ROW(stop_pic_stops.stop, stop_pic_stops.attrs))
        ELSE array[]::record[]
    END as "rels!: Vec<(i32, Vec<String>)>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.uploader = $1
    OR (stop_pics.public AND NOT stop_pics.sensitive)
    OR $2
GROUP BY stop_pics.id
ORDER BY capture_date DESC, upload_date DESC
LIMIT $3 OFFSET $4
    "#,
        uid,
        trusted,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), uid, trusted, take, skip);
        Error::DatabaseExecution})?
    .into_iter()
    .map(|r| responses::PicWithStops {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
        original_filename: r.original_filename,
        sha1: r.sha1,
        tagged: r.tagged,
        uploader: r.uploader,
        upload_date: r.upload_date,
        capture_date: r.capture_date,
        width: r.width,
        height: r.height,
        camera_ref: r.camera_ref,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        quality: r.quality,
        tags: r.tags,
        attrs: r.attrs,
        notes: r.notes,
        stops: r.rels.into_iter().map(Into::into).collect()
    })
    .collect())
}

/// A range of pictures that are tagged and are visible to the user
pub(crate) async fn fetch_tagged_pictures(
    pool: &PgPool,
    trusted: bool,
    uid: Option<i32>,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.attrs, stop_pics.notes, stop_pics.tagged,
    CASE
        WHEN count(stop_pic_stops.stop) > 0
        THEN array_agg(ROW(stop_pic_stops.stop, stop_pic_stops.attrs))
        ELSE array[]::record[]
    END as "rels!: Vec<(i32, Vec<String>)>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE tagged
    AND (stop_pics.uploader = $1
        OR (stop_pics.public AND NOT stop_pics.sensitive)
        OR $2)
GROUP BY stop_pics.id
ORDER BY capture_date DESC, upload_date DESC
LIMIT $3 OFFSET $4
    "#,
        uid,
        trusted,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), uid, trusted, take, skip);
        Error::DatabaseExecution})?
    .into_iter()
    .map(|r| responses::PicWithStops {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
        original_filename: r.original_filename,
        sha1: r.sha1,
        tagged: r.tagged,
        uploader: r.uploader,
        upload_date: r.upload_date,
        capture_date: r.capture_date,
        width: r.width,
        height: r.height,
        camera_ref: r.camera_ref,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        quality: r.quality,
        tags: r.tags,
        attrs: r.attrs,
        notes: r.notes,
        stops: r.rels.into_iter().map(Into::into).collect()
    })
    .collect())
}

/// A range of pictures that are not tagged and are visible to the user
pub(crate) async fn fetch_untagged_pictures(
    pool: &PgPool,
    trusted: bool,
    uid: Option<i32>,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.attrs, stop_pics.notes, stop_pics.tagged,
    CASE
        WHEN count(stop_pic_stops.stop) > 0
        THEN array_agg(ROW(stop_pic_stops.stop, stop_pic_stops.attrs))
        ELSE array[]::record[]
    END as "rels!: Vec<(i32, Vec<String>)>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE NOT tagged
    AND (stop_pics.uploader = $1
        OR (stop_pics.public AND NOT stop_pics.sensitive)
        OR $2)
GROUP BY stop_pics.id
ORDER BY capture_date ASC, upload_date ASC
LIMIT $3 OFFSET $4
    "#,
        uid,
        trusted,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), uid, trusted, take, skip);
        Error::DatabaseExecution})?
    .into_iter()
    .map(|r| responses::PicWithStops {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
        original_filename: r.original_filename,
        sha1: r.sha1,
        tagged: r.tagged,
        uploader: r.uploader,
        upload_date: r.upload_date,
        capture_date: r.capture_date,
        width: r.width,
        height: r.height,
        camera_ref: r.camera_ref,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        quality: r.quality,
        tags: r.tags,
        attrs: r.attrs,
        notes: r.notes,
        stops: r.rels.into_iter().map(Into::into).collect()
    })
    .collect())
}

pub(crate) async fn fetch_picture_stop_rels(
    pool: &PgPool,
) -> Result<HashMap<i32, Vec<i32>>> {
    let res = sqlx::query!(
        r#"
SELECT stop, pic
FROM stop_pic_stops
ORDER BY stop ASC
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?;

    let mut stops = HashMap::<i32, Vec<i32>>::new();

    for row in res {
        if let Some(pics) = stops.get_mut(&row.stop) {
            pics.push(row.pic);
        } else {
            stops.insert(row.stop, vec![row.pic]);
        }
    }
    Ok(stops)
}

/// A range of pictures that are not positioned and are visible to the user
pub(crate) async fn fetch_unpositioned_pictures(
    pool: &PgPool,
    trusted: bool,
    uid: Option<i32>,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::MinimalPic>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.sha1
FROM stop_pics
WHERE (stop_pics.lat IS NULL OR stop_pics.lon IS NULL)
    AND (stop_pics.uploader = $1
        OR (stop_pics.public AND NOT stop_pics.sensitive)
        OR $2)
ORDER BY capture_date ASC, upload_date ASC
LIMIT $3 OFFSET $4
    "#,
        uid,
        trusted,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), uid, trusted, take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| responses::MinimalPic {
        id: r.id,
        url_full: get_full_path(&r.sha1),
        url_medium: get_medium_path(&r.sha1),
        url_thumb: get_thumb_path(&r.sha1),
    })
    .collect())
}

pub(crate) async fn fetch_public_picture_stop_rels(
    pool: &PgPool,
) -> Result<HashMap<i32, Vec<i32>>> {
    let res = sqlx::query!(
        r#"
SELECT stop_pic_stops.stop, stop_pic_stops.pic
FROM stop_pic_stops
JOIN stop_pics ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.public
ORDER BY stop ASC
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?;

    let mut stops = HashMap::<i32, Vec<i32>>::new();

    for row in res {
        if let Some(pics) = stops.get_mut(&row.stop) {
            pics.push(row.pic);
        } else {
            stops.insert(row.stop, vec![row.pic]);
        }
    }
    Ok(stops)
}

pub(crate) async fn insert_stop_pic(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mut pic: pics::StopPic,
    stops: &[i32],
) -> Result<pics::StopPic> {
    let res = sqlx::query!(
        r#"
INSERT INTO stop_pics(
    original_filename, sha1, public, sensitive, tagged, uploader,
    upload_date, capture_date, width, height, lat, lon, camera_ref
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
RETURNING id
        "#,
        pic.original_filename,
        pic.sha1,
        pic.dyn_meta.public,
        pic.dyn_meta.sensitive,
        pic.tagged,
        pic.uploader,
        pic.upload_date,
        pic.capture_date,
        pic.width,
        pic.height,
        pic.dyn_meta.lat,
        pic.dyn_meta.lon,
        pic.camera_ref
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            pic = ?pic
        );
        Error::DatabaseExecution
    })?;

    pic.id = res.id;

    for stop_id in stops {
        let _res = sqlx::query!(
            r#"
INSERT INTO stop_pic_stops(pic, stop)
VALUES ($1, $2)
ON CONFLICT DO NOTHING
    "#,
            pic.id,
            stop_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), pic_id = pic.id, stop_id);
            Error::DatabaseExecution
        })?;
    }

    Ok(pic)
}

pub(crate) async fn update_picture_meta(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stop_pic_id: i32,
    stop_pic_meta: requests::ChangeStopPic,
    user_id: i32,
) -> Result<()> {
    // TODO get rid of this
    let update_date = Local::now().to_string();

    let _res = sqlx::query!(
        r#"
UPDATE stop_pics
SET public=$1, sensitive=$2, lon=$3, lat=$4, tags=$5, attrs=$6, quality=$7,
    updater=$8, update_date=$9, tagged=true
WHERE id=$10
    "#,
        stop_pic_meta.public,
        stop_pic_meta.sensitive,
        stop_pic_meta.lon,
        stop_pic_meta.lat,
        &stop_pic_meta.tags,
        &stop_pic_meta.attrs,
        stop_pic_meta.quality,
        user_id,
        update_date,
        stop_pic_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error=err.to_string(),
            stop_pic_id,
            stop_pic_meta=?stop_pic_meta,
            user_id,
            update_date
        );
        Error::DatabaseExecution
    })?;

    if !stop_pic_meta.stops.is_empty() {
        // TODO add updater and update date
        let stop_ids = stop_pic_meta.stops.iter().map(|rel| rel.id).join(",");

        let _res = sqlx::query(&format!(
            r#"
    DELETE FROM stop_pic_stops
    WHERE pic=$1 AND stop NOT IN ({stop_ids})
        "#
        ))
        .bind(stop_pic_id)
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), stop_ids, stop_pic_id);
            Error::DatabaseExecution
        })?;

        for stop_rel in stop_pic_meta.stops {
            let _res = sqlx::query!(
                r#"
INSERT INTO stop_pic_stops(pic, stop, attrs)
VALUES ($1, $2, $3)
ON CONFLICT (pic, stop)
DO UPDATE SET attrs = EXCLUDED.attrs
    "#,
                stop_pic_id,
                stop_rel.id,
                &stop_rel.attrs
            )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(
                    error = err.to_string(),
                    stop_id = stop_rel.id,
                    attrs = stop_rel.attrs.join(",")
                );
                Error::DatabaseExecution
            })?;
        }
    }

    Ok(())
}

pub(crate) async fn delete_picture(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    pic_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM stop_pic_stops
WHERE pic=$1"#,
        pic_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pic_id);
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        r#"
DELETE FROM stop_pics
WHERE id=$1
        "#,
        pic_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pic_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_picture_count_by_stop(
    pool: &PgPool,
) -> Result<HashMap<i32, i32>> {
    let res = sqlx::query!(
        r#"
SELECT stop_pic_stops.stop, count(*)::int as "pic_count!: i32"
FROM stop_pic_stops
GROUP BY stop_pic_stops.stop
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| (row.stop, row.pic_count))
    .collect();

    Ok(res)
}

/// Every panoramic picture
pub(crate) async fn fetch_panos(
    pool: &PgPool,
    allow_sensitive: bool,
) -> Result<Vec<responses::FullPanoPic>> {
    sqlx::query_as!(
        responses::FullPanoPic,
        r#"
SELECT id, original_filename, sha1, lon, lat, stop_id,
    uploader, upload_date, capture_date, sensitive
FROM panoramas
WHERE NOT sensitive OR $1
"#,
        allow_sensitive
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), allow_sensitive);
        Error::DatabaseExecution
    })
}

/// Fetches a pano picture by its hash.
pub(crate) async fn fetch_pano_by_hash(
    pool: &PgPool,
    pic_hash: &str,
) -> Result<Option<pics::PanoPic>> {
    sqlx::query!(
        r#"
SELECT id, original_filename, sha1, lon, lat, uploader, upload_date, capture_date, sensitive
FROM panoramas
WHERE sha1 = $1
"#,
        pic_hash
    )
        .fetch_optional(pool)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string(), pic_hash);
            Error::DatabaseExecution
        })
        .map(|row| {
            row.map(|row| pics::PanoPic {
                id: row.id,
                original_filename: row.original_filename,
                sha1: row.sha1,
                uploader: row.uploader,
                upload_date: row.upload_date,
                capture_date: row.capture_date,
                stop_id: None,
                lon: row.lon,
                lat: row.lat,
                sensitive: row.sensitive,
            })
        })
}

/// Fetches a pano picture by its attached stop.
pub(crate) async fn fetch_stop_pano(
    pool: &PgPool,
    stop_id: i32,
    allow_sensitive: bool,
) -> Result<Option<responses::PanoPic>> {
    sqlx::query!(
        r#"
SELECT id, sha1, lon, lat, capture_date, sensitive
FROM panoramas
WHERE stop_id = $1 AND (NOT sensitive OR $2)
"#,
        stop_id,
        allow_sensitive
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), stop_id, allow_sensitive);
        Error::DatabaseExecution
    })
    .map(|row| {
        row.map(|row| responses::PanoPic {
            id: row.id,
            sha1: row.sha1,
            capture_date: row.capture_date,
            stop_id: None,
            lon: row.lon,
            lat: row.lat,
        })
    })
}

pub(crate) async fn insert_pano(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mut pic: pics::PanoPic,
) -> Result<pics::PanoPic> {
    let res = sqlx::query!(
        r#"
INSERT INTO panoramas(
    original_filename, sha1, stop_id, lat, lon, uploader, upload_date, capture_date, sensitive
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING id
        "#,
        pic.original_filename,
        pic.sha1,
        pic.stop_id,
        pic.lat,
        pic.lon,
        pic.uploader,
        pic.upload_date,
        pic.capture_date,
        pic.sensitive
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), pic=?pic);
        Error::DatabaseExecution})?;

    pic.id = res.id;

    Ok(pic)
}

/// Fetches the onion skin for a pano picture.
/// The onion skin is going to be composed from the 10 regular pictures
/// before and after the pano picture.
pub(crate) async fn fetch_pano_onion(
    pool: &PgPool,
    pano_id: i32,
) -> Result<responses::PanoOnion> {
    let predecessors = sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.public, stop_pics.sensitive,
    stop_pics.lon, stop_pics.lat, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.capture_date < (
    SELECT capture_date
    FROM panoramas
    WHERE id = $1
)
GROUP BY stop_pics.id
ORDER BY stop_pics.capture_date DESC
LIMIT 10
"#,
        pano_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pano_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| responses::MinimalPicWithStops {
        id: r.id,
        tagged: r.tagged,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        stops: r.stops,
    })
    .collect();

    let successors = sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.public, stop_pics.sensitive,
    stop_pics.lon, stop_pics.lat, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.capture_date >= (
    SELECT capture_date
    FROM panoramas
    WHERE id = $1
)
GROUP BY stop_pics.id
ORDER BY stop_pics.capture_date ASC
LIMIT 10
"#,
        pano_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), pano_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| responses::MinimalPicWithStops {
        id: r.id,
        tagged: r.tagged,
        public: r.public,
        sensitive: r.sensitive,
        lon: r.lon,
        lat: r.lat,
        stops: r.stops,
    })
    .collect();

    Ok(responses::PanoOnion {
        predecessors,
        successors,
    })
}

pub(crate) async fn fetch_operator_logo_hash(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
) -> Result<Option<Option<String>>> {
    sqlx::query!(
        r#"
SELECT logo_sha1
FROM operators
WHERE operators.id=$1
        "#,
        operator_id
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })
    .map(|res| res.map(|row| row.logo_sha1))
}
pub(crate) async fn update_operator_logo_hash(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    hash: Option<&str>,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE operators
SET logo_sha1=$1
WHERE operators.id=$2
        "#,
        hash,
        operator_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), hash, operator_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_news_img_by_hash(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sha1: &str,
) -> Result<Option<pics::NewsImage>> {
    sqlx::query_as!(
        pics::NewsImage,
        r#"
SELECT id, sha1, filename, transcript
FROM news_imgs
WHERE sha1=$1
"#,
        sha1
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), sha1);
        Error::DatabaseExecution
    })
}

pub(crate) async fn insert_news_img(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sha1: &str,
    filename: &str,
) -> Result<i32> {
    let res = sqlx::query!(
        r#"
INSERT INTO news_imgs(sha1, filename)
VALUES ($1, $2)
RETURNING id
        "#,
        sha1,
        filename,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), sha1, filename);
        Error::DatabaseExecution
    })?;

    Ok(res.id)
}

pub(crate) async fn link_news_item_img(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    img_id: i32,
    item_id: i32,
) -> Result<()> {
    let _res = sqlx::query!(
        r#"
INSERT INTO news_items_imgs(item_id, img_id)
VALUES ($1, $2)
        "#,
        item_id,
        img_id,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, img_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn update_news_img_meta(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    meta: &requests::ChangeNewsImgMeta,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE news_imgs
SET transcript=$1
WHERE id=$2"#,
        meta.transcript,
        item_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, meta.transcript);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_external_news_img_hashes(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
) -> Result<Option<Vec<String>>> {
    Ok(sqlx::query!(
        r#"
SELECT external_news_items.id, array_remove(array_agg(sha1), NULL) as "hashes!: Vec<String>"
FROM external_news_items
LEFT JOIN external_news_items_imgs ON external_news_items_imgs.item_id = external_news_items.id
LEFT JOIN external_news_imgs ON external_news_items_imgs.img_id = external_news_imgs.id
WHERE external_news_items.id=$1
GROUP BY external_news_items.id"#,
        item_id
    )
        .fetch_optional(&mut **transaction).await
        .map_err(|err| {
            tracing::error!(error=err.to_string(), item_id);
            Error::DatabaseExecution
        })?
        .map(|row| row.hashes))
}

pub(crate) async fn insert_external_news_img(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    sha1: &str,
    filename: &str,
) -> Result<i32> {
    let res = sqlx::query!(
        r#"
INSERT INTO external_news_imgs(sha1, filename)
VALUES ($1, $2)
RETURNING id"#,
        sha1,
        filename,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, sha1, filename);
        Error::DatabaseExecution
    })?;

    let pic_id = res.id;

    let _res = sqlx::query!(
        r#"
INSERT INTO external_news_items_imgs(item_id, img_id)
VALUES ($1, $2)
        "#,
        item_id,
        pic_id,
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, pic_id);
        Error::DatabaseExecution
    })?;

    Ok(pic_id)
}

pub(crate) async fn fetch_external_news_screenshot_hash(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
) -> Result<Option<Option<String>>> {
    Ok(sqlx::query!(
        r#"
SELECT ss_sha1
FROM external_news_items
WHERE external_news_items.id=$1"#,
        item_id
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id);
        Error::DatabaseExecution
    })?
    .map(|row| row.ss_sha1))
}

pub(crate) async fn update_external_news_screenshot(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    item_id: i32,
    sha1: &str,
) -> Result<()> {
    let _res = sqlx::query!(
        r#"
UPDATE external_news_items
SET ss_sha1 = $1
WHERE id=$2"#,
        sha1,
        item_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), item_id, sha1);
        Error::DatabaseExecution
    })?;

    Ok(())
}
