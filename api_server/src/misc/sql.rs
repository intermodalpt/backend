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

use super::models;
use crate::Error;

pub(crate) async fn get_stats(
    db_pool: &PgPool,
) -> Result<models::responses::Stats, Error> {
    let stop_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Stops
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0);

    let route_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Routes
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0);

    let subroute_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Subroutes
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0);

    let departure_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM Departures
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0);

    let picture_count = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM stop_pics
    "#
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .cnt
    .unwrap_or(0);

    Ok(models::responses::Stats {
        stop_count,
        route_count,
        subroute_count,
        departure_count,
        picture_count,
    })
}
