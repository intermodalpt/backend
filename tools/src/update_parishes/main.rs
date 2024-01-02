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

use std::sync::Mutex;

use config::Config;
use geo::{Contains, LineString, MultiPolygon, Polygon};
use rayon::prelude::*;
use sqlx::postgres::PgPool;

use commons::models::geo::{Geojson, GeojsonGeometry};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

    match update_parishes(&pool).await {
        Ok(_) => {
            println!("Done");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

async fn update_parishes(pool: &PgPool) -> Result<()> {
    let parishes = sql::fetch_parishes(pool).await?;

    let polygons = parishes
        .into_iter()
        .map(|p| {
            let geojson: Geojson = serde_json::from_value(p.geojson)
                .map_err(|err| {
                    eprintln!("Error parsing parish {}: {}", p.id, err);
                })
                .unwrap();

            let multipoly = match geojson.geometry {
                GeojsonGeometry::Polygon { coordinates } => {
                    MultiPolygon::from(vec![poly_from_coords(coordinates)])
                }
                GeojsonGeometry::MultiPolygon { coordinates } => {
                    MultiPolygon::from(
                        coordinates
                            .into_iter()
                            .map(poly_from_coords)
                            .collect::<Vec<_>>(),
                    )
                }
            };

            (p.id, p.name, multipoly)
        })
        .collect::<Vec<_>>();

    let stops = sql::fetch_stops(pool).await?;

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
                    stop.name.as_ref().unwrap_or(
                        stop.name.as_ref().unwrap_or(&"?".to_string())
                    ),
                    stop.id,
                    name
                );
                break;
            }
        }
    });

    // for (stop_id, parish_id) in stop_parish_pairs.into_inner().unwrap() {
    //     sql::update_stop_parish(&pool, stop_id, parish_id).await?;
    // }

    for stop in stops {
        if stop.parish.is_some() {
            continue;
        }
        let point = geo::Point::new(stop.lon, stop.lat);
        for (id, name, multipoly) in &polygons {
            if multipoly.contains(&point) {
                sql::update_stop_parish(pool, stop.id, *id).await?;
                println!(
                    "Stop {} ({}) is in parish {}",
                    stop.name.unwrap(),
                    stop.id,
                    name
                );
                break;
            }
        }
    }

    Ok(())
}

fn poly_from_coords(coordinates: Vec<Vec<Vec<f64>>>) -> Polygon {
    let mut polygons = coordinates.into_iter();
    let outer_coords = polygons.next().unwrap();
    let outer_line = LineString::from(
        outer_coords
            .into_iter()
            .map(|p| (p[0], p[1]))
            .collect::<Vec<_>>(),
    );

    let inner_lines = polygons
        .map(|p| {
            LineString::from(
                p.into_iter().map(|p| (p[0], p[1])).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    Polygon::new(outer_line, inner_lines)
}
