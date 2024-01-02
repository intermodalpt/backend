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

pub mod calendar;
pub mod changes;
pub mod gtfs;
pub mod operators;
pub mod pics;
pub mod routes;
pub mod stops;

pub use changes::{Change, Changeset, Contribution};

// Rust... cmon. Let's pretend I'm not going to use these a million times because they're not part of the stdlib
pub fn vec_into_vec<T, R: From<T>>(vec: Vec<T>) -> Vec<R> {
    vec.into_iter().map(Into::into).collect()
}

pub fn opt_vec_into_opt_vec<T, R: From<T>>(
    opt_vec: Option<Vec<T>>,
) -> Option<Vec<R>> {
    opt_vec.map(|vec| vec.into_iter().map(Into::into).collect())
}

pub fn vec_try_into_vec<T, R: TryFrom<T, Error = Err>, Err>(
    vec: Vec<T>,
) -> Result<Vec<R>, Err> {
    vec.into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, Err>>()
}

pub fn opt_vec_try_into<T, R: TryFrom<T, Error = Err>, Err>(
    opt_vec: Option<Vec<T>>,
) -> Result<Option<Vec<R>>, Err> {
    opt_vec
        .map(|col| {
            col.into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, Err>>()
        })
        .transpose()
}
