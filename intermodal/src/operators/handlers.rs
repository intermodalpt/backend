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

use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::headers::{authorization::Bearer, Authorization};
use axum::{Extension, Json, TypedHeader};
use chrono::NaiveDate;
use serde::Deserialize;

use super::models;
use super::models::responses;
use super::sql;
use crate::{auth, Error, State};

pub(crate) async fn get_operators(
    Extension(state): Extension<Arc<State>>,
) -> Result<Json<Vec<models::Operator>>, Error> {
    Ok(Json(sql::fetch_operators(&state.pool).await?))
}

pub(crate) async fn get_operator_calendars(
    Extension(state): Extension<Arc<State>>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::OperatorCalendar>>, Error> {
    Ok(Json(sql::fetch_calendars(&state.pool, operator_id).await?))
}

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 20;

pub(crate) async fn get_news(
    Extension(state): Extension<Arc<State>>,
    paginator: Query<Page>,
) -> Result<Json<Vec<models::NewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(sql::fetch_news(&state.pool, take, offset).await?))
}

pub(crate) async fn get_operator_news(
    Extension(state): Extension<Arc<State>>,
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
