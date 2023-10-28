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
use chrono::Local;
use serde::Deserialize;

use commons::models::{history, stops};

use super::{logic, requests, responses, sql};
use crate::errors::Error;
use crate::utils::get_exactly_one_field;
use crate::{auth, pics, AppState};

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

#[derive(Deserialize, Default)]
pub(crate) struct PageForUser {
    #[serde(default)]
    p: u32,
    uid: Option<i32>,
}

const PAGE_SIZE: u32 = 20;

pub(crate) async fn get_decided_own_contributions(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }

    let user_id = claims.unwrap().uid;

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_decided_user_contributions(
            &state.pool,
            user_id,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn get_undecided_own_contributions(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }

    let user_id = claims.unwrap().uid;

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_undecided_user_contributions(
            &state.pool,
            user_id,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn get_decided_user_contributions(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(user_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_decided_user_contributions(
            &state.pool,
            user_id,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn get_undecided_user_contributions(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(user_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_undecided_user_contributions(
            &state.pool,
            user_id,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn get_pending_stop_patch(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
) -> Result<Json<Vec<stops::Stop>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    let user_id = claims.uid;

    let contributions =
        sql::fetch_user_stop_meta_contributions(&state.pool, user_id).await?;
    let modified_stops =
        logic::summarize_stop_meta_contributions(contributions);

    Ok(Json(modified_stops))
}

pub(crate) async fn get_undecided_contribution_contributors(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::Contributor>>, Error> {
    Ok(Json(
        sql::fetch_undecided_contribution_contributors(&state.pool).await?,
    ))
}

pub(crate) async fn get_latest_undecided_contributions(
    State(state): State<AppState>,
    _claims: Option<auth::Claims>,
    paginator: Query<PageForUser>,
) -> Result<Json<responses::Page<responses::Contribution>>, Error> {
    // FIXME use the claims to censor pictures and other possibly-sensitive data
    // Maybe do this at worker level because the metadata itself should always
    // be public

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(responses::Page {
        items: sql::fetch_undecided_contributions(
            &state.pool,
            paginator.uid,
            offset,
            take,
        )
        .await?,
        total: sql::count_undecided_contributions(&state.pool, paginator.uid)
            .await? as usize,
    }))
}

pub(crate) async fn get_latest_decided_contributions(
    State(state): State<AppState>,
    _claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<responses::Page<responses::Contribution>>, Error> {
    // FIXME use the claims to censor pictures and other possibly-sensitive data
    // Maybe do this at worker level because the metadata itself should always
    // be public

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(responses::Page {
        items: sql::fetch_decided_contributions(&state.pool, offset, take)
            .await?,
        total: sql::count_decided_contributions(&state.pool).await? as usize,
    }))
}

pub(crate) async fn get_changelog(
    State(state): State<AppState>,
    _claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<responses::Page<responses::Changeset>>, Error> {
    // FIXME use the claims to censor pictures and other possibly-sensitive data
    // Maybe do this at worker level because the metadata itself should always
    // be public

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(responses::Page {
        items: sql::fetch_changeset_logs(&state.pool, offset, take).await?,
        total: sql::count_changeset_logs(&state.pool).await? as usize,
    }))
}

pub(crate) async fn post_contrib_stop_data(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(stop_id): Path<i32>,
    Json(contribution): Json<requests::NewStopMetaContribution>,
) -> Result<Json<HashMap<String, i64>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let user_id = claims.unwrap().uid;

    let stop = crate::stops::sql::fetch_stop(&state.pool, stop_id).await?;

    if stop.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let stop = stop.unwrap();
    let patch = contribution.contribution.derive_patch(&stop);

    if patch.is_empty() {
        return Ok(Json({
            let mut map = HashMap::new();
            map.insert("id".to_string(), -1);
            map
        }));
    }

    let contribution = history::Contribution {
        id: 0,
        author_id: user_id,
        change: history::Change::StopUpdate {
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
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    mut multipart: Multipart,
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
        &[],
    )
    .await?;

    let contribution = history::Contribution {
        id: 0,
        author_id: user_id,
        change: history::Change::StopPicUpload { pic, stops: vec![] },
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
    State(state): State<AppState>,
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
    let mut contribution: history::Contribution = contribution.unwrap();

    if contribution.author_id != claims.uid {
        return Err(Error::Forbidden);
    }

    if contribution.accepted.is_some() {
        return Err(Error::ValidationFailure(
            "Contribution already processed".to_string(),
        ));
    }

    contribution.comment = contribution_meta.comment;

    let mut pic = match contribution.change {
        history::Change::StopPicUpload { pic, .. } => pic,
        _ => {
            return Err(Error::ValidationFailure(
                "Contribution is not a picture".to_string(),
            ))
        }
    };

    pic.dyn_meta = contribution_meta.contribution;

    let db_pic = pics::sql::fetch_picture(&state.pool, pic.id).await?;
    if db_pic.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let db_pic = db_pic.unwrap();

    if db_pic.tagged {
        return Err(Error::DependenciesNotMet);
    }

    contribution.change = history::Change::StopPicUpload {
        pic,
        stops: contribution_meta.stops,
    };

    sql::update_contribution(
        &state.pool,
        contribution_id,
        &contribution.change,
        &contribution.comment,
    )
    .await
}

#[derive(Deserialize)]
pub(crate) struct ContribAcceptanceParam {
    #[serde(default)]
    ignored: Option<String>,
    verify: Option<bool>,
}

pub(crate) async fn post_accept_contrib_data(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    params: Query<ContribAcceptanceParam>,
    Path(contribution_id): Path<i64>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();

    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let user_id = claims.uid;

    let verify = params.verify.unwrap_or(false);

    logic::accept_contribution(
        &state.pool,
        contribution_id,
        user_id,
        verify,
        &params.ignored,
    )
    .await
}

pub(crate) async fn post_decline_contrib_data(
    State(state): State<AppState>,
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
        &mut *transaction,
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