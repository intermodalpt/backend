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

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_with::serde_derive::Serialize;

use commons::models::info;

use super::models::{requests, responses};
use super::sql;
use crate::auth::ClaimPermission;
use crate::responses::Pagination;
use crate::{auth, AppState, Error};

#[derive(Serialize)]
pub struct IdReturn {
    pub id: i32,
}

#[derive(Serialize)]
pub struct UrlReturn {
    pub url: String,
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
) -> Result<Json<Pagination<info::NewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(Pagination {
        items: sql::fetch_news(&state.pool, take, offset).await?,
        total: sql::count_news(&state.pool).await?,
    }))
}

pub(crate) async fn get_operator_news(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<info::NewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(Pagination {
        items: sql::fetch_operator_news(&state.pool, operator_id, take, offset)
            .await?,
        total: sql::count_operator_news(&state.pool, operator_id).await?,
    }))
}

pub(crate) async fn post_news(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Json(news_item): Json<requests::NewNewsItem>,
) -> Result<Json<IdReturn>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let id = sql::insert_news(&mut transaction, news_item).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(IdReturn { id }))
}

pub(crate) async fn get_external_news_item(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(item_id): Path<i32>,
) -> Result<Json<responses::ExternalNewsItem>, Error> {
    let include_private = claims
        .as_ref()
        .map(auth::perms::Trusted::is_valid)
        .unwrap_or(false);

    let item =
        sql::fetch_external_news_item(&state.pool, item_id, include_private)
            .await?;

    if let Some(item) = item {
        Ok(Json(item))
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn get_full_external_news_item(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(item_id): Path<i32>,
) -> Result<Json<responses::FullExternalNewsItem>, Error> {
    let item = sql::fetch_full_external_news_item(&state.pool, item_id).await?;

    if let Some(item) = item {
        Ok(Json(item))
    } else {
        Err(Error::NotFoundUpstream)
    }
}

pub(crate) async fn get_external_news(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::ExternalNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let include_private = claims
        .as_ref()
        .map(auth::perms::Trusted::is_valid)
        .unwrap_or(false);

    Ok(Json(Pagination {
        items: sql::fetch_external_news(
            &state.pool,
            offset,
            take,
            include_private,
        )
        .await?,
        total: sql::count_external_news(&state.pool, include_private).await?,
    }))
}

pub(crate) async fn get_operator_external_news(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Pagination<responses::ExternalNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let include_private = claims
        .as_ref()
        .map(auth::perms::Trusted::is_valid)
        .unwrap_or(false);

    Ok(Json(Pagination {
        items: sql::fetch_operator_external_news(
            &state.pool,
            operator_id,
            offset,
            take,
            include_private,
        )
        .await?,
        total: sql::count_operator_external_news(
            &state.pool,
            operator_id,
            include_private,
        )
        .await?,
    }))
}

pub(crate) async fn get_pending_external_news(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::FullExternalNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(Pagination {
        items: sql::fetch_pending_external_news(&state.pool, offset, take)
            .await?,
        total: sql::count_pending_external_news(&state.pool).await?,
    }))
}

pub(crate) async fn get_operator_pending_external_news(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    paginator: Query<Page>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Pagination<responses::FullExternalNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(Pagination {
        items: sql::fetch_pending_operator_external_news(
            &state.pool,
            operator_id,
            offset,
            take,
        )
        .await?,
        total: sql::count_pending_operator_external_news(
            &state.pool,
            operator_id,
        )
        .await?,
    }))
}

pub(crate) async fn post_external_news(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Json(news_item): Json<requests::NewExternalNewsItem>,
) -> Result<Json<IdReturn>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let id = sql::insert_external_news(&mut transaction, news_item).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(IdReturn { id }))
}

pub(crate) async fn delete_external_news(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(item_id): Path<i32>,
) -> Result<(), Error> {
    sql::delete_external_news_item(&state.pool, item_id).await?;
    Ok(())
}

pub(crate) async fn get_external_news_source_known_urls(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(source): Path<String>,
) -> Result<Json<Vec<String>>, Error> {
    Ok(Json(
        sql::fetch_external_news_source_urls(&state.pool, &source).await?,
    ))
}
pub(crate) async fn get_external_news_source_dump(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::Admin>,
    Path(source): Path<String>,
) -> Result<Json<Vec<responses::SourceExternalNewsItem>>, Error> {
    Ok(Json(
        sql::fetch_external_news_source_dump(&state.pool, &source).await?,
    ))
}
