/*
    Intermodalis, transportation information aggregator
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

use std::io::{BufReader, Cursor};

use base16ct;
use bytes::Bytes;
use chrono::Local;
use s3;
use sha1::{Digest, Sha1};

use crate::models::StopPic;
use crate::utils::Exif;
use crate::{Error, SqlitePool, Stats, Stop};

const THUMBNAIL_MAX_WIDTH: u32 = 300;
const THUMBNAIL_MAX_HEIGHT: u32 = 200;
const THUMBNAIL_MAX_QUALITY: f32 = 90.0;

const MEDIUM_IMG_MAX_WIDTH: u32 = 1200;
const MEDIUM_IMG_MAX_HEIGHT: u32 = 800;
const MEDIUM_IMG_MAX_QUALITY: f32 = 85.0;

pub(crate) async fn get_stops(
    db_pool: &SqlitePool,
) -> Result<Vec<Stop>, Error> {
    Ok(sqlx::query!(
        r#"
SELECT *
FROM Stops
    "#
    )
    .fetch_all(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        let tags: Vec<String> =
            if let Ok(tags) = serde_json::from_str(&row.tags) {
                tags
            } else {
                // todo warn
                vec![]
            };
        Stop {
            id: row.id,
            source: row.source,
            name: row.name,
            official_name: row.official_name,
            osm_name: row.osm_name,
            short_name: row.short_name,
            locality: row.locality,
            street: row.street,
            door: row.door,
            parish: row.parish,
            lat: row.lat,
            lon: row.lon,
            external_id: row.external_id,
            succeeded_by: row.succeeded_by,
            notes: row.notes,
            has_crossing: row.has_crossing.map(|val| val != 0),
            has_accessibility: row.has_accessibility.map(|val| val != 0),
            has_abusive_parking: row.has_abusive_parking.map(|val| val != 0),
            has_outdated_info: row.has_outdated_info.map(|val| val != 0),
            is_damaged: row.is_damaged.map(|val| val != 0),
            is_vandalized: row.is_vandalized.map(|val| val != 0),
            has_flag: row.has_flag.map(|val| val != 0),
            has_schedules: row.has_schedules.map(|val| val != 0),
            has_sidewalk: row.has_sidewalk.map(|val| val != 0),
            has_shelter: row.has_shelter.map(|val| val != 0),
            has_bench: row.has_bench.map(|val| val != 0),
            has_trash_can: row.has_trash_can.map(|val| val != 0),
            is_illuminated: row.is_illuminated.map(|val| val != 0),
            has_illuminated_path: row.has_illuminated_path.map(|val| val != 0),
            has_visibility_from_within: row.has_visibility_from_within.map(|val| val != 0),
            has_visibility_from_area: row.has_visibility_from_area.map(|val| val != 0),
            is_visible_from_outside: row.is_visible_from_outside.map(|val| val != 0),
            updater: row.updater,
            update_date: row.update_date,
            tags,
        }
    })
    .collect::<Vec<_>>())
}

pub(crate) async fn upload_stop_picture(
    user_id: i64,
    name: String,
    bucket: &s3::Bucket,
    db_pool: &SqlitePool,
    content: &Bytes,
) -> Result<i64, Error> {
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let res = sqlx::query!(
        r#"
SELECT id FROM StopPics
WHERE sha1=?"#,
        hex_hash,
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if let Some(res) = res {
        return Ok(res.id);
    }

    let original_img = image::load_from_memory(content.as_ref())
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    let original_img_mime = mime_guess::from_path(&name);

    let medium_img = original_img.resize(
        MEDIUM_IMG_MAX_WIDTH,
        MEDIUM_IMG_MAX_HEIGHT,
        image::imageops::FilterType::CatmullRom,
    );
    let medium_img_webp = webp::Encoder::from_image(&medium_img)
        .map_err(|err| Error::Processing(err.to_string()))?
        .encode(MEDIUM_IMG_MAX_QUALITY)
        .to_vec();
    // TODO handle status codes
    let _status_code = bucket
        .put_object_with_content_type(
            format!("/medium/{}", hex_hash),
            &medium_img_webp,
            "image/webp",
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    let thumbnail_img = original_img.resize(
        THUMBNAIL_MAX_WIDTH,
        THUMBNAIL_MAX_HEIGHT,
        image::imageops::FilterType::CatmullRom,
    );
    let thumbnail_img_webp = webp::Encoder::from_image(&thumbnail_img)
        .map_err(|err| Error::Processing(err.to_string()))?
        .encode(THUMBNAIL_MAX_QUALITY)
        .to_vec();
    let _status_code = bucket
        .put_object_with_content_type(
            format!("/thumb/{}", hex_hash),
            &thumbnail_img_webp,
            "image/webp",
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    let _status_code = if let Some(mime) = original_img_mime.first() {
        bucket
            .put_object_with_content_type(
                format!("/ori/{}", hex_hash),
                content.as_ref(),
                mime.as_ref(),
            )
            .await
            .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?
    } else {
        bucket
            .put_object(format!("/ori/{}", hex_hash), content.as_ref())
            .await
            .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?
    };

    let mut stop_pic_entry = StopPic {
        id: 0,
        original_filename: name,
        sha1: hex_hash,
        public: false,
        sensitive: false,
        tagged: false,
        uploader: user_id,
        upload_date: Local::now().to_string(),
        capture_date: None,
        updater: None,
        update_date: None,
        lon: None,
        lat: None,
        width: original_img.width(),
        height: original_img.height(),
        quality: 0,
        camera_ref: None,
        tags: vec![],
        notes: None,
    };

    let mut source_buffer = BufReader::new(Cursor::new(content.as_ref()));
    if let Ok(exif) =
        exif::Reader::new().read_from_container(&mut source_buffer)
    {
        let exif_data = Exif::from(exif);

        stop_pic_entry.lon = exif_data.lon.map(|lon| lon as f32);
        stop_pic_entry.lat = exif_data.lat.map(|lat| lat as f32);
        stop_pic_entry.camera_ref = exif_data.camera;
        stop_pic_entry.capture_date =
            exif_data.capture.map(|date| date.to_string());
    };

    let res = sqlx::query!(
        r#"
INSERT INTO StopPics(
    original_filename, sha1, public, sensitive, tagged, uploader,
    upload_date, capture_date, width, height, lat, lon, camera_ref
)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING id
        "#,
        stop_pic_entry.original_filename,
        stop_pic_entry.sha1,
        stop_pic_entry.public,
        stop_pic_entry.sensitive,
        stop_pic_entry.tagged,
        stop_pic_entry.uploader,
        stop_pic_entry.upload_date,
        stop_pic_entry.capture_date,
        stop_pic_entry.width,
        stop_pic_entry.height,
        stop_pic_entry.lat,
        stop_pic_entry.lon,
        stop_pic_entry.camera_ref
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn delete_stop_picture(
    stop_picture_id: i64,
    bucket: &s3::Bucket,
    db_pool: &SqlitePool,
) -> Result<(), Error> {
    let tagged_stops: i32 = sqlx::query!(
        r#"
SELECT count(*) as count
FROM StopPicStops
WHERE pic=?
"#,
        stop_picture_id
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .count;

    if tagged_stops > 0 {
        return Err(Error::DependenciesNotMet);
    }

    let stop_pic = sqlx::query!(
        r#"
SELECT sha1 FROM StopPics
WHERE id=?
    "#,
        stop_picture_id,
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if let Some(stop_pic) = stop_pic {
        let hex_hash = stop_pic.sha1;

        bucket
            .delete_object(format!("/thumb/{}", hex_hash))
            .await
            .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;
        bucket
            .delete_object(format!("/medium/{}", hex_hash))
            .await
            .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;
        bucket
            .delete_object(format!("/ori/{}", hex_hash))
            .await
            .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;
    } else {
        return Err(Error::NotFoundUpstream);
    }

    let _res = sqlx::query(
        r#"
DELETE FROM StopPics
WHERE id=?
    "#,
    )
    .bind(stop_picture_id)
    .execute(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()));

    Ok(())
}

pub(crate) async fn try_get_user(
    token: &str,
    db_pool: &SqlitePool,
) -> Result<Option<i64>, Error> {
    let res = sqlx::query!(
        r#"
SELECT id
FROM Users
WHERE token=?
    "#,
        token
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(if let Some(row) = res { row.id } else { None })
}

pub(crate) async fn get_user(
    token: &str,
    db_pool: &SqlitePool,
) -> Result<i64, Error> {
    let user = try_get_user(token, db_pool).await?;
    if let Some(id) = user {
        Ok(id)
    } else {
        Err(Error::Forbidden)
    }
}

pub(crate) async fn get_stats(db_pool: &SqlitePool) -> Result<Stats, Error> {
    let stop_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Stops
WHERE Stops.source = 'osm'
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt;

    let route_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Routes
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt;

    let subroute_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Subroutes
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt;

    let departure_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Departures
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt;

    let picture_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM StopPics
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt;

    Ok(Stats {
        stop_count,
        route_count,
        subroute_count,
        departure_count,
        picture_count,
    })
}
