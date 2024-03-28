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
use axum::Json;
use serde::Serialize;
use thiserror::Error;

use commons::models::pics;
use commons::models::pics::Resource;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    // TODO this error variant can be deleted with sqlx wrappers
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
    #[error("Data could not be handled")]
    Processing,
    #[error("Filesystem error")]
    Filesystem,
    #[error("Error manipulating incompatible data models")]
    ModelCompatibility,
    #[error("Error serializing data")]
    Serialization,
    #[error("Unable to communicate with the storage")]
    ObjectStorageFailure,
    #[error("Unable to execute database transaction")]
    DatabaseExecution,
    #[error("Unable to download an external resource")]
    UpstreamResourceDownload,
    #[error("Attempted to duplicate resource`")]
    DuplicatedResource(Box<Resource>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let message = self.to_string();
        match self {
            Error::DatabaseDeserialization => JsonErrorResponse::new_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                message,
            ),
            Error::NotFoundUpstream => {
                JsonErrorResponse::new_response(StatusCode::NOT_FOUND, message)
            }
            Error::Forbidden => {
                JsonErrorResponse::new_response(StatusCode::FORBIDDEN, message)
            }
            Error::DependenciesNotMet => JsonErrorResponse::new_response(
                StatusCode::FAILED_DEPENDENCY,
                message,
            ),
            Error::ValidationFailure(msg) => JsonErrorResponse::new_response(
                StatusCode::BAD_REQUEST,
                msg,
            ),
            Error::DuplicatedResource(resource) => match *resource {
                Resource::StopPic(pic) => {
                    let detail = DuplicatedPicDetail { existing: pic };
                    DetailedJsonErrorResponse::new_response(
                        StatusCode::CONFLICT,
                        message,
                        detail,
                    )
                }
                Resource::PanoPic(pano) => {
                    let detail = DuplicatedPanoDetail { existing: pano };
                    DetailedJsonErrorResponse::new_response(
                        StatusCode::CONFLICT,
                        message,
                        detail,
                    )
                }
            },
            Error::Processing | Error::UpstreamResourceDownload |
            Error::Serialization | Error::Filesystem | Error::ModelCompatibility  => {
                JsonErrorResponse::new_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "The server had an internal error".to_string(),
                )
            }
            Error::ObjectStorageFailure
            | Error::DatabaseExecution => {
                JsonErrorResponse::new_response(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "The server is unable to complete the request at the moment".to_string(),
                )
            }
        }
    }
}

impl From<commons::errors::Error> for Error {
    fn from(e: commons::errors::Error) -> Self {
        match e {
            commons::errors::Error::Download(_) => {
                Error::UpstreamResourceDownload
            }
            commons::errors::Error::Extraction(_) => Error::Processing,
            commons::errors::Error::Filesystem(_) => Error::Filesystem,
            commons::errors::Error::Conversion
            | commons::errors::Error::Patching { .. } => {
                Error::ModelCompatibility
            }
        }
    }
}

#[derive(Serialize)]
struct JsonErrorResponse {
    code: u16,
    message: String,
}

impl JsonErrorResponse {
    fn new_response(code: StatusCode, message: String) -> Response {
        (
            code,
            Json(Self {
                code: code.as_u16(),
                message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct DetailedJsonErrorResponse<D: serde::Serialize> {
    code: u16,
    message: String,
    detail: D,
}

impl<D: serde::Serialize> DetailedJsonErrorResponse<D> {
    fn new_response(code: StatusCode, message: String, detail: D) -> Response
    where
        D: serde::Serialize,
    {
        (
            code,
            Json(Self {
                code: code.as_u16(),
                message,
                detail,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct DuplicatedPicDetail {
    existing: pics::StopPic,
}

#[derive(Serialize)]
struct DuplicatedPanoDetail {
    existing: pics::PanoPic,
}
