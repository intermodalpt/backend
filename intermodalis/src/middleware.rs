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
use chrono::{Local, NaiveDateTime};
use s3;
use sha1::{Digest, Sha1};

use crate::models::StopPic;
use crate::utils::{
    datetime_from_exif_ascii, extract_f64_gps_coord, string_from_exif_ascii,
};
use crate::{Error, SqlitePool};

const THUMBNAIL_MAX_WIDTH: u32 = 300;
const THUMBNAIL_MAX_HEIGHT: u32 = 200;
const THUMBNAIL_MAX_QUALITY: f32 = 90.0;

const MEDIUM_IMG_MAX_WIDTH: u32 = 1200;
const MEDIUM_IMG_MAX_HEIGHT: u32 = 800;
const MEDIUM_IMG_MAX_QUALITY: f32 = 85.0;

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
            .put_object(format!("/{}", hex_hash), content.as_ref())
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
    upload_date, capture_date, width, height, lon, lat, camera_ref
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

#[derive(Default, Debug)]
struct Exif {
    lat: Option<f64>,
    lon: Option<f64>,
    capture: Option<NaiveDateTime>,
    camera: Option<String>,
}

impl From<exif::Exif> for Exif {
    fn from(data: exif::Exif) -> Self {
        let mut result = Exif::default();

        if let Some(field) =
            data.get_field(exif::Tag::GPSLatitude, exif::In::PRIMARY)
        {
            if let exif::Value::Rational(val) = &field.value {
                if let Ok(coord) = extract_f64_gps_coord(&val) {
                    result.lat = Some(coord);
                } else {
                    println!("Invalid value for GPS Lat");
                }
            } else {
                println!("Invalid value for GPS Lat");
            }
        }

        if let Some(field) =
            data.get_field(exif::Tag::GPSLongitude, exif::In::PRIMARY)
        {
            if let exif::Value::Rational(val) = &field.value {
                if let Ok(coord) = extract_f64_gps_coord(&val) {
                    result.lon = Some(coord);
                } else {
                    println!("Invalid value for GPS Lon");
                }
            } else {
                println!("Invalid value for GPS Lon");
            }
        }

        if let Some(field) =
            data.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
        {
            if let exif::Value::Ascii(val) = &field.value {
                if let Ok(datetime) = datetime_from_exif_ascii(val) {
                    result.capture = Some(datetime);
                }
            } else {
                println!("Invalid value for Original Datetime");
            }
        }

        if result.capture.is_none() {
            if let Some(field) =
                data.get_field(exif::Tag::DateTimeDigitized, exif::In::PRIMARY)
            {
                if let exif::Value::Ascii(val) = &field.value {
                    if let Ok(datetime) = datetime_from_exif_ascii(val) {
                        result.capture = Some(datetime);
                    }
                } else {
                    println!("Invalid value for Digitized Datetime");
                }
            }
        }

        if let Some(field) = data.get_field(exif::Tag::Model, exif::In::PRIMARY)
        {
            if let exif::Value::Ascii(val) = &field.value {
                if let Ok(datetime) = string_from_exif_ascii(val) {
                    result.camera = Some(datetime);
                }
            } else {
                println!("Invalid value for Camera Model");
            }
        }

        result
    }
}
