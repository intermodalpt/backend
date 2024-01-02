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

use serde::{Deserialize, Serialize};

// This is never going to be changed. Is ok to use and re-export
pub use crate::models::calendar::Weekday;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "condition")]
pub enum Condition {
    Holiday,
    Summer,
    School,
    Range { start: (u8, u8), end: (u8, u8) },
    Nth { nth: u8 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub weekdays: Vec<Weekday>,
    pub only_if: Vec<Condition>,
    pub also_if: Vec<Condition>,
    pub except_if: Vec<Condition>,
}
