/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022  Cláudio Pereira

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

use serde::Serialize;
use utoipa::Component;

#[derive(Serialize, Component)]
pub struct Parish {
    pub id: i32,
    #[component(example = "Quinta do Conde")]
    pub name: String,
    #[component(example = "Sesimbra")]
    pub municipality: String,
    #[component(example = 3)]
    pub zone: i32,
    #[component(example = "GeoJSON polygon")]
    pub polygon: Option<String>,
}