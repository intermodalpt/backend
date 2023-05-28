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

use crate::operators::models::IssueState;
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

pub(crate) async fn fetch_issues(
    pool: &PgPool,
) -> Result<Vec<models::Issue>> {
    sqlx::query!(
        r#"
SELECT issues.id, issues.title, issues.message, issues.geojson, issues.category, issues.lat,
    issues.creation, issues.lon, issues.impact, issues.state, issues.state_justification,
    array_agg(issue_operators.operator_id) as "operators!: Vec<i32>",
    array_agg(issue_routes.route_id) as "routes!: Vec<i32>",
    array_agg(issue_stops.stop_id) as "stops!: Vec<i32>"
FROM issues
JOIN issue_operators on issue_operators.issue_id = issues.id
JOIN issue_routes on issue_routes.issue_id = issues.id
JOIN issue_stops on issue_stops.issue_id = issues.id
GROUP BY issues.id
"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(models::Issue {
            id: row.id,
            title: row.title,
            message: row.message,
            geojson: row.geojson,
            category: serde_json::from_str(&row.category)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            creation: row.creation.into(),
            state: serde_json::from_str(&row.state)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            state_justification: row.state_justification,
            lat: row.lat,
            lon: row.lon,
            impact: row.impact,
            operator_ids: row.operators,
            route_ids: row.routes,
            stop_ids: row.stops,
        })
    })
    .collect()
}

pub(crate) async fn fetch_issue_operators(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::Issue>> {
    sqlx::query!(
        r#"
SELECT issues.id, issues.title, issues.message, issues.geojson, issues.category, issues.lat,
    issues.creation, issues.lon, issues.impact, issues.state, issues.state_justification,
    array_agg(issue_operators.operator_id) as "operators!: Vec<i32>",
    array_agg(issue_routes.route_id) as "routes!: Vec<i32>",
    array_agg(issue_stops.stop_id) as "stops!: Vec<i32>"
FROM issues
JOIN issue_operators on issue_operators.issue_id = issues.id
JOIN issue_routes on issue_routes.issue_id = issues.id
JOIN issue_stops on issue_stops.issue_id = issues.id
WHERE issues.id IN (
    SELECT issue_id
    FROM issue_operators
    WHERE issue_operators.operator_id = $1
)
GROUP BY issues.id
"#,
        operator_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|row| {
        Ok(responses::Issue {
            id: row.id,
            title: row.title,
            message: row.message,
            geojson: row.geojson,
            category: serde_json::from_str(&row.category)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            creation: row.creation.into(),
            lat: row.lat,
            lon: row.lon,
            impact: row.impact,
            state: serde_json::from_str(&row.state)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            state_justification: row.state_justification,
            operator_ids: row.operators,
            route_ids: row.routes,
            stop_ids: row.stops,
        })
    })
    .collect()
}

pub(crate) async fn fetch_issue(
    pool: &PgPool,
    issue_id: i32,
) -> Result<models::Issue> {
    sqlx::query!(
        r#"SELECT issues.id, issues.title, issues.message, issues.category, issues.impact,
        issues.creation, issues.lat, issues.lon, issues.geojson,
        issues.state, issues.state_justification,
    array_agg(issue_operators.operator_id) as "operators!: Vec<i32>",
    array_agg(issue_routes.route_id) as "routes!: Vec<i32>",
    array_agg(issue_stops.stop_id) as "stops!: Vec<i32>"
FROM issues
JOIN issue_operators on issue_operators.issue_id = issues.id
JOIN issue_routes on issue_routes.issue_id = issues.id
JOIN issue_stops on issue_stops.issue_id = issues.id
WHERE issues.id = $1
GROUP BY issues.id"#,
        issue_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
    .and_then(|row| {
        Ok(models::Issue {
            id: row.id,
            title: row.title,
            message: row.message,
            creation: row.creation.into(),
            category: serde_json::from_str(&row.category)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            impact: row.impact,
            state: serde_json::from_str(&row.state)
                .map_err(|_e| Error::DatabaseDeserialization)?,
            state_justification: row.state_justification,
            lat: row.lat,
            lon: row.lon,
            geojson: row.geojson,
            operator_ids: row.operators,
            route_ids: row.routes,
            stop_ids: row.stops,
        })
    })
}

pub(crate) async fn insert_issue(
    pool: &PgPool,
    issue: &requests::NewIssue,
) -> Result<i32> {
    let creation = Local::now();
    let mut transaction = pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let row = sqlx::query!(
        r#"
INSERT INTO issues (title, message, category, impact, creation, lat, lon, geojson, state)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING id
"#,
        issue.title,
        issue.message,
        &serde_json::to_string(&issue.category).unwrap(),
        issue.impact,
        creation,
        issue.lat,
        issue.lon,
        issue.geojson,
        &serde_json::to_string(&IssueState::Unanswered).unwrap()
    )
        .fetch_one(&mut transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let id = row.id;

    for operator_id in &issue.operator_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_operators (operator_id, issue_id)
            VALUES ($1, $2)
            "#,
            operator_id,
            id
        )
        .execute(&mut transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    for route_id in &issue.route_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_routes (route_id, issue_id)
            VALUES ($1, $2)
            "#,
            route_id,
            id
        )
        .execute(&mut transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    for stop_id in &issue.stop_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_stops (stop_id, issue_id)
            VALUES ($1, $2)
            "#,
            stop_id,
            id
        )
        .execute(&mut transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(id)
}

pub(crate) async fn update_issue(
    pool: &PgPool,
    issue_id: i32,
    issue: requests::ChangeIssue,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sqlx::query!(
        r#"
        UPDATE issues
        SET title = $1,
            message = $2,
            geojson = $3,
            category = $4,
            lat = $5,
            lon = $6,
            state = $7,
            state_justification = $8
        WHERE id = $9
        "#,
        issue.title,
        issue.message,
        issue.geojson,
        &serde_json::to_string(&issue.category).unwrap(),
        issue.lat,
        issue.lon,
        &serde_json::to_string(&issue.state).unwrap(),
        issue.state_justification,
        issue_id
    )
    .execute(&mut transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sqlx::query!(
        r#"
        DELETE FROM issue_operators
        WHERE issue_id = $1
        "#,
        issue_id
    )
    .execute(&mut transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sqlx::query!(
        r#"
        DELETE FROM issue_routes
        WHERE issue_id = $1
        "#,
        issue_id
    )
    .execute(&mut transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sqlx::query!(
        r#"
        DELETE FROM issue_stops
        WHERE issue_id = $1
        "#,
        issue_id
    )
    .execute(&mut transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    for operator_id in &issue.operator_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_operators (operator_id, issue_id)
            VALUES ($1, $2)
            "#,
            operator_id,
            issue_id
        )
        .execute(&mut transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    for route_id in &issue.route_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_routes (route_id, issue_id)
            VALUES ($1, $2)
            "#,
            route_id,
            issue_id
        )
        .execute(&mut transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    for stop_id in &issue.stop_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_stops (stop_id, issue_id)
            VALUES ($1, $2)
            "#,
            stop_id,
            issue_id
        )
        .execute(&mut transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }

    transaction
        .commit()
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
