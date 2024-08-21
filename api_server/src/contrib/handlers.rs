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

use axum::extract::{Multipart, Path, Query, State};
use axum::Json;
use chrono::Local;
use futures::future;
use serde::Deserialize;

use commons::models::{history, stops};

use super::{logic, requests, responses, sql};
use crate::errors::Error;
use crate::responses::{IdReturn, Pagination};
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
    claims: auth::Claims,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_decided_user_contributions(
            &state.pool,
            claims.uid,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn get_undecided_own_contributions(
    State(state): State<AppState>,
    claims: auth::Claims,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_undecided_user_contributions(
            &state.pool,
            claims.uid,
            offset,
            take,
        )
        .await?,
    ))
}

pub(crate) async fn get_decided_user_contributions(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::HandleContrib>,
    Path(user_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
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
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::HandleContrib>,
    Path(user_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Vec<history::Contribution>>, Error> {
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
    claims: auth::Claims,
) -> Result<Json<Vec<stops::Stop>>, Error> {
    let contributions =
        sql::fetch_user_stop_meta_contributions(&state.pool, claims.uid)
            .await?;
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
    _: Option<auth::Claims>,
    paginator: Query<PageForUser>,
) -> Result<Json<Pagination<responses::Contribution>>, Error> {
    // FIXME use the claims to censor pictures and other possibly-sensitive data
    // Maybe do this at worker level because the metadata itself should always
    // be public

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_undecided_contributions(
            &state.pool,
            paginator.uid,
            offset,
            take,
        ),
        sql::count_undecided_contributions(&state.pool, paginator.uid),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_latest_decided_contributions(
    State(state): State<AppState>,
    _: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::Contribution>>, Error> {
    // FIXME use the claims to censor pictures and other possibly-sensitive data
    // Maybe do this at worker level because the metadata itself should always
    // be public

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_decided_contributions(&state.pool, offset, take),
        sql::count_decided_contributions(&state.pool),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_changelog(
    State(state): State<AppState>,
    _: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::Changeset>>, Error> {
    // FIXME use the claims to censor pictures and other possibly-sensitive data
    // Maybe do this at worker level because the metadata itself should always
    // be public

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_changeset_logs(&state.pool, offset, take),
        sql::count_changeset_logs(&state.pool),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn post_contrib_stop_data(
    State(state): State<AppState>,
    claims: auth::Claims,
    Path(stop_id): Path<i32>,
    Json(contribution): Json<requests::NewStopMetaContribution>,
) -> Result<Json<IdReturn<i64>>, Error> {
    let stop: stops::Stop = crate::stops::sql::fetch_stop(&state.pool, stop_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?
        .into();

    let patch = contribution.contribution.derive_patch(&stop);

    if patch.is_empty() {
        return Ok(Json(IdReturn { id: -1 }));
    }

    let contribution = history::Contribution {
        id: 0,
        author_id: claims.uid,
        change: history::Change::StopUpdate {
            original: stop.into(),
            patch,
        },
        accepted: None,
        evaluator_id: None,
        evaluation_date: None,
        submission_date: Local::now(),
        comment: contribution.comment,
    };

    return Ok(Json(IdReturn {
        id: sql::insert_new_contribution(&state.pool, contribution).await?,
    }));
}

pub(crate) async fn post_contrib_stop_picture(
    State(state): State<AppState>,
    claims: auth::Claims,
    mut multipart: Multipart,
) -> Result<Json<IdReturn<i64>>, Error> {
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
        claims.uid,
        filename.clone(),
        &state.bucket,
        &state.pool,
        &content,
        &[],
    )
    .await?;

    let contribution = history::Contribution {
        id: 0,
        author_id: claims.uid,
        change: history::Change::StopPicUpload {
            pic: pic.into(),
            stops: vec![],
        },
        accepted: None,
        evaluator_id: None,
        evaluation_date: None,
        submission_date: Local::now(),
        comment: None,
    };

    return Ok(Json(IdReturn {
        id: sql::insert_new_contribution(&state.pool, contribution).await?,
    }));
}

pub(crate) async fn patch_contrib_stop_picture_meta(
    State(state): State<AppState>,
    claims: auth::Claims,
    Path(contribution_id): Path<i64>,
    Json(contribution_meta): Json<requests::NewPictureContribution>,
) -> Result<(), Error> {
    let mut contribution =
        sql::fetch_contribution(&state.pool, contribution_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?;

    if contribution.author_id != claims.uid {
        return Err(Error::Forbidden);
    }

    if contribution.accepted.is_some() {
        return Err(Error::ValidationFailure(
            "Contribution already processed".to_string(),
        ));
    }

    contribution.comment = contribution_meta.comment;

    let history::Change::StopPicUpload { mut pic, .. } = contribution.change
    else {
        return Err(Error::ValidationFailure(
            "Contribution is not a picture".to_string(),
        ));
    };

    pic.dyn_meta = contribution_meta.contribution;

    let db_pic = pics::sql::fetch_stop_pic(&state.pool, pic.id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    if db_pic.tagged {
        return Err(Error::DependenciesNotMet);
    }

    contribution.change = history::Change::StopPicUpload {
        pic,
        stops: history::vec_into_vec(contribution_meta.stops),
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
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::HandleContrib>,
    params: Query<ContribAcceptanceParam>,
    Path(contribution_id): Path<i64>,
) -> Result<(), Error> {
    let verify = params.verify.unwrap_or(false);

    logic::accept_contribution(
        &state.pool,
        contribution_id,
        claims.uid,
        verify,
        &params.ignored,
    )
    .await
}

pub(crate) async fn post_decline_contrib_data(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::HandleContrib>,
    Path(contribution_id): Path<i64>,
) -> Result<(), Error> {
    let contribution = sql::fetch_contribution(&state.pool, contribution_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    if contribution.accepted.is_some() {
        return Err(Error::DependenciesNotMet);
    }

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_guest_contribution_to_decline(
        &mut transaction,
        contribution_id,
        claims.uid,
    )
    .await?;

    // TODO, file deletions

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}
