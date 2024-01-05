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

use sqlx::types::Json;

use crate::Error;
use commons::models::gtfs;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn update_operator_validation_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    data: gtfs::OperatorValidation,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Operators
SET validation=$1
WHERE id=$2
    "#,
        Json(data) as _,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn update_route_validation_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    route_id: i32,
    data: gtfs::RouteValidation,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Routes
SET validation=$1
WHERE id=$2
    "#,
        Json(data) as _,
        route_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}

pub(crate) async fn update_subroute_validation_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    subroute_id: i32,
    data: gtfs::SubrouteValidation,
) -> Result<()> {
    sqlx::query!(
        r#"
UPDATE Subroutes
SET validation=$1
WHERE id=$2
    "#,
        Json(data) as _,
        subroute_id
    )
    .execute(&mut **transaction)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}
