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

mod api;
mod error;
mod utils;

use std::collections::HashSet;
use std::io;
use std::process::exit;
use std::sync::Mutex;

use config::Config;
use geo::{BoundingRect, Contains, Coord, Point, Rect};
use rayon::prelude::*;
use serde_derive::Deserialize;

use commons::models::geo::Geojson;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default, Deserialize)]
struct AppConfig {
    jwt: String,
}

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("IML")
                .try_parsing(true)
                .separator("_"),
        )
        .build()
        .unwrap();

    if let Ok(config) = config.try_deserialize() {
        let config: AppConfig = config;
        api::TOKEN.set(Box::leak(Box::new(config.jwt))).unwrap();
    } else {
        eprintln!("Token not found in the environment");
        exit(-1);
    }

    match update_stop_regions().await {
        Ok(_) => {
            println!("Done updating the regions");
        }
        Err(e) => {
            eprintln!("Error updating regions: {}", e);
        }
    }

    match update_parishes().await {
        Ok(_) => {
            println!("Done updating the parishes");
        }
        Err(e) => {
            eprintln!("Error updating parishes: {}", e);
        }
    }
}

fn expand_rect(rect: &Rect) -> Rect {
    let offset = Coord::<f64>::from((1.0, 1.0));

    Rect::new(rect.min() - offset, rect.max() + offset)
}

async fn update_stop_regions() -> Result<()> {
    let regions = api::fetch_regions().await.unwrap();

    for region in regions {
        // TODO
        // - drop stops that aren't in the region any longer

        let region_multipoly = utils::multipoly_from_geometry(region.geometry);
        let bbox = region_multipoly.bounding_rect().unwrap();
        let expanded_bbox = expand_rect(&bbox);

        let region_stops = api::fetch_region_stops(region.id).await?;

        let mut region_stops_id =
            region_stops.iter().map(|s| s.id).collect::<HashSet<_>>();

        let region_route_ids =
            api::fetch_region_route_ids(region.id).await.unwrap();

        for route in &region_route_ids {
            let route_stops =
                api::cached_fetch_route_stops(*route).await.unwrap();

            for stop_id in route_stops {
                if !region_stops_id.contains(&stop_id) {
                    api::attach_stop_to_region(region.id, stop_id).await?;
                    region_stops_id.insert(stop_id);
                }
            }
        }

        let near_region_stops = api::fetch_area_stops(
            expanded_bbox.min().x,
            expanded_bbox.max().y,
            expanded_bbox.max().x,
            expanded_bbox.min().y,
        )
        .await
        .unwrap();

        let mut in_region = 0;
        let mut out_region = 0;
        for stop in near_region_stops {
            let point = Point::new(stop.lon, stop.lat);
            if region_multipoly.contains(&point) {
                in_region += 1;
                if !region_stops_id.contains(&stop.id) {
                    println!(
                        "Add stop {} ({}) to region {} ({})? [y/N]",
                        &stop.name, stop.id, region.name, region.id
                    );
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if input.trim().to_lowercase() == "y" {
                        api::attach_stop_to_region(region.id, stop.id).await?;
                        println!("Added");
                    } else {
                        println!("Skipped");
                        continue;
                    }
                }
            } else {
                out_region += 1;
                if region_stops_id.contains(&stop.id) {
                    println!(
                        "Remove stop {} ({}) from region {} ({})? [y/N]",
                        stop.name, stop.id, region.name, region.id
                    );
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if input.trim().to_lowercase() == "y" {
                        api::detach_stop_from_region(region.id, stop.id)
                            .await?;
                        println!("Removed");
                    } else {
                        println!("Skipped");
                        continue;
                    }
                }
            }
        }

        println!(
            "{} stops in region, {} stops out of region",
            in_region, out_region
        );

        break;
    }

    Ok(())
}

async fn update_parishes() -> Result<()> {
    let parishes = api::fetch_parishes().await?;

    let polygons = parishes
        .into_iter()
        .map(|p| {
            let geojson: Geojson = serde_json::from_value(p.geojson)
                .map_err(|err| {
                    eprintln!("Error parsing parish {}: {}", p.id, err);
                })
                .unwrap();

            let multipoly = utils::multipoly_from_geometry(geojson.geometry);

            (p.id, p.name, multipoly)
        })
        .collect::<Vec<_>>();

    let stops = api::fetch_region_stops(0).await?;

    let stop_parish_pairs = Mutex::new(vec![]);

    stops.par_iter().for_each(|stop| {
        if stop.parish.is_some() {
            return;
        }
        let point = geo::Point::new(stop.lon, stop.lat);
        for (id, name, multipoly) in &polygons {
            if multipoly.contains(&point) {
                stop_parish_pairs.lock().unwrap().push((stop.id, *id));
                println!(
                    "Stop {} ({}) is in parish {}",
                    &stop.name, stop.id, name
                );
                break;
            }
        }
    });

    for stop in stops {
        if stop.parish.is_some() {
            continue;
        }
        let point = Point::new(stop.lon, stop.lat);
        for (id, name, multipoly) in &polygons {
            if multipoly.contains(&point) {
                api::update_stop_parish(stop.id, *id).await?;
                println!(
                    "Stop {} ({}) is in parish {}",
                    stop.name, stop.id, name
                );
                break;
            }
        }
    }

    Ok(())
}
