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

use std::io::{BufReader, Cursor};

use bytes::Bytes;
use chrono::Local;
use sha1::{Digest, Sha1};
use sqlx::PgPool;

use super::models;
use super::sql;
use crate::utils::Exif;
use crate::Error;

const THUMBNAIL_MAX_WIDTH: u32 = 300;
const THUMBNAIL_MAX_HEIGHT: u32 = 200;
const THUMBNAIL_MAX_QUALITY: f32 = 90.0;

const MEDIUM_IMG_MAX_WIDTH: u32 = 1200;
const MEDIUM_IMG_MAX_HEIGHT: u32 = 800;
const MEDIUM_IMG_MAX_QUALITY: f32 = 85.0;

pub(crate) async fn upload_stop_picture(
    user_id: i32,
    name: String,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
    content: &Bytes,
) -> Result<i32, Error> {
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let res = sqlx::query!("SELECT id FROM stop_pics WHERE sha1=$1", hex_hash)
        .fetch_optional(db_pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if let Some(res) = res {
        return Ok(res.id);
    }

    let mut original_img = image::load_from_memory(content.as_ref())
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    let original_img_mime = mime_guess::from_path(&name);

    if !matches!(original_img, image::DynamicImage::ImageRgb8(_)) {
        original_img = image::DynamicImage::ImageRgb8(original_img.into_rgb8());
    }

    let mut stop_pic_entry = models::StopPic {
        id: 0,
        original_filename: name,
        sha1: hex_hash.clone(),
        tagged: false,
        uploader: user_id,
        upload_date: Local::now().to_string(),
        capture_date: None,
        updater: None,
        update_date: None,
        width: original_img.width() as i32,
        height: original_img.height() as i32,
        camera_ref: None,
        dyn_meta: models::StopPicDynMeta {
            public: false,
            sensitive: false,
            lon: None,
            lat: None,
            quality: 0,
            tags: vec![],
            notes: None,
        },
    };

    let mut source_buffer = BufReader::new(Cursor::new(content.as_ref()));
    if let Ok(exif) =
        exif::Reader::new().read_from_container(&mut source_buffer)
    {
        let exif_data = Exif::from(exif);

        stop_pic_entry.dyn_meta.lon = exif_data.lon;
        stop_pic_entry.dyn_meta.lat = exif_data.lat;
        stop_pic_entry.camera_ref = exif_data.camera;
        stop_pic_entry.capture_date =
            exif_data.capture.map(|date| date.to_string());
    };

    upload_picture_to_storage(
        bucket,
        content,
        &original_img,
        original_img_mime,
        &hex_hash,
    ).await?;

    // TODO Delete if insertion fails
    sql::insert_stop_picture(db_pool, stop_pic_entry).await
}

pub(crate) async fn delete_stop_picture(
    stop_picture_id: i32,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
) -> Result<(), Error> {
    let tagged_stops =
        sql::fetch_stop_pic_stop_count(db_pool, stop_picture_id).await?;

    if tagged_stops > 0 {
        return Err(Error::DependenciesNotMet);
    }

    let stop_pic = sql::fetch_stop_picture(db_pool, stop_picture_id)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if let Some(stop_pic) = stop_pic {
        let hex_hash = stop_pic.sha1;
        delete_picture_from_storage(&hex_hash, bucket).await?;
    } else {
        return Err(Error::NotFoundUpstream);
    }

    sql::delete_stop_picture(db_pool, stop_picture_id).await
}

async fn upload_picture_to_storage(
    bucket: &s3::Bucket,
    content: &Bytes,
    original_img: &image::DynamicImage,
    original_img_mime: mime_guess::MimeGuess,
    hex_hash: &str
) -> Result<(), Error>{
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

    Ok(())
}

async fn delete_picture_from_storage(
    hex_hash: &str,
    bucket: &s3::Bucket,
) -> Result<(), Error> {
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

    Ok(())
}
