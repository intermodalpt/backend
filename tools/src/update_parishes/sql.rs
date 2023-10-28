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

use sqlx::postgres::PgPool;

use commons::models::geo;
use commons::models::stops;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) async fn fetch_stops(pool: &PgPool) -> Result<Vec<stops::Stop>> {
    sqlx::query!(
"SELECT id, source, name, osm_name, short_name, locality, street, door, lat, lon, external_id,
    notes, updater, update_date, parish, tags, accessibility_meta,
    verification_level, service_check_date, infrastructure_check_date
FROM stops")
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| {
        Ok(stops::Stop {
            id: r.id,
            name: r.name,
            osm_name: r.osm_name,
            short_name: r.short_name,
            locality: r.locality,
            street: r.street,
            door: r.door,
            lat: r.lat,
            lon: r.lon,
            notes: r.notes,
            update_date: r.update_date,
            parish: r.parish,
            tags: r.tags,
            a11y: serde_json::from_value(r.accessibility_meta)?,
            verification_level: r.verification_level as u8,
            service_check_date: r.service_check_date,
            infrastructure_check_date: r.infrastructure_check_date,
        })
    })
    .collect()
}

pub(crate) async fn fetch_parishes(pool: &PgPool) -> Result<Vec<geo::Parish>> {
    Ok(sqlx::query_as!(
        geo::Parish,
        r#"
SELECT parishes.id, parishes.name, parishes.short_name, municipalities.name as municipality,
    municipalities.zone, parishes.polygon, parishes.geojson
FROM parishes
JOIN municipalities ON parishes.municipality = municipalities.id
    "#
    )
        .fetch_all(pool)
        .await?)
}

pub(crate) async fn update_stop_parish(
    pool: &PgPool,
    stop_id: i32,
    parish: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE stops
SET parish = $1
WHERE id = $2
        "#,
        parish,
        stop_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
