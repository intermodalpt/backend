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

use config::Config;
use sqlx::PgPool;

mod api;
mod models;
mod sql;

const FLOAT_TOLERANCE: f64 = 0.000_001;

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("SETTINGS"))
        .build()
        .unwrap();

    let pool = PgPool::connect(&settings.get_string("db").expect("db not set"))
        .await
        .expect("Unable to connect to the database");

    let counts = import(&pool).await.unwrap();

    println!("New stops: {}", counts.0);
    println!("Updated stops: {}", counts.1);
}

pub(crate) async fn import(
    db_pool: &PgPool,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut new_stops = vec![];
    let mut updated_stops = vec![];

    let stops = sql::fetch_stops(db_pool).await?;

    let stop_index = stops
        .into_iter()
        .map(|stop| (stop.external_id.clone(), stop))
        .collect::<HashMap<String, models::Stop>>();

    let mut osm_stop_ids = vec![];
    api::fetch_osm_stops()
        .await?
        .nodes
        .into_iter()
        .filter_map(|node| {
            if let models::XmlNodeTypes::Node(node) = node {
                Some(models::Stop::from(node))
            } else {
                None
            }
        })
        .for_each(|mut osm_stop| {
            osm_stop_ids.push(osm_stop.external_id.clone());
            if let Some(stop) = stop_index.get(&osm_stop.external_id) {
                osm_stop.id = stop.id;
                if (stop.lat - osm_stop.lat).abs() > FLOAT_TOLERANCE
                    || (stop.lon - osm_stop.lon).abs() > FLOAT_TOLERANCE
                    || stop.osm_name != osm_stop.osm_name
                {
                    updated_stops.push(osm_stop);
                }
            } else {
                new_stops.push(osm_stop);
            }
        });

    let counts = (new_stops.len(), updated_stops.len());

    sql::update_stops(db_pool, updated_stops).await?;
    sql::insert_stops(db_pool, new_stops).await?;
    sql::tag_missing_stops(db_pool, osm_stop_ids).await?;
    Ok(counts)
}
