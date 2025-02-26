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
use futures::future;
use serde::Deserialize;

use super::models::{requests, responses};
use super::sql;
use crate::pics::sql as pics_sql;
use crate::responses::{IdReturn, Pagination};
use crate::{auth, auth::ClaimPermission, AppState, Error};

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 20;

pub(crate) async fn get_news(
    State(state): State<AppState>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::NewsItemListing>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_news(&state.pool, offset, take),
        sql::count_news(&state.pool),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_operator_news(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::NewsItemListing>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_operator_news(&state.pool, operator_id, offset, take),
        sql::count_operator_news(&state.pool, operator_id),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_region_news(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::NewsItemListing>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_region_news(&state.pool, region_id, offset, take),
        sql::count_region_news(&state.pool, region_id),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_news_item(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> Result<Json<responses::NewsItem>, Error> {
    Ok(Json(
        sql::fetch_news_item(&state.pool, item_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?,
    ))
}

pub(crate) async fn get_full_news_item(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> Result<Json<responses::FullNewsItem>, Error> {
    Ok(Json(
        sql::fetch_full_news_item(&state.pool, item_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?,
    ))
}

pub(crate) async fn post_news_item(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::CreateNews>,
    Json(mut news_item): Json<requests::ChangeNewsItem>,
) -> Result<Json<IdReturn<i32>>, Error> {
    news_item
        .validate()
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

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

pub(crate) async fn patch_news_item(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyNews>,
    Path(item_id): Path<i32>,
    Json(mut change): Json<requests::ChangeNewsItem>,
) -> Result<(), Error> {
    change
        .validate()
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_news_item(&mut transaction, item_id, &change).await?;
    pics_sql::unlink_rich_images_from_news(&mut transaction, item_id).await?;
    for img_id in change.content.get_linked_images() {
        pics_sql::link_rich_image_to_news(&mut transaction, img_id, item_id)
            .await?;
    }

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_external_news_item(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    Path(item_id): Path<i32>,
) -> Result<Json<responses::ExternalNewsItem>, Error> {
    let include_private = claims.is_some_and(|c| {
        auth::perms::ReadPrivateExternalNews::is_valid(&c.permissions)
    });

    sql::fetch_external_news_item(&state.pool, item_id, include_private)
        .await?
        .map(Json)
        .ok_or(Error::NotFoundUpstream)
}

pub(crate) async fn get_full_external_news_item(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ReadPrivateExternalNews,
    >,
    Path(item_id): Path<i32>,
) -> Result<Json<responses::FullExternalNewsItem>, Error> {
    sql::fetch_full_external_news_item(&state.pool, item_id)
        .await?
        .map(Json)
        .ok_or(Error::NotFoundUpstream)
}

pub(crate) async fn get_external_news(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::ExternalNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let include_private = claims.is_some_and(|c| {
        auth::perms::ReadPrivateExternalNews::is_valid(&c.permissions)
    });

    let (items, total) = future::join(
        sql::fetch_external_news(&state.pool, offset, take, include_private),
        sql::count_external_news(&state.pool, include_private),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
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

    let include_private = claims.is_some_and(|c| {
        auth::perms::ReadPrivateExternalNews::is_valid(&c.permissions)
    });

    let (items, total) = future::join(
        sql::fetch_operator_external_news(
            &state.pool,
            operator_id,
            offset,
            take,
            include_private,
        ),
        sql::count_operator_external_news(
            &state.pool,
            operator_id,
            include_private,
        ),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_pending_external_news(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ReadPrivateExternalNews,
    >,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::ExternalNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_pending_external_news(&state.pool, offset, take),
        sql::count_pending_external_news(&state.pool),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_operator_pending_external_news(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ReadPrivateExternalNews,
    >,
    paginator: Query<Page>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Pagination<responses::FullExternalNewsItem>>, Error> {
    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_pending_operator_external_news(
            &state.pool,
            operator_id,
            offset,
            take,
        ),
        sql::count_pending_operator_external_news(&state.pool, operator_id),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn post_external_news_item(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyExternalNews>,
    Json(mut news_item): Json<requests::NewExternalNewsItem>,
) -> Result<Json<IdReturn<i32>>, Error> {
    news_item.tidy();

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

pub(crate) async fn patch_external_news_item(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyExternalNews>,
    Path(item_id): Path<i32>,
    Json(mut news_item): Json<requests::ChangeExternalNewsItem>,
) -> Result<(), Error> {
    news_item.tidy();

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_external_news_item(&mut transaction, item_id, news_item)
        .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_external_news_item(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyExternalNews>,
    Path(item_id): Path<i32>,
) -> Result<(), Error> {
    sql::delete_external_news_item(&state.pool, item_id).await?;
    Ok(())
}

pub(crate) async fn get_external_news_source_known_urls(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ReadPrivateExternalNews,
    >,
    Path(source): Path<String>,
) -> Result<Json<Vec<String>>, Error> {
    Ok(Json(
        sql::fetch_external_news_source_urls(&state.pool, &source).await?,
    ))
}
pub(crate) async fn get_external_news_source_dump(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ReadPrivateExternalNews,
    >,
    Path(source): Path<String>,
) -> Result<Json<Vec<responses::SourceExternalNewsItem>>, Error> {
    Ok(Json(
        sql::fetch_external_news_source_dump(&state.pool, &source).await?,
    ))
}
