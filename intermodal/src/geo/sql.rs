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

use sqlx::PgPool;

use commons::models::geo;

use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub(crate) async fn fetch_parishes(pool: &PgPool) -> Result<Vec<geo::Parish>> {
    sqlx::query_as!(
        geo::Parish,
        r#"
SELECT parishes.id, parishes.name, parishes.short_name, municipalities.name as municipality,
    municipalities.zone, parishes.polygon, parishes.geojson
FROM parishes
JOIN municipalities ON parishes.municipality = municipalities.id
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(|err| Error::DatabaseExecution(err.to_string()))
}
