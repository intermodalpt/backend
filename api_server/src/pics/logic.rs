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
            pic: pic.clone().into(),
            stops: stops
                .into_iter()
                .map(|stop_id| {
                    pics::StopAttrs {
                        id: *stop_id,
                        attrs: vec![],
                    }
                    .into()
                })
                .collect::<Vec<history::pics::StopAttrs>>(),
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
    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let stop_rels =
        sql::fetch_picture_stops_rel_attrs(&mut transaction, pic.id).await?;

    let stop_pic = sql::fetch_picture(db_pool, pic.id)
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
            pic: pic.into(),
            stops: history::vec_into_vec(stop_rels),
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

    // TODO log
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

pub(crate) async fn upload_operator_logo(
    operator_id: i32,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
    filename: &str,
    content: &Bytes,
) -> Result<(), Error> {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let res =
        sql::fetch_operator_logo_hash(&mut transaction, operator_id).await?;

    if let Some(Some(existing_hash)) = res {
        if existing_hash == hex_hash {
            return Ok(());
        }
        delete_operator_pic_from_storage(&bucket, &existing_hash, operator_id)
            .await?;
    }

    let path = std::path::Path::new(&filename);
    let ext = path.extension();
    if ext.is_none() {
        return Err(Error::DependenciesNotMet);
    }
    let ext = ext.unwrap();
    // Ensure valid
    if ext.eq_ignore_ascii_case("svg") {
        let svg_data = String::from_utf8(content.to_vec()).map_err(|_e| {
            Error::ValidationFailure("Bad image data".to_string())
        })?;
        let _ = svg::read(&svg_data)
            .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    } else if ext.eq_ignore_ascii_case("png")
        || ext.eq_ignore_ascii_case("jpg")
        || ext.eq_ignore_ascii_case("webp")
    {
        // Ensure it is valid
        let _ = image::load_from_memory(content.as_ref())
            .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    } else {
        return Err(Error::DependenciesNotMet);
    }

    let mime = mime_guess::from_path(path)
        .first()
        .expect("Unable to deduce MIME despite whitelist");

    upload_operator_pic_to_storage(
        bucket,
        content,
        &hex_hash,
        operator_id,
        mime.as_ref(),
    )
    .await?;

    let db_res = sql::update_operator_logo_hash(
        &mut transaction,
        operator_id,
        Some(&hex_hash),
    )
    .await;

    if let Err(db_err) = db_res {
        let storage_res =
            delete_operator_pic_from_storage(&bucket, &hex_hash, operator_id)
                .await;
        if let Err(storage_err) = storage_res {
            eprintln!(
                "Reversion failure.\
                {hex_hash} was stored into opr. {operator_id}.\
                Database threw: {db_err}. Storage threw: {storage_err}"
            )
        }
        return Err(db_err);
    }

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

async fn upload_operator_pic_to_storage(
    bucket: &s3::Bucket,
    content: &Bytes,
    hex_hash: &str,
    operator_id: i32,
    mime: &str,
) -> Result<(), Error> {
    bucket
        .put_object_with_content_type(
            format!("/operators/{operator_id}/{hex_hash}"),
            content.as_ref(),
            mime,
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

async fn delete_operator_pic_from_storage(
    bucket: &s3::Bucket,
    hex_hash: &str,
    operator_id: i32,
) -> Result<(), Error> {
    bucket
        .delete_object(format!("/operators/{operator_id}/{hex_hash}"))
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

pub(crate) async fn upload_news_item_img(
    item_id: i32,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
    filename: &str,
    content: &Bytes,
) -> Result<String, Error> {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let existing_hashes =
        sql::fetch_news_img_hashes(&mut transaction, item_id).await?;
    let existing_hashes = existing_hashes.ok_or(Error::NotFoundUpstream)?;

    if existing_hashes.contains(&hex_hash) {
        return Ok(hex_hash);
    }

    let path = std::path::Path::new(&filename);
    let ext = path.extension();
    if ext.is_none() {
        return Err(Error::DependenciesNotMet);
    }
    let ext = ext.unwrap();
    // Ensure valid
    if ext.eq_ignore_ascii_case("png")
        || ext.eq_ignore_ascii_case("jpg")
        || ext.eq_ignore_ascii_case("webp")
    {
        // Ensure it is valid
        let _ = image::load_from_memory(content.as_ref())
            .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    } else {
        return Err(Error::DependenciesNotMet);
    }

    let mime = mime_guess::from_path(path)
        .first()
        .expect("Unable to deduce MIME despite whitelist");

    upload_news_img_to_storage(bucket, content, &hex_hash, mime.as_ref())
        .await?;

    let db_res =
        sql::insert_news_img(&mut transaction, item_id, &hex_hash).await;

    if let Err(db_err) = db_res {
        let storage_res =
            delete_news_img_from_storage(&bucket, &hex_hash).await;
        if let Err(storage_err) = storage_res {
            eprintln!(
                "Reversion failure.\
                {hex_hash} was stored into news_item. {item_id}.\
                Database threw: {db_err}. Storage threw: {storage_err}"
            )
        }
        return Err(db_err);
    }

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(hex_hash)
}

async fn upload_news_img_to_storage(
    bucket: &s3::Bucket,
    content: &Bytes,
    hex_hash: &str,
    mime: &str,
) -> Result<(), Error> {
    bucket
        .put_object_with_content_type(
            format!("/news/{hex_hash}"),
            content.as_ref(),
            mime,
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

async fn delete_news_img_from_storage(
    bucket: &s3::Bucket,
    hex_hash: &str,
) -> Result<(), Error> {
    bucket
        .delete_object(format!("/news/{hex_hash}"))
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

pub(crate) async fn upload_external_news_item_img(
    item_id: i32,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
    filename: &str,
    content: &Bytes,
) -> Result<String, Error> {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let existing_hashes =
        sql::fetch_external_news_img_hashes(&mut transaction, item_id).await?;
    let existing_hashes = existing_hashes.ok_or(Error::NotFoundUpstream)?;

    if existing_hashes.contains(&hex_hash) {
        return Ok(hex_hash);
    }

    let path = std::path::Path::new(&filename);
    let ext = path.extension();
    if ext.is_none() {
        return Err(Error::DependenciesNotMet);
    }
    let ext = ext.unwrap();
    // Ensure valid
    if ext.eq_ignore_ascii_case("png")
        || ext.eq_ignore_ascii_case("jpg")
        || ext.eq_ignore_ascii_case("webp")
    {
        // Ensure it is valid
        let _ = image::load_from_memory(content.as_ref())
            .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    } else {
        return Err(Error::DependenciesNotMet);
    }

    let mime = mime_guess::from_path(path)
        .first()
        .expect("Unable to deduce MIME despite whitelist");

    upload_external_news_img_to_storage(
        bucket,
        content,
        &hex_hash,
        mime.as_ref(),
    )
    .await?;

    let db_res =
        sql::insert_external_news_img(&mut transaction, item_id, &hex_hash)
            .await;

    if let Err(db_err) = db_res {
        let storage_res =
            delete_external_news_img_from_storage(&bucket, &hex_hash).await;
        if let Err(storage_err) = storage_res {
            eprintln!(
                "Reversion failure.\
                {hex_hash} was stored into news_item. {item_id}.\
                Database threw: {db_err}. Storage threw: {storage_err}"
            )
        }
        return Err(db_err);
    }

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(hex_hash)
}

async fn upload_external_news_img_to_storage(
    bucket: &s3::Bucket,
    content: &Bytes,
    hex_hash: &str,
    mime: &str,
) -> Result<(), Error> {
    bucket
        .put_object_with_content_type(
            format!("/enews/{hex_hash}"),
            content.as_ref(),
            mime,
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

async fn delete_external_news_img_from_storage(
    bucket: &s3::Bucket,
    hex_hash: &str,
) -> Result<(), Error> {
    bucket
        .delete_object(format!("/enews/{hex_hash}"))
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

pub(crate) async fn upload_news_item_screenshot(
    item_id: i32,
    bucket: &s3::Bucket,
    db_pool: &PgPool,
    filename: &str,
    content: &Bytes,
) -> Result<(), Error> {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = hasher.finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let existing_hashes =
        sql::fetch_external_news_screenshot_hashes(&mut transaction, item_id)
            .await?;
    let existing_hashes = existing_hashes.ok_or(Error::NotFoundUpstream)?;

    if existing_hashes.contains(&hex_hash) {
        return Ok(());
    }

    let path = std::path::Path::new(&filename);
    let ext = path.extension();
    if ext.is_none() {
        return Err(Error::DependenciesNotMet);
    }
    let ext = ext.unwrap();
    // Ensure valid
    if ext.eq_ignore_ascii_case("png")
        || ext.eq_ignore_ascii_case("jpg")
        || ext.eq_ignore_ascii_case("webp")
    {
        // Ensure it is valid
        let _ = image::load_from_memory(content.as_ref())
            .map_err(|err| Error::ValidationFailure(err.to_string()))?;
    } else {
        return Err(Error::DependenciesNotMet);
    }

    let mime = mime_guess::from_path(path)
        .first()
        .expect("Unable to deduce MIME despite whitelist");

    upload_external_news_item_screenshot_to_storage(
        bucket,
        content,
        &hex_hash,
        mime.as_ref(),
    )
    .await?;

    let db_res = sql::update_external_news_screenshot(
        &mut transaction,
        item_id,
        &hex_hash,
    )
    .await;

    if let Err(db_err) = db_res {
        let storage_res = delete_external_news_item_screenshot_from_storage(
            &bucket, &hex_hash,
        )
        .await;
        if let Err(storage_err) = storage_res {
            eprintln!(
                "Reversion failure.\
                {hex_hash} was stored into news_item_ss. {item_id}.\
                Database threw: {db_err}. Storage threw: {storage_err}"
            )
        }
        return Err(db_err);
    }

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

async fn upload_external_news_item_screenshot_to_storage(
    bucket: &s3::Bucket,
    content: &Bytes,
    hex_hash: &str,
    mime: &str,
) -> Result<(), Error> {
    bucket
        .put_object_with_content_type(
            format!("/enews_ss/{hex_hash}"),
            content.as_ref(),
            mime,
        )
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}

async fn delete_external_news_item_screenshot_from_storage(
    bucket: &s3::Bucket,
    hex_hash: &str,
) -> Result<(), Error> {
    bucket
        .delete_object(format!("/enews_ss/{hex_hash}"))
        .await
        .map_err(|err| Error::ObjectStorageFailure(err.to_string()))?;

    Ok(())
}
