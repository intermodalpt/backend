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
use axum_client_ip::SecureClientIp;
use futures::future;
use serde::Deserialize;

use commons::models::auth;

use super::{
    logic, models,
    models::{requests, responses},
    sql,
};
use crate::errors::Error;
use crate::responses::Pagination;
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
                return Err(Error::Forbidden);
            }

            logic::register(registration, client_ip.0, &state.pool).await
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
    Json(request): Json<requests::Login>,
) -> Result<String, Error> {
    let user = logic::login(request, client_ip.0, &state.pool).await?;
    Ok(user)
}

pub(crate) async fn post_admin_change_password(
    State(state): State<AppState>,
    super::ScopedClaim(claims, _): super::ScopedClaim<super::perms::Admin>,
    client_ip: SecureClientIp,
    Json(request): Json<requests::ChangeUnknownPassword>,
) -> Result<(), Error> {
    logic::admin_change_password(request, claims.uid, client_ip.0, &state.pool)
        .await
}
pub(crate) async fn post_user_change_password(
    State(state): State<AppState>,
    claims: models::Claims,
    client_ip: SecureClientIp,
    Json(request): Json<requests::ChangeKnownPassword>,
) -> Result<(), Error> {
    let models::Claims {
        uid: requester_id,
        uname: requester_username,
        ..
    } = claims;

    if requester_username != request.username {
        return Err(Error::Forbidden);
    }

    logic::change_password(request, requester_id, client_ip.0, &state.pool)
        .await
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
