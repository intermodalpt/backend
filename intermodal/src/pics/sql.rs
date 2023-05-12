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
use crate::pics::{get_full_path, get_medium_path, get_thumb_path};
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

pub(crate) async fn fetch_stop_picture_by_hash(
    pool: &PgPool,
    pic_hash: &str,
) -> Result<Option<models::StopPic>> {
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
) -> Result<Vec<responses::StopPic>> {
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
    .map(|r| responses::StopPic {
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

pub(crate) async fn fetch_stop_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<responses::StopPic>> {
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
GROUP BY stop_pics.id
ORDER BY quality DESC
    "#,
        stop_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| responses::StopPic {
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

pub(crate) async fn fetch_untagged_stop_pictures(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::StopPic>> {
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
WHERE tagged=false AND uploader=$1
GROUP BY stop_pics.id
ORDER BY capture_date ASC
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
    .map(|r| responses::StopPic {
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

pub(crate) async fn insert_stop_picture(
    pool: &PgPool,
    mut pic: models::StopPic,
    stops: &[i32],
) -> Result<models::StopPic> {
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
    .execute(&mut transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sqlx::query!(
        r#"
DELETE FROM stop_pics
WHERE id=$1
        "#,
        pic_id
    )
    .execute(&mut transaction)
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
