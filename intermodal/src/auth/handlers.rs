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

use axum::headers::{authorization::Bearer, Authorization};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, TypedHeader};

use super::{logic, models};
use crate::errors::Error;
use crate::State;

pub(crate) async fn check_auth(
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<impl IntoResponse, Error> {
    if logic::try_get_user(auth.token(), &state.pool)
        .await?
        .is_some()
    {
        Ok((StatusCode::OK, "Success").into_response())
    } else {
        Ok((StatusCode::UNAUTHORIZED, "Failure").into_response())
    }
}

pub(crate) async fn post_register(
    Extension(state): Extension<Arc<State>>,
    Json(registration): Json<models::requests::Register>,
) -> Result<(), Error> {
    logic::register(registration, &state.pool).await
}

pub(crate) async fn post_login(
    Extension(state): Extension<Arc<State>>,
    Json(request): Json<models::requests::Login>,
) -> Result<String, Error> {
    let user = logic::login(request, &state.pool).await?;
    Ok(user)
}
