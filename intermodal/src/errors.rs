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

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, PartialEq, Eq, ToSchema)]
pub enum Error {
    #[error("Failed to deserialize data from the database")]
    DatabaseDeserialization,
    #[error("Requested data not in the storage")]
    NotFoundUpstream,
    #[error("Access denied")]
    Forbidden,
    #[error("Dependencies for this action were not met")]
    DependenciesNotMet,
    #[error("The provided information failed validation:: `{0}`")]
    ValidationFailure(String),
    #[error("The data could not be handled: `{0}`")]
    Processing(String),
    #[error("Unable to comunicate with the storage: `{0}`")]
    ObjectStorageFailure(String),
    #[error("Unable to execute database transaction: `{0}`")]
    DatabaseExecution(String),
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
            Error::Forbidden => {
                (StatusCode::FORBIDDEN, format!("{}", &self)).into_response()
            }
            Error::DependenciesNotMet => {
                (StatusCode::FAILED_DEPENDENCY, format!("{}", &self)).into_response()
            }
            Error::ValidationFailure(_) => {
                (StatusCode::BAD_REQUEST, format!("{}", &self)).into_response()
            }
            Error::Processing(_) | Error::ObjectStorageFailure(_) | Error::DatabaseExecution(_) => {
                eprintln!("{:?}", &self);
                (StatusCode::INTERNAL_SERVER_ERROR, "The server had an internal error").into_response()
            }
            // _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &self))
            //     .into_response(),
        }
    }
}
