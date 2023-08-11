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

use commons::models::osm::StoredStopMeta;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Stop {
    pub(crate) id: i32,
    #[serde(default)]
    pub(crate) external_id: String,
    // This is a bit flag made of 4 duets.
    // The four binary duets are for: Position, Service, Infra and [reserved]
    // 0 => Not verified; 1 => Wrong; 2 => Likely; 3 => Verified
    #[serde(default)]
    pub(crate) verification_level: i16,
    pub(crate) osm_history: StoredStopMeta,
}