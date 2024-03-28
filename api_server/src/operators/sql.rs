/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2024  Cláudio Pereira

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
use sqlx::types::Json;
use sqlx::PgPool;

use commons::models::calendar::Calendar;
use commons::models::operators;

use super::models::{self, requests, responses};
use crate::pics::get_logo_path;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_operator(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Option<models::Operator>> {
    sqlx::query_as!(
        models::Operator,
        r#"
SELECT id, name, tag, logo_sha1
FROM Operators
WHERE id = $1
"#,
        operator_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id = operator_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_operators(
    pool: &PgPool,
) -> Result<Vec<responses::OperatorWithRegions>> {
    Ok(sqlx::query!(
        r#"
SELECT id, name, tag, description, logo_sha1, array_remove(array_agg(region_id), NULL) as "regions!: Vec<i32>"
FROM operators
LEFT JOIN region_operators ON region_operators.operator_id = operators.id
GROUP BY operators.id
"#
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string());
            Error::DatabaseExecution
        })?
        .into_iter()
        .map(|row| responses::OperatorWithRegions {
            id: row.id,
            name: row.name,
            tag: row.tag,
            description: row.description,
            logo_url: row.logo_sha1.map(|sha1| get_logo_path(row.id, &sha1)),
            regions: row.regions,
        })
        .collect())
}

pub(crate) async fn insert_operator(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    change: requests::ChangeOperator,
) -> Result<responses::Operator> {
    // FIXME this is a workaround because SQLX is not figuring the description's Option<String> properly
    let id = if let Some(description) = &change.description {
        sqlx::query!(
            r#"
INSERT INTO operators(name, tag, description)
VALUES ($1, $2, $3)
RETURNING id
"#,
            &change.name,
            &change.tag,
            description
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), change = ?change, description);
            Error::DatabaseExecution
        })?
        .id
    } else {
        sqlx::query!(
            r#"
INSERT INTO operators(name, tag)
VALUES ($1, $2)
RETURNING id
"#,
            &change.name,
            &change.tag
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                name = change.name,
                tag = change.tag
            );
            Error::DatabaseExecution
        })?
        .id
    };

    Ok(responses::Operator {
        id,
        name: change.name,
        tag: change.tag,
        description: change.description,
        logo_url: None,
    })
}

pub(crate) async fn update_operator(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    change: requests::ChangeOperator,
) -> Result<()> {
    // FIXME this is a workaround because SQLX is not figuring the description's Option<String> properly
    if let Some(description) = change.description {
        sqlx::query!(
            r#"
UPDATE operators
SET name = $1,
    tag = $2,
    description = $3
WHERE id = $4
"#,
            &change.name,
            &change.tag,
            &description,
            operator_id
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                name = &change.name,
                tag = &change.tag,
                description = description,
                operator_id = operator_id
            );
            Error::DatabaseExecution
        })?;
    } else {
        sqlx::query!(
            r#"
UPDATE operators
SET name = $1,
    tag = $2
WHERE id = $3
"#,
            &change.name,
            &change.tag,
            operator_id
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                name = &change.name,
                tag = &change.tag,
                operator_id = operator_id
            );
            Error::DatabaseExecution
        })?;
    };

    Ok(())
}

pub(crate) async fn fetch_operator_stops(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::OperatorStop>> {
    sqlx::query_as!(
        responses::OperatorStop,
        r#"
SELECT stops.id, stops.lat, stops.lon, stop_operators.official_name, stop_ref, stop_operators.source
FROM stops
JOIN stop_operators ON stop_operators.stop_id = stops.id
WHERE stop_operators.operator_id = $1
        "#,
        operator_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), operator_id=operator_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn upsert_operator_stop(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    stop_id: i32,
    change: requests::ChangeOperatorStop,
) -> Result<()> {
    let existing = sqlx::query!(
        r#"
        SELECT official_name, stop_ref
        FROM stop_operators
        WHERE operator_id = $1 AND stop_id = $2
        "#,
        operator_id,
        stop_id
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            stop_id = stop_id,
            operator_id = operator_id
        );
        Error::DatabaseExecution
    })?;

    match existing.len() {
        0 => {
            sqlx::query!(
                r#"
                INSERT INTO stop_operators (operator_id, stop_id, official_name, stop_ref, source)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                operator_id,
                stop_id,
                change.official_name,
                change.stop_ref,
                change.source
            )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(
                    error=err.to_string(),
                    stop_id=stop_id,
                    operator_id=operator_id,
                    change=?change
                );
                Error::DatabaseExecution
            })?;
        }
        1 => {
            sqlx::query!(
                r#"
                UPDATE stop_operators
                SET official_name = $1,
                    stop_ref = $2,
                    source = $3
                WHERE operator_id = $4 AND stop_id = $5
                "#,
                change.official_name,
                change.stop_ref,
                change.source,
                operator_id,
                stop_id
            )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(error = err.to_string(), stop_id = stop_id);
                Error::DatabaseExecution
            })?;
        }
        _ => {
            // TODO This should never happen. Ensure that the constraints are
            // properly set up.
            tracing::error!(
                "Multiple stop_operators for the same operator_id and stop_id"
            );
            return Err(Error::DatabaseExecution);
        }
    }
    Ok(())
}

pub(crate) async fn delete_operator_stop(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    stop_id: i32,
) -> Result<()> {
    let existing = sqlx::query!(
        r#"
        SELECT official_name, stop_ref
        FROM stop_operators
        WHERE operator_id = $1 AND stop_id = $2
        "#,
        operator_id,
        stop_id
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), stop_id = stop_id);
        Error::DatabaseExecution
    })?;

    match existing.len() {
        0 => {
            return Err(Error::NotFoundUpstream);
        }
        1 => {
            sqlx::query!(
                r#"
                DELETE FROM stop_operators
                WHERE operator_id = $1 AND stop_id = $2
                "#,
                operator_id,
                stop_id
            )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(error = err.to_string(), stop_id = stop_id);
                Error::DatabaseExecution
            })?;
        }
        _ => {
            // TODO This should never happen. Ensure that the constraints are
            // properly set up.
            tracing::error!(
                "Multiple stop_operators for the same operator_id and stop_id"
            );
            return Err(Error::DatabaseExecution);
        }
    }
    Ok(())
}

pub(crate) async fn fetch_operator_route_types(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::OperatorRouteType>> {
    sqlx::query_as!(
        responses::OperatorRouteType,
        r#"
SELECT id, name, zapping_cost, board_cost, multi_trip, badge_text_color, badge_bg_color
FROM route_types
WHERE operator = $1
"#,
        operator_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error=err.to_string(), operator_id=operator_id);
            Error::DatabaseExecution
        })
}

pub(crate) async fn insert_operator_route_type(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    change: requests::ChangeOperatorRouteType,
) -> Result<i32> {
    let res = sqlx::query!(
        r#"
INSERT INTO route_types (operator, name, zapping_cost, board_cost, multi_trip, badge_text_color, badge_bg_color)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id
"#,
        operator_id,
        change.name,
        change.zapping_cost,
        change.board_cost,
        change.multi_trip,
        change.badge_text_color,
        change.badge_bg_color
    )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error=err.to_string(),
                operator_id=operator_id,
                change=?change
            );
            Error::DatabaseExecution
        })?;

    Ok(res.id)
}
pub(crate) async fn update_operator_route_type(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    type_id: i32,
    change: requests::ChangeOperatorRouteType,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE route_types
SET name = $1,
    zapping_cost = $2,
    board_cost = $3,
    multi_trip = $4,
    badge_text_color = $5,
    badge_bg_color = $6
WHERE operator = $7 AND id = $8
"#,
        change.name,
        change.zapping_cost,
        change.board_cost,
        change.multi_trip,
        change.badge_text_color,
        change.badge_bg_color,
        operator_id,
        type_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            change = ?change,
            operator_id=operator_id,
            type_id=type_id
        );
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_operator_route_type(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    type_id: i32,
) -> Result<()> {
    let uses = sqlx::query!(
        r#"
SELECT count(*) as cnt FROM routes
WHERE type = $1
"#,
        type_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), type_id = type_id);
        Error::DatabaseExecution
    })?
    .cnt
    .unwrap_or(0);

    if uses > 0 {
        return Err(Error::DependenciesNotMet);
    }

    sqlx::query!(
        r#"
DELETE FROM route_types
WHERE operator = $1 AND id = $2
"#,
        operator_id,
        type_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            operator_id = operator_id,
            type_id = type_id
        );
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_issues(
    pool: &PgPool,
) -> Result<Vec<operators::Issue>> {
    sqlx::query!(
        r#"
SELECT issues.id, issues.title, issues.message, issues.geojson, issues.category, issues.lat,
    issues.creation, issues.lon, issues.impact, issues.state, issues.state_justification,
    array_agg(issue_operators.operator_id) as "operators!: Vec<i32>",
    array_agg(issue_routes.route_id) as "routes!: Vec<i32>",
    array_agg(issue_stops.stop_id) as "stops!: Vec<i32>",
    array_agg(issue_pics.pic_id) as "pics!: Vec<i32>"
FROM issues
JOIN issue_operators on issue_operators.issue_id = issues.id
JOIN issue_routes on issue_routes.issue_id = issues.id
JOIN issue_stops on issue_stops.issue_id = issues.id
JOIN issue_pics on issue_pics.issue_id = issues.id
GROUP BY issues.id
"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string());
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(operators::Issue {
            id: row.id,
            title: row.title,
            message: row.message,
            geojson: row.geojson,
            category: serde_json::from_str(&row.category)
                .map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
            creation: row.creation.into(),
            state: serde_json::from_str(&row.state)
                .map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
            state_justification: row.state_justification,
            lat: row.lat,
            lon: row.lon,
            impact: row.impact,
            operator_ids: row.operators,
            route_ids: row.routes,
            stop_ids: row.stops,
            pic_ids: row.pics,
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
    array_agg(issue_stops.stop_id) as "stops!: Vec<i32>",
    array_agg(issue_pics.pic_id) as "pics!: Vec<i32>"
FROM issues
JOIN issue_operators on issue_operators.issue_id = issues.id
JOIN issue_routes on issue_routes.issue_id = issues.id
JOIN issue_stops on issue_stops.issue_id = issues.id
JOIN issue_pics on issue_pics.issue_id = issues.id
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
    .map_err(|err| {
        tracing::error!(
            error=err.to_string(),
            operator_id=operator_id
        );
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::Issue {
            id: row.id,
            title: row.title,
            message: row.message,
            geojson: row.geojson,
            category: serde_json::from_str(&row.category)
                .map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
            creation: row.creation.into(),
            lat: row.lat,
            lon: row.lon,
            impact: row.impact,
            state: serde_json::from_str(&row.state)
                .map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
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
) -> Result<operators::Issue> {
    sqlx::query!(
        r#"SELECT issues.id, issues.title, issues.message, issues.category, issues.impact,
        issues.creation, issues.lat, issues.lon, issues.geojson,
        issues.state, issues.state_justification,
    array_agg(issue_operators.operator_id) as "operators!: Vec<i32>",
    array_agg(issue_routes.route_id) as "routes!: Vec<i32>",
    array_agg(issue_stops.stop_id) as "stops!: Vec<i32>",
    array_agg(issue_pics.pic_id) as "pics!: Vec<i32>"
FROM issues
JOIN issue_operators on issue_operators.issue_id = issues.id
JOIN issue_routes on issue_routes.issue_id = issues.id
JOIN issue_stops on issue_stops.issue_id = issues.id
JOIN issue_pics on issue_pics.issue_id = issues.id
WHERE issues.id = $1
GROUP BY issues.id"#,
        issue_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), issue_id=issue_id);
        Error::DatabaseExecution
    })
    .and_then(|row| {
        Ok(operators::Issue {
            id: row.id,
            title: row.title,
            message: row.message,
            creation: row.creation.into(),
            category: serde_json::from_str(&row.category)
                .map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
            impact: row.impact,
            state: serde_json::from_str(&row.state)
                .map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
            state_justification: row.state_justification,
            lat: row.lat,
            lon: row.lon,
            geojson: row.geojson,
            operator_ids: row.operators,
            route_ids: row.routes,
            stop_ids: row.stops,
            pic_ids: row.pics,
        })
    })
}

pub(crate) async fn insert_issue(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    issue: &requests::NewIssue,
) -> Result<i32> {
    let creation = Local::now();

    let row = sqlx::query!(
        r#"
INSERT INTO issues (title, message, category, impact, creation, lat, lon, geojson, state)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING id
"#,
        issue.title,
        issue.message,
        Json(&issue.category) as _,
        issue.impact,
        creation,
        issue.lat,
        issue.lon,
        issue.geojson,
        Json(&operators::IssueState::Unanswered) as _
    )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error=err.to_string(),
                issue=?issue,
                creation=?creation
            );
            Error::DatabaseExecution
        })?;

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
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                operator_id = operator_id,
                id = id
            );
            Error::DatabaseExecution
        })?;
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
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                route_id = route_id,
                id = id
            );
            Error::DatabaseExecution
        })?;
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
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), stop_id = stop_id);
            Error::DatabaseExecution
        })?;
    }

    for pic_id in &issue.pic_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_pics (pic_id, issue_id)
            VALUES ($1, $2)
            "#,
            pic_id,
            id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), pic_id = pic_id, id = id);
            Error::DatabaseExecution
        })?;
    }

    Ok(id)
}

pub(crate) async fn update_issue(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    issue_id: i32,
    issue: requests::ChangeIssue,
) -> Result<()> {
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
        Json(&issue.category) as _,
        issue.lat,
        issue.lon,
        Json(&issue.state) as _,
        issue.state_justification,
        issue_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), issue_id=issue_id, issue = ?issue);
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        r#"
        DELETE FROM issue_operators
        WHERE issue_id = $1
        "#,
        issue_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), issue_id = issue_id);
        Error::DatabaseExecution
    })?;

    delete_issue_related(transaction, issue_id).await?;
    insert_issue_related(transaction, issue_id, issue).await?;

    Ok(())
}

async fn insert_issue_related(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    issue_id: i32,
    issue: requests::ChangeIssue,
) -> Result<()> {
    for operator_id in &issue.operator_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_operators (operator_id, issue_id)
            VALUES ($1, $2)
            "#,
            operator_id,
            issue_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                operator_id = operator_id,
                issue_id = issue_id
            );
            Error::DatabaseExecution
        })?;
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
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                route_id = route_id,
                issue_id = issue_id
            );
            Error::DatabaseExecution
        })?;
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
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                stop_id = stop_id,
                issue_id = issue_id
            );
            Error::DatabaseExecution
        })?;
    }

    for pic_id in &issue.pic_ids {
        sqlx::query!(
            r#"
            INSERT INTO issue_pics (pic_id, issue_id)
            VALUES ($1, $2)
            "#,
            pic_id,
            issue_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                pic_id = pic_id,
                issue_id = issue_id
            );
            Error::DatabaseExecution
        })?;
    }

    Ok(())
}

async fn delete_issue_related(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    issue_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM issue_routes
        WHERE issue_id = $1
        "#,
        issue_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), issue_id = issue_id);
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        r#"
        DELETE FROM issue_stops
        WHERE issue_id = $1
        "#,
        issue_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), issue_id = issue_id);
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        r#"
        DELETE FROM issue_pics
        WHERE issue_id = $1
        "#,
        issue_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), issue_id = issue_id);
        Error::DatabaseExecution
    })?;

    Ok(())
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
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::OperatorCalendar {
            id: row.id,
            name: row.name,
            calendar: serde_json::from_value(row.calendar).map_err(|e| {
                tracing::error!("Error deserializing {e}");
                Error::DatabaseDeserialization
            })?,
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
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id = operator_id);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| {
        Ok(responses::OperatorCalendar {
            id: row.id,
            name: row.name,
            calendar: serde_json::from_value(row.calendar).map_err(|e| {
                tracing::error!("Error deserializing {e}");
                Error::DatabaseDeserialization
            })?,
            operator_id,
        })
    })
    .collect()
}

pub(crate) async fn insert_calendar(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
        Json(calendar.calendar) as _
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id = operator_id);
        Error::DatabaseExecution
    })?;
    Ok(row.id)
}

pub(crate) async fn delete_calendar(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            operator_id = operator_id,
            calendar_id = calendar_id
        );
        Error::DatabaseExecution
    })?;
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
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id = operator_id);
        Error::DatabaseExecution
    })?
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
