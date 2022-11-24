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

use chrono::Local;
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::HashMap;

use super::{models, models::requests, models::responses};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_stop_picture(
    pool: &PgPool,
    picture_id: i32,
) -> Result<Option<models::StopPic>> {
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
        row.map(|row| models::StopPic {
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
            dyn_meta: models::StopPicDynMeta {
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

pub(crate) async fn fetch_stop_pictures(
    pool: &PgPool,
) -> Result<Vec<responses::TaggedStopPic>> {
    sqlx::query_as!(
        responses::TaggedStopPic,
        r#"
SELECT id, original_filename, sha1, public, sensitive, uploader,
    upload_date, capture_date, lon, lat, quality, width,
    height, camera_ref, tags, notes
FROM stop_pics
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_public_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<responses::PublicStopPic>> {
    sqlx::query_as!(
        responses::PublicStopPic,
        r#"
SELECT stop_pics.id, stop_pics.sha1, stop_pics.capture_date, stop_pics.lon, stop_pics.lat, stop_pics.tags, stop_pics.quality
FROM stop_pics
JOIN stop_pic_stops on stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.tagged = false AND stop_pics.sensitive = false
    AND stop_pics.public = true AND stop_pic_stops.stop=$1
ORDER BY stop_pics.capture_date DESC
    "#,
        stop_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_tagged_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<responses::TaggedStopPic>> {
    sqlx::query_as!(
        responses::TaggedStopPic,
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes
FROM stop_pics
JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.tagged = true AND stop_pic_stops.stop=$1
ORDER BY quality DESC
    "#,
        stop_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_untagged_stop_pictures(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::UntaggedStopPic>> {
    sqlx::query_as!(
        responses::UntaggedStopPic,
        r#"
SELECT id, original_filename, sha1, public, sensitive, uploader, upload_date,
    capture_date, width, height, lon, lat, camera_ref, tags, notes
FROM stop_pics
WHERE tagged=false AND uploader=$1
ORDER BY capture_date ASC
LIMIT $2 OFFSET $3
    "#,
        user_id,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_picture_stop_rels(
    pool: &PgPool,
) -> Result<HashMap<i32, Vec<i32>>> {
    let res = sqlx::query!(
        r#"
SELECT stop, pic
FROM  stop_pic_stops
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

pub(crate) async fn insert_stop_picture(
    pool: &PgPool,
    pic: models::StopPic,
) -> Result<i32> {
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

    Ok(res.id)
}

pub(crate) async fn update_stop_picture_meta(
    pool: &PgPool,
    stop_picture_id: i32,
    stop_pic_meta: requests::ChangeStopPic,
    user_id: i32,
) -> Result<()> {
    let update_date = Local::now().to_string();

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
    .execute(pool)
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
        .execute(pool)
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
            .execute(pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    }

    Ok(())
}

pub(crate) async fn delete_stop_picture(
    pool: &PgPool,
    pic_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM stop_pics
WHERE id=$1
        "#,
        pic_id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_stop_pic_stop_count(
    pool: &PgPool,
    pic_id: i32,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as count
FROM stop_pic_stops
WHERE pic=$1"#,
        pic_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .count
    .unwrap_or(0))
}
