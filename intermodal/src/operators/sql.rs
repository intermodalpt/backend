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

use crate::calendar::Calendar;
use chrono::{Local, NaiveDate};
use sqlx::PgPool;

use super::models::{self, requests, responses};

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
) -> Result<Vec<responses::OperatorCalendar>> {
    sqlx::query!(
        r#"
SELECT id, name, calendar, operator_id
FROM operator_calendars
"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(responses::OperatorCalendar {
            id: row.id,
            name: row.name,
            calendar: serde_json::from_value(row.calendar)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            operator_id: row.operator_id,
        })
    })
    .collect()
}

pub(crate) async fn fetch_operator_calendars(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::OperatorCalendar>> {
    sqlx::query!(
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
    .map(|row| {
        Ok(responses::OperatorCalendar {
            id: row.id,
            name: row.name,
            calendar: serde_json::from_value(row.calendar)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            operator_id,
        })
    })
    .collect()
}

pub(crate) async fn insert_calendar(
    pool: &PgPool,
    operator_id: i32,
    calendar: requests::NewOperatorCalendar,
) -> Result<i32> {
    let row = sqlx::query!(
        r#"
INSERT INTO operator_calendars (operator_id, name, calendar)
VALUES ($1, $2, $3)
RETURNING id
"#,
        operator_id,
        calendar.name,
        serde_json::to_value(calendar.calendar).unwrap()
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(row.id)
}

pub(crate) async fn delete_calendar(
    pool: &PgPool,
    operator_id: i32,
    calendar_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM operator_calendars
WHERE operator_id=$1 AND id=$2
"#,
        operator_id,
        calendar_id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn fetch_calendars_for_date(
    pool: &PgPool,
    operator_id: i32,
    date: NaiveDate,
) -> Result<Vec<responses::OperatorCalendar>> {
    sqlx::query!(
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
    .filter_map(
        |row| match serde_json::from_value::<Calendar>(row.calendar) {
            Ok(calendar) => {
                if calendar.includes(date) {
                    Some(Ok(responses::OperatorCalendar {
                        id: row.id,
                        name: row.name,
                        calendar,
                        operator_id,
                    }))
                } else {
                    None
                }
            }
            Err(_e) => Some(Err(Error::DatabaseDeserialization)),
        },
    )
    .collect()
}

pub(crate) async fn fetch_news(
    pool: &PgPool,
    skip: i64,
    take: i64,
) -> Result<Vec<models::NewsItem>> {
    sqlx::query!(
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
    .map(|row| {
        Ok(models::NewsItem {
            id: row.id,
            operator_id: row.operator_id,
            summary: row.summary,
            content: row.content,
            datetime: row.datetime.with_timezone(&Local),
            geojson: if let Some(geojson) = row.geojson {
                Some(
                    serde_json::from_value(geojson)
                        .map_err(|_e| Error::DatabaseDeserialization)?,
                )
            } else {
                None
            },
            visible: row.visible,
        })
    })
    .collect()
}

pub(crate) async fn fetch_operator_news(
    pool: &PgPool,
    operator_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::OperatorNewsItem>> {
    sqlx::query!(
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
    .map(|row| {
        Ok(responses::OperatorNewsItem {
            id: row.id,
            summary: row.summary,
            content: row.content,
            datetime: row.datetime.with_timezone(&Local),
            geojson: if let Some(geojson) = row.geojson {
                Some(
                    serde_json::from_value(geojson)
                        .map_err(|_e| Error::DatabaseDeserialization)?,
                )
            } else {
                None
            },
            visible: row.visible,
        })
    })
    .collect()
}
