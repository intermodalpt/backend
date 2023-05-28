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

use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::NaiveDate;
use serde::Deserialize;

use super::models::{self, requests, responses};
use super::sql;
use crate::{auth, contrib, AppState, Error};

pub(crate) async fn get_operators(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::Operator>>, Error> {
    Ok(Json(sql::fetch_operators(&state.pool).await?))
}

pub(crate) async fn get_issues(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::Issue>>, Error> {
    Ok(Json(sql::fetch_issues(&state.pool).await?))
}

pub(crate) async fn get_issue(
    State(state): State<AppState>,
    Path(issue_id): Path<i32>,
) -> Result<Json<models::Issue>, Error> {
    Ok(Json(sql::fetch_issue(&state.pool, issue_id).await?))
}

pub(crate) async fn get_operator_issues(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::Issue>>, Error> {
    Ok(Json(
        sql::fetch_issue_operators(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn post_issue(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Json(issue): Json<requests::NewIssue>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }
    let id = sql::insert_issue(&state.pool, &issue).await?;

    let issue = models::Issue {
        id,
        ..models::Issue::from(issue)
    };

    // TODO transaction
    contrib::sql::insert_changeset_log(
        &state.pool,
        claims.uid,
        &[contrib::models::Change::IssueCreation { data: issue }],
        None,
    )
    .await?;

    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn patch_issue(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(issue_id): Path<i32>,
    Json(change): Json<requests::ChangeIssue>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    // TODO transaction
    let issue = sql::fetch_issue(&state.pool, issue_id).await?;

    let patch = change.derive_patch(&issue);
    if patch.is_empty() {
        return Ok(());
    }
    sql::update_issue(&state.pool, issue_id, change).await?;

    // TODO transaction
    contrib::sql::insert_changeset_log(
        &state.pool,
        claims.uid,
        &[contrib::models::Change::IssueUpdate {
            original: issue,
            patch,
        }],
        None,
    )
    .await?;

    Ok(())
}

pub(crate) async fn get_calendars(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::OperatorCalendar>>, Error> {
    Ok(Json(sql::fetch_calendars(&state.pool).await?))
}

pub(crate) async fn get_operator_calendars(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::OperatorCalendar>>, Error> {
    Ok(Json(
        sql::fetch_operator_calendars(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn post_operator_calendar(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    claims: Option<auth::Claims>,
    Json(calendar): Json<requests::NewOperatorCalendar>,
) -> Result<Json<HashMap<String, i32>>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }
    let id = sql::insert_calendar(&state.pool, operator_id, calendar).await?;
    Ok(Json({
        let mut map = HashMap::new();
        map.insert("id".to_string(), id);
        map
    }))
}

pub(crate) async fn delete_operator_calendar(
    State(state): State<AppState>,
    Path((operator_id, calendar_id)): Path<(i32, i32)>,
    claims: Option<auth::Claims>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }
    // TODO do not allow deletion of calendars that are in use

    Ok(sql::delete_calendar(&state.pool, operator_id, calendar_id).await?)
}

pub(crate) async fn get_operator_calendars_for_date(
    State(state): State<AppState>,
    Path((operator_id, date)): Path<(i32, String)>,
) -> Result<Json<Vec<responses::OperatorCalendar>>, Error> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    Ok(Json(
        sql::fetch_calendars_for_date(&state.pool, operator_id, date).await?,
    ))
}

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 20;

pub(crate) async fn get_news(
    State(state): State<AppState>,
    paginator: Query<Page>,
) -> Result<Json<Vec<models::NewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(sql::fetch_news(&state.pool, take, offset).await?))
}

pub(crate) async fn get_operator_news(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Vec<responses::OperatorNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(
        sql::fetch_operator_news(&state.pool, operator_id, take, offset)
            .await?,
    ))
}
