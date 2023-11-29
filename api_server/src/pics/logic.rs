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

use std::io::{BufReader, Cursor};

use bytes::Bytes;
use chrono::Utc;
use mime_guess::mime;
use sha1::{Digest, Sha1};
use sqlx::PgPool;

use commons::models::{history, pics};
use commons::utils::exif::{Exif, Orientation};

use super::sql;
use crate::contrib;
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
    stops: &[i32],
) -> Result<pics::StopPic, Error> {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let res = sql::fetch_picture_by_hash(db_pool, &hex_hash).await?;

    if let Some(pic) = res {
        return Err(Error::DuplicatedResource(pics::Resource::StopPic(pic)));
    }

    let mut original_img = image::load_from_memory(content.as_ref())
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    let original_img_mime = mime_guess::from_path(&name);

    if !matches!(original_img, image::DynamicImage::ImageRgb8(_)) {
        original_img = image::DynamicImage::ImageRgb8(original_img.into_rgb8());
    }

    let mut stop_pic_entry = pics::StopPic {
        id: 0,
        original_filename: name,
        sha1: hex_hash.clone(),
        tagged: false,
        uploader: user_id,
        upload_date: Utc::now(),
        capture_date: None,
        updater: None,
        update_date: None,
        width: original_img.width() as i32,
        height: original_img.height() as i32,
        camera_ref: None,
        dyn_meta: pics::StopPicDynMeta {
            public: false,
            sensitive: false,
            lon: None,
            lat: None,
            quality: 0,
            tags: vec![],
            attrs: vec![],
            notes: None,
        },
    };

    let mut source_buffer = BufReader::new(Cursor::new(content.as_ref()));
    if let Ok(exif) =
        exif::Reader::new().read_from_container(&mut source_buffer)
    {
        let exif_data = Exif::from(exif);

        if let Some(orientation) = exif_data.orientation {
            match orientation {
                Orientation::Mirror => {
                    original_img = original_img.fliph();
                }
                Orientation::Rotate180 => {
                    original_img = original_img.rotate180();
                }
                Orientation::MirrorVertical => {
                    original_img = original_img.flipv();
                }
                Orientation::MirrorHorizontalRotate270 => {
                    // FIXME This is broken
                    original_img = original_img.fliph().rotate270();
                }
                Orientation::Rotate90 => {
                    original_img = original_img.rotate90();
                }
                Orientation::MirrorHorizontalRotate90 => {
                    // FIXME This is broken
                    original_img = original_img.fliph().rotate90();
                }
                Orientation::Rotate270 => {
                    original_img = original_img.rotate270();
                }
                Orientation::Horizontal => {}
            }
        }

        stop_pic_entry.dyn_meta.lon = exif_data.lon;
        stop_pic_entry.dyn_meta.lat = exif_data.lat;
        stop_pic_entry.camera_ref = exif_data.camera;
        stop_pic_entry.capture_date = exif_data.capture;
    };

    upload_picture_to_storage(
        bucket,
        content,
        &original_img,
        original_img_mime,
        &hex_hash,
    )
    .await?;

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    // TODO Delete if insertion fails
    let pic =
        sql::insert_picture(&mut transaction, stop_pic_entry, stops).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &[history::Change::StopPicUpload {
            pic: pic.clone(),
            stops: stops
                .into_iter()
                .map(|stop_id| pics::StopAttrs {
                    id: *stop_id,
                    attrs: vec![],
                })
                .collect(),
        }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(pic)
}

pub(crate) async fn delete_picture(
    pic: pics::StopPic,
    author_id: i32,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
) -> Result<(), Error> {
    let stop_rels = sql::fetch_picture_stops_rel_attrs(db_pool, pic.id).await?;

    let stop_pic = sql::fetch_picture(db_pool, pic.id)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if let Some(stop_pic) = stop_pic {
        let hex_hash = stop_pic.sha1;
        delete_picture_from_storage(&hex_hash, bucket).await?;
    } else {
        return Err(Error::NotFoundUpstream);
    }

    sql::delete_picture(&mut transaction, pic.id).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        author_id,
        &[history::Change::StopPicDeletion {
            pic,
            stops: stop_rels,
        }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

async fn upload_picture_to_storage(
    bucket: &s3::Bucket,
    content: &Bytes,
    original_img: &image::DynamicImage,
    original_img_mime: mime_guess::MimeGuess,
    hex_hash: &str,
) -> Result<(), Error> {
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
            format!("/medium/{hex_hash}"),
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
            format!("/thumb/{hex_hash}"),
            &thumbnail_img_webp,
            "image/webp",
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    let _status_code = if let Some(mime) = original_img_mime.first() {
        bucket
            .put_object_with_content_type(
                format!("/ori/{hex_hash}"),
                content.as_ref(),
                mime.as_ref(),
            )
            .await
            .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?
    } else {
        bucket
            .put_object(format!("/ori/{hex_hash}"), content.as_ref())
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
        .delete_object(format!("/thumb/{hex_hash}"))
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;
    bucket
        .delete_object(format!("/medium/{hex_hash}"))
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;
    bucket
        .delete_object(format!("/ori/{hex_hash}"))
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

pub(crate) async fn upload_pano_picture(
    user_id: i32,
    name: String,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
    content: &Bytes,
) -> Result<pics::PanoPic, Error> {
    if !std::path::Path::new(&name)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("insp"))
    {
        return Err(Error::DependenciesNotMet);
    }

    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let res = sql::fetch_pano_by_hash(db_pool, &hex_hash).await?;

    if let Some(pic) = res {
        return Err(Error::DuplicatedResource(pics::Resource::PanoPic(pic)));
    }

    let _ = image::load_from_memory(content.as_ref())
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let mut source_buffer = BufReader::new(Cursor::new(content.as_ref()));

    let exif = Exif::from(
        exif::Reader::new()
            .read_from_container(&mut source_buffer)
            .map_err(|e| {
                Error::Processing(format!("Panorama exif error: {e}"))
            })?,
    );

    let stop_pic_entry = pics::PanoPic {
        id: 0,
        original_filename: name,
        sha1: hex_hash.clone(),
        uploader: user_id,
        upload_date: Utc::now(),
        capture_date: exif.capture,
        stop_id: None,
        lon: exif.lon,
        lat: exif.lat,
        sensitive: true,
    };

    upload_pano_to_storage(bucket, content, &hex_hash).await?;

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    // TODO Delete if insertion fails
    let inserted_pic =
        sql::insert_pano(&mut transaction, stop_pic_entry).await?;

    /*contrib::sql::insert_changeset_log(
        &mut *transaction,
        claims.uid,
        &[history::Change::StopPicUpload {
            pic: pic.clone(),
            stops: vec![stop_id],
        }],
        None,
    )
    .await?;*/

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(inserted_pic)
}

async fn upload_pano_to_storage(
    bucket: &s3::Bucket,
    content: &Bytes,
    hex_hash: &str,
) -> Result<(), Error> {
    bucket
        .put_object_with_content_type(
            format!("/pano/{hex_hash}"),
            content.as_ref(),
            mime::IMAGE_JPEG.as_ref(),
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}
