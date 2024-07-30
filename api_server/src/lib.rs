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

#![allow(unused)]
pub mod auth;
pub mod contrib;
pub mod errors;
pub mod geo;
pub mod gtfs;
pub mod http;
pub mod info;
pub mod operators;
pub mod osm;
pub mod pics;
mod responses;
pub mod routes;
pub mod settings;
pub mod state;
pub mod stops;
pub mod utils;

pub use errors::Error;
pub use state::{AppState, Cached, State};
