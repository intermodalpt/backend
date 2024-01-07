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

use serde::Deserialize;

use commons::models::geo;
pub(crate) type OperatorId = i32;
pub(crate) type StopId = i32;
pub(crate) type RouteId = i32;
pub(crate) type SubrouteId = i32;

// const API_URL: &str = "https://api.intermodal.pt";
const API_URL: &str = "http://localhost:1893";
#[derive(Deserialize)]
pub(crate) struct Stop {
    pub(crate) id: i32,
    pub(crate) name: Option<String>,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
    pub(crate) parish: Option<i32>,
    // pub(crate) operators: Vec<StopOperatorRel>,
}

// #[derive(Deserialize)]
// pub(crate) struct StopOperatorRel {
//     pub(crate) operator_id: OperatorId,
//     pub(crate) name: Option<String>,
//     pub(crate) stop_ref: Option<String>,
// }

#[derive(Deserialize)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub geometry: geo::GeojsonGeometry,
}

pub(crate) async fn fetch_parishes(
) -> Result<Vec<geo::Parish>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/parishes", API_URL);
    println!("Requesting {}", url);
    let parishes: Vec<geo::Parish> = reqwest::get(&url).await?.json().await?;
    Ok(parishes)
}

pub(crate) async fn update_stop_parish(
    stop_id: i32,
    parish_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/stops/{}/parishes", API_URL, stop_id);
    println!("Requesting {}", url);
    reqwest::Client::builder().build()?.put(url).send().await?;
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
    println!("Requesting {}", url);
    let stops: Vec<Stop> = reqwest::get(&url).await?.json().await?;
    Ok(stops)
}

pub(crate) async fn fetch_region_stops(
    region_id: i32,
) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/regions/{}/stops/detailed", API_URL, region_id);
    println!("Requesting {}", url);
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
    println!("Requesting {}", url);
    reqwest::Client::builder().build()?.put(url).send().await?;
    Ok(())
}

pub(crate) async fn detach_stop_from_region(
    stop_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/stops/{}/region", API_URL, stop_id);
    println!("Requesting {}", url);
    reqwest::Client::builder()
        .build()?
        .delete(url)
        .send()
        .await?;
    Ok(())
}
