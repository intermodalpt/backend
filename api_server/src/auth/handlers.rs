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
use axum::response::IntoResponse;
use axum::Json;
use axum_client_ip::SecureClientIp;
use futures::future;
use serde::Deserialize;
use uuid::Uuid;

use commons::models::auth;

use super::{
    logic, models,
    models::{requests, responses},
    sql,
};
use crate::auth::extractor::UserAgent;
use crate::errors::Error;
use crate::responses::{json_response_with_cookie_set, Pagination};
use crate::AppState;

#[derive(Deserialize, Default)]
pub(crate) struct Page {
    #[serde(default)]
    p: u32,
}
#[derive(Deserialize, Default)]
pub(crate) struct RegistrationArgs {
    #[serde(default)]
    dry: bool,
}

const PAGE_SIZE: u32 = 100;

pub(crate) async fn check_auth(_: models::Claims) {}

pub(crate) async fn post_register(
    State(state): State<AppState>,
    args: Query<RegistrationArgs>,
    client_ip: SecureClientIp,
    Json(registration): Json<requests::Register>,
) -> Result<(), Error> {
    if args.dry {
        logic::is_valid_registration(&registration, &state.pool).await
    } else {
        if let Some(captcha) = &registration.captcha {
            let is_valid = state
                .captchas
                .attempt_captcha(captcha.uuid, &captcha.answer)?;

            if !is_valid {
                return Err(Error::ValidationFailure(
                    "Captcha validation failed".to_string(),
                ));
            }

            logic::register(&state.pool, registration, client_ip.0).await
        } else {
            // Are we going to ever have a registration without a captcha?
            // Maybe if nobody has registered in the past hour
            // logic::register(registration, client_ip.0, &state.pool).await
            Err(Error::Forbidden)
        }
    }
}

pub(crate) async fn post_username_availability(
    State(state): State<AppState>,
    Json(request): Json<requests::UsernameAvailability>,
) -> Result<Json<responses::UsernameAvailability>, Error> {
    if let Err(reason) = logic::validate_username(&request.username) {
        return Ok(Json(responses::UsernameAvailability::Invalid { reason }));
    }
    if sql::fetch_username_exists(&state.pool, &request.username).await? {
        return Ok(Json(responses::UsernameAvailability::Taken));
    }
    Ok(Json(responses::UsernameAvailability::Available))
}

pub(crate) async fn get_captcha(
    State(state): State<AppState>,
    // _client_ip: SecureClientIp,
) -> Result<Json<responses::CaptchaChallenge>, Error> {
    let (uuid, img) = state.captchas.gen_captcha()?;
    Ok(Json(responses::CaptchaChallenge { png: img, uuid }))
}

pub(crate) async fn post_login(
    State(state): State<AppState>,
    client_ip: SecureClientIp,
    UserAgent(user_agent): UserAgent,
    Json(request): Json<requests::Login>,
) -> Result<impl IntoResponse, Error> {
    let (refresh_claims, refresh_token) =
        logic::login(request, &state.pool, client_ip.0, &user_agent).await?;

    json_response_with_cookie_set(
        "refresh_token",
        refresh_token.0,
        refresh_claims,
    )
}

pub(crate) async fn get_renew_access_token(
    State(state): State<AppState>,
    claims: models::RefreshClaims,
    client_ip: SecureClientIp,
    UserAgent(user_agent): UserAgent,
) -> Result<impl IntoResponse, Error> {
    let (access_claims, access_token) =
        logic::renew_token(claims, &state.pool, client_ip.0, &user_agent)
            .await?;

    json_response_with_cookie_set("access_token", access_token.0, access_claims)
}

pub(crate) async fn get_management_tokens(
    State(state): State<AppState>,
    super::ScopedClaim(claims, _): super::ScopedClaim<super::perms::Admin>,
    client_ip: SecureClientIp,
) -> Result<Json<Vec<responses::ManagementToken>>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    // TODO have this as a param
    let show_revoked = false;

    let tokens = sql::fetch_user_management_tokens(
        &mut transaction,
        claims.uid,
        show_revoked,
    )
    .await?;

    sql::insert_audit_log_entry(
        &mut transaction,
        auth::AuditLogAction::QueryManagementTokens,
        claims.uid,
        Some(claims.jti),
        &client_ip.0.into(),
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Failed to commit transaction: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(tokens))
}

pub(crate) async fn post_create_management_token(
    State(state): State<AppState>,
    super::ScopedClaim(claims, _): super::ScopedClaim<super::perms::Admin>,
    client_ip: SecureClientIp,
    UserAgent(user_agent): UserAgent,
    Json(request): Json<requests::NewManagementToken>,
) -> Result<Json<responses::ManagementToken>, Error> {
    Ok(Json(
        logic::create_management_token(
            request,
            &state.pool,
            &claims,
            client_ip.0,
            &user_agent,
        )
        .await?,
    ))
}

pub(crate) async fn delete_revoke_management_token(
    State(state): State<AppState>,
    Path(token_id): Path<Uuid>,
    super::ScopedClaim(claims, _): super::ScopedClaim<super::perms::Admin>,
    client_ip: SecureClientIp,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_set_session_revoked(&mut transaction, token_id).await?;

    sql::insert_audit_log_entry(
        &mut transaction,
        auth::AuditLogAction::SessionRevoked {
            session_id: token_id,
            was_logout: false,
        },
        claims.uid,
        Some(claims.jti),
        &client_ip.0.into(),
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Failed to commit transaction: {err}");
        Error::DatabaseExecution
    })?;
    Ok(())
}

pub(crate) async fn post_admin_change_password(
    State(state): State<AppState>,
    super::ScopedClaim(claims, _): super::ScopedClaim<super::perms::Admin>,
    client_ip: SecureClientIp,
    Json(request): Json<requests::ChangeUnknownPassword>,
) -> Result<(), Error> {
    logic::admin_change_password(&state.pool, request, &claims, client_ip.0)
        .await
}
pub(crate) async fn post_user_change_password(
    State(state): State<AppState>,
    claims: models::Claims,
    client_ip: SecureClientIp,
    Json(request): Json<requests::ChangeKnownPassword>,
) -> Result<(), Error> {
    let requester = sql::fetch_user_by_id(&state.pool, claims.uid)
        .await?
        .ok_or(Error::Forbidden)?;

    if requester.username != request.username {
        return Err(Error::Forbidden);
    }

    logic::change_password(&state.pool, request, &claims, client_ip.0).await
}

pub(crate) async fn get_user_audit_log(
    State(state): State<AppState>,
    super::ScopedClaim(_, _): super::ScopedClaim<super::perms::Admin>,
    _: SecureClientIp,
    paginator: Query<Page>,
    Path(user_id): Path<i32>,
) -> Result<Json<Pagination<auth::AuditLogEntry>>, Error> {
    // TODO log this access

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_user_audit_log(&state.pool, user_id, offset, take),
        sql::count_user_audit_logs(&state.pool, user_id),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_audit_log(
    State(state): State<AppState>,
    super::ScopedClaim(_, _): super::ScopedClaim<super::perms::Admin>,
    _: SecureClientIp,
    paginator: Query<Page>,
) -> Result<Json<Pagination<responses::AuditLogEntry>>, Error> {
    // TODO log this access

    let offset = i64::from(paginator.p * PAGE_SIZE);
    let take = i64::from(PAGE_SIZE);

    let (items, total) = future::join(
        sql::fetch_audit_log_entries(&state.pool, offset, take),
        sql::count_audit_logs(&state.pool),
    )
    .await;

    Ok(Json(Pagination {
        items: items?,
        total: total?,
    }))
}

pub(crate) async fn get_user_sessions(
    State(state): State<AppState>,
    super::ScopedClaim(_, _): super::ScopedClaim<super::perms::Admin>,
    Path(user_id): Path<i32>,
    _: SecureClientIp,
) -> Result<Json<Vec<responses::UserSession>>, Error> {
    // TODO log this access
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let sessions = sql::fetch_user_sessions(&mut *transaction, user_id).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Failed to commit transaction: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(sessions))
}

pub(crate) async fn get_session_accesses(
    State(state): State<AppState>,
    super::ScopedClaim(_, _): super::ScopedClaim<super::perms::Admin>,
    Path(session_id): Path<Uuid>,
    _: SecureClientIp,
) -> Result<Json<Vec<responses::UserAccessSession>>, Error> {
    // TODO log this access

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let accesses =
        sql::fetch_session_accesses(&mut *transaction, session_id).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Failed to commit transaction: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(accesses))
}
