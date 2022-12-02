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

use axum::extract::{ContentLengthLimit, Multipart, Path};
use axum::{Extension, Json};
use chrono::Local;

use super::{models, models::requests, sql};
use crate::errors::Error;
use crate::utils::get_exactly_one_field;
use crate::{auth, pics, stops, State};

pub(crate) async fn get_contributions(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<models::Contribution>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let user_id = claims.unwrap().uid;

    Ok(Json(
        sql::fetch_user_contributions(&state.pool, user_id).await?,
    ))
}
pub(crate) async fn get_changelog(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<models::Changeset>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }

    Ok(Json(sql::fetch_changeset_logs(&state.pool).await?))
}

pub(crate) async fn post_contrib_stop_data(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path(stop_id): Path<i32>,
    Json(contribution): Json<requests::NewStopMetaContribution>,
) -> Result<Json<HashMap<String, i64>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let user_id = claims.unwrap().uid;

    let stop = stops::sql::fetch_stop(&state.pool, stop_id).await?;

    if stop.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let stop = stop.unwrap();
    let patch = contribution.contribution.derive_patch(&stop);

    if patch.is_empty() {
        return Err(Error::ValidationFailure(
            "No changes were made".to_string(),
        ));
    }

    let contribution = models::Contribution {
        id: 0,
        author_id: user_id,
        change: models::Change::StopUpdate {
            original: stop,
            patch,
        },
        accepted: None,
        evaluator_id: None,
        evaluation_date: None,
        submission_date: Local::now(),
        comment: contribution.comment,
    };

    let id = sql::insert_new_contribution(&state.pool, contribution).await?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn post_contrib_stop_picture(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        { 30 * 1024 * 1024 },
    >,
) -> Result<Json<HashMap<String, i64>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let user_id = claims.unwrap().uid;

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

    // TODO limit maximum number of unverified pictures per user
    let pic = pics::logic::upload_stop_picture(
        user_id,
        filename.clone(),
        &state.bucket,
        &state.pool,
        &content,
    )
    .await?;

    let contribution = models::Contribution {
        id: 0,
        author_id: user_id,
        change: models::Change::StopPicUpload { pic, stops: vec![] },
        accepted: None,
        evaluator_id: None,
        evaluation_date: None,
        submission_date: Local::now(),
        comment: None,
    };

    let id = sql::insert_new_contribution(&state.pool, contribution).await?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn patch_contrib_stop_picture_meta(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path(contribution_id): Path<i64>,
    Json(contribution_meta): Json<requests::NewPictureContribution>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    let contribution =
        sql::fetch_contribution(&state.pool, contribution_id).await?;
    if contribution.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let mut contribution: models::Contribution = contribution.unwrap();

    if contribution.author_id != claims.uid {
        return Err(Error::Forbidden);
    }

    if contribution.accepted.is_some() {
        return Err(Error::ValidationFailure(
            "Contribution already processed".to_string(),
        ));
    }

    contribution.comment = contribution_meta.comment;

    let pic = match contribution.change {
        models::Change::StopPicUpload { pic, .. } => pic,
        _ => {
            return Err(Error::ValidationFailure(
                "Contribution is not a picture".to_string(),
            ))
        }
    };

    let pic = pics::sql::fetch_stop_picture(&state.pool, pic.id).await?;
    if pic.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let pic = pic.unwrap();

    contribution.change = models::Change::StopPicUpload {
        pic,
        stops: contribution_meta.stops,
    };

    sql::update_contribution(&state.pool, &contribution).await
}

pub(crate) async fn post_accept_contrib_data(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path(contribution_id): Path<i64>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let user_id = claims.unwrap().uid;

    let contribution =
        sql::fetch_contribution(&state.pool, contribution_id).await?;

    if contribution.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let contribution = contribution.unwrap();

    if contribution.accepted.is_some() {
        return Err(Error::DependenciesNotMet);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    match &contribution.change {
        models::Change::StopUpdate { original, patch } => {
            let stop = stops::sql::fetch_stop(&state.pool, original.id).await?;
            if stop.is_none() {
                return Err(Error::NotFoundUpstream);
            }
            let mut stop = stop.unwrap();
            patch.apply(&mut stop);

            stops::sql::update_stop(
                &mut transaction,
                original.id,
                stop.into(),
                user_id,
            )
            .await?;
        }
        models::Change::StopPicUpload { pic, stops } => {
            todo!()
        }
        _ => {
            unreachable!()
        }
    }

    sql::insert_changeset_log(
        &mut transaction,
        user_id,
        &vec![contribution.change],
        Some(contribution_id),
    )
    .await?;

    sql::update_guest_contribution_to_accept(
        &mut transaction,
        contribution_id,
        user_id,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn post_decline_contrib_data(
    Extension(state): Extension<Arc<State>>,
    claims: Option<auth::Claims>,
    Path(contribution_id): Path<i64>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let user_id = claims.unwrap().uid;

    let contribution =
        sql::fetch_contribution(&state.pool, contribution_id).await?;

    if contribution.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let contribution = contribution.unwrap();

    if contribution.accepted.is_some() {
        return Err(Error::DependenciesNotMet);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_guest_contribution_to_decline(
        &mut transaction,
        contribution_id,
        user_id,
    )
    .await?;

    // TODO, file deletions

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}
