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

use axum::extract::{Multipart, Path, Query, State};
use axum::Json;
use serde::Deserialize;

use commons::models::{history, pics};

use super::{logic, models::requests, models::responses, sql};
use crate::pics::logic::import_external_news_img;
use crate::utils::get_exactly_one_field;
use crate::Error;
use crate::{auth, contrib, AppState};

pub(crate) async fn get_stop_pictures(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<responses::PicWithStops>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::perms::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.map(|c| c.uid);

    Ok(Json(
        sql::fetch_stop_pictures(&state.pool, stop_id, is_trusted, uid).await?,
    ))
}

pub(crate) async fn get_public_stop_pictures(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<Vec<responses::PublicStopPic>>, Error> {
    Ok(Json(
        sql::fetch_public_stop_pictures(&state.pool, stop_id).await?,
    ))
}

pub(crate) async fn get_picture_stop_rels(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
) -> Result<Json<HashMap<i32, Vec<i32>>>, Error> {
    if claims.is_none() {
        Ok(Json(
            sql::fetch_public_picture_stop_rels(&state.pool).await?,
        ))
    } else {
        Ok(Json(sql::fetch_picture_stop_rels(&state.pool).await?))
    }
}

pub(crate) async fn get_pictures(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
) -> Result<Json<Vec<responses::PicWithStops>>, Error> {
    Ok(Json(sql::fetch_pictures_with_stops(&state.pool).await?))
}

pub(crate) async fn get_pictures_map(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<responses::MinimalPicWithStops>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::perms::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.map(|c| c.uid);

    Ok(Json(
        sql::fetch_minimal_pictures_with_stops(&state.pool, is_trusted, uid)
            .await?,
    ))
}

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 20;

#[derive(Deserialize, Default)]
pub(crate) struct PicsPage {
    #[serde(default)]
    p: u32,
    #[serde(default)]
    tagged_only: bool,
    #[serde(default)]
    untagged_only: bool,
    #[serde(default)]
    user: Option<i32>,
}

pub(crate) async fn get_latest_stop_pictures(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    qs: Query<PicsPage>,
) -> Result<Json<Vec<responses::PicWithStops>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::perms::Permissions { is_admin: true, .. },
            ..
        })
    );

    let claims_matches_user = matches!(
        claims,
        Some(auth::Claims {
            uid,
            ..
        })
        if Some(uid) == qs.user
    );

    if !claims_matches_user && (qs.user.is_some() && !is_trusted) {
        return Err(Error::Forbidden);
    }

    let uid = qs.user.or(claims.map(|c| c.uid));
    let offset = i64::from(qs.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    if qs.tagged_only && qs.untagged_only {
        return Ok(Json(vec![]));
    }

    if qs.untagged_only {
        return Ok(Json(
            sql::fetch_untagged_pictures(
                &state.pool,
                is_trusted,
                uid,
                offset,
                take,
            )
            .await?,
        ));
    }

    if qs.tagged_only {
        return Ok(Json(
            sql::fetch_tagged_pictures(
                &state.pool,
                is_trusted,
                uid,
                offset,
                take,
            )
            .await?,
        ));
    }

    Ok(Json(
        sql::fetch_latest_pictures(&state.pool, is_trusted, uid, offset, take)
            .await?,
    ))
}

pub(crate) async fn get_user_stop_pictures(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    page_qs: Query<PicsPage>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<responses::PicWithStops>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::perms::Permissions { is_admin: true, .. },
            ..
        })
    );

    let requester_uid = claims.map(|c| c.uid);

    let is_self = requester_uid == Some(user_id);

    let offset = i64::from(page_qs.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_user_pictures(
            &state.pool,
            user_id,
            is_trusted || is_self,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn get_unpositioned_stop_pictures(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Vec<responses::MinimalPic>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::perms::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.map(|c| c.uid);

    Ok(Json(
        sql::fetch_unpositioned_pictures(
            &state.pool,
            is_trusted,
            uid,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn upload_dangling_stop_picture(
    State(state): State<AppState>,
    claims: auth::Claims,
    mut multipart: Multipart,
) -> Result<Json<pics::StopPic>, Error> {
    // TODO have some sort of rate limiting for untrusted users

    let field = get_exactly_one_field(&mut multipart).await?;

    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();
    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let pic = logic::upload_stop_picture(
        claims.uid,
        filename.clone(),
        &state.bucket,
        &state.pool,
        &content,
        &[],
    )
    .await?;

    Ok(Json(pic))
}

pub(crate) async fn upload_stop_picture(
    State(state): State<AppState>,
    claims: auth::Claims,
    Path(stop_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<responses::PicWithStops>, Error> {
    // TODO replace this with some rate limited for untrusted users
    // if !(claims.permissions.is_admin) {
    //     return Err(Error::Forbidden);
    // }

    let field = get_exactly_one_field(&mut multipart).await?;

    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();
    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let pic = logic::upload_stop_picture(
        claims.uid,
        filename.clone(),
        &state.bucket,
        &state.pool,
        &content,
        &[stop_id],
    )
    .await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::StopPicUpload {
            pic: pic.clone().into(),
            stops: vec![pics::StopAttrs {
                id: stop_id,
                attrs: vec![],
            }
            .into()],
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    let pic = responses::PicWithStops::from((
        pic,
        vec![pics::StopAttrs {
            id: stop_id,
            attrs: vec![],
        }],
    ));

    Ok(Json(pic))
}

pub(crate) async fn get_stop_picture_meta(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(picture_id): Path<i32>,
) -> Result<Json<responses::PicWithStops>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::perms::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.map(|c| c.uid);

    let pic = sql::fetch_picture_with_stops(&state.pool, picture_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    if (pic.tagged && !pic.sensitive) || Some(pic.uploader) == uid || is_trusted
    {
        Ok(Json(pic))
    } else {
        Err(Error::Forbidden)
    }
}

pub(crate) async fn patch_stop_picture_meta(
    State(state): State<AppState>,
    claims: auth::Claims,
    Path(stop_picture_id): Path<i32>,
    Json(stop_pic_meta): Json<requests::ChangeStopPic>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let pic = sql::fetch_picture(&mut *transaction, stop_picture_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    if !(claims.permissions.is_admin
        || !pic.tagged && pic.uploader == claims.uid)
    {
        return Err(Error::Forbidden);
    }

    let original_rels =
        sql::fetch_picture_stops_rel_attrs(&mut transaction, pic.id).await?;

    let new_stop_attrs = stop_pic_meta
        .stops
        .iter()
        .map(|rel| (rel.id, &rel.attrs))
        .collect::<HashMap<i32, &Vec<String>>>();

    if new_stop_attrs.len() != stop_pic_meta.stops.len() {
        return Err(Error::ValidationFailure(
            "Duplicate stop ids in the request".to_string(),
        ));
    }

    let attrs_changed = !(original_rels.len() == new_stop_attrs.len()
        && original_rels.iter().all(|stop_rel| {
            new_stop_attrs
                .get(&stop_rel.id)
                .is_some_and(|new_attrs| stop_rel.attrs == **new_attrs)
        }));

    let patch = stop_pic_meta.derive_patch(&pic);

    let changed = !patch.is_empty() || attrs_changed;

    if changed {
        contrib::sql::insert_changeset_log(
            &mut transaction,
            claims.uid,
            &[history::Change::StopPicMetaUpdate {
                pic_id: Some(stop_picture_id),
                original_meta: pic.dyn_meta.into(),
                original_stops: history::vec_into_vec(original_rels),
                meta_patch: patch,
                stops: history::vec_into_vec(stop_pic_meta.stops.clone()),
            }],
            None,
        )
        .await?;

        sql::update_picture_meta(
            &mut transaction,
            stop_picture_id,
            stop_pic_meta,
            claims.uid,
        )
        .await?;

        transaction.commit().await.map_err(|err| {
            tracing::error!("Transaction failed to commit: {err}");
            Error::DatabaseExecution
        })?;
    }

    Ok(())
}

pub(crate) async fn delete_picture(
    State(state): State<AppState>,
    claims: auth::Claims,
    Path(picture_id): Path<i32>,
) -> Result<(), Error> {
    let pic = sql::fetch_picture(&state.pool, picture_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    if !(claims.permissions.is_admin || pic.uploader == claims.uid) {
        return Err(Error::Forbidden);
    }

    logic::delete_picture(pic, claims.uid, &state.bucket, &state.pool).await
}

pub(crate) async fn get_picture_count_by_stop(
    State(state): State<AppState>,
) -> Result<Json<HashMap<i32, i32>>, Error> {
    Ok(Json(sql::fetch_picture_count_by_stop(&state.pool).await?))
}

pub(crate) async fn get_panos(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
) -> Result<Json<Vec<responses::FullPanoPic>>, Error> {
    Ok(Json(sql::fetch_panos(&state.pool, true).await?))
}
pub(crate) async fn upload_pano_picture(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::Admin>,
    mut multipart: Multipart,
) -> Result<Json<pics::PanoPic>, Error> {
    let field = get_exactly_one_field(&mut multipart).await?;

    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();
    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let pic = logic::upload_pano_picture(
        claims.uid,
        filename,
        &state.bucket,
        &state.pool,
        &content,
    )
    .await?;

    Ok(Json(pic))
}

pub(crate) async fn get_stop_pano(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    claims: Option<auth::Claims>,
) -> Result<Json<Option<responses::PanoPic>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::perms::Permissions { is_admin: true, .. },
            ..
        })
    );

    Ok(Json(
        sql::fetch_stop_pano(&state.pool, stop_id, is_trusted).await?,
    ))
}

pub(crate) async fn get_onion_skin(
    State(state): State<AppState>,
    Path(pano_id): Path<i32>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Trusted>,
) -> Result<Json<responses::PanoOnion>, Error> {
    Ok(Json(sql::fetch_pano_onion(&state.pool, pano_id).await?))
}

pub(crate) async fn post_upload_operator_logo(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(operator_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<(), Error> {
    let field = get_exactly_one_field(&mut multipart).await?;
    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();

    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    logic::upload_operator_logo(
        operator_id,
        &state.bucket,
        &state.pool,
        &filename,
        &content,
    )
    .await?;

    Ok(())
}

pub(crate) async fn post_news_image(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    mut multipart: Multipart,
) -> Result<Json<responses::FullNewsImg>, Error> {
    let field = get_exactly_one_field(&mut multipart).await?;

    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();
    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let img = logic::upload_standalone_news_img(
        &state.bucket,
        &state.pool,
        filename,
        &content,
    )
    .await?;

    Ok(Json(img.into()))
}

pub(crate) async fn post_news_item_image(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(item_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<responses::FullNewsImg>, Error> {
    let field = get_exactly_one_field(&mut multipart).await?;

    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();
    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let img = logic::upload_news_item_img(
        item_id,
        &state.bucket,
        &state.pool,
        filename,
        &content,
    )
    .await?;

    Ok(Json(img.into()))
}

pub(crate) async fn post_import_external_news_image(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(external_image_id): Path<i32>,
) -> Result<Json<responses::FullNewsImg>, Error> {
    let img =
        import_external_news_img(&state.bucket, &state.pool, external_image_id)
            .await?;

    Ok(Json(img.into()))
}

pub(crate) async fn patch_news_image_meta(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(img_id): Path<i32>,
    Json(mut news_img_meta): Json<requests::ChangeNewsPicMeta>,
) -> Result<(), Error> {
    news_img_meta.tidy();

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_news_img_meta(&mut transaction, img_id, &news_img_meta).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn post_external_news_image(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(item_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<responses::ExternalNewsImg>, Error> {
    let field = get_exactly_one_field(&mut multipart).await?;

    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();
    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let pic = logic::upload_external_news_item_img(
        item_id,
        &state.bucket,
        &state.pool,
        &filename,
        &content,
    )
    .await?;

    Ok(Json(pic.into()))
}

pub(crate) async fn put_external_news_screenshot(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(item_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<(), Error> {
    let field = get_exactly_one_field(&mut multipart).await?;

    let filename = field
        .file_name()
        .ok_or_else(|| {
            Error::ValidationFailure("File without a filename".to_string())
        })?
        .to_string();
    let content = field
        .bytes()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    logic::upload_news_item_screenshot(
        item_id,
        &state.bucket,
        &state.pool,
        &filename,
        &content,
    )
    .await?;

    Ok(())
}
