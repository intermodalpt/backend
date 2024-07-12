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
    // TODO get rid of this default once the API returns stops for each subroute
    #[serde(default)]
    pub(crate) stops: Vec<StopId>,
    pub(crate) validation: SubrouteValidation,
}

#[derive(Clone, Deserialize)]
pub(crate) struct SubrouteValidation {
    pub(crate) current: Vec<i32>,
    pub(crate) gtfs: Option<gtfs::PatternCluster>,
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

#[derive(Serialize, Debug)]
pub(crate) struct RouteValidationData {
    pub(crate) validation: gtfs::RouteValidation,
    pub(crate) subroutes: HashMap<i32, gtfs::SubrouteValidation>,
}

#[derive(Serialize, Debug)]
pub struct ChangeSubrouteStops {
    pub from: Vec<i32>,
    pub to: Vec<i32>,
}

pub(crate) struct Data {
    pub(crate) stops: HashMap<StopId, Stop>,
    pub(crate) routes: HashMap<RouteId, Route>,
}

pub(crate) async fn load_base_data(
    operator_id: OperatorId,
) -> Result<Data, Error> {
    let iml_stops = fetch_iml_stops(operator_id).await.unwrap();
    println!("Downloaded IML stops");
    let mut iml_routes = fetch_iml_routes(operator_id).await.unwrap();
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
    operator_id: OperatorId,
) -> Result<Vec<Stop>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/operators/{operator_id}/stops/full", API_URL);
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
        Err(Box::new(Error::Http(format!(
            "Status: {} Response: {}",
            res.status(),
            res.text().await?
        ))))
    }
}
pub(crate) async fn fetch_iml_routes(
    operator_id: OperatorId,
) -> Result<Vec<Route>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/operators/{operator_id}/routes/full", API_URL);
    println!("Fetching {}", url);
    let mut routes: Vec<Route> = reqwest::get(&url).await?.json().await?;

    // Do some trickery while the API only returns the current
    // stops as part of the validation data
    // FIXME fix the API, drop this
    routes.iter_mut().for_each(|route| {
        route.subroutes.iter_mut().for_each(|subroute| {
            std::mem::swap(
                &mut subroute.stops,
                &mut subroute.validation.current,
            );
        });
    });

    Ok(routes)
}
pub(crate) async fn patch_route_validation(
    route_id: i32,
    validation_data: RouteValidationData,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/routes/{}/validation", API_URL, route_id);
    println!("Calling {}", &url);
    let res = reqwest::Client::new()
        .patch(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .json(&validation_data)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(Error::Http(format!(
            "Status: {}. Response: {}",
            res.status(),
            res.text().await?
        ))))
    }
}

pub(crate) async fn patch_operator_validation(
    operator_id: i32,
    validation_data: OperatorValidationData,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/operators/{}/validation", API_URL, operator_id);
    let res = reqwest::Client::new()
        .patch(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .json(&validation_data)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(Error::Http(format!(
            "Status: {}. Response: {}",
            res.status(),
            res.text().await?
        ))))
    }
}

pub(crate) async fn patch_subroute_stops(
    subroute_id: i32,
    change: ChangeSubrouteStops,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/subroutes/{}/stops", API_URL, subroute_id);
    dbg!(&url);
    let res = reqwest::Client::new()
        .patch(&url)
        .bearer_auth(TOKEN.get().unwrap())
        .json(&change)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(Error::Http(format!(
            "Status: {}. Response: {}",
            res.status(),
            res.text().await?
        ))))
    }
}
