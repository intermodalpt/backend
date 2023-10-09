/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

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

use sqlx::PgPool;

use crate::models;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) async fn fetch_stops(pool: &PgPool) -> Result<Vec<models::Stop>> {
    sqlx::query!(
        "SELECT id, name, official_name, osm_name, lat, lon, external_id, refs FROM stops"
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| {
        Ok(models::Stop {
            id: r.id,
            name: r.name,
            official_name: r.official_name,
            osm_name: r.osm_name,
            lat: r.lat,
            lon: r.lon,
            external_id: r.external_id,
            refs: r.refs,
        })
    })
    .collect()
}

pub(crate) async fn insert_stops(
    db_pool: &PgPool,
    stops: Vec<models::Stop>,
) -> Result<()> {
    for stop in stops {
        let _res = sqlx::query!(
            r#"
INSERT INTO Stops(name, osm_name, official_name, lon, lat, external_id)
VALUES ($1, $2, $3, $4, $5, $6)
    "#,
            stop.name,
            stop.osm_name,
            stop.official_name,
            stop.lon,
            stop.lat,
            stop.external_id,
        )
        .execute(db_pool)
        .await?;
    }
    Ok(())
}

pub(crate) async fn update_stops(
    db_pool: &PgPool,
    stops: Vec<models::Stop>,
) -> Result<()> {
    for stop in stops {
        let _res = sqlx::query!(
            r#"
UPDATE Stops
SET official_name=$1, osm_name=$2, lon=$3, lat=$4, refs=$5
WHERE id=$6 AND external_id=$7
    "#,
            stop.official_name,
            stop.osm_name,
            stop.lon,
            stop.lat,
            &stop.refs,
            stop.id,
            stop.external_id,
        )
        .execute(db_pool)
        .await?;
    }
    Ok(())
}

pub(crate) async fn tag_missing_stops(
    db_pool: &PgPool,
    osm_ids: Vec<String>,
) -> Result<()> {
    let db_ids: Vec<String> = sqlx::query!("SELECT external_id FROM Stops")
        .fetch_all(db_pool)
        .await?
        .into_iter()
        .map(|s| s.external_id)
        .collect();

    // Check which IDs in the db have disappeared from osm
    let missing_ids: Vec<String> = db_ids
        .iter()
        .filter(|id| !osm_ids.contains(id))
        .map(|id| id.clone())
        .collect();

    for missing_id in missing_ids {
        let _res = sqlx::query!(
            r#"
    UPDATE Stops
    SET deleted_upstream=true
    WHERE external_id=$1
        "#,
            missing_id
        )
        .execute(db_pool)
        .await?;
    }
    Ok(())
}
