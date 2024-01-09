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

use itertools::Itertools;
use sqlx::PgPool;

use commons::models::routes;

use crate::Error;

use super::models::{requests, responses};

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_commons_route(
    pool: &PgPool,
    route_id: i32,
) -> Result<Option<routes::Route>> {
    sqlx::query_as!(
        routes::Route,
        r#"
SELECT routes.id as id,
    routes.type as type_id,
    routes.operator as operator_id,
    routes.code as code,
    routes.name as name,
    routes.main_subroute as main_subroute,
    routes.circular as circular,
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

pub(crate) async fn fetch_simple_subroute(
    pool: &PgPool,
    subroute_id: i32,
) -> Result<Option<routes::Subroute>> {
    let res = sqlx::query!(
        r#"
SELECT subroutes.id,
    subroutes.route as route_id,
    subroutes.flag,
    subroutes.circular,
    subroutes.polyline,
    subroutes.group,
    subroutes.origin,
    subroutes.destination,
    subroutes.headsign,
    subroutes.via
FROM Subroutes
WHERE subroutes.id = $1
"#,
        subroute_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if let Some(row) = res {
        Ok(Some(routes::Subroute {
            id: row.id,
            route_id: row.route_id,
            group: row.group,
            flag: row.flag,
            origin: row.origin,
            destination: row.destination,
            headsign: row.headsign,
            via: serde_json::from_value(row.via).map_err(|e| {
                log::error!("Error deserializing: {}", e);
                Error::DatabaseDeserialization
            })?,
            circular: row.circular,
            polyline: row.polyline,
            validation: None,
        }))
    } else {
        Ok(None)
    }
}

pub(crate) async fn fetch_route_with_subroutes(
    pool: &PgPool,
    route_id: i32,
) -> Result<Option<responses::Route>> {
    sqlx::query_as!(
        responses::Route,
        r#"
SELECT routes.*, route_types.badge_text_color as badge_text, route_types.badge_bg_color as badge_bg
FROM (
    SELECT routes.id, routes.code, routes.name, routes.operator, routes.type as type_id,
        routes.circular, routes.main_subroute, routes.active, routes.parishes,
        COALESCE(
            array_agg((subroutes.id, subroutes.group, subroutes.flag, subroutes.headsign, subroutes.origin,
                subroutes.destination, subroutes.via, subroutes.circular, subroutes.polyline))
            FILTER (WHERE subroutes.id IS NOT NULL),
            '{}'
        ) AS "subroutes!: Vec<responses::Subroute>"
    FROM routes
    LEFT JOIN subroutes ON routes.id = subroutes.route
    WHERE routes.id = $1
    GROUP BY routes.id
    ORDER BY routes.id asc
) as routes
JOIN route_types on routes.type_id = route_types.id"#,
        route_id
    )
        .fetch_optional(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_routes(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::Route>> {
    sqlx::query_as!(
        responses::Route,
        r#"
SELECT routes.*, route_types.badge_text_color as badge_text, route_types.badge_bg_color as badge_bg
FROM (
    SELECT routes.id, routes.code, routes.name, routes.operator, routes.type as type_id,
        routes.circular, routes.main_subroute, routes.active, routes.parishes,
        COALESCE(
            array_agg((subroutes.id, subroutes.group, subroutes.flag, subroutes.headsign, subroutes.origin,
                subroutes.destination, subroutes.via, subroutes.circular, subroutes.polyline))
            FILTER (WHERE subroutes.id IS NOT NULL),
            '{}'
        ) AS "subroutes!: Vec<responses::Subroute>"
    FROM routes
    LEFT JOIN subroutes ON routes.id = subroutes.route
    GROUP BY routes.id
    ORDER BY routes.id asc
) as routes
JOIN route_types on routes.type_id = route_types.id
JOIN region_routes on routes.id = region_routes.region_id
WHERE region_routes.region_id = $1
"#,
        region_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_operator_routes(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::Route>> {
    sqlx::query_as!(
        responses::Route,
        r#"
SELECT routes.*, route_types.badge_text_color as badge_text, route_types.badge_bg_color as badge_bg
FROM (
    SELECT routes.id, routes.code, routes.name, routes.operator, routes.type as type_id,
        routes.circular, routes.main_subroute, routes.active, routes.parishes,
        COALESCE(
            array_agg((subroutes.id, subroutes.group, subroutes.flag, subroutes.headsign, subroutes.origin,
                subroutes.destination, subroutes.via, subroutes.circular, subroutes.polyline))
            FILTER (WHERE subroutes.id IS NOT NULL),
            '{}'
        ) AS "subroutes!: Vec<responses::Subroute>"
    FROM routes
    LEFT JOIN subroutes ON routes.id = subroutes.route
    WHERE routes.operator = $1
    GROUP BY routes.id
    ORDER BY routes.id asc
) as routes
JOIN route_types on routes.type_id = route_types.id
        "#,
        operator_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_full_routes(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::FullRoute>> {
    sqlx::query_as!(
        responses::FullRoute,
        r#"
SELECT routes.*, route_types.badge_text_color as badge_text, route_types.badge_bg_color as badge_bg
FROM (
    SELECT routes.id, routes.code, routes.name, routes.operator, routes.type as type_id,
        routes.circular, routes.main_subroute, routes.active, routes.parishes,
        COALESCE(
            array_agg((subroutes.id, subroutes.group, subroutes.flag, subroutes.headsign, subroutes.origin,
                subroutes.destination, subroutes.via, subroutes.circular, subroutes.polyline, subroutes.validation))
            FILTER (WHERE subroutes.id IS NOT NULL),
            '{}'
        ) AS "subroutes!: Vec<responses::FullSubroute>"
    FROM routes
    LEFT JOIN subroutes ON routes.id = subroutes.route
    GROUP BY routes.id
    ORDER BY routes.id asc
) as routes
JOIN route_types on routes.type_id = route_types.id
JOIN region_routes on routes.id = region_routes.region_id
WHERE region_routes.region_id = $1
    "#,
        region_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn insert_route(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route: requests::ChangeRoute,
) -> Result<routes::Route> {
    let res = sqlx::query!(
        r#"
INSERT INTO routes(code, name, main_subroute, operator, circular, active, type)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id
    "#,
        route.code,
        route.name,
        route.main_subroute,
        route.operator_id,
        route.circular,
        route.active,
        route.type_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(routes::Route {
        id: res.id,
        type_id: route.type_id,
        operator_id: route.operator_id,
        code: route.code,
        name: route.name,
        circular: route.circular,
        main_subroute: route.main_subroute,
        active: route.active,
    })
}

pub(crate) async fn update_route(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    changes: requests::ChangeRoute,
) -> Result<()> {
    let _res = sqlx::query!(
        r#"
UPDATE Routes
SET code=$1, name=$2, main_subroute=$3, operator=$4, circular=$5, active=$6, type=$7
WHERE id=$8
    "#,
        changes.code,
        changes.name,
        changes.main_subroute,
        changes.operator_id,
        changes.circular,
        changes.active,
        changes.type_id,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_route(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
) -> Result<()> {
    let subroute_count: i64 = sqlx::query!(
        r#"
SELECT count(*) as count
FROM subroutes
WHERE route=$1
"#,
        route_id
    )
    .fetch_one(&mut **transaction)
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
    .execute(&mut **transaction)
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
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    change: requests::ChangeSubroute,
) -> Result<routes::Subroute> {
    let res = sqlx::query!(
        r#"
INSERT INTO subroutes(route, "group", flag, origin, destination, headsign,  via, circular)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING id
    "#,
        route_id,
        change.group,
        change.flag,
        change.origin,
        change.destination,
        change.headsign,
        sqlx::types::Json(&change.via) as _,
        change.circular,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(routes::Subroute {
        id: res.id,
        route_id,
        group: change.group,
        flag: change.flag,
        origin: change.origin,
        destination: change.destination,
        headsign: change.headsign,
        via: change.via,
        circular: change.circular,
        polyline: None,
        validation: None,
    })
}

pub(crate) async fn update_subroute(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    subroute_id: i32,
    changes: requests::ChangeSubroute,
) -> Result<()> {
    let _res = sqlx::query!(
        r#"
UPDATE subroutes
SET "group"=$1, flag=$2, origin=$3, destination=$4, headsign=$5, via=$6, circular=$7
WHERE id=$8 AND route=$9
    "#,
        changes.group,
        changes.flag,
        changes.origin,
        changes.destination,
        changes.headsign,
        sqlx::types::Json(&changes.via) as _,
        changes.circular,
        subroute_id,
        route_id,
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_subroute(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    subroute_id: i32,
) -> Result<()> {
    let deleted_rows = sqlx::query!(
        r#"
DELETE FROM subroutes
WHERE id=$1 AND route=$2
    "#,
        subroute_id,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .rows_affected();

    match deleted_rows {
        0 => Err(Error::NotFoundUpstream),
        1 => Ok(()),
        _ => unreachable!(),
    }
}

pub(crate) async fn delete_subroute_stops(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM subroute_stops
WHERE subroute=$1
    "#,
        subroute_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_subroute_departures(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM departures
WHERE subroute=$1
    "#,
        subroute_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_route_stops(
    pool: &PgPool,
    route_id: i32,
) -> Result<Vec<responses::SubrouteStops>> {
    let res = sqlx::query!(
        r#"
SELECT subroutes.id as subroute, subroute_stops.stop as stop
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

    // TODO Consider moving the stop indexes to an array (in the DB)
    let subroute_stops = res
        .into_iter()
        .group_by(|row| row.subroute)
        .into_iter()
        .map(|(subroute, group)| responses::SubrouteStops {
            subroute,
            stops: group.map(|stop| stop.stop).collect(),
        })
        .collect::<Vec<_>>();

    Ok(subroute_stops)
}

pub(crate) async fn update_subroute_stops(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    subroute_id: i32,
    request: requests::ChangeSubrouteStops,
) -> Result<()> {
    let existing_query_res = sqlx::query!(
        r#"
SELECT subroute_stops.stop as stop
FROM subroutes
JOIN subroute_stops on subroute_stops.subroute = subroutes.id
WHERE subroutes.route=$1 AND subroutes.id=$2
ORDER BY subroute_stops.idx ASC
    "#,
        route_id,
        subroute_id
    )
    .fetch_all(&mut **transaction)
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
        .zip(request.from.stops.iter())
        .all(|(row, from_stop)| row.stop == *from_stop);

    if !check_matched {
        return Err(Error::ValidationFailure("Check mismatch".to_string()));
    }

    let existing_duplicates_count = existing_query_res
        .iter()
        .zip(request.to.stops.iter())
        .filter(|(row, from_stop)| row.stop == **from_stop)
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
        .execute(&mut **transaction)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
        .rows_affected();

        if deleted_rows != u64::from(stored_changes.unsigned_abs()) {
            return Err(Error::Processing(
                "Detected an unexpected amount of rows".to_string(),
            ));
        }
    } else if stored_changes > 0 {
        let additional_entries =
            request.to.stops.iter().skip(stored_len).enumerate();

        for (index, stop) in additional_entries {
            let index = (stored_len + index) as i16;
            let _res = sqlx::query!(
                r#"
INSERT INTO subroute_stops(subroute, stop, idx)
VALUES ($1, $2, $3)
    "#,
                subroute_id,
                stop,
                index
            )
            .execute(&mut **transaction)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    };

    if existing_duplicates_count != stored_len {
        // Update the already existing records
        let overlapping_entries =
            request.to.stops.iter().take(stored_len).enumerate();

        for (index, stop) in overlapping_entries {
            let index = index as i16;
            let _res = sqlx::query!(
                r#"
UPDATE subroute_stops
SET stop=$1
WHERE  subroute=$2 AND idx=$3
    "#,
                stop,
                subroute_id,
                index
            )
            .execute(&mut **transaction)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    }
    Ok(())
}

pub(crate) async fn fetch_subroute_stops(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
) -> Result<Vec<i32>> {
    Ok(sqlx::query!(
        r#"
SELECT stop
FROM subroute_stops
WHERE subroute=$1
"#,
        subroute_id
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    .into_iter()
    .map(|r| r.stop)
    .collect())
}

pub(crate) async fn fetch_subroute_departures(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
) -> Result<Vec<routes::Departure>> {
    sqlx::query_as!(
        routes::Departure,
        r#"
SELECT departures.id as id,
    departures.time as time,
    departures.subroute as subroute_id,
    departures.calendar_id as "calendar_id!: i32"
FROM departures
WHERE departures.subroute = $1
"#,
        subroute_id
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
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
    departures.calendar_id as "calendar_id!: i32"
FROM departures
INNER JOIN subroutes on departures.subroute = subroutes.id
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
        });
    }

    Ok(departures)
}

pub(crate) async fn fetch_departure<'c, E>(
    executor: E,
    departure_id: i32,
) -> Result<Option<routes::Departure>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        routes::Departure,
        r#"
SELECT departures.id as id,
    Departures.time as time,
    Departures.subroute as subroute_id,
    Departures.calendar_id as "calendar_id!: i32"
FROM Departures
WHERE departures.id = $1
"#,
        departure_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn insert_departure(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    departure: requests::ChangeDeparture,
) -> Result<routes::Departure> {
    let res = sqlx::query!(
        r#"
INSERT INTO departures(subroute, time, calendar_id)
VALUES($1, $2, $3)
RETURNING id
    "#,
        subroute_id,
        departure.time,
        departure.calendar_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(routes::Departure {
        id: res.id,
        subroute_id,
        time: departure.time,
        calendar_id: departure.calendar_id,
    })
}

pub(crate) async fn update_departure(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    departure_id: i32,
    departure: requests::ChangeDeparture,
) -> Result<()> {
    let _res = sqlx::query!(
        r#"
UPDATE departures
SET time=$1, calendar_id=$2
WHERE id=$3 AND subroute=$4
    "#,
        departure.time,
        departure.calendar_id,
        departure_id,
        subroute_id,
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn delete_departure(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    departure_id: i32,
) -> Result<()> {
    let _res = sqlx::query!(
        r#"
DELETE FROM departures
WHERE id=$1 AND subroute=$2
    "#,
        departure_id,
        subroute_id,
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}

pub(crate) async fn migrate_stop(
    pool: &PgPool,
    original_id: i32,
    destination_id: i32,
) -> Result<()> {
    let res = sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM stops
WHERE id = ($1) OR id = ($2)
    "#,
        destination_id,
        original_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if res.cnt != Some(2) {
        return Err(Error::ValidationFailure("Invalid stop id".to_string()));
    }

    let _res = sqlx::query!(
        r#"
UPDATE subroute_stops
SET stop=$1
WHERE stop=$2
    "#,
        destination_id,
        original_id,
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    Ok(())
}
