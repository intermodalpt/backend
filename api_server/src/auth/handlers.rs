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
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_client_ip::SecureClientIp;
use serde::Deserialize;

use super::{logic, models, sql};
use crate::auth::models::responses;
use crate::errors::Error;
use crate::responses::Pagination;
use crate::AppState;

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}

const PAGE_SIZE: u32 = 100;

pub(crate) async fn check_auth(
    claims: Option<models::Claims>,
) -> Result<impl IntoResponse, Error> {
    println!("{claims:?}");
    if claims.is_some() {
        Ok(StatusCode::OK)
    } else {
        Err(Error::Forbidden)
    }
}

pub(crate) async fn post_register(
    State(state): State<AppState>,
    client_ip: SecureClientIp,
    Json(registration): Json<models::requests::Register>,
) -> Result<(), Error> {
    logic::register(registration, client_ip.0, &state.pool).await
}

pub(crate) async fn post_login(
    State(state): State<AppState>,
    client_ip: SecureClientIp,
    Json(request): Json<models::requests::Login>,
) -> Result<String, Error> {
    let user = logic::login(request, client_ip.0, &state.pool).await?;
    Ok(user)
}

pub(crate) async fn post_admin_change_password(
    State(state): State<AppState>,
    claims: Option<models::Claims>,
    client_ip: SecureClientIp,
    Json(request): Json<models::requests::ChangeUnknownPassword>,
) -> Result<(), Error> {
    let is_admin = matches!(
        claims,
        Some(models::Claims {
            permissions: models::Permissions { is_admin: true, .. },
            ..
        })
    );

    if !is_admin {
        return Err(Error::Forbidden);
    }

    let Some(models::Claims {
        uid: requester_id, ..
    }) = claims
    else {
        return Err(Error::Forbidden);
    };

    logic::admin_change_password(
        request,
        requester_id,
        client_ip.0,
        &state.pool,
    )
    .await
}
pub(crate) async fn post_user_change_password(
    State(state): State<AppState>,
    claims: Option<models::Claims>,
    client_ip: SecureClientIp,
    Json(request): Json<models::requests::ChangeKnownPassword>,
) -> Result<(), Error> {
    let Some(models::Claims {
        uid: requester_id,
        uname: requester_username,
        ..
    }) = claims
    else {
        return Err(Error::Forbidden);
    };

    if requester_username != request.username {
        return Err(Error::Forbidden);
    }

    logic::change_password(request, requester_id, client_ip.0, &state.pool)
        .await
}

pub(crate) async fn get_user_audit_log(
    State(state): State<AppState>,
    claims: Option<models::Claims>,
    _client_ip: SecureClientIp,
    paginator: Query<Page>,
    Path(user_id): Path<i32>,
) -> Result<Json<Pagination<responses::AuditLogEntry>>, Error> {
    let is_admin = matches!(
        claims,
        Some(models::Claims {
            permissions: models::Permissions { is_admin: true, .. },
            ..
        })
    );

    if !is_admin {
        return Err(Error::Forbidden);
    }

    // TODO log this access

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(Pagination {
        items: sql::fetch_audit_log_entries(&state.pool, offset, take).await?,
        total: sql::count_user_audit_logs(&state.pool, user_id).await? as usize,
    }))
}

pub(crate) async fn get_audit_log(
    State(state): State<AppState>,
    claims: Option<models::Claims>,
    _client_ip: SecureClientIp,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::AuditLogEntry>>, Error> {
    let is_admin = matches!(
        claims,
        Some(models::Claims {
            permissions: models::Permissions { is_admin: true, .. },
            ..
        })
    );

    if !is_admin {
        return Err(Error::Forbidden);
    }

    // TODO log this access

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    Ok(Json(Pagination {
        items: sql::fetch_audit_log_entries(&state.pool, offset, take).await?,
        total: sql::count_audit_logs(&state.pool).await? as usize,
    }))
}
