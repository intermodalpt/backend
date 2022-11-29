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

use super::SECRET_KEY;
use chrono::Utc;
use jsonwebtoken;
use pbkdf2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Pbkdf2,
};
use sqlx::PgPool;
use std::ops::Add;

use super::{models, models::requests, sql};
use crate::errors::Error;

pub(crate) async fn try_get_user(
    token: &str,
    db_pool: &PgPool,
) -> Result<Option<i32>, Error> {
    let user_id = sql::fetch_user_id(db_pool, token).await?;
    Ok(user_id)
}

pub(crate) async fn get_user(
    token: &str,
    db_pool: &PgPool,
) -> Result<i32, Error> {
    let user = try_get_user(token, db_pool).await?;
    if let Some(id) = user {
        Ok(id)
    } else {
        Err(Error::Forbidden)
    }
}

pub(crate) async fn login(
    request: requests::Login,
    db_pool: &PgPool,
) -> Result<String, Error> {
    let user = sql::fetch_user(db_pool, &request.username).await?;
    if user.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let user = user.unwrap();

    if user.password.is_none() {
        return Err(Error::DependenciesNotMet);
    }

    let user_password = user.password.unwrap();

    let parsed_hash = PasswordHash::new(&user_password).map_err(|_err| {
        Error::Processing("Unable to parse existing hash".to_string())
    })?;

    Pbkdf2
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Forbidden)?;

    let issue_time = Utc::now();
    let expiration_time = issue_time.add(chrono::Duration::days(1));
    let claims = models::Claims {
        iat: issue_time.timestamp(),
        exp: expiration_time.timestamp(),
        uid: user.id as i64,
        uname: user.username,
        permissions: models::Permissions {
            is_admin: user.is_admin,
        },
    };
    encode_claims(claims)
}

pub(crate) async fn register(
    request: requests::Register,
    db_pool: &PgPool,
) -> Result<(), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Pbkdf2
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|_err| {
            Error::Processing("Unable to hash password".to_string())
        })?
        .to_string();
    let registration = models::HashedRegistration {
        username: request.username,
        password,
        email: request.email,
    };
    let user_id = sql::register_user(db_pool, registration).await?;
    // TODO log
    Ok(())
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

pub(crate) async fn decode_claims(
    jwt: String,
) -> Result<models::Claims, Error> {
    let decoded_token = jsonwebtoken::decode::<models::Claims>(
        &jwt,
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
        let claims = models::Claims {
            iat: 0,
            exp: 0,
            uid: 0,
            uname: "test".to_string(),
            permissions: models::Permissions { is_admin: false },
        };
        let encoded = encode_claims(claims.clone()).unwrap();
        let decoded = decode_claims(encoded).unwrap();
        assert_eq!(claims, decoded);
    }
}
