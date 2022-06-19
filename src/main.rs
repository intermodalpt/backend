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

#![deny(
    nonstandard_style,
    warnings,
    unused,
    future_incompatible,
    clippy::all,
    clippy::pedantic
)]

mod consts;
mod errors;
mod handlers;
mod models;
mod utils;

use errors::Error;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::{Extension, Router};
use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub(crate) struct State {
    pub(crate) pool: SqlitePool,
}

pub(crate) fn build_paths(state: State) -> Router {
    Router::new()
        // Basic roots
        .route("/parishes", get(handlers::get_parishes))
        .route("/stops", get(handlers::get_stops))
        .route("/routes", get(handlers::get_routes))
        .route("/route/:route_id/schedule", get(handlers::get_schedule))
        .route(
            "/route/:route_id/schedule/:date",
            get(handlers::get_schedule_for_date),
        )
        .route("/route/:route_id/stops", get(handlers::get_route_stops))
        .layer(Extension(Arc::new(state)))
}

#[tokio::main]
async fn main() {
    let state = State {
        pool: SqlitePool::connect("sqlite:db.sqlite").await.expect(""),
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(build_paths(state).into_make_service())
        .await
        .expect("Unable to start service");
}
