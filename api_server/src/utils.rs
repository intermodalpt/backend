/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022-2023  Cláudio Pereira

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

use crate::Error;
use axum::extract::multipart::{Field, Multipart};

pub(crate) async fn get_exactly_one_field(
    multipart: &mut Multipart,
) -> Result<Field, Error> {
    let field = multipart
        .next_field()
        .await
        .map_err(|err| Error::ValidationFailure(err.to_string()))?
        .ok_or(Error::ValidationFailure("No file was provided".to_string()))?;

    Ok(field)
}
