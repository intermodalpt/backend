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

use chrono::Utc;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::types::Json;
use sqlx::PgPool;

use commons::models::auth;

use super::models::responses;
use crate::auth::models;
use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_user_by_id(
    pool: &PgPool,
    uid: i32,
) -> Result<Option<auth::User>> {
    sqlx::query_as!(
        auth::User,
        r#"
SELECT id, username, password, email, is_admin, is_trusted, works_for
FROM Users
WHERE id=$1
    "#,
        uid
    )
    .fetch_optional(pool)
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
SELECT id, username, password, email, is_admin, is_trusted, works_for
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
SELECT id, username, password, email, is_admin, is_trusted, works_for
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

pub(crate) async fn register_user(
    pool: &PgPool,
    request: &models::HashedRegistration,
) -> Result<i32> {
    let res = sqlx::query!(
        r#"INSERT INTO Users (username, password, email)
VALUES ($1, $2, $3)
RETURNING id"#,
        request.username,
        request.password,
        request.email
    )
    .fetch_one(pool)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            username = request.username,
            email = request.email
        );
        Error::DatabaseExecution
    })?;
    Ok(res.id)
}

pub(crate) async fn change_user_password(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    username: &str,
    password: &str,
) -> Result<()> {
    sqlx::query!(
        r#"UPDATE users SET password=$1 WHERE username=$2"#,
        password,
        username
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(error = err.to_string(), username);
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
    user_id: i32,
    addr: &IpNetwork,
    action: auth::AuditLogAction,
) -> Result<i64> {
    let res = sqlx::query!(
        r#"
INSERT INTO audit_log(user_id, action, addr)
VALUES ($1, $2, $3)
RETURNING id
    "#,
        user_id,
        Json(&action) as _,
        addr
    )
    .fetch_one(&mut **transaction)
    .await
    .map_err(|err| {
        tracing::error!(
            error = err.to_string(),
            user_id,
            action = ?action,
            addr = ?addr
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
