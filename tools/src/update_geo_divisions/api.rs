/*
    Intermodal, transportation information aggregator
    Copyright (C) 2024  Cláudio Pereira

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
use serde::Deserialize;
use std::fs;
use std::io::{Read, Write};

use commons::models::geo;

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
    pub(crate) name: String,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
    pub(crate) parish: Option<i32>,
}

#[derive(Deserialize)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub geometry: geo::GeojsonGeometry,
}

pub(crate) async fn fetch_parishes(
) -> Result<Vec<geo::Parish>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/parishes", API_URL);
    let parishes: Vec<geo::Parish> = reqwest::get(&url).await?.json().await?;
    Ok(parishes)
}

pub(crate) async fn update_stop_parish(
    stop_id: i32,
    parish_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/stops/{}/parish/{}", API_URL, stop_id, parish_id);
    reqwest::Client::builder()
        .build()?
        .put(url)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await?;
    Ok(())
}

pub(crate) async fn fetch_area_stops(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/v1/stops/within_boundary/{}/{}/{}/{}",
        API_URL, x0, y0, x1, y1
    );
    let stops: Vec<Stop> = reqwest::get(&url).await?.json().await?;
    Ok(stops)
}

pub(crate) async fn fetch_region_route_ids(
    region_id: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    #[derive(Deserialize)]
    pub struct Route {
        pub(crate) id: i32,
    }

    let url = format!("{}/v1/regions/{}/routes", API_URL, region_id);
    println!("Calling {}", url);
    let routes: Vec<Route> = reqwest::get(&url).await?.json().await?;

    Ok(routes.into_iter().map(|route| route.id).collect())
}

pub(crate) async fn fetch_route_stops(
    route_id: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    #[derive(Deserialize)]
    pub struct SubrouteStops {
        pub(crate) stops: Vec<i32>,
    }

    let url = format!("{}/v1/routes/{}/stops", API_URL, route_id);
    println!("Calling {}", url);
    let subroutes: Vec<SubrouteStops> =
        reqwest::get(&url).await?.json().await?;

    let stops = subroutes
        .into_iter()
        .flat_map(|subroute_stops| subroute_stops.stops)
        .collect();
    Ok(stops)
}

pub(crate) async fn cached_fetch_route_stops(
    route_id: i32,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let cache_path = format!("cache/flattened_route_stops/{}.json", route_id);

    if fs::metadata(&cache_path).is_ok() {
        let mut file = fs::File::open(&cache_path)
            .map_err(|e| Error::Files(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| Error::Files(e.to_string()))?;
        let route_stops: Vec<StopId> = serde_json::from_str(&contents)
            .map_err(|e| Error::Files(e.to_string()))?;
        Ok(route_stops)
    } else {
        let route_stops = fetch_route_stops(route_id)
            .await
            .map_err(|e| Error::HTTPError(e.to_string()))?;

        // Create cache dir if it doesn't exist
        fs::create_dir_all("cache/flattened_route_stops")
            .map_err(|e| Error::Files(e.to_string()))?;

        let mut file = fs::File::create(&cache_path)
            .map_err(|e| Error::Files(e.to_string()))?;
        file.write_all(serde_json::to_string(&route_stops).unwrap().as_bytes())
            .map_err(|e| Error::Files(e.to_string()))?;
        Ok(route_stops)
    }
}

pub(crate) async fn fetch_region_stops(
    region_id: i32,
) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/regions/{}/stops/detailed", API_URL, region_id);
    let stops: Vec<Stop> = reqwest::get(&url).await?.json().await?;
    Ok(stops)
}

pub(crate) async fn fetch_regions(
) -> Result<Vec<Region>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/regions", API_URL);
    let stops: Vec<Region> = reqwest::get(&url).await?.json().await?;
    Ok(stops)
}

pub(crate) async fn attach_stop_to_region(
    region_id: i32,
    stop_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/regions/{}/stops/{}", API_URL, region_id, stop_id);
    let res = reqwest::Client::builder()
        .build()?
        .put(url)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await
        .unwrap();

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(Error::HTTPError(format!(
            "Failed to attach stop {} to region {}",
            stop_id, region_id
        ))))
    }
}

pub(crate) async fn detach_stop_from_region(
    region_id: i32,
    stop_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/regions/{}/stops/{}", API_URL, region_id, stop_id);
    let res = reqwest::Client::builder()
        .build()?
        .delete(url)
        .bearer_auth(TOKEN.get().unwrap())
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(Error::HTTPError(format!(
            "Failed to dettach stop {} to region {}",
            stop_id, region_id
        ))))
    }
}
