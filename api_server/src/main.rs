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

#![warn(
    nonstandard_style,
    warnings,
    unused,
    future_incompatible,
    clippy::all,
    clippy::pedantic
)]

mod auth;
mod contrib;
mod errors;
mod geo;
pub(crate) mod gtfs;
mod http;
pub mod info;
mod operators;
mod osm;
mod pics;
mod responses;
mod routes;
pub(crate) mod settings;
pub(crate) mod state;
mod stops;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

use errors::Error;
pub(crate) use settings::SETTINGS;
use state::{AppState, State};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_line_number(true)
        .init();

    settings::load();

    let settings = SETTINGS.get().unwrap();
    let credentials = s3::creds::Credentials::new(
        Some(settings.s3.access_key.as_str()),
        Some(settings.s3.secret_key.as_str()),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket_name = settings.s3.bucket_name.as_str();
    let bucket = s3::Bucket::new(
        bucket_name,
        s3::Region::R2 {
            account_id: settings.s3.account_id.clone(),
        },
        credentials,
    )
    .unwrap()
    .with_path_style();
    tracing::info!("Configured to use the {bucket_name} bucket");
    if !(bucket_name.ends_with("dev") || bucket_name.ends_with("test")) {
        tracing::warn!("Using a production bucket");
    }

    let db_url = settings.db.url.as_str();
    let (_, db_selection) =
        db_url.rsplit_once('@').expect("Invalid database URL");
    let pool = PgPool::connect(db_url)
        .await
        .expect("Unable to connect to the database");
    tracing::info!("Connected to the {db_selection} database");
    if !(db_selection.ends_with("tests") || db_selection.ends_with("dev")) {
        tracing::warn!("Using the production database");
    }

    let state = Arc::new(State::new(bucket, pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.http.port));

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => {
            tracing::info!("Listening on {}", addr);
            listener
        }
        Err(err) => {
            tracing::error!("Unable to bind to socket: {}", err);
            return;
        }
    };

    axum::serve(
        listener,
        http::build_paths(AppState(state))
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Unable to start service");
}
