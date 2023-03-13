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

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_admin: bool,
    pub is_trusted: bool,
    pub works_for: Option<i32>,
}

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
    pub permissions: Permissions,
}

// TODO complete this later
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Permissions {
    pub is_admin: bool,
    pub is_trusted: bool,
}

pub(crate) mod requests {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Login {
        pub username: String,
        pub password: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Register {
        pub username: String,
        pub password: String,
        pub email: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct ChangePassword {
        pub username: String,
        pub old_password: String,
        pub new_password: String,
    }
}
