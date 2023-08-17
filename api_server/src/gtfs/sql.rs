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

use sqlx::PgPool;

use commons::models::stops;

use super::models;
use crate::errors::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_gtfs_stops(
    pool: &PgPool,
) -> Result<Vec<models::TMLStop>> {
    sqlx::query!(
        r#"
SELECT id, source, name, official_name, osm_name, short_name, locality, street,
    door, lat, lon, external_id, notes, updater, update_date,
    parish, tags, accessibility_meta, refs, tml_id, tml_id_verified,
    tml_id_source, deleted_upstream,
    verification_level, service_check_date, infrastructure_check_date, verified_position
FROM Stops
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| {
        Ok(models::TMLStop {
            stop: stops::Stop {
                id: r.id,
                source: r.source,
                name: r.name,
                official_name: r.official_name,
                osm_name: r.osm_name,
                short_name: r.short_name,
                locality: r.locality,
                street: r.street,
                door: r.door,
                lat: r.lat,
                lon: r.lon,
                external_id: r.external_id,
                refs: r.refs,
                notes: r.notes,
                updater: r.updater,
                update_date: r.update_date,
                parish: r.parish,
                tags: r.tags,
                a11y: serde_json::from_value(r.accessibility_meta).map_err(
                    |e| {
                        log::error!("Error deserializing: {}", e);
                        Error::DatabaseDeserialization
                    },
                )?,
                verification_level:  if r.verified_position { r.verification_level as u8 | 0b11000000 } else { r.verification_level as u8 & 0b00111111 },
                service_check_date: r.service_check_date,
                infrastructure_check_date: r.infrastructure_check_date,
            },
            tml_id: r.tml_id,
            tml_id_verified: r.tml_id_verified,
            deleted_upstream: r.deleted_upstream,
            tml_id_source: r.tml_id_source,
            verified_position: r.verified_position,
        })
    })
    .collect()
}

pub(crate) async fn gtfs_match<'c, E>(
    executor: E,
    stop_id: i64,
    gtfs_id: String,
    verified: bool,
    source: &str,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET tml_id=$1, tml_id_verified=$2, tml_id_source=$3
WHERE id=$4
    "#,
        gtfs_id,
        verified,
        source,
        stop_id as i32
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}
