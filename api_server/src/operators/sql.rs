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
use commons::models::content::RichContent;
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
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_operator_with_regions(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Option<responses::OperatorWithRegions>> {
    Ok(sqlx::query!(
        r#"
SELECT id, name, tag, description, logo_sha1, is_complete, website_url,
    forum_url, library_url, contact_uris,
    array_remove(array_agg(region_id), NULL) as "regions!: Vec<i32>"
FROM operators
LEFT JOIN region_operators ON region_operators.operator_id = operators.id
WHERE operators.id = $1
GROUP BY operators.id
"#,
        operator_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })?
    .map(|row| responses::OperatorWithRegions {
        id: row.id,
        name: row.name,
        tag: row.tag,
        description: row.description,
        logo_url: row
            .logo_sha1
            .map(|sha1| get_logo_path(row.id, sha1.as_ref())),
        is_complete: row.is_complete,
        website_url: row.website_url,
        forum_url: row.forum_url,
        library_url: row.library_url,
        contact_uris: row.contact_uris,
        regions: row.regions,
    }))
}

pub(crate) async fn fetch_operators(
    pool: &PgPool,
) -> Result<Vec<responses::OperatorWithRegions>> {
    Ok(sqlx::query!(
        r#"
SELECT id, name, tag, description, logo_sha1, is_complete, website_url,
    forum_url, library_url, contact_uris,
    array_remove(array_agg(region_id), NULL) as "regions!: Vec<i32>"
FROM operators
LEFT JOIN region_operators ON region_operators.operator_id = operators.id
GROUP BY operators.id
"#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| responses::OperatorWithRegions {
        id: row.id,
        name: row.name,
        tag: row.tag,
        description: row.description,
        logo_url: row
            .logo_sha1
            .map(|sha1| get_logo_path(row.id, sha1.as_ref())),
        is_complete: row.is_complete,
        website_url: row.website_url,
        forum_url: row.forum_url,
        library_url: row.library_url,
        contact_uris: row.contact_uris,
        regions: row.regions,
    })
    .collect())
}

pub(crate) async fn fetch_region_operators(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<operators::Operator>> {
    Ok(sqlx::query!(
        r#"
SELECT id, name, tag, description, logo_sha1, is_complete, website_url,
    forum_url, library_url, contact_uris
FROM operators
JOIN region_operators ON region_operators.operator_id = operators.id
WHERE region_id = $1
GROUP BY operators.id
"#,
        region_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|row| operators::Operator {
        id: row.id,
        name: row.name,
        tag: row.tag,
        description: row.description,
        logo_url: row
            .logo_sha1
            .map(|sha1| get_logo_path(row.id, sha1.as_ref())),
        is_complete: row.is_complete,
        website_url: row.website_url,
        forum_url: row.forum_url,
        library_url: row.library_url,
        contact_uris: row.contact_uris,
    })
    .collect())
}

pub(crate) async fn insert_operator(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    change: requests::ChangeOperator,
) -> Result<responses::Operator> {
    let id = sqlx::query!(
        r#"
INSERT INTO operators(name, tag, description, is_complete, website_url,
    forum_url, library_url, contact_uris)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING id
"#,
        &change.name,
        &change.tag,
        change.description,
        change.is_complete,
        change.website_url,
        change.forum_url,
        change.library_url,
        &change.contact_uris,
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), change = ?change);
        Error::DatabaseExecution
    })?
    .id;

    Ok(responses::Operator {
        id,
        name: change.name,
        tag: change.tag,
        description: change.description,
        logo_url: None,
        is_complete: change.is_complete,
        website_url: change.website_url,
        forum_url: change.forum_url,
        library_url: change.library_url,
        contact_uris: change.contact_uris,
    })
}

pub(crate) async fn update_operator(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    change: requests::ChangeOperator,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE operators
SET name = $1,
    tag = $2,
    description = $3,
    is_complete = $4,
    website_url = $5,
    forum_url = $6,
    library_url = $7,
    contact_uris = $8
WHERE id = $9
"#,
        &change.name,
        &change.tag,
        change.description,
        change.is_complete,
        change.website_url,
        change.forum_url,
        change.library_url,
        &change.contact_uris,
        operator_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            change = ?change,
            operator_id
        );
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_operator_stop_rels(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::OperatorStopRel>> {
    sqlx::query_as!(
        responses::OperatorStopRel,
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
    sqlx::query!(
                r#"
INSERT INTO stop_operators (operator_id, stop_id, official_name, stop_ref, source)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (operator_id, stop_id) DO UPDATE
    SET official_name = EXCLUDED.official_name,
        stop_ref = EXCLUDED.stop_ref,
        source = EXCLUDED.source
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
    change: &requests::ChangeOperatorRouteType,
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
            operator_id,
            type_id = type_id
        );
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_issue(
    pool: &PgPool,
    issue_id: i32,
) -> Result<Option<operators::Issue>> {
    sqlx::query!(
        r#"SELECT issues.id, issues.title, issues.category, issues.impact,
        issues.creation, issues.lat, issues.lon,
        issues.content as "content!: sqlx::types::Json<RichContent>",
        issues.state, issues.state_justification,
    array_remove(array_agg(distinct issue_regions.region_id), NULL) as "regions!: Vec<i32>",
    array_remove(array_agg(distinct issue_operators.operator_id), NULL) as "operators!: Vec<i32>",
    array_remove(array_agg(distinct issue_routes.route_id), NULL) as "routes!: Vec<i32>",
    array_remove(array_agg(distinct issue_stops.stop_id), NULL) as "stops!: Vec<i32>"
FROM issues
LEFT JOIN issue_regions on issue_regions.issue_id = issues.id
LEFT JOIN issue_operators on issue_operators.issue_id = issues.id
LEFT JOIN issue_routes on issue_routes.issue_id = issues.id
LEFT JOIN issue_stops on issue_stops.issue_id = issues.id
WHERE issues.id = $1
GROUP BY issues.id"#,
        issue_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), issue_id = issue_id);
        Error::DatabaseExecution
    }).and_then(|res|
        res.map(|row| Ok(operators::Issue {
            id: row.id,
            title: row.title,
            creation: row.creation.into(),
            category: serde_json::from_str(&row.category).map_err(|e| {
                tracing::error!("Error deserializing {e}");
                Error::DatabaseDeserialization
            })?,
            impact: row.impact,
            state: serde_json::from_str(&row.state).map_err(|e| {
                tracing::error!("Error deserializing {e}");
                Error::DatabaseDeserialization
            })?,
            state_justification: row.state_justification,
            lat: row.lat,
            lon: row.lon,
            content: row.content.0,
            region_ids: row.regions,
            operator_ids: row.operators,
            route_ids: row.routes,
            stop_ids: row.stops,
        }))
        .transpose()
    )
}

pub(crate) async fn insert_issue(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    issue: &requests::NewIssue,
) -> Result<i32> {
    let creation = Local::now();

    let row = sqlx::query!(
        r#"
INSERT INTO issues (title, category, impact, creation, lat, lon, content, state)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING id
"#,
        issue.title,
        Json(&issue.category) as _,
        issue.impact,
        creation,
        issue.lat,
        issue.lon,
        Json(&issue.content) as _,
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

    insert_issue_related(
        transaction,
        id,
        &issue.region_ids,
        &issue.operator_ids,
        &issue.route_ids,
        &issue.stop_ids,
        &issue.content,
    )
    .await?;

    Ok(id)
}

pub(crate) async fn update_issue(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    issue_id: i32,
    issue: &requests::ChangeIssue,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE issues
        SET title = $1,
            content = $2,
            category = $3,
            lat = $4,
            lon = $5,
            state = $6,
            state_justification = $7
        WHERE id = $8
        "#,
        issue.title,
        Json(&issue.content) as _,
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

    delete_issue_related(transaction, issue_id).await?;
    insert_issue_related(
        transaction,
        issue_id,
        &issue.region_ids,
        &issue.operator_ids,
        &issue.route_ids,
        &issue.stop_ids,
        &issue.content,
    )
    .await?;

    Ok(())
}

async fn insert_issue_related(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    issue_id: i32,
    region_ids: &[i32],
    operator_ids: &[i32],
    route_ids: &[i32],
    stop_ids: &[i32],
    content: &RichContent,
) -> Result<()> {
    for region_id in region_ids {
        sqlx::query!(
            "INSERT INTO issue_regions (region_id, issue_id) VALUES ($1, $2)",
            region_id,
            issue_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), region_id, issue_id);
            Error::DatabaseExecution
        })?;
    }

    for operator_id in operator_ids {
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
            tracing::error!(error = err.to_string(), operator_id, issue_id);
            Error::DatabaseExecution
        })?;
    }

    for route_id in route_ids {
        sqlx::query!(
            "INSERT INTO issue_routes (route_id, issue_id) VALUES ($1, $2)",
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

    for stop_id in stop_ids {
        sqlx::query!(
            "INSERT INTO issue_stops (stop_id, issue_id) VALUES ($1, $2)",
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

    for img_id in content.get_linked_images() {
        sqlx::query!(
            "INSERT INTO issue_imgs (img_id, issue_id) VALUES ($1, $2)",
            img_id,
            issue_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                img_id = ?img_id,
                issue_id
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
    sqlx::query!("DELETE FROM issue_operators WHERE issue_id = $1", issue_id)
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), issue_id = issue_id);
            Error::DatabaseExecution
        })?;

    sqlx::query!("DELETE FROM issue_routes WHERE issue_id = $1", issue_id)
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), issue_id = issue_id);
            Error::DatabaseExecution
        })?;

    sqlx::query!("DELETE FROM issue_stops WHERE issue_id = $1", issue_id)
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), issue_id = issue_id);
            Error::DatabaseExecution
        })?;

    sqlx::query!("DELETE FROM issue_imgs WHERE issue_id = $1", issue_id)
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), issue_id = issue_id);
            Error::DatabaseExecution
        })?;

    Ok(())
}

pub(crate) async fn fetch_operator_issues(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<operators::Issue>> {
    sqlx::query!(
        r#"
SELECT issues.id, issues.title,
    issues.content as "content!: sqlx::types::Json<RichContent>",
    issues.category, issues.lat, issues.creation, issues.lon, issues.impact,
    issues.state, issues.state_justification,
    array_remove(array_agg(distinct issue_regions.region_id), NULL) as "regions!: Vec<i32>",
    array_remove(array_agg(distinct issue_operators.operator_id), NULL) as "operators!: Vec<i32>",
    array_remove(array_agg(distinct issue_routes.route_id), NULL) as "routes!: Vec<i32>",
    array_remove(array_agg(distinct issue_stops.stop_id), NULL) as "stops!: Vec<i32>"
FROM issues
JOIN issue_operators on issue_operators.issue_id = issues.id
LEFT JOIN issue_regions on issue_regions.issue_id = issues.id
LEFT JOIN issue_routes on issue_routes.issue_id = issues.id
LEFT JOIN issue_stops on issue_stops.issue_id = issues.id
WHERE issue_operators.operator_id = $1
GROUP BY issues.id
"#,
        operator_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string());
            Error::DatabaseExecution
        })?
        .into_iter()
        .map(|row| {
            Ok(operators::Issue {
                id: row.id,
                title: row.title,
                content: row.content.0,
                category: serde_json::from_str(&row.category).map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
                creation: row.creation.into(),
                state: serde_json::from_str(&row.state).map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
                state_justification: row.state_justification,
                lat: row.lat,
                lon: row.lon,
                impact: row.impact,
                region_ids: row.regions,
                operator_ids: row.operators,
                route_ids: row.routes,
                stop_ids: row.stops,
            })
        })
        .collect()
}

pub(crate) async fn fetch_region_issues(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<operators::Issue>> {
    sqlx::query!(
        r#"
SELECT issues.id, issues.title,
    issues.content as "content!: sqlx::types::Json<RichContent>",
    issues.category, issues.lat, issues.creation, issues.lon, issues.impact,
    issues.state, issues.state_justification,
    array_remove(array_agg(distinct issue_regions.region_id), NULL) as "regions!: Vec<i32>",
    array_remove(array_agg(distinct issue_operators.operator_id), NULL) as "operators!: Vec<i32>",
    array_remove(array_agg(distinct issue_routes.route_id), NULL) as "routes!: Vec<i32>",
    array_remove(array_agg(distinct issue_stops.stop_id), NULL) as "stops!: Vec<i32>"
FROM issues
JOIN issue_regions on issue_regions.issue_id = issues.id
LEFT JOIN issue_operators on issue_operators.issue_id = issues.id
LEFT JOIN issue_routes on issue_routes.issue_id = issues.id
LEFT JOIN issue_stops on issue_stops.issue_id = issues.id
WHERE issue_operators.operator_id = $1
GROUP BY issues.id
"#,
        operator_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string());
            Error::DatabaseExecution
        })?
        .into_iter()
        .map(|row| {
            Ok(operators::Issue {
                id: row.id,
                title: row.title,
                content: row.content.0,
                category: serde_json::from_str(&row.category).map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
                creation: row.creation.into(),
                state: serde_json::from_str(&row.state).map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
                state_justification: row.state_justification,
                lat: row.lat,
                lon: row.lon,
                impact: row.impact,
                region_ids: row.regions,
                operator_ids: row.operators,
                route_ids: row.routes,
                stop_ids: row.stops,
            })
        })
        .collect()
}

pub(crate) async fn fetch_operator_issue_operators(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::SimpleOperator>> {
    sqlx::query_as!(
        responses::SimpleOperator,
        r#"
SELECT operators.id, operators.name, operators.tag
FROM operators
JOIN issue_operators on issue_operators.operator_id = operators.id
WHERE issue_operators.issue_id IN (
    SELECT issue_id
    FROM issue_operators
    WHERE operator_id = $1
)
"#,
        operator_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_region_issue_operators(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::SimpleOperator>> {
    sqlx::query_as!(
        responses::SimpleOperator,
        r#"
SELECT operators.id, operators.name, operators.tag
FROM operators
JOIN issue_operators on issue_operators.operator_id = operators.id
WHERE issue_operators.issue_id IN (
    SELECT issue_id
    FROM issue_regions
    WHERE region_id = $1
)
"#,
        region_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_issue_operators(
    pool: &PgPool,
    issue_id: i32,
) -> Result<Vec<responses::SimpleOperator>> {
    sqlx::query_as!(
        responses::SimpleOperator,
        r#"
SELECT operators.id, operators.name, operators.tag
FROM issue_operators
JOIN operators on issue_operators.operator_id = operators.id
WHERE issue_operators.issue_id = $1
"#,
        issue_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), issue_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_abnormality(
    pool: &PgPool,
    abnormality_id: i32,
) -> Result<Option<operators::Abnormality>> {
    sqlx::query!(
        r#"
SELECT abnormalities.id, abnormalities.summary, abnormalities.creation,
    abnormalities.from_datetime, abnormalities.to_datetime,
    abnormalities.content as "content!: sqlx::types::Json<RichContent>",
    abnormalities.mark_resolved,
    array_remove(array_agg(distinct abnormality_regions.region_id), NULL) as "regions!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_operators.operator_id), NULL) as "operators!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_routes.route_id), NULL) as "routes!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_stops.stop_id), NULL) as "stops!: Vec<i32>"
FROM abnormalities
LEFT JOIN abnormality_regions on abnormality_regions.abnormality_id = abnormalities.id
LEFT JOIN abnormality_operators on abnormality_operators.abnormality_id = abnormalities.id
LEFT JOIN abnormality_routes on abnormality_routes.abnormality_id = abnormalities.id
LEFT JOIN abnormality_stops on abnormality_stops.abnormality_id = abnormalities.id
WHERE abnormalities.id = $1
GROUP BY abnormalities.id"#,
        abnormality_id
    )
        .fetch_optional(pool)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), abnormality_id = abnormality_id);
            Error::DatabaseExecution
        }).and_then(|res|
        res.map(|row| Ok(operators::Abnormality {
            id: row.id,
            summary: row.summary,
            content: row.content.0,
            creation: row.creation.into(),
            from_datetime: row.from_datetime.map(Into::into),
            to_datetime: row.to_datetime.map(Into::into),
            mark_resolved: row.mark_resolved,
            region_ids: row.regions,
            operator_ids: row.operators,
            route_ids: row.routes,
            stop_ids: row.stops,
        }))
            .transpose()
    )
}

pub(crate) async fn insert_abnormality(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    abnormality: &requests::NewAbnormality,
) -> Result<i32> {
    let creation = Local::now();

    let row = sqlx::query!(
        r#"
INSERT INTO abnormalities (summary, creation, from_datetime, to_datetime, content, mark_resolved)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING id
"#,
        abnormality.summary,
        creation,
        abnormality.from_datetime,
        abnormality.to_datetime,
        Json(&abnormality.content) as _,
        abnormality.mark_resolved
    )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
            error=err.to_string(),
            issue=?abnormality,
            creation=?creation
        );
            Error::DatabaseExecution
        })?;

    let id = row.id;

    insert_abnormality_related(
        transaction,
        id,
        &abnormality.region_ids,
        &abnormality.operator_ids,
        &abnormality.route_ids,
        &abnormality.stop_ids,
        &abnormality.content,
    )
    .await?;

    Ok(id)
}

pub(crate) async fn update_abnormality(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    abnormality_id: i32,
    abnormality: &requests::ChangeAbnormality,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE abnormalities
        SET summary = $1,
            from_datetime = $2,
            to_datetime = $3,
            content = $4,
            mark_resolved = $5
        WHERE id = $6
        "#,
        abnormality.summary,
        abnormality.from_datetime,
        abnormality.to_datetime,
        Json(&abnormality.content) as _,
        abnormality.mark_resolved,
        abnormality_id
    )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), abnormality_id=abnormality_id, abnormality = ?abnormality);
            Error::DatabaseExecution
        })?;

    delete_abnormality_related(transaction, abnormality_id).await?;
    insert_abnormality_related(
        transaction,
        abnormality_id,
        &abnormality.region_ids,
        &abnormality.operator_ids,
        &abnormality.route_ids,
        &abnormality.stop_ids,
        &abnormality.content,
    )
    .await?;

    Ok(())
}

async fn insert_abnormality_related(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    abnormality_id: i32,
    region_ids: &[i32],
    operator_ids: &[i32],
    route_ids: &[i32],
    stop_ids: &[i32],
    content: &RichContent,
) -> Result<()> {
    for operator_id in operator_ids {
        sqlx::query!(
            r#"
            INSERT INTO abnormality_operators (operator_id, abnormality_id)
            VALUES ($1, $2)
            "#,
            operator_id,
            abnormality_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(
                error = err.to_string(),
                operator_id,
                abnormality_id
            );
            Error::DatabaseExecution
        })?;
    }

    for route_id in route_ids {
        sqlx::query!(
            "INSERT INTO abnormality_routes (route_id, abnormality_id) VALUES ($1, $2)",
            route_id,
            abnormality_id
        )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(
                error = err.to_string(),
                route_id = route_id,
                abnormality_id = abnormality_id
            );
                Error::DatabaseExecution
            })?;
    }

    for stop_id in stop_ids {
        sqlx::query!(
            "INSERT INTO abnormality_stops (stop_id, abnormality_id) VALUES ($1, $2)",
            stop_id,
            abnormality_id
        )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(
                error = err.to_string(),
                stop_id = stop_id,
                abnormality_id = abnormality_id
            );
                Error::DatabaseExecution
            })?;
    }

    for region_id in region_ids {
        sqlx::query!(
            "INSERT INTO abnormality_regions (region_id, abnormality_id) VALUES ($1, $2)",
            region_id,
            abnormality_id
        )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(
                    error = err.to_string(),
                    region_id = region_id,
                    abnormality_id = abnormality_id
                );
                Error::DatabaseExecution
            })?;
    }

    for img_id in content.get_linked_images() {
        sqlx::query!(
            "INSERT INTO abnormality_imgs (img_id, abnormality_id) VALUES ($1, $2)",
            img_id,
            abnormality_id
        )
            .execute(&mut **transaction)
            .await
            .map_err(|err| {
                tracing::error!(
                error = err.to_string(),
                img_id = ?img_id,
                abnormality_id = abnormality_id
            );
                Error::DatabaseExecution
            })?;
    }

    Ok(())
}

async fn delete_abnormality_related(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    abnormality_id: i32,
) -> Result<()> {
    sqlx::query!(
        "DELETE FROM abnormality_operators WHERE abnormality_id = $1",
        abnormality_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            abnormality_id = abnormality_id
        );
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        "DELETE FROM abnormality_routes WHERE abnormality_id = $1",
        abnormality_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            abnormality_id = abnormality_id
        );
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        "DELETE FROM abnormality_stops WHERE abnormality_id = $1",
        abnormality_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            abnormality_id = abnormality_id
        );
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        "DELETE FROM abnormality_regions WHERE abnormality_id = $1",
        abnormality_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            abnormality_id = abnormality_id
        );
        Error::DatabaseExecution
    })?;

    sqlx::query!(
        "DELETE FROM abnormality_imgs WHERE abnormality_id = $1",
        abnormality_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            abnormality_id = abnormality_id
        );
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn fetch_operator_abnormalities(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<operators::Abnormality>> {
    sqlx::query!(
        r#"
SELECT abnormalities.id, abnormalities.summary, abnormalities.creation,
    abnormalities.from_datetime, abnormalities.to_datetime,
    abnormalities.content as "content!: sqlx::types::Json<RichContent>",
    abnormalities.mark_resolved,
    array_remove(array_agg(distinct abnormality_regions.region_id), NULL) as "regions!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_operators.operator_id), NULL) as "operators!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_routes.route_id), NULL) as "routes!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_stops.stop_id), NULL) as "stops!: Vec<i32>"
FROM abnormalities
JOIN abnormality_operators on abnormality_operators.abnormality_id = abnormalities.id
LEFT JOIN abnormality_regions on abnormality_regions.abnormality_id = abnormalities.id
LEFT JOIN abnormality_routes on abnormality_routes.abnormality_id = abnormalities.id
LEFT JOIN abnormality_stops on abnormality_stops.abnormality_id = abnormalities.id
WHERE abnormality_operators.operator_id = $1
GROUP BY abnormalities.id
"#,
        operator_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string());
            Error::DatabaseExecution
        })?
        .into_iter()
        .map(|row| {
            Ok(operators::Abnormality {
                id: row.id,
                summary: row.summary,
                content: row.content.0,
                creation: row.creation.into(),
                from_datetime: row.from_datetime.map(Into::into),
                to_datetime: row.to_datetime.map(Into::into),
                mark_resolved: row.mark_resolved,
                region_ids: row.regions,
                operator_ids: row.operators,
                route_ids: row.routes,
                stop_ids: row.stops,
            })
        })
        .collect()
}

pub(crate) async fn fetch_region_abnormalities(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<operators::Abnormality>> {
    sqlx::query!(
        r#"
SELECT abnormalities.id, abnormalities.summary, abnormalities.creation,
    abnormalities.from_datetime, abnormalities.to_datetime,
    abnormalities.content as "content!: sqlx::types::Json<RichContent>",
    abnormalities.mark_resolved,
    array_remove(array_agg(distinct abnormality_regions.region_id), NULL) as "regions!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_operators.operator_id), NULL) as "operators!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_routes.route_id), NULL) as "routes!: Vec<i32>",
    array_remove(array_agg(distinct abnormality_stops.stop_id), NULL) as "stops!: Vec<i32>"
FROM abnormalities
JOIN abnormality_regions on abnormality_regions.abnormality_id = abnormalities.id
LEFT JOIN abnormality_operators on abnormality_operators.abnormality_id = abnormalities.id
LEFT JOIN abnormality_routes on abnormality_routes.abnormality_id = abnormalities.id
LEFT JOIN abnormality_stops on abnormality_stops.abnormality_id = abnormalities.id
WHERE abnormality_operators.operator_id = $1
GROUP BY abnormalities.id
"#,
        operator_id
    )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string());
            Error::DatabaseExecution
        })?
        .into_iter()
        .map(|row| {
            Ok(operators::Abnormality {
                id: row.id,
                summary: row.summary,
                content: row.content.0,
                creation: row.creation.into(),
                from_datetime: row.from_datetime.map(Into::into),
                to_datetime: row.to_datetime.map(Into::into),
                mark_resolved: row.mark_resolved,
                region_ids: row.regions,
                operator_ids: row.operators,
                route_ids: row.routes,
                stop_ids: row.stops,
            })
        })
        .collect()
}

pub(crate) async fn fetch_operator_abnormalities_operators(
    pool: &PgPool,
    operator_id: i32,
) -> Result<Vec<responses::SimpleOperator>> {
    sqlx::query_as!(
        responses::SimpleOperator,
        r#"
SELECT operators.id, operators.name, operators.tag
FROM operators
JOIN abnormality_operators on abnormality_operators.operator_id = operators.id
WHERE abnormality_operators.abnormality_id IN (
    SELECT abnormality_id
    FROM abnormality_operators
    WHERE operator_id = $1
)
"#,
        operator_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_region_abnormalities_operators(
    pool: &PgPool,
    region_id: i32,
) -> Result<Vec<responses::SimpleOperator>> {
    sqlx::query_as!(
        responses::SimpleOperator,
        r#"
SELECT operators.id, operators.name, operators.tag
FROM operators
JOIN abnormality_operators on abnormality_operators.operator_id = operators.id
WHERE abnormality_operators.abnormality_id IN (
    SELECT abnormality_id
    FROM abnormality_regions
    WHERE region_id = $1
)
"#,
        region_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), region_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_abnormality_operators(
    pool: &PgPool,
    abnormality_id: i32,
) -> Result<Vec<responses::SimpleOperator>> {
    sqlx::query_as!(
        responses::SimpleOperator,
        r#"
SELECT operators.id, operators.name, operators.tag
FROM abnormality_operators
JOIN operators on abnormality_operators.operator_id = operators.id
WHERE abnormality_operators.abnormality_id = $1
"#,
        abnormality_id
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), abnormality_id);
        Error::DatabaseExecution
    })
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
        tracing::error!(error = err.to_string(), operator_id);
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
        tracing::error!(error = err.to_string(), operator_id);
        Error::DatabaseExecution
    })?;
    Ok(row.id)
}

pub(crate) async fn update_calendar(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    operator_id: i32,
    calendar_id: i32,
    request: requests::NewOperatorCalendar,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE operator_calendars
SET name = $1,
    calendar = $2
WHERE operator_id=$3 AND id=$4
"#,
        &request.name,
        Json(&request.calendar) as _,
        operator_id,
        calendar_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            name = request.name,
            calendar = ?request.calendar
        );
        Error::DatabaseExecution
    })?;

    Ok(())
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
        tracing::error!(error = err.to_string(), operator_id, calendar_id);
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
        tracing::error!(error = err.to_string(), operator_id);
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
