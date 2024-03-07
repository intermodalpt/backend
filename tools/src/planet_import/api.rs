/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2024  Cláudio Pereira

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

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use commons::models::osm;

const API_URL: &str = "https://api.intermodal.pt";

pub(crate) static TOKEN: OnceCell<&'static str> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize)]
pub struct OsmHistoryPatch {
    pub id: i64,
    pub history: osm::NodeHistory,
}

pub(crate) async fn fetch_cached_osm_stop_versions(
) -> Result<HashMap<i64, Vec<i32>>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/osm/stops/versions", API_URL);
    println!("Fetching {}", url);
    let res = reqwest::Client::new()
        .get(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await?;

    if res.status().is_success() {
        let stops: HashMap<i64, Vec<i32>> = res.json().await?;
        Ok(stops)
    } else {
        eprintln!("API error");
        eprintln!("Status: {}", res.status());
        std::process::exit(1);
    }
}

pub(crate) async fn patch_osm_stops_history(
    osm_histories: &[OsmHistoryPatch],
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/osm/stops", API_URL);
    println!("Fetching {}", url);
    let res = reqwest::Client::new()
        .patch(&url)
        .json(osm_histories)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        eprintln!("API error");
        eprintln!("Status: {}", res.status());
        std::process::exit(1);
    }
}
