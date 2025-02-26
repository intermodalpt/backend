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

use chrono::Utc;
use serde_json::json;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::types::Json;
use sqlx::PgPool;
use uuid::Uuid;

use commons::models::auth;

use super::models::{requests, responses};
use crate::auth::models;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_user_by_id<'c, E>(
    executor: E,
    uid: i32,
) -> Result<Option<auth::User>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        auth::User,
        r#"
SELECT id, username, password, email, is_superuser, works_for
FROM Users
WHERE id=$1
    "#,
        uid
    )
    .fetch_optional(executor)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), uid);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<auth::User>> {
    sqlx::query_as!(
        auth::User,
        r#"
SELECT id, username, password, email, is_superuser, works_for
FROM Users
WHERE username=$1
    "#,
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), username);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_user_by_username_or_email(
    pool: &PgPool,
    username: &str,
    email: &str,
) -> Result<Option<auth::User>> {
    sqlx::query_as!(
        auth::User,
        r#"
SELECT id, username, password, email, is_superuser, works_for
FROM Users
WHERE username=$1 or email = $2
    "#,
        username,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), username, email);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_username_exists(
    pool: &PgPool,
    username: &str,
) -> Result<bool> {
    sqlx::query!("SELECT 1 as _u FROM users WHERE username=$1", username)
        .fetch_optional(pool)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), username);
            Error::DatabaseExecution
        })
        .map(|r| r.is_some())
}

pub(crate) async fn fetch_user_management_tokens(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i32,
    show_revoked: bool,
) -> Result<Vec<responses::ManagementToken>> {
    sqlx::query_as!(
        responses::ManagementToken,
        r#"
SELECT user_sessions.id, management_tokens.name, management_tokens.token,
    user_sessions.revoked,
    management_tokens.permissions
        as "permissions!: sqlx::types::Json<auth::Permissions>"
FROM management_tokens
JOIN user_sessions ON user_sessions.id = management_tokens.session_id
WHERE user_sessions.user_id=$1 AND (NOT revoked OR $2)
    "#,
        user_id,
        show_revoked
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn insert_management_token(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    token_id: Uuid,
    token_name: &str,
    models::JwtManagement(token): &models::JwtManagement,
    permissions: &auth::Permissions,
) -> Result<()> {
    sqlx::query_as!(
        responses::ManagementToken,
        r#"
INSERT INTO management_tokens(session_id, name, token, permissions)
VALUES ($1, $2, $3, $4)
    "#,
        token_id,
        token_name,
        token,
        json!(permissions)
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), token_id=?token_id, token_name, token);
        Error::DatabaseExecution
    })?;
    Ok(())
}

pub(crate) async fn update_set_session_revoked(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    token_id: Uuid,
) -> Result<()> {
    sqlx::query!(
        "UPDATE user_sessions SET revoked=true WHERE id=$1",
        token_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), token_id=?token_id);
        Error::DatabaseExecution
    })?;
    Ok(())
}

pub(crate) async fn fetch_user_permissions(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i32,
) -> Result<Option<auth::Permissions>> {
    sqlx::query!(
        r#"
SELECT permissions as "permissions!: sqlx::types::Json<auth::Permissions>"
FROM users
WHERE id=$1"#,
        user_id
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id);
        Error::DatabaseExecution
    })
    .map(|res| res.map(|res| res.permissions.0))
}

pub(crate) async fn fetch_permission_assignment(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    assignment_id: i32,
) -> Result<Option<responses::UserPermAssignment>> {
    sqlx::query_as!(
        models::UserPermAssignment,
        r#"
SELECT id, user_id, issuer_id, priority,
    permissions as "permissions!: sqlx::types::Json<auth::Permissions>"
FROM user_permissions
WHERE id=$1"#,
        assignment_id
    )
    .fetch_optional(&mut **transaction)
    .await
    .map(|res| res.map(Into::into))
    .map_err(|err| {
        tracing::error!(error = err.to_string(), assignment_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_user_permission_assignments<'c, E>(
    executor: E,
    user_id: i32,
) -> Result<Vec<responses::UserPermAssignment>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        models::UserPermAssignment,
        r#"
SELECT id, user_id, issuer_id, priority,
    permissions as "permissions!: sqlx::types::Json<auth::Permissions>"
FROM user_permissions
WHERE user_id=$1"#,
        user_id
    )
    .fetch_all(executor)
    .await
    .map(|res| res.into_iter().map(Into::into).collect())
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn insert_user_permission_assignment(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    permissions: &auth::Permissions,
    user_id: i32,
    issuer_id: Option<i32>,
    priority: i32,
) -> Result<i32> {
    let res = sqlx::query!(
        r#"
INSERT INTO user_permissions(user_id, issuer_id, priority, permissions)
VALUES ($1, $2, $3, $4)
RETURNING id
        "#,
        user_id,
        issuer_id,
        priority,
        json!(permissions)
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            user_id,
            issuer_id,
            priority,
            permissions = ?permissions
        );
        Error::DatabaseExecution
    })?;

    Ok(res.id)
}

pub(crate) async fn delete_permission_assignment(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    assignment_id: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
DELETE FROM user_permissions
WHERE id=$1
    "#,
        assignment_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), assignment_id);
        Error::DatabaseExecution
    })?;

    Ok(())
}

// These do not necessarily need to be the same as above
// A token should be able to have a more limited scope than its user
pub(crate) async fn fetch_management_token_permissions(
    pool: &PgPool,
    token_id: Uuid,
) -> Result<Option<responses::ManagementToken>> {
    let res = sqlx::query!(
        r#"
SELECT user_sessions.id, user_sessions.revoked, user_sessions.expiration,
    management_tokens.name, management_tokens.token,
    management_tokens.permissions
        as "permissions!: sqlx::types::Json<auth::Permissions>"
FROM management_tokens
RIGHT JOIN user_sessions ON user_sessions.id = management_tokens.session_id
WHERE user_sessions.id=$1"#,
        token_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), token_id=?token_id);
        Error::DatabaseExecution
    })?;

    Ok(res.map(|res| responses::ManagementToken {
        id: res.id,
        name: res.name,
        permissions: res.permissions,
        revoked: res.revoked,
        token: models::JwtManagement(res.token),
    }))
}

pub(crate) async fn register_user(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    request: &models::HashedRegistration,
    permissions: &auth::Permissions,
    consent: models::ConsentAnswer,
    survey: serde_json::Value,
) -> Result<i32> {
    let now = Utc::now();
    let res = sqlx::query!(
        r#"
INSERT INTO Users (username, password, email, permissions, consent, consent_date, survey)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id"#,
        request.username,
        request.password,
        request.email,
        json!(permissions),
        json!(consent),
        now,
        survey
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            username = request.username,
            email = request.email,
            permissions = ?permissions,
            consent = ?consent,
            survey = ?survey,
        );
        Error::DatabaseExecution
    })?;
    Ok(res.id)
}

pub(crate) async fn update_user_cached_permissions(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i32,
    permissions: &auth::Permissions,
) -> Result<()> {
    sqlx::query!(
        "UPDATE users SET permissions=$1 WHERE id=$2",
        json!(permissions),
        user_id
        ).execute(&mut **transaction).await.map_err(|err| {
        tracing::error!(error = err.to_string(), user_id, permissions = ?permissions);
        Error::DatabaseExecution
    })?;
    Ok(())
}

pub(crate) async fn fetch_user_session<'c, E>(
    executor: E,
    token_id: Uuid,
) -> Result<Option<responses::UserSession>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        responses::UserSession,
        r#"
SELECT id, user_id, ip, user_agent, expiration, revoked
FROM user_sessions
WHERE id=$1
    "#,
        token_id
    )
    .fetch_optional(executor)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), token_id = ?token_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn fetch_user_sessions<'c, E>(
    executor: E,
    uid: i32,
) -> Result<Vec<responses::UserSession>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        responses::UserSession,
        r#"
SELECT id, user_id, ip, user_agent, expiration, revoked
FROM user_sessions
WHERE user_id=$1
    "#,
        uid
    )
    .fetch_all(executor)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), uid);
        Error::DatabaseExecution
    })
}

pub(crate) async fn insert_user_session(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    session_meta: models::NewUserSessionMeta<'_>,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO user_sessions(id, user_id, ip, user_agent, expiration)
VALUES ($1, $2, $3, $4, $5)
    "#,
        session_meta.id,
        session_meta.user_id,
        session_meta.ip,
        session_meta.user_agent,
        session_meta.expiration
    )
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(|err| {
        tracing::error!(error = err.to_string(), session_meta = ?session_meta);
        Error::DatabaseExecution
    })?;
    Ok(())
}

pub(crate) async fn fetch_session_accesses<'c, E>(
    executor: E,
    session_id: Uuid,
) -> Result<Vec<responses::UserAccessSession>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        responses::UserAccessSession,
        r#"
SELECT id, session_id, ip, user_agent, creation, last_active, expiration
FROM user_session_access
WHERE session_id=$1
    "#,
        session_id
    )
    .fetch_all(executor)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), session_id = ?session_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn insert_user_session_renewal(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    session_meta: models::NewUserSessionAccessMeta<'_>,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO user_session_access(id, session_id, ip, user_agent, expiration)
VALUES ($1, $2, $3, $4, $5)
    "#,
        session_meta.access,
        session_meta.session,
        session_meta.ip,
        session_meta.user_agent,
        session_meta.expiration
    )
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(|err| {
        tracing::error!(error = err.to_string(), session_meta = ?session_meta);
        Error::DatabaseExecution
    })
}

pub(crate) async fn change_user_password(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    uid: i32,
    password: &str,
) -> Result<()> {
    sqlx::query!(r#"UPDATE users SET password=$1 WHERE id=$2"#, password, uid)
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!(error = err.to_string(), uid);
            Error::DatabaseExecution
        })?;
    Ok(())
}

pub(crate) async fn fetch_audit_log_entries<'c, E>(
    executor: E,
    skip: i64,
    take: i64,
) -> Result<Vec<responses::AuditLogEntry>>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    sqlx::query!(
        r#"
SELECT audit_log.id, audit_log.user_id, audit_log.action, audit_log.datetime, audit_log.addr,
    users.username as user_username
FROM audit_log
INNER JOIN users ON user_id = users.id
ORDER BY datetime DESC
LIMIT $1 OFFSET $2
    "#,
        take,
        skip
    )
    .fetch_all(executor)
    .await
    .map_err(|err| {
        tracing::error!(error=err.to_string(), take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| {
        Ok(responses::AuditLogEntry {
            entry: auth::AuditLogEntry {
                id: r.id,
                user_id: r.user_id,
                action: serde_json::from_value(r.action).map_err(|e| {
                    tracing::error!("Error deserializing {e}");
                    Error::DatabaseDeserialization
                })?,
                datetime: r.datetime.with_timezone(&Utc),
                addr: r.addr,
            },
            user_username: r.user_username,
        })
    })
    .collect()
}

pub(crate) async fn insert_audit_log_entry(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    action: auth::AuditLogAction,
    user_id: i32,
    session_id: Option<Uuid>,
    addr: &IpNetwork,
) -> Result<i64> {
    let action_type = action.action_type_name();
    let res = sqlx::query!(
        r#"
INSERT INTO audit_log(user_id, action_type, action, addr, session_id)
VALUES ($1, $2, $3, $4, $5)
RETURNING id
    "#,
        user_id,
        action_type,
        Json(&action) as _,
        addr,
        session_id
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            user_id,
            action = ?action,
            addr = ?addr,
            session_id = ?session_id
        );
        Error::DatabaseExecution
    })?;

    Ok(res.id)
}

pub(crate) async fn fetch_user_audit_log(
    pool: &PgPool,
    user_id: i32,
    skip: i64,
    take: i64,
) -> Result<Vec<auth::AuditLogEntry>> {
    sqlx::query!(
        r#"
SELECT id, action, datetime, addr
FROM audit_log
WHERE user_id=$1
ORDER BY datetime DESC
LIMIT $2 OFFSET $3
    "#,
        user_id,
        take,
        skip
    )
    .fetch_all(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id, take, skip);
        Error::DatabaseExecution
    })?
    .into_iter()
    .map(|r| {
        Ok(auth::AuditLogEntry {
            id: r.id,
            user_id,
            action: serde_json::from_value(r.action).map_err(|e| {
                tracing::error!("Error deserializing {e}");
                Error::DatabaseDeserialization
            })?,
            datetime: r.datetime.with_timezone(&Utc),
            addr: r.addr,
        })
    })
    .collect::<Result<Vec<auth::AuditLogEntry>>>()
}

pub(crate) async fn count_audit_logs(pool: &PgPool) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM audit_log
    "#
    )
    .fetch_one(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string());
        Error::DatabaseExecution
    })?
    .cnt
    .unwrap_or(0))
}

pub(crate) async fn count_user_audit_logs(
    pool: &PgPool,
    user_id: i32,
) -> Result<i64> {
    Ok(sqlx::query!(
        r#"
SELECT count(*) as cnt
FROM audit_log
WHERE user_id=$1
    "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id);
        Error::DatabaseExecution
    })?
    .cnt
    .unwrap_or(0))
}

pub(crate) async fn get_user_info(
    pool: &PgPool,
    user_id: i32,
) -> Result<Option<responses::UserInfo>> {
    sqlx::query_as!(
        responses::UserInfo,
        r#"
SELECT email, registration_date, is_superuser, is_suspended, verification_level,
    consent, consent_date, survey_version
FROM users
WHERE users.id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn get_user_stats(
    pool: &PgPool,
    user_id: i32,
) -> Result<Option<responses::UserStats>> {
    sqlx::query_as!(
        responses::UserStats,
        r#"
SELECT COALESCE(changelog.cnt, 0) AS "changelog_cnt!: i64",
    COALESCE(contributions.cnt, 0) AS "contributions_cnt!: i64",
    COALESCE(stop_pics.cnt, 0) AS "pics_cnt!: i64"
FROM users
LEFT JOIN (
    SELECT author_id, count(*) AS cnt
    FROM changelog
    WHERE author_id=$1
    GROUP BY author_id
    ) AS changelog ON users.id = changelog.author_id
LEFT JOIN (
    SELECT author_id, count(*) AS cnt
    FROM contributions
    WHERE author_id=$1
    GROUP BY author_id
    ) AS contributions ON users.id = contributions.author_id
LEFT JOIN (
    SELECT uploader, count(*) AS cnt
    FROM stop_pics
    WHERE uploader=$1
    GROUP BY uploader
) AS stop_pics ON users.id = stop_pics.uploader
WHERE users.id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn get_user_survey(
    pool: &PgPool,
    user_id: i32,
) -> Result<Option<responses::Survey>> {
    sqlx::query_as!(
        responses::Survey,
        r#"
SELECT survey as data, survey_version as version
FROM users WHERE id=$1"#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id);
        Error::DatabaseExecution
    })
}

pub(crate) async fn update_user_survey(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: i32,
    survey: &serde_json::Value,
    survey_version: i32,
) -> Result<()> {
    sqlx::query!(
        r#"UPDATE users SET survey=$1, survey_version=$2 WHERE id=$3"#,
        survey,
        survey_version,
        user_id
    )
    .execute(&mut **transaction)
    .await
    .map(|_| ())
    .map_err(|err| {
        tracing::error!(error = err.to_string(), user_id, survey = ?survey);
        Error::DatabaseExecution
    })
}

pub(crate) async fn insert_survey_fill(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    survey_fill: &requests::SurveyFill,
    addr: &IpNetwork,
    user_agent: &str,
) -> Result<()> {
    sqlx::query!(
        r#"
INSERT INTO attempted_surveys(user_id, username, email, survey, ip, user_agent)
VALUES ($1, $2, $3, $4, $5, $6)
    "#,
        survey_fill.user_id,
        survey_fill.username,
        survey_fill.email,
        json!(survey_fill.survey),
        addr,
        user_agent
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), survey_fill = ?survey_fill);
        Error::DatabaseExecution
    })?;
    Ok(())
}
