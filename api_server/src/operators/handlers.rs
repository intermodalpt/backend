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
use axum::extract::{Path, State};
use axum::Json;
use chrono::NaiveDate;
use futures::future;
use std::collections::HashMap;

use commons::models::{history, operators};

use super::models::{requests, responses};
use super::sql;
use crate::pics::sql as pics_sql;
use crate::responses::IdReturn;
use crate::{auth, contrib, geo, routes, stops, AppState, Error};

pub(crate) async fn get_operators(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::OperatorWithRegions>>, Error> {
    Ok(Json(sql::fetch_operators(&state.pool).await?))
}

pub(crate) async fn get_operator(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<responses::OperatorWithRegions>, Error> {
    Ok(Json(
        sql::fetch_operator_with_regions(&state.pool, operator_id)
            .await?
            .ok_or(Error::NotFoundUpstream)?,
    ))
}

pub(crate) async fn post_operator(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::CreateOperator>,
    Json(mut change): Json<requests::ChangeOperator>,
) -> Result<Json<responses::Operator>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    change.tidy();
    let operator = sql::insert_operator(&mut transaction, change).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(operator))
}

pub(crate) async fn patch_operator(
    State(state): State<AppState>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyOperatorMeta>,
    Path(operator_id): Path<i32>,
    Json(mut change): Json<requests::ChangeOperator>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    change.tidy();
    sql::update_operator(&mut transaction, operator_id, change).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_operator_stop_rels(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::OperatorStopRel>>, Error> {
    Ok(Json(
        sql::fetch_operator_stop_rels(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn put_operator_stop(
    State(state): State<AppState>,
    Path((operator_id, stop_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ModifyOperatorStops,
    >,
    Json(change): Json<requests::ChangeOperatorStop>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    // TODO log
    sql::upsert_operator_stop(&mut transaction, operator_id, stop_id, change)
        .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;
    Ok(())
}

pub(crate) async fn delete_operator_stop(
    State(state): State<AppState>,
    Path((operator_id, stop_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ModifyOperatorStops,
    >,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    // TODO log
    sql::delete_operator_stop(&mut transaction, operator_id, stop_id).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_operator_route_types(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::OperatorRouteType>>, Error> {
    Ok(Json(
        sql::fetch_operator_route_types(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn post_operator_route_type(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyOperatorMeta>,
    Json(route_type): Json<requests::ChangeOperatorRouteType>,
) -> Result<Json<responses::OperatorRouteType>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let id = sql::insert_operator_route_type(
        &mut transaction,
        operator_id,
        &route_type,
    )
    .await?;

    let route_type = responses::OperatorRouteType {
        id,
        name: route_type.name,
        zapping_cost: route_type.zapping_cost,
        board_cost: route_type.board_cost,
        multi_trip: route_type.multi_trip,
        badge_text_color: route_type.badge_text_color,
        badge_bg_color: route_type.badge_bg_color,
    };

    // TODO log

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(route_type))
}

pub(crate) async fn patch_operator_route_type(
    State(state): State<AppState>,
    Path((operator_id, type_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyOperatorMeta>,
    Json(route_type): Json<requests::ChangeOperatorRouteType>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_operator_route_type(
        &mut transaction,
        operator_id,
        type_id,
        route_type,
    )
    .await?;

    // TODO log

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_operator_route_type(
    State(state): State<AppState>,
    Path((operator_id, type_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<auth::perms::ModifyOperatorMeta>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::delete_operator_route_type(&mut transaction, operator_id, type_id)
        .await?;

    // TODO log

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_issue(
    State(state): State<AppState>,
    Path(issue_id): Path<i32>,
) -> Result<Json<responses::FullIssue>, Error> {
    let (issue, regions, operators, stops, routes) = future::join5(
        sql::fetch_issue(&state.pool, issue_id),
        geo::sql::fetch_issue_regions(&state.pool, issue_id),
        sql::fetch_issue_operators(&state.pool, issue_id),
        stops::sql::fetch_issue_stops(&state.pool, issue_id),
        routes::sql::fetch_issue_routes(&state.pool, issue_id),
    )
    .await;

    let issue = issue?.ok_or(Error::NotFoundUpstream)?;

    Ok(Json(responses::FullIssue {
        id: issue.id,
        title: issue.title,
        category: issue.category,
        impact: issue.impact,
        creation: issue.creation,
        lat: issue.lat,
        lon: issue.lon,
        content: issue.content,
        state: issue.state,
        state_justification: issue.state_justification,
        regions: regions?,
        operators: operators?,
        routes: routes?,
        stops: stops?,
    }))
}

pub(crate) async fn get_operator_issues(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::FullIssue>>, Error> {
    let (issues, regions, operators, routes, stops) = future::join5(
        sql::fetch_operator_issues(&state.pool, operator_id),
        geo::sql::fetch_simple_regions(&state.pool),
        sql::fetch_operator_issue_operators(&state.pool, operator_id),
        routes::sql::fetch_operator_issue_routes(&state.pool, operator_id),
        stops::sql::fetch_operator_issue_stops(&state.pool, operator_id),
    )
    .await;

    let issues = fuse_issues(issues?, regions?, operators?, routes?, stops?);

    Ok(Json(issues))
}

pub(crate) async fn get_region_issues(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::FullIssue>>, Error> {
    let (issues, regions, operators, routes, stops) = future::join5(
        sql::fetch_region_issues(&state.pool, region_id),
        geo::sql::fetch_region_issue_regions(&state.pool, region_id),
        sql::fetch_region_issue_operators(&state.pool, region_id),
        routes::sql::fetch_region_issue_routes(&state.pool, region_id),
        stops::sql::fetch_region_issue_stops(&state.pool, region_id),
    )
    .await;

    let issues = fuse_issues(issues?, regions?, operators?, routes?, stops?);

    Ok(Json(issues))
}

fn fuse_issues(
    issues: Vec<operators::Issue>,
    regions: Vec<geo::models::responses::SimpleRegion>,
    issue_operators: Vec<responses::SimpleOperator>,
    issue_routes: Vec<routes::models::responses::SimpleRoute>,
    issue_stops: Vec<stops::models::responses::SimpleStop>,
) -> Vec<responses::FullIssue> {
    let region_index = regions
        .into_iter()
        .map(|region| (region.id, region))
        .collect::<HashMap<_, _>>();

    let operator_index = issue_operators
        .into_iter()
        .map(|operator| (operator.id, operator))
        .collect::<HashMap<_, _>>();

    let route_index = issue_routes
        .into_iter()
        .map(|route| (route.id, route))
        .collect::<HashMap<_, _>>();

    let stop_index = issue_stops
        .into_iter()
        .map(|stop| (stop.id, stop))
        .collect::<HashMap<_, _>>();

    issues
        .into_iter()
        .map(|issue| {
            let issue_regions = issue
                .region_ids
                .iter()
                .filter_map(|id| region_index.get(id))
                .cloned()
                .collect();
            let issue_operators = issue
                .operator_ids
                .iter()
                .filter_map(|id| operator_index.get(id))
                .cloned()
                .collect();
            let issue_routes = issue
                .route_ids
                .iter()
                .filter_map(|id| route_index.get(id))
                .cloned()
                .collect();
            let issue_stops = issue
                .stop_ids
                .iter()
                .filter_map(|id| stop_index.get(id))
                .cloned()
                .collect();

            responses::FullIssue {
                id: issue.id,
                title: issue.title,
                category: issue.category,
                impact: issue.impact,
                creation: issue.creation,
                content: issue.content,
                lat: issue.lat,
                lon: issue.lon,
                state: issue.state,
                state_justification: issue.state_justification,
                regions: issue_regions,
                operators: issue_operators,
                routes: issue_routes,
                stops: issue_stops,
            }
        })
        .collect::<Vec<_>>()
}

pub(crate) async fn post_issue(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::ModifyIssues>,
    Json(issue): Json<requests::NewIssue>,
) -> Result<Json<IdReturn<i32>>, Error> {
    issue.validate()?;

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let id = sql::insert_issue(&mut transaction, &issue).await?;

    let issue = operators::Issue {
        id,
        ..operators::Issue::from(issue)
    };

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::IssueCreation { data: issue.into() }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(IdReturn { id }))
}

pub(crate) async fn patch_issue(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::ModifyIssues>,
    Path(issue_id): Path<i32>,
    Json(change): Json<requests::ChangeIssue>,
) -> Result<(), Error> {
    change.validate()?;

    let issue = sql::fetch_issue(&state.pool, issue_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let patch = change.derive_patch(&issue);
    if patch.is_empty() {
        return Ok(());
    }

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_issue(&mut transaction, issue_id, &change).await?;
    // This code is very suboptimal but it'll do for now
    // TODO: optimize
    pics_sql::unlink_rich_images_from_issue(&mut transaction, issue_id).await?;
    if patch.content.is_some() {
        for img_id in change.content.get_linked_images() {
            pics_sql::link_rich_image_to_issue(
                &mut transaction,
                img_id,
                issue_id,
            )
            .await?;
        }
    }

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::IssueUpdate {
            original: issue.into(),
            patch,
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_abnormality(
    State(state): State<AppState>,
    Path(abnormality_id): Path<i32>,
) -> Result<Json<responses::FullAbnormality>, Error> {
    let (abnormality, regions, operators, stops, routes) = future::join5(
        sql::fetch_abnormality(&state.pool, abnormality_id),
        geo::sql::fetch_abnormality_regions(&state.pool, abnormality_id),
        sql::fetch_abnormality_operators(&state.pool, abnormality_id),
        stops::sql::fetch_abnormality_stops(&state.pool, abnormality_id),
        routes::sql::fetch_abnormality_routes(&state.pool, abnormality_id),
    )
    .await;

    let abnormality = abnormality?.ok_or(Error::NotFoundUpstream)?;

    Ok(Json(responses::FullAbnormality {
        id: abnormality.id,
        summary: abnormality.summary,
        creation: abnormality.creation,
        content: abnormality.content,
        from_datetime: abnormality.from_datetime,
        to_datetime: abnormality.to_datetime,
        mark_resolved: abnormality.mark_resolved,
        regions: regions?,
        operators: operators?,
        routes: routes?,
        stops: stops?,
    }))
}

pub(crate) async fn get_operator_abnormalities(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::FullAbnormality>>, Error> {
    let (abnormalities, regions, operators, routes, stops) = future::join5(
        sql::fetch_operator_abnormalities(&state.pool, operator_id),
        geo::sql::fetch_simple_regions(&state.pool),
        sql::fetch_operator_abnormalities_operators(&state.pool, operator_id),
        routes::sql::fetch_operator_abnormalities_routes(
            &state.pool,
            operator_id,
        ),
        stops::sql::fetch_operator_abnormalities_stops(
            &state.pool,
            operator_id,
        ),
    )
    .await;

    let abnormalities = fuse_abnormalities(
        abnormalities?,
        regions?,
        operators?,
        routes?,
        stops?,
    );

    Ok(Json(abnormalities))
}

pub(crate) async fn get_region_abnormalities(
    State(state): State<AppState>,
    Path(region_id): Path<i32>,
) -> Result<Json<Vec<responses::FullAbnormality>>, Error> {
    let (abnormalities, regions, operators, routes, stops) = future::join5(
        sql::fetch_region_abnormalities(&state.pool, region_id),
        geo::sql::fetch_region_abnormalities_regions(&state.pool, region_id),
        sql::fetch_region_abnormalities_operators(&state.pool, region_id),
        routes::sql::fetch_region_abnormalities_routes(&state.pool, region_id),
        stops::sql::fetch_region_abnormalities_stops(&state.pool, region_id),
    )
    .await;

    let abnormalities = fuse_abnormalities(
        abnormalities?,
        regions?,
        operators?,
        routes?,
        stops?,
    );

    Ok(Json(abnormalities))
}

fn fuse_abnormalities(
    abnormalities: Vec<operators::Abnormality>,
    regions: Vec<geo::models::responses::SimpleRegion>,
    operators: Vec<responses::SimpleOperator>,
    routes: Vec<routes::models::responses::SimpleRoute>,
    stops: Vec<stops::models::responses::SimpleStop>,
) -> Vec<responses::FullAbnormality> {
    let region_index = regions
        .into_iter()
        .map(|region| (region.id, region))
        .collect::<HashMap<_, _>>();

    let operator_index = operators
        .into_iter()
        .map(|operator| (operator.id, operator))
        .collect::<HashMap<_, _>>();

    let route_index = routes
        .into_iter()
        .map(|route| (route.id, route))
        .collect::<HashMap<_, _>>();

    let stop_index = stops
        .into_iter()
        .map(|stop| (stop.id, stop))
        .collect::<HashMap<_, _>>();

    abnormalities
        .into_iter()
        .map(|abnormality| {
            let abn_regions = abnormality
                .region_ids
                .iter()
                .filter_map(|id| region_index.get(id))
                .cloned()
                .collect();
            let abn_operators = abnormality
                .operator_ids
                .iter()
                .filter_map(|id| operator_index.get(id))
                .cloned()
                .collect();
            let abn_routes = abnormality
                .route_ids
                .iter()
                .filter_map(|id| route_index.get(id))
                .cloned()
                .collect();
            let abn_stops = abnormality
                .stop_ids
                .iter()
                .filter_map(|id| stop_index.get(id))
                .cloned()
                .collect();

            responses::FullAbnormality {
                id: abnormality.id,
                summary: abnormality.summary,
                creation: abnormality.creation,
                content: abnormality.content,
                from_datetime: abnormality.from_datetime,
                to_datetime: abnormality.to_datetime,
                regions: abn_regions,
                operators: abn_operators,
                routes: abn_routes,
                stops: abn_stops,
                mark_resolved: abnormality.mark_resolved,
            }
        })
        .collect::<Vec<_>>()
}

pub(crate) async fn post_abnormality(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::ModifyIssues>,
    Json(abnormality): Json<requests::NewAbnormality>,
) -> Result<Json<IdReturn<i32>>, Error> {
    abnormality.validate()?;

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let id = sql::insert_abnormality(&mut transaction, &abnormality).await?;

    let abnormality = operators::Abnormality {
        id,
        ..operators::Abnormality::from(abnormality)
    };

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::AbnormalityCreation {
            data: abnormality.into(),
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(IdReturn { id }))
}

pub(crate) async fn patch_abnormality(
    State(state): State<AppState>,
    auth::ScopedClaim(claims, _): auth::ScopedClaim<auth::perms::ModifyIssues>,
    Path(abnormality_id): Path<i32>,
    Json(change): Json<requests::ChangeAbnormality>,
) -> Result<(), Error> {
    change.validate()?;

    let abnormality = sql::fetch_abnormality(&state.pool, abnormality_id)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let patch = change.derive_patch(&abnormality);
    if patch.is_empty() {
        return Ok(());
    }

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::update_abnormality(&mut transaction, abnormality_id, &change).await?;
    // This code is very suboptimal but it'll do for now
    // TODO: optimize
    pics_sql::unlink_rich_images_from_abnormality(
        &mut transaction,
        abnormality_id,
    )
    .await?;
    if patch.content.is_some() {
        for img_id in change.content.get_linked_images() {
            pics_sql::link_rich_image_to_issue(
                &mut transaction,
                img_id,
                abnormality_id,
            )
            .await?;
        }
    }

    contrib::sql::insert_changeset_log(
        &mut transaction,
        claims.uid,
        &[history::Change::AbnormalityUpdate {
            original: abnormality.into(),
            patch,
        }],
        None,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_calendars(
    State(state): State<AppState>,
) -> Result<Json<Vec<responses::OperatorCalendar>>, Error> {
    Ok(Json(sql::fetch_calendars(&state.pool).await?))
}

pub(crate) async fn get_operator_calendars(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
) -> Result<Json<Vec<responses::OperatorCalendar>>, Error> {
    Ok(Json(
        sql::fetch_operator_calendars(&state.pool, operator_id).await?,
    ))
}

pub(crate) async fn post_operator_calendar(
    State(state): State<AppState>,
    Path(operator_id): Path<i32>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ModifyOperatorCalendars,
    >,
    Json(calendar): Json<requests::NewOperatorCalendar>,
) -> Result<Json<IdReturn<i32>>, Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    // TODO log
    let id =
        sql::insert_calendar(&mut transaction, operator_id, calendar).await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(Json(IdReturn { id }))
}

pub(crate) async fn patch_operator_calendar(
    State(state): State<AppState>,
    Path((operator_id, calendar_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ModifyOperatorCalendars,
    >,
    Json(request): Json<requests::NewOperatorCalendar>,
) -> Result<(), Error> {
    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    // TODO check if noop

    sql::update_calendar(&mut transaction, operator_id, calendar_id, request)
        .await?;

    // TODO log

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn delete_operator_calendar(
    State(state): State<AppState>,
    Path((operator_id, calendar_id)): Path<(i32, i32)>,
    auth::ScopedClaim(_, _): auth::ScopedClaim<
        auth::perms::ModifyOperatorCalendars,
    >,
) -> Result<(), Error> {
    // TODO forbid the deletion of calendars that are in use

    let mut transaction = state.pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::delete_calendar(&mut transaction, operator_id, calendar_id).await?;

    // TODO log

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(())
}

pub(crate) async fn get_operator_calendars_for_date(
    State(state): State<AppState>,
    Path((operator_id, date)): Path<(i32, String)>,
) -> Result<Json<Vec<responses::OperatorCalendar>>, Error> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|err| Error::ValidationFailure(err.to_string()))?;

    Ok(Json(
        sql::fetch_calendars_for_date(&state.pool, operator_id, date).await?,
    ))
}
