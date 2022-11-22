/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022  Cláudio Pereira

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

use chrono::Local;
use sqlx::PgPool;

use super::models;
use super::models::responses;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_operators(
    pool: &PgPool,
) -> Result<Vec<models::Operator>> {
    sqlx::query_as!(
        models::Operator,
        r#"
SELECT id, name, tag
FROM Operators
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_calendars(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::OperatorCalendar>> {
    Ok(sqlx::query!(
        r#"
SELECT id, name, calendar
FROM operator_calendars
WHERE operator_id=$1
"#,
        operator_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| responses::OperatorCalendar {
        id: row.id,
        name: row.name,
        calendar: serde_json::from_value(row.calendar).unwrap(),
    })
    .collect())
}

pub(crate) async fn fetch_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<models::NewsItem>> {
    Ok(sqlx::query!(
        r#"
SELECT id, operator_id, summary, content, datetime, geojson, visible
FROM news_items
LIMIT $1 OFFSET $2
"#,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| models::NewsItem {
        id: row.id,
        operator_id: row.operator_id,
        summary: row.summary,
        content: row.content,
        datetime: row.datetime.with_timezone(&Local),
        geojson: if let Some(geojson) = row.geojson {
            Some(serde_json::from_value(geojson).unwrap())
        } else {
            None
        },
        visible: row.visible,
    })
    .collect())
}

pub(crate) async fn fetch_operator_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::OperatorNewsItem>> {
    Ok(sqlx::query!(
        r#"
SELECT id, operator_id, summary, content, datetime, geojson, visible
FROM news_items
WHERE operator_id=$1
LIMIT $2 OFFSET $3
"#,
        operator_id,
        take,
        skip,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| responses::OperatorNewsItem {
        id: row.id,
        summary: row.summary,
        content: row.content,
        datetime: row.datetime.with_timezone(&Local),
        geojson: if let Some(geojson) = row.geojson {
            Some(serde_json::from_value(geojson).unwrap())
        } else {
            None
        },
        visible: row.visible,
    })
    .collect())
}
