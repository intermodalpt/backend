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

use crate::models;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

use commons::models::osm;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) async fn fetch_stops(pool: &PgPool) -> Result<Vec<models::Stop>> {
    Ok(sqlx::query!(
        "SELECT id, external_id, verification_level, osm_history
         FROM stops"
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .filter_map(|r| {
        if let Ok(history) = serde_json::from_value(r.osm_history) {
            Some(models::Stop {
                id: r.id,
                external_id: r.external_id,
                verification_level: r.verification_level,
                osm_history: history,
            })
        } else {
            None
        }
    })
    .collect::<Vec<models::Stop>>())
}

pub(crate) async fn update_stop_osm_versions<'c, E>(
    executor: E,
    node: osm::StopNode,
    authors: &HashMap<i32, String>,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let mut coord_author = -1;
    let mut lat = 0.0;
    let mut lon = 0.0;
    for version in node.versions.iter() {
        if version.lat != lat || version.lon != lon {
            coord_author = version.author;
            lat = version.lat;
            lon = version.lon;
        }
    }

    let last_version = node.versions.last().unwrap();
    let deleted = last_version.deleted;
    let verified_position =
        !last_version.attributes.iter().any(|(k, _)| k == "fixme");

    let meta = osm::StoredStopMeta {
        deleted,
        versions: node.versions,
    };

    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET osm_history=$1, deleted_upstream=$2, verified_position=$3
WHERE external_id=$4
    "#,
        &serde_json::to_value(meta).unwrap(),
        deleted,
        verified_position,
        &node.id.to_string(),
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub(crate) async fn update_pos<'c, E>(
    executor: E,
    stop: &models::Stop,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let last_version = stop.osm_history.versions.last().unwrap();
    let verified_position =
        !last_version.attributes.iter().any(|(k, _)| k == "fixme");

    if verified_position {
        let _res = sqlx::query!(
            r#"
UPDATE Stops
SET verified_position=$1
WHERE id=$2
    "#,
            verified_position,
            stop.id
        )
        .execute(executor)
        .await?;
    }

    Ok(())
}
