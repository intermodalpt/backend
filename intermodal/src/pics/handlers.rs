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

use std::collections::HashMap;

use axum::extract::{Multipart, Path, Query, State};
use axum::Json;
use serde::Deserialize;

use super::{logic, models::requests, models::responses, sql};
use crate::utils::get_exactly_one_field;
use crate::Error;
use crate::{auth, contrib, AppState};

pub(crate) async fn get_public_stop_pictures(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
) -> Result<Json<Vec<responses::PublicStopPic>>, Error> {
    Ok(Json(
        sql::fetch_public_stop_pictures(&state.pool, stop_id).await?,
    ))
}

pub(crate) async fn get_tagged_stop_pictures(
    State(state): State<AppState>,
    Path(stop_id): Path<i32>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<responses::StopPic>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }

    Ok(Json(
        sql::fetch_stop_stop_pictures(&state.pool, stop_id).await?,
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
) -> Result<Json<Vec<responses::StopPic>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }

    Ok(Json(sql::fetch_stop_pictures(&state.pool).await?))
}

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 20;

pub(crate) async fn get_dangling_stop_pictures(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Vec<responses::StopPic>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_untagged_stop_pictures(
            &state.pool,
            claims.uid,
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
) -> Result<Json<responses::StopPic>, Error> {
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
        &[contrib::models::Change::StopPicUpload {
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
) -> Result<Json<responses::StopPic>, Error> {
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
        &[contrib::models::Change::StopPicUpload {
            pic: pic.clone(),
            stops: vec![stop_id],
        }],
        None,
    )
    .await?;

    let mut pic = responses::StopPic::from(pic);
    pic.stops.push(stop_id);

    Ok(Json(pic))
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

    let pic = sql::fetch_stop_picture(&state.pool, stop_picture_id).await?;
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
    let pic = sql::fetch_stop_picture(&state.pool, stop_picture_id).await?;
    if pic.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let pic = pic.unwrap();

    let stops = sql::fetch_picture_stops(&state.pool, pic.id).await?;

    let patch = stop_pic_meta.derive_patch(&pic);

    if patch.is_empty() {
        return Err(Error::ValidationFailure(
            "No changes were made".to_string(),
        ));
    }

    if pic.tagged {
        contrib::sql::insert_changeset_log(
            &state.pool,
            claims.uid,
            &[contrib::models::Change::StopPicMetaUpdate {
                original_meta: pic.dyn_meta,
                original_stops: stops,
                meta_patch: patch,
                stops: stop_pic_meta.stops.clone(),
            }],
            None,
        )
        .await?;
    } else {
        contrib::sql::insert_changeset_log(
            &state.pool,
            claims.uid,
            &[contrib::models::Change::StopPicUpload {
                pic,
                stops: stop_pic_meta.stops.clone(),
            }],
            None,
        )
        .await?;
        // pic.tagged = true;
    }

    // TODO Do this
    // patch.apply(&mut pic);
    // and change the function bellow to take a pic instead of a change request
    // + uncomment above pic.tagged

    sql::update_stop_picture_meta(
        &state.pool,
        stop_picture_id,
        stop_pic_meta,
        claims.uid,
    )
    .await
}

pub(crate) async fn delete_stop_picture(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(stop_picture_id): Path<i32>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    // TODO put all of this in a transaction
    let pic = sql::fetch_stop_picture(&state.pool, stop_picture_id).await?;
    if pic.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let pic = pic.unwrap();

    if !(claims.permissions.is_admin || pic.uploader == claims.uid) {
        return Err(Error::Forbidden);
    }

    let stops = sql::fetch_picture_stops(&state.pool, pic.id).await?;

    logic::delete_stop_picture(stop_picture_id, &state.bucket, &state.pool)
        .await?;

    contrib::sql::insert_changeset_log(
        &state.pool,
        claims.uid,
        &[contrib::models::Change::StopPicDeletion { pic, stops }],
        None,
    )
    .await?;

    Ok(())
}
