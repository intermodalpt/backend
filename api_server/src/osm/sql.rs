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

use sqlx::types::Json;
use sqlx::{PgPool, QueryBuilder};
use std::borrow::Cow;
use std::collections::HashMap;

use commons::models::osm;

use super::models::{requests, responses};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_osm_stops(
    pool: &PgPool,
) -> Result<Vec<responses::OsmStop>> {
    Ok(sqlx::query_as!(
        responses::OsmStop,
        r#"
SELECT id, lat, lon, name, pos_author, last_author, creation, modification,
    version, deleted
FROM osm_stops
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?)
}

pub(crate) async fn fetch_osm_stop_history(
    pool: &PgPool,
    id: String,
) -> Result<osm::NodeHistory> {
    sqlx::query!(
        r#"
SELECT history as "history!: Json<osm::NodeHistory>"
FROM osm_stops
WHERE id=$1
    "#,
        id
    )
    .fetch_one(pool)
    .await
    .map(|r| r.history.0)
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_osm_stop_histories(
    pool: &PgPool,
) -> Result<HashMap<String, osm::NodeHistory>> {
    Ok(sqlx::query!(
        r#"
SELECT id, history as "history!: Json<osm::NodeHistory>"
FROM osm_stops
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| (r.id, r.history.0))
    .collect())
}

pub(crate) async fn upsert_osm_stops(
    pool: &PgPool,
    osm_stops: &[requests::OsmStop],
) -> Result<()> {
    if osm_stops.iter().any(|osm_stop| osm_stop.history.is_empty()) {
        return Err(Error::ValidationFailure(
            "History cannot be empty".to_string(),
        ));
    }

    let mut qb = QueryBuilder::new(
        "INSERT INTO osm_stops (id, history, name, lat, lon, pos_author, last_author, creation, modification, version, deleted)",
    );

    qb.push_values(osm_stops, |mut b, osm_stop| {
        const FLOAT_TOLERANCE: f64 = 0.000_001;

        let history = &osm_stop.history;

        let mut coord_author = Cow::Borrowed("");
        let mut lat = 0.0;
        let mut lon = 0.0;
        for version in history.iter() {
            if (version.lat - lat).abs() > FLOAT_TOLERANCE
                || (version.lon - lon).abs() > FLOAT_TOLERANCE
            {
                coord_author = Cow::Owned(version.author.to_string());
                lat = version.lat;
                lon = version.lon;
            }
        }

        let name;
        let version;
        let last_author;
        let modification;
        let deleted;

        {
            let last_version = history.as_ref().last().unwrap();

            name = last_version.attributes.iter().find_map(|(k, v)| {
                if k == "name" {
                    Some(v.to_string())
                } else {
                    None
                }
            });

            version = last_version.version;
            last_author = last_version.author;
            modification = last_version.timestamp;
            deleted = last_version.deleted;
        }

        let creation = {
            let first_version = history.as_ref().first().unwrap();
            first_version.timestamp
        };

        b.push_bind(&osm_stop.id)
            .push_bind(Json(history))
            .push_bind(name)
            .push_bind(lat)
            .push_bind(lon)
            .push_bind(coord_author)
            .push_bind(last_author)
            .push_bind(creation)
            .push_bind(modification)
            .push_bind(version)
            .push_bind(deleted);
    });

    qb.push(
        "ON CONFLICT (id) do UPDATE SET
            name = EXCLUDED.name,
            history = EXCLUDED.history,
            lat = EXCLUDED.lat,
            lon = EXCLUDED.lon,
            pos_author = EXCLUDED.pos_author,
            last_author = EXCLUDED.last_author,
            creation = EXCLUDED.creation,
            modification = EXCLUDED.modification,
            version = EXCLUDED.version,
            deleted = EXCLUDED.deleted"
    );

    let query = qb.build();

    query
        .execute(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn delete_osm_stop(pool: &PgPool, id: String) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM osm_stops
WHERE id=$1
    "#,
        id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}
