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
pub(crate) mod misc;
mod operators;
mod osm;
mod pics;
mod responses;
mod routes;
pub(crate) mod state;
mod stops;
mod utils;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use config::Config;
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

use errors::Error;
use state::{AppState, Cached, State};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_line_number(true)
        .init();

    let settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("SETTINGS"))
        .build()
        .unwrap();

    let _ = auth::SECRET_KEY.set(Box::leak(Box::new(
        settings
            .get_string("jwt_secret")
            .expect("jwt_secret not set"),
    )));

    let _ = pics::IMG_ROOT.set(Box::leak(Box::new(
        settings.get_string("img_root").expect("img_root not set"),
    )));

    let credentials = s3::creds::Credentials::new(
        Some(
            &settings
                .get_string("s3_access_key")
                .expect("s3_access_key not set"),
        ),
        Some(
            &settings
                .get_string("s3_secret_key")
                .expect("s3_secret_key not set"),
        ),
        None,
        None,
        None,
    )
    .unwrap();

    let bucket = s3::Bucket::new(
        &settings
            .get_string("s3_bucket_name")
            .expect("s3_bucket_name not set"),
        s3::Region::R2 {
            account_id: settings
                .get_string("s3_account_id")
                .expect("s3_account_id not set"),
        },
        credentials,
    )
    .unwrap()
    .with_path_style();

    let pool = PgPool::connect(&settings.get_string("db").expect("db not set"))
        .await
        .expect("Unable to connect to the database");
    let stats = misc::sql::get_stats(&pool).await.unwrap();
    let state = Arc::new(State {
        bucket,
        pool,
        stats,
        cached: Cached {
            gtfs_stops: RwLock::new(HashMap::new()),
            tml_routes: RwLock::new(HashMap::new()),
        },
    });

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        settings.get_int("port").expect("port not set") as u16,
    ));

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
        http::build_paths(state)
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Unable to start service");
}
