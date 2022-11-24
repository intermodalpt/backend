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
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use axum::extract::{ContentLengthLimit, Multipart, Path, Query};
use axum::headers::{authorization::Bearer, Authorization};
use axum::{Extension, Json, TypedHeader};
use serde::Deserialize;

use super::{logic, models, sql};
use crate::auth;
use crate::utils::get_exactly_one_field;
use crate::{Error, State};

pub(crate) async fn get_public_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i32>,
) -> Result<Json<Vec<models::responses::PublicStopPic>>, Error> {
    Ok(Json(
        sql::fetch_public_stop_pictures(&state.pool, stop_id).await?,
    ))
}

pub(crate) async fn get_tagged_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    Path(stop_id): Path<i32>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<models::responses::TaggedStopPic>>, Error> {
    let _user_id = auth::get_user(auth.token(), &state.pool).await?;

    Ok(Json(
        sql::fetch_tagged_stop_pictures(&state.pool, stop_id).await?,
    ))
}

pub(crate) async fn get_picture_stop_rels(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<HashMap<i32, Vec<i32>>>, Error> {
    let _user_id = auth::get_user(auth.token(), &state.pool).await?;

    Ok(Json(sql::fetch_picture_stop_rels(&state.pool).await?))
}

pub(crate) async fn get_pictures(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<models::responses::TaggedStopPic>>, Error> {
    let _user_id = auth::get_user(auth.token(), &state.pool).await?;

    Ok(Json(sql::fetch_stop_pictures(&state.pool).await?))
}

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 20;

pub(crate) async fn get_untagged_stop_pictures(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    paginator: Query<Page>,
) -> Result<Json<Vec<models::responses::UntaggedStopPic>>, Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_untagged_stop_pictures(&state.pool, user_id, offset, take)
            .await?,
    ))
}

pub(crate) async fn upload_stop_picture(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        { 30 * 1024 * 1024 },
    >,
) -> Result<(), Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;

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

    let res = logic::upload_stop_picture(
        user_id,
        filename.clone(),
        &state.bucket,
        &state.pool,
        &content,
    )
    .await;

    if res.is_err() {
        sleep(Duration::from_secs(1));
        // Retry, just in case
        logic::upload_stop_picture(
            user_id,
            filename.clone(),
            &state.bucket,
            &state.pool,
            &content,
        )
        .await?;
    }

    Ok(())
}

pub(crate) async fn patch_stop_picture_meta(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(stop_picture_id): Path<i32>,
    Json(stop_pic_meta): Json<models::requests::ChangeStopPic>,
) -> Result<(), Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;

    sql::update_stop_picture_meta(
        &state.pool,
        stop_picture_id,
        stop_pic_meta,
        user_id,
    )
    .await
}

pub(crate) async fn delete_stop_picture(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(stop_picture_id): Path<i32>,
) -> Result<(), Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;

    if user_id != 1 && user_id != 2 {
        return Err(Error::Forbidden);
    }

    logic::delete_stop_picture(stop_picture_id, &state.bucket, &state.pool)
        .await
}
