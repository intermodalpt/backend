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

#![allow(dead_code)]

use itertools::Itertools;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};

use commons::models::gtfs;

use crate::error::Error;

pub(crate) type OperatorId = i32;
pub(crate) type StopId = i32;
pub(crate) type RouteId = i32;
pub(crate) type SubrouteId = i32;

const API_URL: &str = "https://api.intermodal.pt";
pub(crate) static TOKEN: OnceCell<&'static str> = OnceCell::new();

#[derive(Deserialize)]
pub(crate) struct Stop {
    pub(crate) id: i32,
    name: String,
    lat: f64,
    lon: f64,
    pub(crate) operators: Vec<StopOperatorRel>,
}
#[derive(Deserialize)]
pub(crate) struct StopOperatorRel {
    pub(crate) operator_id: OperatorId,
    name: Option<String>,
    pub(crate) stop_ref: Option<String>,
    source: String,
}
#[derive(Deserialize)]
pub(crate) struct Route {
    pub(crate) id: RouteId,
    pub(crate) name: String,
    pub(crate) code: Option<String>,
    pub(crate) operator: OperatorId,
    pub(crate) circular: bool,
    pub(crate) badge_text: String,
    pub(crate) badge_bg: String,
    pub(crate) type_id: i32,
    pub(crate) active: bool,
    pub(crate) subroutes: Vec<Subroute>,
}

#[derive(Clone, Deserialize)]
pub(crate) struct Subroute {
    pub(crate) id: SubrouteId,
    pub(crate) flag: String,
    pub(crate) circular: bool,
    pub(crate) headsign: Option<String>,
    pub(crate) destination: Option<String>,
    #[serde(default)]
    pub(crate) stops: Vec<StopId>,
    #[serde(default)]
    pub(crate) prematched_gtfs_pattern: Option<String>,
}

#[derive(Deserialize)]
pub struct SubrouteStops {
    pub(crate) subroute: SubrouteId,
    pub(crate) stops: Vec<StopId>,
}
#[derive(Serialize)]
pub(crate) struct OperatorValidationData {
    pub(crate) gtfs_lints: Vec<gtfs::Lint>,
}

#[derive(Serialize)]
pub(crate) struct RouteValidationData {
    pub(crate) validation: gtfs::RouteValidation,
    pub(crate) subroutes: HashMap<i32, gtfs::SubrouteValidation>,
}

pub(crate) struct Data {
    pub(crate) stops: HashMap<StopId, Stop>,
    pub(crate) routes: HashMap<RouteId, Route>,
}

pub(crate) async fn load_base_data() -> Result<Data, Error> {
    let iml_stops = fetch_iml_stops().await.unwrap();
    println!("Downloaded IML stops");
    let mut iml_routes = fetch_iml_routes().await.unwrap();
    println!("Downloaded IML routes");

    for route in &mut iml_routes {
        for subroute in &mut route.subroutes {
            if subroute.headsign.is_some() {
                subroute.headsign =
                    subroute.headsign.as_ref().map(|s| s.trim().to_lowercase());
                continue;
            }
            if subroute.flag.contains('-') {
                let flag_parts = subroute.flag.split('-').collect_vec();
                if flag_parts.len() == 2 {
                    let trimmed = flag_parts[1].trim();
                    if trimmed.len() > 5 {
                        subroute.headsign = Some(trimmed.to_lowercase());
                        continue;
                    }
                }
            }
            subroute.headsign = subroute
                .destination
                .as_ref()
                .map(|s| s.trim().to_lowercase());
        }

        let iml_subroute_stops = fetch_subroute_stops(route.id).await.unwrap();
        println!("Downloaded IML subroute stops for route {}", route.id);
        iml_subroute_stops
            .into_iter()
            .for_each(|(subroute_id, stops)| {
                if let Some(subroute) = route
                    .subroutes
                    .iter_mut()
                    .find(|subroute| subroute.id == subroute_id)
                {
                    subroute.stops = stops;
                } else {
                    eprintln!(
                        "Subroute {} not found in route {}",
                        subroute_id, route.id
                    );
                }
            });
    }

    Ok(Data {
        stops: iml_stops
            .into_iter()
            .map(|stop| (stop.id, stop))
            .collect::<HashMap<StopId, Stop>>(),
        routes: iml_routes
            .into_iter()
            .map(|route| (route.id, route))
            .collect::<HashMap<RouteId, Route>>(),
    })
}

pub(crate) async fn fetch_iml_stops(
) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/regions/1/stops/full", API_URL);
    println!("Fetching {}", url);
    let res = reqwest::Client::new()
        .get(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await?;

    if res.status().is_success() {
        let stops: Vec<Stop> = res.json().await?;
        Ok(stops)
    } else {
        Err(Box::new(Error::HTTPError(format!(
            "Status: {} Response: {}",
            res.status(),
            res.text().await?
        ))))
    }
}
pub(crate) async fn fetch_iml_routes(
) -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/routes/all", API_URL);
    let routes = reqwest::get(&url).await?.json().await?;
    Ok(routes)
}

pub(crate) async fn fetch_subroute_stops(
    route_id: RouteId,
) -> Result<HashMap<SubrouteId, Vec<StopId>>, Error> {
    let cache_path = format!("cache/route_stops/{}.json", route_id);
    if fs::metadata(&cache_path).is_ok() {
        let mut file = fs::File::open(&cache_path)
            .map_err(|e| Error::Files(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| Error::Files(e.to_string()))?;
        let route_stops: HashMap<SubrouteId, Vec<StopId>> =
            serde_json::from_str(&contents)
                .map_err(|e| Error::Files(e.to_string()))?;
        Ok(route_stops)
    } else {
        let route_stops = fetch_route_stops(route_id)
            .await
            .map_err(|e| Error::HTTPError(e.to_string()))?;

        // Create cache dir if it doesn't exist
        fs::create_dir_all("cache/route_stops")
            .map_err(|e| Error::Files(e.to_string()))?;

        let mut file = fs::File::create(&cache_path)
            .map_err(|e| Error::Files(e.to_string()))?;
        file.write_all(serde_json::to_string(&route_stops).unwrap().as_bytes())
            .map_err(|e| Error::Files(e.to_string()))?;
        Ok(route_stops)
    }
}

async fn fetch_route_stops(
    route_id: RouteId,
) -> Result<HashMap<SubrouteId, Vec<StopId>>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/routes/{}/stops", API_URL, route_id);
    let subroute_stops: Vec<SubrouteStops> =
        reqwest::get(&url).await?.json().await?;

    Ok(subroute_stops
        .into_iter()
        .map(|ss| (ss.subroute, ss.stops))
        .collect())
}

pub(crate) async fn put_route_validation(
    route_id: i32,
    validation_data: RouteValidationData,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/routes/{}/validation", API_URL, route_id);
    println!("Calling {}", &url);
    let res = reqwest::Client::new()
        .put(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .json(&validation_data)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(Error::HTTPError(format!(
            "Status: {}. Response: {}",
            res.status(),
            res.text().await?
        ))))
    }
}

pub(crate) async fn put_operator_validation(
    operator_id: i32,
    validation_data: OperatorValidationData,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/operators/{}/validation", API_URL, operator_id);
    let res = reqwest::Client::new()
        .put(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .json(&validation_data)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(Error::HTTPError(format!(
            "Status: {}. Response: {}",
            res.status(),
            res.text().await?
        ))))
    }
}
