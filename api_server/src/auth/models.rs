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

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HashedRegistration {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Claims {
    // Issued at
    pub iat: i64,
    // Expiration
    pub exp: i64,
    // User id
    pub uid: i32,
    // Username
    pub uname: String,
    // Perms
    pub permissions: perms::Permissions,
}
pub(crate) trait ClaimPermission {
    fn is_valid(claims: &Claims) -> bool;
}

pub(crate) struct ScopedClaim<P: ClaimPermission>(
    pub(crate) Claims,
    pub(crate) std::marker::PhantomData<P>,
);

pub(crate) mod perms {
    use super::{ClaimPermission, Claims};
    use serde::{Deserialize, Serialize};

    // TODO complete this later
    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Permissions {
        pub is_admin: bool,
        pub is_trusted: bool,
    }

    pub struct Admin;

    impl ClaimPermission for Admin {
        fn is_valid(claims: &Claims) -> bool {
            claims.permissions.is_admin
        }
    }

    pub struct Trusted;

    impl ClaimPermission for Trusted {
        fn is_valid(claims: &Claims) -> bool {
            claims.permissions.is_admin || claims.permissions.is_trusted
        }
    }
}

pub(crate) mod requests {
    use std::fmt::{Debug, Formatter};

    use serde::Deserialize;
    use sqlx::types::JsonValue;
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
        pub inquiry: JsonValue,
    }

    impl Debug for Register {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Register")
                .field("username", &self.username)
                .field("email", &self.email)
                .field("captcha", &self.captcha)
                .field("inquiry", &self.inquiry)
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
}

pub(crate) mod responses {
    use serde::Serialize;
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
}
