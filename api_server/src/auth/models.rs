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
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use std::collections::HashMap;
use uuid::Uuid;

use crate::auth::Permission;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct JwtAccess(pub(crate) String);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct JwtRefresh(pub(crate) String);
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct JwtManagement(pub(crate) String);

impl From<String> for JwtAccess {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<String> for JwtRefresh {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<String> for JwtManagement {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug)]
pub struct HashedRegistration {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Claims {
    // Expiration
    pub exp: i64,
    // Issued at
    pub iat: i64,
    // Not before
    pub nbf: i64,
    // JWT ID
    pub jti: Uuid,
    // Refresh token jti
    pub origin: Uuid,
    // User id
    pub uid: i32,
    // Permissions this user has
    pub permissions: Vec<super::Permission>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RefreshClaims {
    // Issued at
    pub iat: i64,
    // Expiration
    pub exp: i64,
    // User id
    pub uid: i32,
    // JWT ID
    pub jti: Uuid,
    // Username
    pub uname: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ManagementClaims {
    // Issued at
    pub iat: i64,
    // Expiration
    pub exp: i64,
    // User id
    pub uid: i32,
    // JWT ID
    pub jti: Uuid,
}

#[derive(Debug)]
pub(crate) struct NewUserSessionMeta<'a> {
    pub(crate) id: Uuid,
    pub(crate) user_id: i32,
    pub(crate) ip: IpNetwork,
    pub(crate) user_agent: &'a str,
    pub(crate) expiration: chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub(crate) struct NewUserSessionAccessMeta<'a> {
    pub(crate) access: Uuid,
    // I hate the name of this field. "session" is the refresh token ID
    pub(crate) session: Uuid,
    pub(crate) ip: IpNetwork,
    pub(crate) user_agent: &'a str,
    pub(crate) expiration: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsentAnswer {
    pub copyright: bool,
    pub privacy: bool,
    pub terms: bool,
    #[serde(flatten)]
    pub(crate) other: HashMap<String, serde_json::Value>,
}

pub(crate) mod requests {
    use std::fmt::{Debug, Formatter};

    use serde::Deserialize;
    use serde_json::Value;
    use uuid::Uuid;

    #[derive(Deserialize)]
    pub struct Login {
        pub username: String,
        pub password: String,
    }

    impl Debug for Login {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Login")
                .field("username", &self.username)
                .finish()
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct CaptchaAnswer {
        pub uuid: Uuid,
        pub answer: String,
    }

    #[derive(Deserialize)]
    pub struct Register {
        pub username: String,
        pub password: String,
        pub email: String,
        pub captcha: Option<CaptchaAnswer>,
        pub survey: Value,
        pub consent: super::ConsentAnswer,
    }

    impl Debug for Register {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Register")
                .field("username", &self.username)
                .field("email", &self.email)
                .field("captcha", &self.captcha)
                .field("inquiry", &self.survey)
                .finish()
        }
    }

    #[derive(Deserialize)]
    pub struct ChangeKnownPassword {
        pub username: String,
        pub old_password: String,
        pub new_password: String,
    }

    impl Debug for ChangeKnownPassword {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ChangeKnownPassword")
                .field("username", &self.username)
                .finish()
        }
    }

    #[derive(Deserialize)]
    pub struct ChangeUnknownPassword {
        pub username: String,
        pub new_password: String,
    }

    impl Debug for ChangeUnknownPassword {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ChangeKnownPassword")
                .field("username", &self.username)
                .finish()
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct UsernameAvailability {
        pub username: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct NewManagementToken {
        pub name: String,
    }
}

pub(crate) mod responses {
    use chrono::Utc;
    use serde::Serialize;
    use sqlx::types::ipnetwork::IpNetwork;
    use uuid::Uuid;

    use commons::models::auth;

    #[derive(Serialize)]
    pub struct CaptchaChallenge {
        pub png: String,
        pub uuid: Uuid,
    }

    #[derive(Serialize)]
    pub struct AuditLogEntry {
        #[serde(flatten)]
        pub entry: auth::AuditLogEntry,
        pub user_username: String,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum UsernameAvailability {
        Available,
        Invalid { reason: String },
        Taken,
    }

    // These are a 1:1 with the issued JWT refresh tokens
    #[derive(Debug, Serialize)]
    pub(crate) struct UserSession {
        pub(crate) id: Uuid,
        pub(crate) user_id: i32,
        pub(crate) ip: IpNetwork,
        pub(crate) user_agent: String,
        pub(crate) expiration: chrono::DateTime<Utc>,
        pub(crate) revoked: bool,
    }

    // These are a 1:1 with the issued JWT access tokens
    #[derive(Debug, Serialize)]
    pub(crate) struct UserAccessSession {
        pub(crate) id: Uuid,
        pub(crate) session_id: Uuid,
        pub(crate) ip: IpNetwork,
        pub(crate) user_agent: String,
        pub(crate) creation: chrono::DateTime<Utc>,
        pub(crate) last_active: chrono::DateTime<Utc>,
        pub(crate) expiration: chrono::DateTime<Utc>,
    }

    #[derive(Debug, Serialize)]
    pub struct ManagementToken {
        pub id: Uuid,
        pub name: String,
        pub token: super::JwtManagement,
        pub revoked: bool,
        pub permissions: sqlx::types::Json<Vec<super::Permission>>,
    }
}
