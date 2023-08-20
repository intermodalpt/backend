/*
    Intermodal, transportation information aggregator
    Copyright (C) 2023  Cláudio Pereira

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

use crate::errors::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn gtfs_match<'c, E>(
    executor: E,
    stop_id: i64,
    gtfs_id: String,
    verified: bool,
    source: &str,
) -> Result<()>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let _res = sqlx::query!(
        r#"
UPDATE Stops
SET tml_id=$1, tml_id_verified=$2, tml_id_source=$3
WHERE id=$4
    "#,
        gtfs_id,
        verified,
        source,
        stop_id as i32
    )
    .execute(executor)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    Ok(())
}
