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
use pbkdf2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Pbkdf2,
};
use sqlx::PgPool;
use std::net::IpAddr;
use uuid::Uuid;

use commons::models::auth;

use super::{
    models, models::requests, perms, sql, ACCESS_MINUTES, ACCESS_SECRET_KEY,
    MANAGEMENT_DAYS, MANAGEMENT_SECRET_KEY, REFRESH_DAYS, REFRESH_SECRET_KEY,
};
use crate::auth::models::responses;
use crate::errors::Error;

pub(crate) async fn login(
    request: requests::Login,
    db_pool: &PgPool,
    requester_ip: IpAddr,
    user_agent: &str,
) -> Result<(models::RefreshClaims, models::JwtRefresh), Error> {
    let user = sql::fetch_user_by_username(db_pool, &request.username)
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

    let issue_time = Utc::now();
    let expiration_time = issue_time
        + chrono::Duration::try_days(*REFRESH_DAYS.get().unwrap()).unwrap();
    let refresh_claims = models::RefreshClaims {
        iat: issue_time.timestamp(),
        exp: expiration_time.timestamp(),
        uid: user.id,
        jti: Uuid::new_v4(),
        uname: user.username,
    };
    let encoded_claims = encode_refresh_claims(&refresh_claims)?;

    let mut transaction = db_pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    sql::insert_user_session(
        &mut transaction,
        models::NewUserSessionMeta {
            id: refresh_claims.jti,
            user_id: user.id,
            ip: requester_ip.into(),
            user_agent,
            expiration: expiration_time,
        },
    )
    .await?;

    sql::insert_audit_log_entry(
        &mut transaction,
        user.id,
        &requester_ip.into(),
        auth::AuditLogAction::Login {
            refresh_jti: refresh_claims.jti,
            // TODO add me once we also generate access tokens
            access_jti: None,
        },
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok((refresh_claims, encoded_claims))
}

pub(crate) async fn renew_token(
    refresh_claims: models::RefreshClaims,
    db_pool: &PgPool,
    requester_ip: IpAddr,
    user_agent: &str,
) -> Result<(models::Claims, models::JwtAccess), Error> {
    let user = sql::fetch_user_by_id(db_pool, refresh_claims.uid)
        .await?
        .ok_or(Error::IllegalState)
        .inspect_err(|_| {
            tracing::error!(
                msg = "Valid JWT for unknown user".to_string(),
                jti = refresh_claims.jti.to_string(),
                uid = refresh_claims.uid
            )
        })?;

    let mut transaction = db_pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let session = sql::fetch_user_session(&mut *transaction, refresh_claims.jti)
        .await
        .map_err(|_| Error::Forbidden)?
        .ok_or_else(|| {
            tracing::error!(msg="Problem retrieving the access token", claims=?refresh_claims);
            Error::IllegalState
        })?;

    if session.revoked {
        return Err(Error::Unauthorized);
    }

    // TODO
    // Better user permission management
    let permissions = if user.is_admin {
        perms::Permission::admin_default()
    } else if user.is_trusted {
        perms::Permission::trusted_default()
    } else if let Some(operator_id) = user.works_for {
        perms::Permission::operator_default(operator_id)
    } else {
        perms::Permission::user_default()
    };

    let issue_time = Utc::now();
    let expiration_time = issue_time
        + chrono::Duration::try_minutes(*ACCESS_MINUTES.get().unwrap())
            .unwrap();
    let claims = models::Claims {
        iat: issue_time.timestamp(),
        nbf: issue_time.timestamp(),
        exp: expiration_time.timestamp(),
        jti: Uuid::new_v4(),
        origin: refresh_claims.jti,
        uid: refresh_claims.uid,
        permissions,
    };
    let encoded_claims = encode_access_claims(&claims)?;

    sql::insert_user_session_renewal(
        &mut transaction,
        models::NewUserSessionAccessMeta {
            access: claims.jti,
            session: refresh_claims.jti,
            ip: requester_ip.into(),
            user_agent,
            expiration: expiration_time,
        },
    )
    .await?;
    sql::insert_audit_log_entry(
        &mut transaction,
        refresh_claims.uid,
        &requester_ip.into(),
        auth::AuditLogAction::RefreshToken {
            refresh_jti: refresh_claims.jti,
            access_jti: claims.jti,
        },
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok((claims, encoded_claims))
}

pub(crate) async fn create_management_token(
    request: requests::NewManagementToken,
    db_pool: &PgPool,
    uid: i32,
    requester_ip: IpAddr,
    user_agent: &str,
) -> Result<responses::ManagementToken, Error> {
    let mut transaction = db_pool.begin().await.map_err(|err| {
        tracing::error!("Failed to open transaction: {err}");
        Error::DatabaseExecution
    })?;

    let user = sql::fetch_user_by_id(&mut *transaction, uid)
        .await?
        .ok_or(Error::Forbidden)?;

    let permissions = sql::fetch_user_permissions(&mut *transaction, uid)
        .await?
        .ok_or(Error::IllegalState)?;

    let session_id = Uuid::new_v4();
    let issue_time = Utc::now();
    let expiration_time = issue_time
        + chrono::Duration::try_days(*MANAGEMENT_DAYS.get().unwrap()).unwrap();
    let claims = models::ManagementClaims {
        iat: issue_time.timestamp(),
        exp: expiration_time.timestamp(),
        uid: user.id,
        jti: session_id,
    };
    let encoded_claims = encode_management_claims(&claims)?;

    sql::insert_user_session(
        &mut transaction,
        models::NewUserSessionMeta {
            id: session_id,
            user_id: user.id,
            ip: requester_ip.into(),
            user_agent,
            expiration: expiration_time,
        },
    )
    .await?;

    sql::insert_management_token(
        &mut transaction,
        session_id,
        &request.name,
        &encoded_claims,
    )
    .await?;

    sql::insert_audit_log_entry(
        &mut transaction,
        user.id,
        &requester_ip.into(),
        auth::AuditLogAction::ManagementTokenIssued { session_id },
    )
    .await?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("Transaction failed to commit: {err}");
        Error::DatabaseExecution
    })?;

    Ok(responses::ManagementToken {
        id: session_id,
        name: request.name,
        permissions: sqlx::types::Json(permissions),
        revoked: false,
        token: encoded_claims,
    })
}

pub(crate) async fn is_user_password(
    username: &str,
    password: &str,
    db_pool: &PgPool,
) -> Result<bool, Error> {
    let user = sql::fetch_user_by_username(db_pool, username)
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

pub(crate) fn validate_username(username: &str) -> Result<(), String> {
    if username.trim().len() < 3 {
        return Err("Username must be at least 3 characters long".to_string());
    }

    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
    {
        return Err(
            "Username must contain only alphanumeric characters".to_string()
        );
    }

    Ok(())
}

fn validate_email(email: &str) -> Result<(), String> {
    let re =
        regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
            .unwrap();
    if !re.is_match(email) {
        return Err("Invalid email address".to_string());
    }

    Ok(())
}

pub(crate) fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 7 {
        return Err("Password must be at least 7 characters long".to_string());
    }

    Ok(())
}

pub(crate) fn validate_consent(
    consent: &models::ConsentAnswer,
) -> Result<(), String> {
    if !consent.privacy || !consent.terms || !consent.copyright {
        return Err("Consent was not adequately given".to_string());
    }

    Ok(())
}

pub(crate) async fn is_valid_registration(
    request: &requests::Register,
    db_pool: &PgPool,
) -> Result<(), Error> {
    validate_username(&request.username).map_err(Error::ValidationFailure)?;
    validate_password(&request.password).map_err(Error::ValidationFailure)?;
    validate_email(&request.email).map_err(Error::ValidationFailure)?;
    validate_consent(&request.consent).map_err(Error::ValidationFailure)?;

    let existing_user = sql::fetch_user_by_username_or_email(
        db_pool,
        &request.username,
        &request.email,
    )
    .await?;

    if let Some(existing_user) = &existing_user {
        if existing_user.username == request.username {
            return Err(Error::ValidationFailure(
                "Username already in use".to_string(),
            ));
        }
    }

    if let Some(existing_user) = &existing_user {
        if existing_user.email == request.email {
            return Err(Error::ValidationFailure(
                "Email already in use".to_string(),
            ));
        }
    }

    Ok(())
}

pub(crate) async fn register(
    request: requests::Register,
    requester_ip: IpAddr,
    db_pool: &PgPool,
) -> Result<(), Error> {
    is_valid_registration(&request, db_pool).await?;

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

    let user_id = sql::register_user(
        &mut transaction,
        &registration,
        request.consent,
        request.survey,
    )
    .await?;
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

    let changed_user = sql::fetch_user_by_username(db_pool, &request.username)
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

pub(crate) fn encode_access_claims(
    claims: &models::Claims,
) -> Result<models::JwtAccess, Error> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    let encoding_key = jsonwebtoken::EncodingKey::from_secret(
        ACCESS_SECRET_KEY.get().unwrap().as_ref(),
    );
    jsonwebtoken::encode(&header, claims, &encoding_key)
        .map(models::JwtAccess)
        .map_err(|err| {
            tracing::error!("Failed to encode Access JWT: {err}");
            Error::Processing
        })
}

pub(crate) fn decode_access_claims(jwt: &str) -> Result<models::Claims, Error> {
    let mut validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_nbf = true;
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(
        ACCESS_SECRET_KEY.get().unwrap().as_ref(),
    );
    let decoded_token =
        jsonwebtoken::decode::<models::Claims>(jwt, &decoding_key, &validation)
            .map_err(|err| {
                tracing::error!("Failed to decode Access JWT: {err}");
                Error::Forbidden
            })?;

    Ok(decoded_token.claims)
}

pub(crate) fn encode_refresh_claims(
    claims: &models::RefreshClaims,
) -> Result<models::JwtRefresh, Error> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    jsonwebtoken::encode(
        &header,
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            REFRESH_SECRET_KEY.get().unwrap().as_ref(),
        ),
    )
    .map(models::JwtRefresh)
    .map_err(|err| {
        tracing::error!("Failed to encode Refresh JWT: {err}");
        Error::Processing
    })
}

pub(crate) fn decode_refresh_claims(
    jwt: &str,
) -> Result<models::RefreshClaims, Error> {
    let mut validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_nbf = true;
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(
        REFRESH_SECRET_KEY.get().unwrap().as_ref(),
    );
    let decoded_token = jsonwebtoken::decode::<models::RefreshClaims>(
        jwt,
        &decoding_key,
        &validation,
    )
    .map_err(|err| {
        tracing::error!("Failed to decode Refresh JWT: {err}");
        Error::Forbidden
    })?;

    Ok(decoded_token.claims)
}

pub(crate) fn encode_management_claims(
    claims: &models::ManagementClaims,
) -> Result<models::JwtManagement, Error> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    jsonwebtoken::encode(
        &header,
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            MANAGEMENT_SECRET_KEY.get().unwrap().as_ref(),
        ),
    )
    .map(|token| models::JwtManagement(format!("manag.{token}")))
    .map_err(|err| {
        tracing::error!("Failed to encode Management JWT: {err}");
        Error::Processing
    })
}

pub(crate) fn decode_management_claims(
    jwt: &str,
) -> Result<models::ManagementClaims, Error> {
    let validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(
        MANAGEMENT_SECRET_KEY.get().unwrap().as_ref(),
    );
    let decoded_token = jsonwebtoken::decode::<models::ManagementClaims>(
        &jwt[6..],
        &decoding_key,
        &validation,
    )
    .map_err(|err| {
        tracing::error!("Failed to decode Management JWT: {err}");
        Error::Forbidden
    })?;

    Ok(decoded_token.claims)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use std::net::{IpAddr, Ipv4Addr};

    use crate::auth::{models, models::requests, Permission};
    use crate::errors::Error;

    #[test]
    fn encode_decode_access_claims() {
        use super::*;

        //The key must be set
        let _ = ACCESS_SECRET_KEY
            .set(Box::leak(Box::new("super_secret_key".to_string())));

        let claims = models::Claims {
            iat: 0,
            nbf: 0,
            jti: Default::default(),
            exp: 2000000000,
            uid: 0,
            permissions: vec![],
            origin: Default::default(),
        };
        let encoded = encode_access_claims(&claims).unwrap();
        let decoded = decode_access_claims(&encoded.0).unwrap();
        assert_eq!(claims, decoded);
    }

    #[sqlx::test]
    async fn ok_register_login(pool: PgPool) {
        // REGISTER
        let req = requests::Register {
            username: "username".to_string(),
            password: "password".to_string(),
            email: "user@intermodal.pt".to_string(),
            captcha: None,
            survey: Default::default(),
            consent: models::ConsentAnswer {
                privacy: true,
                terms: true,
                copyright: true,
                other: Default::default(),
            },
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

        let res = super::login(req, &pool, req_ori, "").await;
        assert!(res.is_ok())
    }

    #[sqlx::test]
    async fn err_register_bad_username_spaces(pool: PgPool) {
        let req = requests::Register {
            username: "invalid username".to_string(),
            password: "password".to_string(),
            email: "user@intermodal.pt".to_string(),
            captcha: None,
            survey: Default::default(),
            consent: models::ConsentAnswer {
                privacy: true,
                terms: true,
                copyright: true,
                other: Default::default(),
            },
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
            captcha: None,
            survey: Default::default(),
            consent: models::ConsentAnswer {
                privacy: true,
                terms: true,
                copyright: true,
                other: Default::default(),
            },
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
            captcha: None,
            survey: Default::default(),
            consent: models::ConsentAnswer {
                privacy: true,
                terms: true,
                copyright: true,
                other: Default::default(),
            },
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(res, Ok(()));

        let req = requests::Register {
            username: "username".to_string(),
            password: "password2".to_string(),
            email: "user2@intermodal.pt".to_string(),
            captcha: None,
            survey: Default::default(),
            consent: models::ConsentAnswer {
                privacy: true,
                terms: true,
                copyright: true,
                other: Default::default(),
            },
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
            captcha: None,
            survey: Default::default(),
            consent: models::ConsentAnswer {
                privacy: true,
                terms: true,
                copyright: true,
                other: Default::default(),
            },
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(res, Ok(()));

        let req = requests::Register {
            username: "username2".to_string(),
            password: "password2".to_string(),
            email: "user@intermodal.pt".to_string(),
            captcha: None,
            survey: Default::default(),
            consent: models::ConsentAnswer {
                privacy: true,
                terms: true,
                copyright: true,
                other: Default::default(),
            },
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::register(req, req_ori, &pool).await;
        assert_eq!(
            res,
            Err(Error::ValidationFailure("Email already in use".to_string()))
        );
    }

    async fn login_request(
        req: requests::Login,
        req_ori: IpAddr,
        pool: &PgPool,
    ) -> (models::RefreshClaims, models::Claims) {
        let (refresh_claims, token) =
            super::login(req, &pool, req_ori, "").await.unwrap();
        let decoded_refresh_claims =
            super::decode_refresh_claims(&token.0).unwrap();
        let (access_claims, token) =
            super::renew_token(refresh_claims.clone(), &pool, req_ori, "")
                .await
                .unwrap();
        let decoded_access_claims =
            super::decode_access_claims(&token.0).unwrap();

        assert_eq!(&access_claims, &decoded_access_claims);
        assert_eq!(&refresh_claims, &decoded_refresh_claims);

        return (refresh_claims, access_claims);
    }

    #[sqlx::test(fixtures("users"))]
    async fn ok_login_admin(pool: PgPool) {
        let req = requests::Login {
            username: "admin".to_string(),
            password: "password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let (refresh_claims, access_claims) =
            login_request(req, req_ori, &pool).await;

        assert_eq!(refresh_claims.uid, 1);
        assert_eq!(&refresh_claims.uname, "admin");
        assert!(access_claims
            .permissions
            .iter()
            .any(|perm| perm == &Permission::Admin));
    }

    #[sqlx::test(fixtures("users"))]
    async fn ok_login_user(pool: PgPool) {
        let req = requests::Login {
            username: "user".to_string(),
            password: "password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let (refresh_claims, access_claims) =
            login_request(req, req_ori, &pool).await;

        assert_eq!(refresh_claims.uid, 2);
        assert_eq!(&refresh_claims.uname, "user");
        assert!(!access_claims
            .permissions
            .iter()
            .any(|perm| perm == &Permission::Admin));
    }

    #[sqlx::test(fixtures("users"))]
    async fn err_login_user_bad_password(pool: PgPool) {
        let req = requests::Login {
            username: "user".to_string(),
            password: "wrong_password".to_string(),
        };
        let req_ori = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));

        let res = super::login(req, &pool, req_ori, "").await;
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

        assert!(super::login(req, &pool, req_ori, "").await.is_ok());
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

        assert!(super::login(req, &pool, req_ori, "").await.is_ok());
    }
}
