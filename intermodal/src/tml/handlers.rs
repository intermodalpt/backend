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

use std::{fs, io};

use axum::extract::{Path, Query, State};
use axum::Json;

use super::{models, sql};
use crate::{auth, AppState, Error};

pub(crate) async fn tml_get_stops(
    State(state): State<AppState>,
) -> Result<Json<Vec<models::TMLStop>>, Error> {
    Ok(Json(sql::fetch_gtfs_stops(&state.pool).await?))
}

pub(crate) async fn tml_get_gtfs_stops(
) -> Result<Json<Vec<models::GTFSStop>>, Error> {
    let f = fs::File::open("gtfs/stops.txt").unwrap();
    let reader = io::BufReader::new(f);

    let csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader);
    let mut rdr = csv_reader;

    let gtfs_stops = rdr
        .deserialize()
        .into_iter()
        .map(|result| result.unwrap())
        .collect();

    Ok(Json(gtfs_stops))
}

pub(crate) async fn tml_match_stop(
    State(state): State<AppState>,
    claims: Option<auth::Claims>,
    params: Query<models::MatchVerification>,
    Path((stop_id, gtfs_id)): Path<(i64, String)>,
) -> Result<(), Error> {
    if claims.is_none() {
        return Err(Error::Forbidden);
    }
    let claims = claims.unwrap();
    if !claims.permissions.is_admin {
        return Err(Error::Forbidden);
    }

    Ok(sql::gtfs_match(
        &state.pool,
        stop_id,
        gtfs_id,
        params.verified,
        &params.source.to_string(),
    )
    .await?)
}
