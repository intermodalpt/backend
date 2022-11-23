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

use axum::extract::Path;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::{Extension, Json, TypedHeader};
use chrono::Local;

use super::{models, models::requests, sql};
use crate::errors::Error;
use crate::{auth, stops, State};

pub(crate) async fn post_stop_contrib_data(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(stop_id): Path<i32>,
    Json(contribution): Json<requests::NewStopMetaContribution>,
) -> Result<Json<HashMap<String, i64>>, Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;
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
        change: models::Change::Stop {
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

pub(crate) async fn post_accept_contrib_data(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(contribution_id): Path<i64>,
) -> Result<(), Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;
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
        models::Change::Stop { original, patch } => {
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
        models::Change::StopPicContribution(pic_contribution) => {
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
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Path(contribution_id): Path<i64>,
) -> Result<(), Error> {
    let user_id = auth::get_user(auth.token(), &state.pool).await?;
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
