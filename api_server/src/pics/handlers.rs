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
            permissions: auth::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.and_then(|c| Some(c.uid));

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
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<responses::PicWithStops>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    Ok(Json(sql::fetch_pictures_with_stops(&state.pool).await?))
}

pub(crate) async fn get_pictures_map(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<responses::MinimalPicWithStops>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.and_then(|c| Some(c.uid));

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

// TODO deprecate
pub(crate) async fn get_dangling_stop_pictures(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Vec<responses::PicWithStops>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.and_then(|c| Some(c.uid));

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_untagged_pictures(
            &state.pool,
            is_trusted,
            uid,
            offset,
            take,
        )
        .await?,
    ))
}

#[derive(Deserialize, Default)]
pub(crate) struct PicsPage {
    #[serde(default)]
    p: u32,
    #[serde(default)]
    tagged_only: bool,
    #[serde(default)]
    untagged_only: bool,
}

pub(crate) async fn get_latest_stop_pictures(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    qs: Query<PicsPage>,
) -> Result<Json<Vec<responses::PicWithStops>>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.and_then(|c| Some(c.uid));
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
            permissions: auth::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.and_then(|c| Some(c.uid));

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
    claims: Option<auth::Claims>,
    mut multipart: Multipart,
) -> Result<Json<pics::StopPic>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

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

    let pic = logic::upload_stop_picture(
        claims.uid,
        filename.clone(),
        &state.bucket,
        &state.pool,
        &content,
        &[],
    )
    .await?;

    contrib::sql::insert_changeset_log(
        &state.pool,
        claims.uid,
        &[history::Change::StopPicUpload {
            pic: pic.clone(),
            stops: vec![],
        }],
        None,
    )
    .await?;

    Ok(Json(pic.into()))
}

pub(crate) async fn upload_stop_picture(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(stop_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<responses::PicWithStops>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

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
        &state.pool,
        claims.uid,
        &[history::Change::StopPicUpload {
            pic: pic.clone(),
            stops: vec![stop_id],
        }],
        None,
    )
    .await?;

    let pic = responses::PicWithStops::from((pic, vec![stop_id]));

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
            permissions: auth::Permissions { is_admin: true, .. },
            ..
        })
    );
    let uid = claims.and_then(|c| Some(c.uid));

    let pic = sql::fetch_picture_with_stops(&state.pool, picture_id).await?;
    if pic.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let pic = pic.unwrap();

    if (pic.tagged && !pic.sensitive) || Some(pic.uploader) == uid || is_trusted
    {
        Ok(Json(pic))
    } else {
        Err(Error::Forbidden)
    }
}

pub(crate) async fn patch_stop_picture_meta(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(stop_picture_id): Path<i32>,
    Json(stop_pic_meta): Json<requests::ChangeStopPic>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    let pic = sql::fetch_picture(&state.pool, stop_picture_id).await?;
    if pic.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let pic = pic.unwrap();

    if !(claims.permissions.is_admin
        || !pic.tagged && pic.uploader == claims.uid)
    {
        return Err(Error::Forbidden);
    }

    //TODO as a transaction
    let stops = sql::fetch_picture_stops(&state.pool, pic.id).await?;

    let patch = stop_pic_meta.derive_patch(&pic);

    let changed = !(patch.is_empty() && stops == stop_pic_meta.stops);

    if changed {
        contrib::sql::insert_changeset_log(
            &state.pool,
            claims.uid,
            &[history::Change::StopPicMetaUpdate {
                original_meta: pic.dyn_meta,
                original_stops: stops,
                meta_patch: patch,
                stops: stop_pic_meta.stops.clone(),
            }],
            None,
        )
        .await?;
    }

    sql::update_picture_meta(
        &state.pool,
        stop_picture_id,
        stop_pic_meta,
        claims.uid,
    )
    .await
}

pub(crate) async fn delete_picture(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(picture_id): Path<i32>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    // TODO put all of this in a transaction
    let pic = sql::fetch_picture(&state.pool, picture_id).await?;
    if pic.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let pic = pic.unwrap();

    if !(claims.permissions.is_admin || pic.uploader == claims.uid) {
        return Err(Error::Forbidden);
    }

    let stops = sql::fetch_picture_stops(&state.pool, pic.id).await?;

    logic::delete_picture(picture_id, &state.bucket, &state.pool).await?;

    contrib::sql::insert_changeset_log(
        &state.pool,
        claims.uid,
        &[history::Change::StopPicDeletion { pic, stops }],
        None,
    )
    .await?;

    Ok(())
}

pub(crate) async fn get_picture_count_by_stop(
    State(state): State<AppState>,
) -> Result<Json<HashMap<i32, i32>>, Error> {
    Ok(Json(sql::fetch_picture_count_by_stop(&state.pool).await?))
}

pub(crate) async fn get_panos(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<responses::FullPanoPic>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    if !(claims.permissions.is_admin) {
        return Err(Error::Forbidden);
    }

    Ok(Json(sql::fetch_panos(&state.pool, true).await?))
}
pub(crate) async fn upload_pano_picture(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    mut multipart: Multipart,
) -> Result<Json<pics::PanoPic>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    if !(claims.permissions.is_admin) {
        return Err(Error::Forbidden);
    }

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
        filename.clone(),
        &state.bucket,
        &state.pool,
        &content,
    )
    .await?;

    /*contrib::sql::insert_changeset_log(
        &state.pool,
        claims.uid,
        &[history::Change::StopPicUpload {
            pic: pic.clone(),
            stops: vec![stop_id],
        }],
        None,
    )
    .await?;*/

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
            permissions: auth::Permissions { is_admin: true, .. },
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
    claims: Option<auth::Claims>,
) -> Result<Json<responses::PanoOnion>, Error> {
    let is_trusted = matches!(
        claims,
        Some(auth::Claims {
            permissions: auth::Permissions { is_admin: true, .. },
            ..
        })
    );

    if !is_trusted {
        return Err(Error::Forbidden);
    }

    Ok(Json(sql::fetch_pano_onion(&state.pool, pano_id).await?))
}
