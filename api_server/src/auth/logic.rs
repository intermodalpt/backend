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

use std::net::IpAddr;
use std::ops::Add;

use chrono::Utc;

use commons::models::auth;
use pbkdf2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Pbkdf2,
};
use sqlx::PgPool;

use super::SECRET_KEY;
use super::{models, models::requests, sql};
use crate::errors::Error;

pub(crate) async fn login(
    request: requests::Login,
    requester_ip: IpAddr,
    db_pool: &PgPool,
) -> Result<String, Error> {
    let user = sql::fetch_user(db_pool, &request.username).await?;
    if user.is_none() {
        return Err(Error::Forbidden);
    }
    let user = user.unwrap();

    let parsed_hash = PasswordHash::new(&user.password).map_err(|_err| {
        Error::Processing("Unable to parse existing hash".to_string())
    })?;

    Pbkdf2
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Forbidden)?;

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::insert_audit_log_entry(
        &mut transaction,
        user.id,
        &requester_ip.into(),
        auth::AuditLogAction::Login,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let issue_time = Utc::now();
    let expiration_time = issue_time.add(chrono::Duration::days(90));
    let claims = models::Claims {
        iat: issue_time.timestamp(),
        exp: expiration_time.timestamp(),
        uid: user.id,
        uname: user.username,
        permissions: models::Permissions {
            is_admin: user.is_admin,
            is_trusted: user.is_trusted,
        },
    };
    encode_claims(claims)
}

pub(crate) async fn is_user_password(
    username: &str,
    password: &str,
    db_pool: &PgPool,
) -> Result<bool, Error> {
    let user = sql::fetch_user(db_pool, username).await?;
    if user.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let user = user.unwrap();

    let parsed_hash = PasswordHash::new(&user.password).map_err(|_err| {
        Error::Processing("Unable to parse existing hash".to_string())
    })?;

    Ok(Pbkdf2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |()| true))
}

fn gen_kdf_password_string(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_err| {
            Error::Processing("Unable to hash password".to_string())
        })?
        .to_string())
}

pub(crate) async fn register(
    request: requests::Register,
    requester_ip: IpAddr,
    db_pool: &PgPool,
) -> Result<(), Error> {
    let password_kdf = gen_kdf_password_string(&request.password)?;
    let registration = models::HashedRegistration {
        username: request.username,
        password: password_kdf,
        email: request.email,
    };

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    let user_id = sql::register_user(db_pool, &registration).await?;
    sql::insert_audit_log_entry(
        &mut transaction,
        user_id,
        &requester_ip.into(),
        auth::AuditLogAction::Register {
            username: registration.username,
            email: registration.email,
        },
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn change_password(
    request: requests::ChangeKnownPassword,
    requester_id: i32,
    requester_ip: IpAddr,
    db_pool: &PgPool,
) -> Result<(), Error> {
    if !is_user_password(&request.username, &request.old_password, db_pool)
        .await?
    {
        return Err(Error::Forbidden);
    }
    let password_kdf = gen_kdf_password_string(&request.new_password)?;

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::change_user_password(
        &mut transaction,
        &request.username,
        &password_kdf,
    )
    .await?;
    sql::insert_audit_log_entry(
        &mut transaction,
        requester_id,
        &requester_ip.into(),
        auth::AuditLogAction::ChangePassword,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) async fn admin_change_password(
    request: requests::ChangeUnknownPassword,
    requester_id: i32,
    requester_ip: IpAddr,
    db_pool: &PgPool,
) -> Result<(), Error> {
    let password_kdf = gen_kdf_password_string(&request.new_password)?;

    let changed_user = sql::fetch_user(db_pool, &request.username)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let mut transaction = db_pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    sql::change_user_password(
        &mut transaction,
        &request.username,
        &password_kdf,
    )
    .await?;
    sql::insert_audit_log_entry(
        &mut transaction,
        requester_id,
        &requester_ip.into(),
        auth::AuditLogAction::AdminChangePassword {
            for_user_id: changed_user.id,
        },
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) fn encode_claims(claims: models::Claims) -> Result<String, Error> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(
            SECRET_KEY.get().unwrap().as_ref(),
        ),
    )
    .map_err(|_err| Error::Processing("Failed to encode JWT".to_string()))
}

pub(crate) fn decode_claims(jwt: &str) -> Result<models::Claims, Error> {
    let decoded_token = jsonwebtoken::decode::<models::Claims>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(
            SECRET_KEY.get().unwrap().as_ref(),
        ),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_err| Error::Forbidden)?;
    Ok(decoded_token.claims)
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_decode_claims() {
        use super::*;

        //The key must be set
        let _ =
            SECRET_KEY.set(Box::leak(Box::new("super_secret_key".to_string())));

        let claims = models::Claims {
            iat: 0,
            exp: 2000000000,
            uid: 0,
            uname: "test".to_string(),
            permissions: models::Permissions {
                is_admin: false,
                is_trusted: false,
            },
        };
        let encoded = encode_claims(claims.clone()).unwrap();
        let decoded = decode_claims(&encoded).unwrap();
        assert_eq!(claims, decoded);
    }
}
