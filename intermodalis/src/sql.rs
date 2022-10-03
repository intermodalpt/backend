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

use chrono::{Local, NaiveDate};
use itertools::Itertools;
use sqlx::PgPool;
use std::collections::{hash_map, HashMap};

use crate::calendar::Calendar;
use crate::models::requests::{
    ChangeRoute, ChangeStop, ChangeStopPic, ChangeSubroute,
    ChangeSubrouteStops, NewStop,
};
use crate::models::responses::{
    DateDeparture, Departure, Parish, PublicStopPic, Route, SpiderMap,
    SpiderRoute, SpiderStop, SpiderSubroute, Subroute, SubrouteStops,
    TaggedStopPic, UntaggedStopPic,
};
use crate::models::Stop;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_parishes(pool: &PgPool) -> Result<Vec<Parish>> {
    sqlx::query_as!(
        Parish,
        r#"
SELECT parishes.id, parishes.name, municipalities.name as municipality,
    municipalities.zone, parishes.polygon
FROM parishes
JOIN municipalities ON parishes.municipality = municipalities.id
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_stops(
    pool: &PgPool,
    filter_used: bool,
) -> Result<Vec<Stop>> {
    Ok(if filter_used {
        sqlx::query_as!(
            Stop,
            r#"
SELECT *
FROM Stops
WHERE id IN (
    SELECT DISTINCT stop
    FROM subroute_stops
)
    "#
        )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    } else {
        sqlx::query_as!(Stop, "SELECT * FROM stops")
            .fetch_all(pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?
    })
}

pub(crate) async fn fetch_bounded_stops(
    pool: &PgPool,
    (x0, y0, x1, y1): (f64, f64, f64, f64),
) -> Result<Vec<Stop>> {
    sqlx::query_as!(
        Stop,
        r#"
SELECT *
FROM Stops
WHERE lon >= $1 AND lon <= $2 AND lat <= $3 AND lat >= $4 AND id IN (
    SELECT DISTINCT stop FROM subroute_stops
)
    "#,
        x0,
        x1,
        y0,
        y1
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn insert_stop(
    pool: &PgPool,
    stop: NewStop,
    user_id: i32,
) -> Result<i32> {
    let update_date = Local::now().to_string();

    let res = sqlx::query!(
        r#"
INSERT INTO Stops(name, short_name, official_name, locality, street, door,
    lon, lat, notes, tags, has_crossing, has_accessibility,
    has_abusive_parking, has_outdated_info, is_damaged,
    is_vandalized, has_flag, has_schedules, has_sidewalk,
    has_shelter, has_bench, has_trash_can, is_illuminated,
    has_illuminated_path, has_visibility_from_within,
    has_visibility_from_area, is_visible_from_outside,
    updater, update_date, source)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30)
RETURNING id
    "#,
        stop.name,
        stop.short_name,
        stop.official_name,
        stop.locality,
        stop.street,
        stop.door,
        stop.lon,
        stop.lat,
        stop.notes,
        &stop.tags,
        stop.has_crossing,
        stop.has_accessibility,
        stop.has_abusive_parking,
        stop.has_outdated_info,
        stop.is_damaged,
        stop.is_vandalized,
        stop.has_flag,
        stop.has_schedules,
        stop.has_sidewalk,
        stop.has_shelter,
        stop.has_bench,
        stop.has_trash_can,
        stop.is_illuminated,
        stop.has_illuminated_path,
        stop.has_visibility_from_within,
        stop.has_visibility_from_area,
        stop.is_visible_from_outside,
        user_id,
        update_date,
        stop.source
    )
        .fetch_one(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_stop(
    pool: &PgPool,
    stop_id: i32,
    changes: ChangeStop,
    user_id: i32,
) -> Result<()> {
    let update_date = Local::now().to_string();

    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET name=$1, short_name=$2, official_name=$3, locality=$4, street=$5, door=$6,
    lon=$7, lat=$8, notes=$9, tags=$10, has_crossing=$11, has_accessibility=$12,
    has_abusive_parking=$13, has_outdated_info=$14, is_damaged=$15,
    is_vandalized=$16, has_flag=$17, has_schedules=$18, has_sidewalk=$19,
    has_shelter=$20, has_bench=$21, has_trash_can=$22, is_illuminated=$23,
    has_illuminated_path=$24, has_visibility_from_within=$25,
    has_visibility_from_area=$26, is_visible_from_outside=$27,
    updater=$28, update_date=$29
WHERE id=$30
    "#,
        changes.name,
        changes.short_name,
        changes.official_name,
        changes.locality,
        changes.street,
        changes.door,
        changes.lon,
        changes.lat,
        changes.notes,
        &changes.tags,
        changes.has_crossing,
        changes.has_accessibility,
        changes.has_abusive_parking,
        changes.has_outdated_info,
        changes.is_damaged,
        changes.is_vandalized,
        changes.has_flag,
        changes.has_schedules,
        changes.has_sidewalk,
        changes.has_shelter,
        changes.has_bench,
        changes.has_trash_can,
        changes.is_illuminated,
        changes.has_illuminated_path,
        changes.has_visibility_from_within,
        changes.has_visibility_from_area,
        changes.is_visible_from_outside,
        user_id,
        update_date,
        stop_id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_stop_spider(
    pool: &PgPool,
    stops: &[i32],
) -> Result<SpiderMap> {
    let res = sqlx::query!(
        r#"
SELECT Routes.id as route_id,
    routes.code as "route_code!: Option<String>",
    routes.name as route_name,
    routes.circular as route_circular,
    subroutes.id as subroute_id,
    subroutes.flag as subroute_flag,
    subroute_stops.stop as stop_id,
    stops.name as stop_name,
    stops.lon as lon,
    stops.lat as lat
FROM routes
JOIN subroutes ON routes.id = subroutes.route
JOIN subroute_stops ON subroutes.id = subroute_stops.subroute
JOIN stops ON stops.id = subroute_stops.stop
WHERE subroutes.id IN (
    SELECT subroutes.id
    FROM subroutes
    JOIN subroute_stops ON subroutes.id = subroute_stops.subroute
    WHERE subroute_stops.stop = ANY($1)
)
ORDER BY subroute_stops.idx"#,
        &stops[..]
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut routes: HashMap<i32, SpiderRoute> = HashMap::new();
    let mut subroutes: HashMap<i32, SpiderSubroute> = HashMap::new();
    let mut stops: HashMap<i32, SpiderStop> = HashMap::new();

    for row in res {
        if let hash_map::Entry::Vacant(e) = routes.entry(row.route_id) {
            e.insert(SpiderRoute {
                code: row.route_code,
                name: row.route_name,
                circular: row.route_circular,
            });
        }

        if let Some(subroute) = subroutes.get_mut(&row.subroute_id) {
            subroute.stop_sequence.push(row.stop_id);
        } else {
            subroutes.insert(
                row.subroute_id,
                SpiderSubroute {
                    route: row.route_id,
                    flag: row.subroute_flag,
                    stop_sequence: vec![],
                },
            );
        }

        if let hash_map::Entry::Vacant(e) = stops.entry(row.stop_id) {
            e.insert(SpiderStop {
                name: row.stop_name,
                lat: row.lat,
                lon: row.lon,
            });
        }
    }

    Ok(SpiderMap {
        routes,
        subroutes,
        stops,
    })
}

pub(crate) async fn fetch_route(
    pool: &PgPool,
    route_id: i32,
) -> Result<Option<Route>> {
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
    subroutes.circular as subroute_circular
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
        let mut route = Route {
            id: row.route,
            service_type: row.service_type,
            name: row.name,
            code: row.code,
            circular: row.circular,
            main_subroute: row.main_subroute,
            badge_text: row.text_color,
            badge_bg: row.bg_color,
            subroutes: vec![Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                circular: row.subroute_circular,
            }],
            active: row.active,
            operator: row.operator,
        };

        for row in row_iter {
            route.subroutes.push(Subroute {
                id: row.subroute,
                flag: row.subroute_flag,
                circular: row.subroute_circular,
            });
        }
        Ok(Some(route))
    } else {
        Ok(None)
    }
}

pub(crate) async fn fetch_routes(pool: &PgPool) -> Result<Vec<Route>> {
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
    subroutes.circular as "subroute_circular!: Option<bool>"
FROM routes
JOIN route_types on routes.type = route_types.id
LEFT JOIN subroutes ON routes.id = subroutes.route
ORDER BY routes.id asc
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut routes: HashMap<i32, Route> = HashMap::new();

    for row in res {
        routes
            .entry(row.route)
            .and_modify(|route| {
                if let (Some(id), Some(flag), Some(circular)) = (
                    row.subroute,
                    row.subroute_flag.clone(),
                    row.subroute_circular,
                ) {
                    route.subroutes.push(Subroute { id, flag, circular });
                }
            })
            .or_insert(Route {
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
                    vec![Subroute { id, flag, circular }]
                } else {
                    vec![]
                },
                active: row.active,
                operator: row.operator,
                service_type: row.service_type,
            });
    }

    Ok(routes.into_values().collect::<Vec<_>>())
}

pub(crate) async fn insert_route(
    pool: &PgPool,
    route: ChangeRoute,
) -> Result<i32> {
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
        route.service_type,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_route(
    pool: &PgPool,
    route_id: i32,
    changes: ChangeRoute,
) -> Result<()> {
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
        changes.service_type,
        route_id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_route(pool: &PgPool, route_id: i32) -> Result<()> {
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
    subroute: ChangeSubroute,
) -> Result<i32> {
    let res = sqlx::query!(
        r#"
INSERT INTO subroutes(route, flag, circular)
VALUES ($1, $2, $3)
RETURNING id
    "#,
        route_id,
        subroute.flag,
        subroute.circular,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(res.id)
}

pub(crate) async fn update_subroute(
    pool: &PgPool,
    route_id: i32,
    subroute_id: i32,
    changes: ChangeSubroute,
) -> Result<()> {
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
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn delete_subroute(
    pool: &PgPool,
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

pub(crate) async fn fetch_route_stops(
    pool: &PgPool,
    route_id: i32,
) -> Result<Vec<SubrouteStops>> {
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
        .map(|(key, (stops, diffs))| SubrouteStops {
            subroute: key,
            stops,
            diffs,
        })
        .collect::<Vec<_>>();

    Ok(subroute_stops)
}

pub(crate) async fn update_subroute_stops(
    pool: &PgPool,
    route_id: i32,
    subroute_id: i32,
    request: ChangeSubrouteStops,
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
) -> Result<Vec<Departure>> {
    let res = sqlx::query!(
        r#"
SELECT departures.id as id,
    subroutes.id as subroute,
    departures.time as time,
    departures.calendar as calendar
FROM subroutes
JOIN departures on departures.subroute = subroutes.id
WHERE subroutes.route=$1
    "#,
        route_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut departures = vec![];
    for row in res {
        departures.push(Departure {
            id: row.id,
            subroute: row.subroute,
            time: row.time,
            calendar: serde_json::from_value(row.calendar)
                .map_err(|_err| Error::DatabaseDeserialization)?,
        });
    }

    Ok(departures)
}

pub(crate) async fn fetch_schedule_for_date(
    pool: &PgPool,
    route_id: i32,
    date: NaiveDate,
) -> Result<Vec<DateDeparture>> {
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
        let calendar: Calendar = serde_json::from_value(row.calendar)
            .map_err(|_err| Error::DatabaseDeserialization)?;
        if calendar.includes(date) {
            departures.push(DateDeparture {
                subroute: row.subroute,
                time: row.time,
            });
        }
    }

    Ok(departures)
}

pub(crate) async fn fetch_stop_pictures(
    pool: &PgPool,
) -> Result<Vec<TaggedStopPic>> {
    sqlx::query_as!(
        TaggedStopPic,
        r#"
SELECT id, original_filename, sha1, public, sensitive, uploader,
    upload_date, capture_date, lon, lat, quality, width,
    height, camera_ref, tags, notes
FROM stop_pics
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_public_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<PublicStopPic>> {
    sqlx::query_as!(
        PublicStopPic,
        r#"
SELECT stop_pics.id, stop_pics.sha1, stop_pics.capture_date, stop_pics.lon, stop_pics.lat, stop_pics.tags, stop_pics.quality
FROM stop_pics
JOIN stop_pic_stops on stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.tagged = false AND stop_pics.sensitive = false
    AND stop_pics.public = true AND stop_pic_stops.stop=$1
ORDER BY stop_pics.capture_date DESC
    "#,
        stop_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_tagged_stop_pictures(
    pool: &PgPool,
    stop_id: i32,
) -> Result<Vec<TaggedStopPic>> {
    sqlx::query_as!(
        TaggedStopPic,
        r#"
SELECT stop_pics.id, stop_pics.original_filename, stop_pics.sha1,
    stop_pics.public, stop_pics.sensitive, stop_pics.uploader,
    stop_pics.upload_date, stop_pics.capture_date, stop_pics.quality,
    stop_pics.width, stop_pics.height, stop_pics.lon, stop_pics.lat,
    stop_pics.camera_ref, stop_pics.tags, stop_pics.notes
FROM stop_pics
JOIN stop_pic_stops ON stop_pic_stops.pic = stop_pics.id
WHERE stop_pics.tagged = true AND stop_pic_stops.stop=$1
ORDER BY quality DESC
    "#,
        stop_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_untagged_stop_pictures(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<UntaggedStopPic>> {
    sqlx::query_as!(
        UntaggedStopPic,
        r#"
SELECT id, original_filename, sha1, public, sensitive, uploader, upload_date,
    capture_date, width, height, lon, lat, camera_ref, tags, notes
FROM stop_pics
WHERE tagged=false AND uploader=$1
ORDER BY capture_date ASC
LIMIT $2 OFFSET $3
    "#,
        user_id,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn fetch_picture_stop_rels(
    pool: &PgPool,
) -> Result<HashMap<i32, Vec<i32>>> {
    let res = sqlx::query!(
        r#"
SELECT stop, pic
FROM  stop_pic_stops
ORDER BY stop ASC
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let mut stops = HashMap::<i32, Vec<i32>>::new();

    for row in res {
        if let Some(pics) = stops.get_mut(&row.stop) {
            pics.push(row.pic);
        } else {
            stops.insert(row.stop, vec![row.pic]);
        }
    }
    Ok(stops)
}

pub(crate) async fn update_stop_picture_meta(
    pool: &PgPool,
    stop_picture_id: i32,
    stop_pic_meta: ChangeStopPic,
    user_id: i32,
) -> Result<()> {
    let update_date = Local::now().to_string();

    let _res = sqlx::query!(
        r#"
UPDATE stop_pics
SET public=$1, sensitive=$2, lon=$3, lat=$4, tags=$5, quality=$6, updater=$7,
    update_date=$8, tagged=true
WHERE id=$9
    "#,
        stop_pic_meta.public,
        stop_pic_meta.sensitive,
        stop_pic_meta.lon,
        stop_pic_meta.lat,
        &stop_pic_meta.tags,
        stop_pic_meta.quality,
        user_id,
        update_date,
        stop_picture_id
    )
    .execute(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    if !stop_pic_meta.stops.is_empty() {
        // TODO add updater and update date
        let stop_ids = stop_pic_meta.stops.iter().join(",");

        let _res = sqlx::query(&format!(
            r#"
    DELETE FROM stop_pic_stops
    WHERE pic=$1 AND stop NOT IN ({stop_ids})
        "#
        ))
        .bind(stop_picture_id)
        .execute(pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

        for stop_id in stop_pic_meta.stops {
            let _res = sqlx::query!(
                r#"
INSERT INTO stop_pic_stops(pic, stop)
VALUES ($1, $2)
ON CONFLICT DO NOTHING
    "#,
                stop_picture_id,
                stop_id
            )
            .execute(pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
        }
    }

    Ok(())
}
