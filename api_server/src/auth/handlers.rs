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

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use super::{logic, models};
use crate::errors::Error;
use crate::AppState;

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
    Json(registration): Json<models::requests::Register>,
) -> Result<(), Error> {
    logic::register(registration, &state.pool).await
}

pub(crate) async fn post_login(
    State(state): State<AppState>,
    Json(request): Json<models::requests::Login>,
) -> Result<String, Error> {
    let user = logic::login(request, &state.pool).await?;
    Ok(user)
}

pub(crate) async fn post_admin_change_password(
    State(state): State<AppState>,
    claims: Option<models::Claims>,
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

    logic::admin_change_password(request, requester_id, &state.pool).await
}
pub(crate) async fn post_user_change_password(
    State(state): State<AppState>,
    claims: Option<models::Claims>,
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

    logic::change_password(request, requester_id, &state.pool).await
}
