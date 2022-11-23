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

use chrono::NaiveDate;
use itertools::Itertools;
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;

use super::models::{self, requests, responses};
use crate::calendar::Calendar;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_route(
    pool: &PgPool,
    route_id: i32,
) -> Result<Option<models::Route>> {
    sqlx::query_as!(
        models::Route,
        r#"
SELECT routes.id as id,
    routes.type as type_id,
    routes.operator as operator,
    routes.code as code,
    routes.name as name,
    routes.circular as circular,
    routes.main_subroute as main_subroute,
    routes.active as active
FROM routes
WHERE routes.id = $1
"#,
        route_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_subroute(
    pool: &PgPool,
    subroute_id: i32,
) -> Result<Option<models::Subroute>> {
    sqlx::query_as!(
        models::Subroute,
        r#"
SELECT subroutes.id,
    subroutes.route as route_id,
    subroutes.flag,
    subroutes.circular,
    subroutes.polyline
FROM Subroutes
WHERE subroutes.id = $1
"#,
        subroute_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_route_with_subroutes(
    pool: &PgPool,
    route_id: i32,
) -> Result<Option<responses::Route>> {
    let res = sqlx::query!(
        r#"
SELECT routes.id as route,
    routes.operator as operator,
    routes.code as code,
    routes.name as name,
    routes.circular as circular,
    routes.main_subroute as main_subroute,
    routes.active as active,
    routes.type as service_type,
    route_types.badge_text_color as text_color,
    route_types.badge_bg_color as bg_color,
    subroutes.id as subroute,
    subroutes.flag as subroute_flag,
    subroutes.circular as subroute_circular,
    subroutes.polyline as subroute_polyline
FROM routes
JOIN route_types on routes.type = route_types.id
LEFT JOIN subroutes on routes.id = subroutes.route
WHERE routes.id = $1
ORDER BY routes.id asc
"#,
        route_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut row_iter = res.into_iter();

    if let Some(row) = row_iter.next() {
        let mut route = responses::Route {
            id: row.route,
            type_id: row.service_type,
            name: row.name,
            code: row.code,
            circular: row.circular,
            main_subroute: row.main_subroute,
            badge_text: row.text_color,
            badge_bg: row.bg_color,
            subroutes: vec![responses::Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                circular: row.subroute_circular,
                polyline: row.subroute_polyline,
            }],
            active: row.active,
            operator: row.operator,
        };

        for row in row_iter {
            route.subroutes.push(responses::Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                circular: row.subroute_circular,
                polyline: row.subroute_polyline,
            });
        }
        Ok(Some(route))
    } else {
        Ok(None)
    }
}

pub(crate) async fn fetch_routes_with_subroutes(
    pool: &PgPool,
) -> Result<Vec<responses::Route>> {
    let res = sqlx::query!(
        r#"
SELECT routes.id as route,
    routes.code as code,
    routes.name as name,
    routes.operator as operator,
    routes.type as service_type,
    routes.circular as circular,
    routes.main_subroute as main_subroute,
    routes.active as active,
    route_types.badge_text_color as text_color,
    route_types.badge_bg_color as bg_color,
    subroutes.id as "subroute!: Option<i32>",
    subroutes.flag as "subroute_flag!: Option<String>",
    subroutes.circular as "subroute_circular!: Option<bool>",
    subroutes.polyline as "subroute_polyline!: Option<String>"
FROM routes
JOIN route_types on routes.type = route_types.id
LEFT JOIN subroutes ON routes.id = subroutes.route
ORDER BY routes.id asc
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut routes: HashMap<i32, responses::Route> = HashMap::new();

    for row in res {
        routes
            .entry(row.route)
            .and_modify(|route| {
                if let (Some(id), Some(flag), Some(circular)) = (
                    row.subroute,
                    row.subroute_flag.clone(),
                    row.subroute_circular,
                ) {
                    route.subroutes.push(responses::Subroute {
                        id,
                        flag,
                        circular,
                        polyline: row.subroute_polyline.clone(),
                    });
                }
            })
            .or_insert(responses::Route {
                id: row.route,
                code: row.code,
                name: row.name,
                circular: row.circular,
                main_subroute: row.main_subroute,
                badge_text: row.text_color,
                badge_bg: row.bg_color,
                subroutes: if let (Some(id), Some(flag), Some(circular)) =
                    (row.subroute, row.subroute_flag, row.subroute_circular)
                {
                    vec![responses::Subroute {
                        id,
                        flag,
                        circular,
                        polyline: row.subroute_polyline,
                    }]
                } else {
                    vec![]
                },
                active: row.active,
                operator: row.operator,
                type_id: row.service_type,
            });
    }

    Ok(routes.into_values().collect::<Vec<_>>())
}

pub(crate) async fn insert_route<'c, E>(
    executor: E,
    route: requests::ChangeRoute,
) -> Result<i32>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let res = sqlx::query!(
        r#"
INSERT INTO routes(code, name, main_subroute, operator, active, type)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING id
    "#,
        route.code,
        route.name,
        route.main_subroute,
        route.operator,
        route.active,
        route.type_id,
    )
    .fetch_one(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_route<'c, E>(
    executor: E,
    route_id: i32,
    changes: requests::ChangeRoute,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let _res = sqlx::query!(
        r#"
UPDATE Routes
SET code=$1, name=$2, main_subroute=$3, operator=$4, active=$5, type=$6
WHERE id=$7
    "#,
        changes.code,
        changes.name,
        changes.main_subroute,
        changes.operator,
        changes.active,
        changes.type_id,
        route_id
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_route(pool: &PgPool, route_id: i32) -> Result<()> {
    // TODO Break this up
    // Make it a transaction
    let subroute_count: i64 = sqlx::query!(
        r#"
SELECT count(*) as count
FROM subroutes
WHERE route=$1
"#,
        route_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .count
    .unwrap_or(0);

    if subroute_count > 0 {
        return Err(Error::DependenciesNotMet);
    }

    let deleted_rows = sqlx::query!(
        r#"
DELETE FROM Routes
WHERE id=$1
    "#,
        route_id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .rows_affected();

    match deleted_rows {
        0 => Err(Error::NotFoundUpstream),
        1 => Ok(()),
        _ => unreachable!(),
    }
}

pub(crate) async fn insert_subroute(
    pool: &PgPool,
    route_id: i32,
    subroute: requests::ChangeSubroute,
) -> Result<i32> {
    let res = sqlx::query!(
        r#"
INSERT INTO subroutes(route, flag, circular, polyline)
VALUES ($1, $2, $3, $4)
RETURNING id
    "#,
        route_id,
        subroute.flag,
        subroute.circular,
        subroute.polyline,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_subroute<'c, E>(
    executor: E,
    route_id: i32,
    subroute_id: i32,
    changes: requests::ChangeSubroute,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let _res = sqlx::query!(
        r#"
UPDATE subroutes
SET flag=$1, circular=$2
WHERE id=$3 AND route=$4
    "#,
        changes.flag,
        changes.circular,
        subroute_id,
        route_id,
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_subroute<'c, E>(
    executor: E,
    route_id: i32,
    subroute_id: i32,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let deleted_rows = sqlx::query!(
        r#"
DELETE FROM subroutes
WHERE id=$1 AND route=$2
    "#,
        subroute_id,
        route_id
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .rows_affected();

    match deleted_rows {
        0 => Err(Error::NotFoundUpstream),
        1 => Ok(()),
        _ => unreachable!(),
    }
}

pub(crate) async fn fetch_route_stops(
    pool: &PgPool,
    route_id: i32,
) -> Result<Vec<responses::SubrouteStops>> {
    let res = sqlx::query!(
        r#"
SELECT subroutes.id as subroute, subroute_stops.stop as stop, subroute_stops.time_to_next as diff
FROM subroutes
JOIN subroute_stops ON subroute_stops.subroute = subroutes.id
WHERE subroutes.route=$1
ORDER BY subroutes.id ASC, subroute_stops.idx ASC
    "#,
        route_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let subroute_stops = res
        .into_iter()
        .group_by(|row| row.subroute)
        .into_iter()
        .map(|(subroute, group)| {
            (
                subroute,
                group
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|stop| (stop.stop, stop.diff))
                    .unzip(),
            )
        })
        .map(|(key, (stops, diffs))| responses::SubrouteStops {
            subroute: key,
            stops,
            diffs,
        })
        .collect::<Vec<_>>();

    Ok(subroute_stops)
}

pub(crate) async fn update_subroute_stops(
    // TODO change to transaction-able
    pool: &PgPool,
    route_id: i32,
    subroute_id: i32,
    request: requests::ChangeSubrouteStops,
) -> Result<()> {
    // Check if the current stops match the requests's check
    if request.from.stops.len() != request.from.diffs.len()
        || request.to.stops.len() != request.to.diffs.len()
    {
        return Err(Error::ValidationFailure("Size divergence".to_string()));
    }

    let existing_query_res = sqlx::query!(
        r#"
SELECT subroute_stops.stop as stop, subroute_stops.time_to_next as diff
FROM subroutes
JOIN subroute_stops on subroute_stops.subroute = subroutes.id
WHERE subroutes.route=$1 AND subroutes.id=$2
ORDER BY subroute_stops.idx ASC
    "#,
        route_id,
        subroute_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    // Check for the difference from stored to future
    let stored_len = existing_query_res.len();
    let check_len = request.from.stops.len();
    let to_store_len = request.to.stops.len() as i16;
    let stored_changes = to_store_len - stored_len as i16;

    if check_len != stored_len {
        return Err(Error::ValidationFailure("Check mismatch".to_string()));
    }

    let check_matched = existing_query_res
        .iter()
        .zip(request.from.stops.iter().zip(request.from.diffs.iter()))
        .all(|(row, (from_stop, from_diff))| {
            row.stop == *from_stop && row.diff == *from_diff
        });

    if !check_matched {
        return Err(Error::ValidationFailure("Check mismatch".to_string()));
    }

    let existing_duplicates_count = existing_query_res
        .iter()
        .zip(request.to.stops.iter().zip(request.to.diffs.iter()))
        .filter(|(row, (from_stop, from_diff))| {
            row.stop == **from_stop && row.diff == **from_diff
        })
        .count();

    if stored_changes == 0 && existing_duplicates_count == stored_len {
        return Ok(());
    }

    if stored_changes < 0 {
        let deleted_rows = sqlx::query!(
            r#"
DELETE FROM subroute_stops
WHERE Subroute=$1 AND idx>=$2
    "#,
            subroute_id,
            to_store_len
        )
        .execute(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .rows_affected();

        if deleted_rows != stored_changes.unsigned_abs() as u64 {
            return Err(Error::Processing(
                "Detected an unexpected amount of rows".to_string(),
            ));
        }
    } else if stored_changes > 0 {
        let additional_entries = request
            .to
            .stops
            .iter()
            .zip(request.to.diffs.iter())
            .skip(stored_len)
            .enumerate();

        for (index, (stop, _diff)) in additional_entries {
            let index = (stored_len + index) as i16;
            let _res = sqlx::query!(
                r#"
INSERT INTO subroute_stops(subroute, stop, idx, time_to_next)
VALUES ($1, $2, $3, $4)
    "#,
                subroute_id,
                stop,
                index,
                // FIXME this should be the diff variable
                0
            )
            .execute(pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    };

    if existing_duplicates_count != stored_len {
        // Update the already existing records
        let overlapping_entries = request
            .to
            .stops
            .iter()
            .zip(request.to.diffs.into_iter())
            .take(stored_len)
            .enumerate();

        for (index, (stop, diff)) in overlapping_entries {
            let index = index as i16;
            let _res = sqlx::query!(
                r#"
UPDATE subroute_stops
SET stop=$1, time_to_next=$2
WHERE  subroute=$3 AND idx=$4
    "#,
                stop,
                diff,
                subroute_id,
                index
            )
            .execute(pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    }
    Ok(())
}

pub(crate) async fn fetch_subroute_stop_count(
    pool: &PgPool,
    subroute_id: i32,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as count
FROM subroute_stops
WHERE subroute=$1
"#,
        subroute_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .count
    .unwrap_or(0))
}

pub(crate) async fn fetch_subroute_departure_count(
    pool: &PgPool,
    subroute_id: i32,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as count
FROM departures
WHERE subroute=$1
"#,
        subroute_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .count
    .unwrap_or(0))
}

pub(crate) async fn fetch_schedule(
    pool: &PgPool,
    route_id: i32,
) -> Result<Vec<responses::Departure>> {
    let res = sqlx::query!(
        r#"
SELECT departures.id as id,
    subroutes.id as subroute,
    departures.time as time,
    departures.calendar_id as calendar_id,
    departures.calendar as calendar
FROM subroutes
JOIN departures on departures.subroute = subroutes.id
WHERE subroutes.route=$1
ORDER BY time ASC
    "#,
        route_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut departures = vec![];
    for row in res {
        departures.push(responses::Departure {
            id: row.id,
            subroute: row.subroute,
            time: row.time,
            calendar_id: row.calendar_id,
            calendar: if let Some(calendar) = row.calendar {
                Some(
                    serde_json::from_value(calendar)
                        .map_err(|_err| Error::DatabaseDeserialization)?,
                )
            } else {
                None
            },
        });
    }

    Ok(departures)
}

pub(crate) async fn fetch_schedule_for_date(
    pool: &PgPool,
    route_id: i32,
    date: NaiveDate,
) -> Result<Vec<responses::DateDeparture>> {
    let res = sqlx::query!(
        r#"
SELECT subroutes.id as subroute, departures.time as time, departures.calendar as calendar
FROM subroutes
JOIN departures on departures.subroute = subroutes.id
WHERE subroutes.route=$1
ORDER BY time asc
    "#,
        route_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut departures = vec![];
    for row in res {
        if let Some(calendar) = row.calendar {
            let calendar: Calendar = serde_json::from_value(calendar)
                .map_err(|_err| Error::DatabaseDeserialization)?;
            if calendar.includes(date) {
                departures.push(responses::DateDeparture {
                    subroute: row.subroute,
                    time: row.time,
                });
            }
        } else {
            // TODO
            continue;
        };
    }

    Ok(departures)
}

pub(crate) async fn insert_departure<'c, E>(
    executor: E,
    subroute_id: i32,
    departure: requests::ChangeDeparture,
) -> Result<i32>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let res = sqlx::query!(
        r#"
INSERT INTO departures(subroute, time, calendar, calendar_id)
VALUES($1, $2, $3, $4)
RETURNING id
    "#,
        subroute_id,
        departure.time,
        json!(departure.calendar),
        departure.calendar_id
    )
    .fetch_one(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(res.id)
}

pub(crate) async fn update_departure<'c, E>(
    executor: E,
    subroute_id: i32,
    departure_id: i32,
    departure: requests::ChangeDeparture,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let _res = sqlx::query!(
        r#"
UPDATE departures
SET time=$1, calendar=$2
WHERE id=$3 AND subroute=$4
    "#,
        departure.time,
        json!(departure.calendar),
        departure_id,
        subroute_id,
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn delete_departure<'c, E>(
    executor: E,
    subroute_id: i32,
    departure_id: i32,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let _res = sqlx::query!(
        r#"
DELETE FROM departures
WHERE id=$1 AND subroute=$2
    "#,
        departure_id,
        subroute_id,
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}
