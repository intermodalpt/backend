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
pub(crate) async fn fetch_picture(
    pool: &PgPool,
    picture_id: i32,
) -> Result<Option<pics::StopPic>> {
    sqlx::query!(
        r#"
SELECT id, original_filename, sha1, tagged, public, sensitive, uploader,
    upload_date, capture_date, lon, lat, quality, width,
    height, camera_ref, tags, notes
FROM stop_pics
WHERE id = $1
"#,
        picture_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
    height, camera_ref, tags, notes
FROM stop_pics
WHERE sha1 = $1
"#,
        pic_hash
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
                notes: row.notes,
            },
        })
    })
}

/// A specific picture and its stops
pub(crate) async fn fetch_picture_with_stops(
    pool: &PgPool,
    picture_id: i32,
) -> Result<Option<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.lon, stop_pics.lat,
    stop_pics.quality, stop_pics.width, stop_pics.height, stop_pics.camera_ref,
    stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.id = $1
GROUP BY stop_pics.id
"#,
        picture_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
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
    stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
GROUP BY stop_pics.id
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
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
    OR (stop_pics.public = true and stop_pics.sensitive = false)
    OR $2 = true
GROUP BY stop_pics.id
"#,
        uid,
        trusted
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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

/// All of the stops that are linked to a picture
pub(crate) async fn fetch_picture_stops(
    pool: &PgPool,
    picture_id: i32,
) -> Result<Vec<i32>> {
    Ok(sqlx::query!(
        r#"
SELECT stop
FROM stop_pic_stops
WHERE pic = $1
"#,
        picture_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| r.stop)
    .collect())
}

/// All of the pictures that are attached to a stop and meant to be public
pub(crate) async fn fetch_public_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<responses::PublicStopPic>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.sha1, stop_pics.capture_date, stop_pics.lon, stop_pics.lat, stop_pics.tags, stop_pics.quality
FROM stop_pics
JOIN stop_pic_stops on stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.tagged = true AND stop_pics.sensitive = false
    AND stop_pics.public = true AND stop_pic_stops.stop=$1
ORDER BY stop_pics.capture_date DESC
    "#,
        stop_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
            quality: r.quality,
        }).collect()
    )
}

/// All of the pictures that are attached to a stop and visible to the user
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
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pic_stops.stop=$1
    AND (stop_pics.uploader = $2
        OR (stop_pics.public = true AND stop_pics.sensitive = false)
        OR $3 = true)
GROUP BY stop_pics.id
ORDER BY quality DESC
    "#,
        stop_id,
        uid,
        trusted
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
    })
    .collect())
}

/// A range of pictures that have been uploaded by a user
pub(crate) async fn fetch_user_pictures(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE uploader=$1
GROUP BY stop_pics.id
ORDER BY capture_date DESC, upload_date DESC
LIMIT $2 OFFSET $3
    "#,
        user_id,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
    })
    .collect())
}

/// A range of pictures that are meant to be public (auth-free)
pub(crate) async fn fetch_latest_public_pictures(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::PicWithStops>> {
    Ok(sqlx::query!(
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE public=true AND sensitive=false
GROUP BY stop_pics.id
ORDER BY capture_date DESC, upload_date DESC
LIMIT $1 OFFSET $2
    "#,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
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
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.uploader = $1
    OR (stop_pics.public = true AND stop_pics.sensitive = false)
    OR $2 = true
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
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
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE tagged=true
    AND (stop_pics.uploader = $1
        OR (stop_pics.public = true AND stop_pics.sensitive = false)
        OR $2 = true)
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
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
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes, stop_pics.tagged,
    array_remove(array_agg(stop_pic_stops.stop), NULL) as "stops!: Vec<i32>"
FROM stop_pics
LEFT JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE tagged=false
    AND (stop_pics.uploader = $1
        OR (stop_pics.public = true AND stop_pics.sensitive = false)
        OR $2 = true)
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
        notes: r.notes,
        stops: r.stops,
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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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
        OR (stop_pics.public = true AND stop_pics.sensitive = false)
        OR $2 = true)
LIMIT $3 OFFSET $4
    "#,
        uid,
        trusted,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
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
WHERE stop_pics.public = true
ORDER BY stop ASC
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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

pub(crate) async fn insert_picture(
    pool: &PgPool,
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
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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
        .execute(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    Ok(pic)
}

pub(crate) async fn update_picture_meta(
    pool: &PgPool,
    stop_picture_id: i32,
    stop_pic_meta: requests::ChangeStopPic,
    user_id: i32,
) -> Result<()> {
    let update_date = Local::now().to_string();

    let mut transaction = pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let _res = sqlx::query!(
        r#"
UPDATE stop_pics
SET public=$1, sensitive=$2, lon=$3, lat=$4, tags=$5, quality=$6, updater=$7,
    update_date=$8, tagged=true
WHERE id=$9
    "#,
        stop_pic_meta.public,
        stop_pic_meta.sensitive,
        stop_pic_meta.lon,
        stop_pic_meta.lat,
        &stop_pic_meta.tags,
        stop_pic_meta.quality,
        user_id,
        update_date,
        stop_picture_id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if !stop_pic_meta.stops.is_empty() {
        // TODO add updater and update date
        let stop_ids = stop_pic_meta.stops.iter().join(",");

        let _res = sqlx::query(&format!(
            r#"
    DELETE FROM stop_pic_stops
    WHERE pic=$1 AND stop NOT IN ({stop_ids})
        "#
        ))
        .bind(stop_picture_id)
        .execute(&mut *transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

        for stop_id in stop_pic_meta.stops {
            let _res = sqlx::query!(
                r#"
INSERT INTO stop_pic_stops(pic, stop)
VALUES ($1, $2)
ON CONFLICT DO NOTHING
    "#,
                stop_picture_id,
                stop_id
            )
            .execute(&mut *transaction)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    }

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_picture(pool: &PgPool, pic_id: i32) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sqlx::query!(
        r#"
DELETE FROM stop_pic_stops
WHERE pic=$1"#,
        pic_id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sqlx::query!(
        r#"
DELETE FROM stop_pics
WHERE id=$1
        "#,
        pic_id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| (row.stop, row.pic_count))
    .collect();

    Ok(res)
}
