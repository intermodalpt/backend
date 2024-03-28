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
use std::collections::HashMap;

use commons::models::osm;

use super::models::{requests, responses};
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_osm_stops(
    pool: &PgPool,
) -> Result<Vec<responses::OsmStop>> {
    sqlx::query_as!(
        responses::OsmStop,
        r#"
SELECT id, lat, lon, name, pos_author, last_author, creation, modification,
    version, deleted
FROM osm_stops
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_osm_stop_history(
    pool: &PgPool,
    id: i64,
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
    .map_err(|err| {
        tracing::error!(error = err.to_string(), id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_osm_stop_histories(
    pool: &PgPool,
) -> Result<HashMap<i64, osm::NodeHistory>> {
    Ok(sqlx::query!(
        r#"
SELECT id, history as "history!: Json<osm::NodeHistory>"
FROM osm_stops
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
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
    // Upsert in chunks to avoid exceeding the query param limit
    for chunk in osm_stops.chunks(5000) {
        upsert_osm_stops_chunk(pool, chunk).await?;
    }

    Ok(())
}

pub(crate) async fn upsert_osm_stops_chunk(
    pool: &PgPool,
    osm_stops: &[requests::OsmStop],
) -> Result<()> {
    let mut qb = QueryBuilder::new(
        "INSERT INTO osm_stops (id, history, name, lat, lon, pos_author, last_author, creation, modification, version, deleted)",
    );

    qb.push_values(osm_stops, |mut b, osm_stop| {
        const FLOAT_TOLERANCE: f64 = 0.000_001;

        let history = &osm_stop.history;

        let mut coord_author = "";
        let (mut lat, mut lon) = (0.0, 0.0);
        for version in history {
            if (version.lat - lat).abs() > FLOAT_TOLERANCE
                || (version.lon - lon).abs() > FLOAT_TOLERANCE
            {
                coord_author = &version.author_uname;
                lat = version.lat;
                lon = version.lon;
            }
        }

        let last_version: &osm::NodeVersion = history.last().unwrap();

        let name = last_version.attributes.iter().find_map(|(k, v)| {
            if k == "name" {
                Some(v.to_string())
            } else {
                None
            }
        });

        let version = last_version.version;
        let last_author = &last_version.author_uname;
        let modification = last_version.timestamp;
        let deleted = last_version.deleted;

        let creation = {
            let first_version = history.first().unwrap();
            first_version.timestamp
        };

        b.push_bind(osm_stop.id)
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
            deleted = EXCLUDED.deleted",
    );

    qb.build().execute(pool).await.map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_osm_stop(pool: &PgPool, id: i64) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM osm_stops
WHERE id=$1
    "#,
        id
    )
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_osm_stop_versions(
    pool: &PgPool,
) -> Result<HashMap<i64, Vec<i32>>> {
    Ok(sqlx::query!(
        r#"
SELECT id, array_agg(versions->'version')::int[] as "versions!: Vec<i32>"
FROM osm_stops, jsonb_array_elements(osm_stops.history) as versions
GROUP BY id
    "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| (r.id, r.versions))
    .collect())
}

pub(crate) async fn fetch_paired_osm_stop(
    pool: &PgPool,
    iml_stop_id: i32,
) -> Result<Option<responses::FullOsmStop>> {
    sqlx::query_as!(
        responses::FullOsmStop,
        r#"SELECT osm_stops.id, osm_stops.name, osm_stops.lat, osm_stops.lon,
    osm_stops.pos_author, osm_stops.last_author, osm_stops.creation,
    osm_stops.modification, osm_stops.version, osm_stops.deleted,
    osm_stops.history as "history!: sqlx::types::Json<osm::NodeHistory>",
    stops.osm_map_quality
FROM stops
JOIN osm_stops ON stops.osm_id = osm_stops.id
WHERE stops.id = $1
"#,
        iml_stop_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), iml_stop_id);
        Error::DatabaseExecution
    })
}
