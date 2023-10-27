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

mod sql;

use config::Config;
use itertools::Itertools;
use sqlx::PgPool;

use crate::sql::{fetch_faulty_changeset_logs, JsonParseResult};

#[tokio::main]
async fn main() {
    env_logger::init();

    let settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("SETTINGS"))
        .build()
        .unwrap();

    let pool = PgPool::connect(&settings.get_string("db").expect("db not set"))
        .await
        .expect("Unable to connect to the database");

    let logs = fetch_faulty_changeset_logs(&pool).await;

    let mut ok_count = 0;
    let mut nok_count = 0;
    for log in logs {
        match log {
            JsonParseResult::Ok(changeset) => {
                ok_count += 1;
            }
            JsonParseResult::Err { raw, error, data } => {
                nok_count += 1;
                println!("Changeset: {:?}", data);
                println!("\tChanges: {}", raw);
                println!("\tError: {:?}", error);
            }
        }
    }
    println!("Ok: {}", ok_count);
    println!("Nok: {}", nok_count);
}
