/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023  Cláudio Pereira

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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use sqlx::PgPool;

use crate::gtfs;

#[allow(clippy::module_name_repetitions)]
pub type AppState = Arc<State>;

pub struct State {
    pub bucket: s3::Bucket,
    pub pool: PgPool,
    pub cached: Cached,
}

impl State {
    // For integration tests
    #[allow(unused)]
    pub fn test_state(pool: PgPool) -> State {
        let credentials =
            s3::creds::Credentials::new(Some(""), Some(""), None, None, None)
                .unwrap();
        let bucket = s3::Bucket::new(
            "",
            s3::Region::R2 {
                account_id: String::new(),
            },
            credentials,
        )
        .unwrap()
        .with_path_style();

        State {
            bucket,
            pool,
            cached: Cached {
                gtfs_stops: RwLock::new(HashMap::new()),
                tml_routes: RwLock::new(HashMap::new()),
            },
        }
    }
}

pub struct Cached {
    pub gtfs_stops: RwLock<HashMap<i32, Arc<Vec<commons::models::gtfs::Stop>>>>,
    pub tml_routes: RwLock<HashMap<i32, Arc<Vec<gtfs::models::TMLRoute>>>>,
}
