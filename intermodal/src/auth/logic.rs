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

use sqlx::PgPool;

use super::sql;
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
