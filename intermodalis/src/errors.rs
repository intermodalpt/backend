/*
    Intermodalis, transportation information aggregator
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

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use utoipa::Component;

#[derive(Error, Debug, PartialEq, Eq, Component)]
pub enum Error {
    #[error("Storage credentials refused")]
    DatabaseDeserialization,
    #[error("Requested data not in the storage")]
    NotFoundUpstream,
    #[error("The provided information failed validation")]
    ValidationFailure,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DatabaseDeserialization => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &self))
                    .into_response()
            }
            Error::NotFoundUpstream => {
                (StatusCode::NOT_FOUND, format!("{}", &self)).into_response()
            }
            Error::ValidationFailure => {
                (StatusCode::BAD_REQUEST, format!("{}", &self)).into_response()
            }
            // _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &self))
            //     .into_response(),
        }
    }
}
