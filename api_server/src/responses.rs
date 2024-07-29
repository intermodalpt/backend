/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023 - 2024  Cláudio Pereira

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

use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct IdReturn<T> {
    pub id: T,
}

#[derive(Serialize)]
pub struct UrlReturn {
    pub url: String,
}

#[derive(Serialize)]
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total: i64,
}

pub(crate) fn json_response_with_cookie_set<T>(
    cookie_key: &str,
    cookie_value: String,
    payload: T,
) -> Result<impl IntoResponse, Error>
where
    T: Serialize,
{
    let cookie = Cookie::build((cookie_key, cookie_value))
        .max_age(time::Duration::days(365))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Json(payload).into_response().into_body())
        .expect("Unable to create a body");

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}
