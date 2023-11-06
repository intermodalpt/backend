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

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use sqlx::PgPool;

use commons::models::history;
use commons::models::stops;

use super::sql;
use crate::errors::Error;

pub(crate) fn summarize_stop_meta_contributions(
    contributions: Vec<history::Contribution>,
) -> Vec<stops::Stop> {
    let mut modified_stops = HashMap::new();

    for contribution in contributions {
        match contribution.change {
            history::Change::StopUpdate {
                mut original,
                patch,
            } => match modified_stops.entry(original.id) {
                Entry::Occupied(mut entry) => patch.apply(entry.get_mut()),
                Entry::Vacant(entry) => {
                    patch.apply(&mut original);
                    entry.insert(original);
                }
            },
            _ => {
                unreachable!()
            }
        }
    }

    modified_stops.into_values().collect()
}

pub(crate) async fn accept_contribution(
    pool: &PgPool,
    contribution_id: i64,
    user_id: i32,
    verify: bool,
    ignored: &Option<String>,
) -> Result<(), Error> {
    let contribution = sql::fetch_contribution(pool, contribution_id).await?;

    if contribution.is_none() {
        return Err(Error::NotFoundUpstream);
    }
    let mut contribution = contribution.unwrap();

    if contribution.accepted.is_some() {
        return Err(Error::DependenciesNotMet);
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;

    match &mut contribution.change {
        history::Change::StopUpdate { original, patch } => {
            let stop = crate::stops::sql::fetch_stop(pool, original.id).await?;
            if stop.is_none() {
                // TODO Do something about this
                // TODO Prevent patches from reaching this state
                return Err(Error::ValidationFailure(
                    "Stop no longer exists".to_string(),
                ));
            }
            let stop = stop.unwrap();

            *original = stop.clone();
            // FIXME This we might want to check if the original has been patched
            // and if that conflicts with the patch
            let stop = accept_stop_contribution(stop, patch, verify, ignored)?;

            crate::stops::sql::update_stop(
                &mut transaction,
                stop.id,
                stop.into(),
                user_id,
            )
            .await?;
        }
        history::Change::StopPicUpload { .. } => {
            todo!()
        }
        _ => {
            unreachable!()
        }
    }

    sql::insert_changeset_log(
        &mut transaction,
        contribution.author_id,
        &vec![contribution.change],
        Some(contribution_id),
    )
    .await?;

    sql::update_guest_contribution_to_accept(
        &mut transaction,
        contribution_id,
        user_id,
    )
    .await?;

    transaction
        .commit()
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))
}

pub(crate) fn accept_stop_contribution(
    mut current: stops::Stop,
    patch: &mut history::StopPatch,
    verify: bool,
    ignored: &Option<String>,
) -> Result<stops::Stop, Error> {
    let ignored_fields = if let Some(ignored) = ignored {
        ignored
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .collect()
    } else {
        HashSet::new()
    };

    patch.drop_fields(&ignored_fields);

    if !verify {
        patch.deverify(current.verification_level.into());
    }

    patch.drop_noops(&current);

    if patch.is_empty() {
        // TODO Prevent patches from reaching this state
        return Err(Error::ValidationFailure(
            "Patch no longer does anything".to_string(),
        ));
    }

    patch.apply(&mut current);

    Ok(current)
}
