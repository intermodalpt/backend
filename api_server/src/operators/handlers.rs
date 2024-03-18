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

use axum::extract::{Path, State};
use axum::Json;
use chrono::NaiveDate;

use commons::models::{history, operators};

use super::models::{requests, responses};
use super::sql;
use crate::{auth, contrib, AppState, Error};

pub(crate) async fn get_operators(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::OperatorWithRegions>>, Error> {
    Ok(Json(sql::fetch_operators(&state.pool).await?))
}

pub(crate) async fn post_operator(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Json(change): Json<requests::ChangeOperator>,
) -> Result<Json<responses::Operator>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let operator = sql::insert_operator(&mut transaction, change).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(operator))
}

pub(crate) async fn patch_operator(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(operator_id): Path<i32>,
    Json(change): Json<requests::ChangeOperator>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_operator(&mut transaction, operator_id, change).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn get_operator_stops(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::OperatorStop>>, Error> {
    Ok(Json(
        sql::fetch_operator_stops(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn put_operator_stop(
    State(state): State<AppState>,
    Path((operator_id, stop_id)): Path<(i32, i32)>,
    claims: Option<auth::Claims>,
    Json(change): Json<requests::ChangeOperatorStop>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    // TODO log
    sql::upsert_operator_stop(&mut transaction, operator_id, stop_id, change)
        .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn delete_operator_stop(
    State(state): State<AppState>,
    Path((operator_id, stop_id)): Path<(i32, i32)>,
    claims: Option<auth::Claims>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    // TODO log
    sql::delete_operator_stop(&mut transaction, operator_id, stop_id).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn get_issues(
    State(state): State<AppState>,
) -> Result<Json<Vec<operators::Issue>>, Error> {
    Ok(Json(sql::fetch_issues(&state.pool).await?))
}

pub(crate) async fn get_issue(
    State(state): State<AppState>,
    Path(issue_id): Path<i32>,
) -> Result<Json<operators::Issue>, Error> {
    Ok(Json(sql::fetch_issue(&state.pool, issue_id).await?))
}

pub(crate) async fn get_operator_route_types(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::OperatorRouteType>>, Error> {
    Ok(Json(
        sql::fetch_operator_route_types(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn post_operator_route_type(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    claims: Option<auth::Claims>,
    Json(type_id): Json<requests::ChangeOperatorRouteType>,
) -> Result<Json<i32>, Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let id =
        sql::insert_operator_route_type(&mut transaction, operator_id, type_id)
            .await?;

    // TODO log

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(Json(id))
}

pub(crate) async fn patch_operator_route_type(
    State(state): State<AppState>,
    Path((operator_id, type_id)): Path<(i32, i32)>,
    claims: Option<auth::Claims>,
    Json(route_type): Json<requests::ChangeOperatorRouteType>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_operator_route_type(
        &mut transaction,
        operator_id,
        type_id,
        route_type,
    )
    .await?;

    // TODO log

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_operator_route_type(
    State(state): State<AppState>,
    Path((operator_id, type_id)): Path<(i32, i32)>,
    claims: Option<auth::Claims>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::delete_operator_route_type(&mut transaction, operator_id, type_id)
        .await?;

    // TODO log

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
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

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let id = sql::insert_issue(&mut transaction, &issue).await?;

    let issue = operators::Issue {
        id,
        ..operators::Issue::from(issue)
    };

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::IssueCreation { data: issue.into() }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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

    let issue = sql::fetch_issue(&state.pool, issue_id).await?;

    let patch = change.derive_patch(&issue);
    if patch.is_empty() {
        return Ok(());
    }

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::update_issue(&mut transaction, issue_id, change).await?;

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::IssueUpdate {
            original: issue.into(),
            patch,
        }],
        None,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    // TODO log
    let id =
        sql::insert_calendar(&mut transaction, operator_id, calendar).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

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

    let mut transaction = state
        .pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::delete_calendar(&mut transaction, operator_id, calendar_id).await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
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
