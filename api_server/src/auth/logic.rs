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
    let user = sql::fetch_user(db_pool, &request.username)
        .await?
        .ok_or(Error::Forbidden)?;

    let parsed_hash = PasswordHash::new(&user.password).map_err(|err| {
        tracing::error!(
            msg="Unable to parse existing hash",
            err=?err,
            username=request.username);
        Error::Processing
    })?;

    Pbkdf2
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Forbidden)?;

    let mut transaction = db_pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::insert_audit_log_entry(
        &mut transaction,
        user.id,
        &requester_ip.into(),
        auth::AuditLogAction::Login,
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    let issue_time = Utc::now();
    let expiration_time =
        issue_time.add(chrono::Duration::try_days(90).unwrap());
    let claims = models::Claims {
        iat: issue_time.timestamp(),
        exp: expiration_time.timestamp(),
        uid: user.id,
        uname: user.username,
        permissions: models::perms::Permissions {
            is_admin: user.is_admin,
            is_trusted: user.is_trusted,
        },
    };
    encode_claims(&claims)
}

pub(crate) async fn is_user_password(
    username: &str,
    password: &str,
    db_pool: &PgPool,
) -> Result<bool, Error> {
    let user = sql::fetch_user(db_pool, username)
        .await?
        .ok_or(Error::NotFoundUpstream)?;

    let parsed_hash = PasswordHash::new(&user.password).map_err(|err| {
        tracing::error!(msg="Unable to parse existing hash", err=?err, username);
        Error::Processing
    })?;

    Ok(Pbkdf2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |()| true))
}

fn gen_kdf_password_string(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| {
            tracing::error!(msg="Unable to hash password", err=?err);
            Error::Processing
        })?
        .to_string())
}

pub(crate) async fn register(
    request: requests::Register,
    requester_ip: IpAddr,
    db_pool: &PgPool,
) -> Result<(), Error> {
    let existing_user = sql::fetch_user_by_username_or_email(
        db_pool,
        &request.username,
        &request.email,
    )
    .await?;

    // TODO something more robust than this
    if request.username.contains(' ') {
        return Err(Error::ValidationFailure(
            "Username cannot contain spaces".to_string(),
        ));
    }

    if request.password.len() < 7 {
        return Err(Error::ValidationFailure(
            "Password must be at least 7 characters long".to_string(),
        ));
    }

    if existing_user.is_some() {
        let existing_user = existing_user.unwrap();
        if existing_user.username == request.username {
            return Err(Error::ValidationFailure(
                "Username already in use".to_string(),
            ));
        }

        if existing_user.email == request.email {
            return Err(Error::ValidationFailure(
                "Email already in use".to_string(),
            ));
        }
    }

    let password_kdf = gen_kdf_password_string(&request.password)?;
    let registration = models::HashedRegistration {
        username: request.username,
        password: password_kdf,
        email: request.email,
    };

    let mut transaction = db_pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

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

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

#[allow(clippy::similar_names)]
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

    let mut transaction = db_pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

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

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

#[allow(clippy::similar_names)]
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

    let mut transaction = db_pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

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

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })
}

pub(crate) fn encode_claims(claims: &models::Claims) -> Result<String, Error> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            SECRET_KEY.get().unwrap().as_ref(),
        ),
    )
    .map_err(|err| {
        tracing::error!("Failed to encode JWT: {err}");
        Error::Processing
    })
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
    use sqlx::PgPool;
    use std::net::{IpAddr, Ipv4Addr};

    use crate::auth::models::requests;
    use crate::errors::Error;

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
            permissions: models::perms::Permissions {
                is_admin: false,
                is_trusted: false,
            },
        };
        let encoded = encode_claims(&claims).unwrap();
        let decoded = decode_claims(&encoded).unwrap();
        assert_eq!(claims, decoded);
    }

    #[sqlx::test]
    async fn ok_register_login(pool: PgPool) {
        // REGISTER
        let req = requests::Register {
            username: "username".to_string(),
            password: "password".to_string(),
            email: "user@intermodal.pt".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(res, Ok(()));

        // LOGIN
        let req = requests::Login {
            username: "username".to_string(),
            password: "password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::login(req, req_ori, &pool).await;
        assert!(res.is_ok())
    }

    #[sqlx::test]
    async fn err_register_bad_username_spaces(pool: PgPool) {
        let req = requests::Register {
            username: "invalid username".to_string(),
            password: "password".to_string(),
            email: "user@intermodal.pt".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(
            res,
            Err(Error::ValidationFailure(
                "Username cannot contain spaces".into()
            ))
        );
    }

    #[sqlx::test]
    async fn err_register_bad_password(pool: PgPool) {
        let req = requests::Register {
            username: "username".to_string(),
            password: "".to_string(),
            email: "user@intermodal.pt".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(
            res,
            Err(Error::ValidationFailure(
                "Password must be at least 7 characters long".to_string()
            ))
        );
    }

    #[sqlx::test]
    async fn err_register_duplicated_username(pool: PgPool) {
        let req = requests::Register {
            username: "username".to_string(),
            password: "password".to_string(),
            email: "user@intermodal.pt".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(res, Ok(()));

        let req = requests::Register {
            username: "username".to_string(),
            password: "password2".to_string(),
            email: "user2@intermodal.pt".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(
            res,
            Err(Error::ValidationFailure(
                "Username already in use".to_string()
            ))
        );
    }

    #[sqlx::test]
    async fn err_register_duplicated_email(pool: PgPool) {
        let req = requests::Register {
            username: "username".to_string(),
            password: "password".to_string(),
            email: "user@intermodal.pt".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(res, Ok(()));

        let req = requests::Register {
            username: "username2".to_string(),
            password: "password2".to_string(),
            email: "user@intermodal.pt".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(
            res,
            Err(Error::ValidationFailure("Email already in use".to_string()))
        );
    }

    #[sqlx::test(fixtures("users"))]
    async fn ok_login_admin(pool: PgPool) {
        let req = requests::Login {
            username: "admin".to_string(),
            password: "password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let jwt = super::login(req, req_ori, &pool).await.unwrap();
        let claims = super::decode_claims(&jwt).unwrap();

        assert_eq!(claims.uid, 1);
        assert_eq!(&claims.uname, "admin");
        assert_eq!(claims.permissions.is_admin, true);
    }

    #[sqlx::test(fixtures("users"))]
    async fn ok_login_user(pool: PgPool) {
        let req = requests::Login {
            username: "user".to_string(),
            password: "password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let jwt = super::login(req, req_ori, &pool).await.unwrap();
        let claims = super::decode_claims(&jwt).unwrap();

        assert_eq!(claims.uid, 2);
        assert_eq!(&claims.uname, "user");
        assert_eq!(claims.permissions.is_admin, false);
    }

    #[sqlx::test(fixtures("users"))]
    async fn err_login_user_bad_password(pool: PgPool) {
        let req = requests::Login {
            username: "user".to_string(),
            password: "wrong_password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::login(req, req_ori, &pool).await;
        assert_eq!(res, Err(Error::Forbidden));
    }

    #[sqlx::test(fixtures("users"))]
    async fn ok_change_password(pool: PgPool) {
        // PASSWORD CHANGE
        let req = requests::ChangeKnownPassword {
            username: "user".to_string(),
            old_password: "password".to_string(),
            new_password: "new_password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        super::change_password(req, 1, req_ori, &pool)
            .await
            .unwrap();

        // LOGIN
        let req = requests::Login {
            username: "user".to_string(),
            password: "new_password".to_string(),
        };

        assert!(super::login(req, req_ori, &pool).await.is_ok());
    }

    #[sqlx::test(fixtures("users"))]
    async fn err_change_password_bad_old_password(pool: PgPool) {
        let req = requests::ChangeKnownPassword {
            username: "user".to_string(),
            old_password: "wrong_password".to_string(),
            new_password: "new_password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        assert_eq!(
            super::change_password(req, 1, req_ori, &pool).await,
            Err(Error::Forbidden)
        );
    }

    #[sqlx::test(fixtures("users"))]
    async fn ok_admin_change_password(pool: PgPool) {
        // PASSWORD CHANGE
        let req = requests::ChangeUnknownPassword {
            username: "user".to_string(),
            new_password: "new_password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        super::admin_change_password(req, 1, req_ori, &pool)
            .await
            .unwrap();

        // LOGIN
        let req = requests::Login {
            username: "user".to_string(),
            password: "new_password".to_string(),
        };

        assert!(super::login(req, req_ori, &pool).await.is_ok());
    }
}
