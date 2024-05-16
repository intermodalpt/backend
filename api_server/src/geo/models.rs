/*
    Intermodal, transportation information aggregator
    Copyright (C) 2024  Cláudio Pereira

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

pub(crate) mod responses {
    use commons::models::operators;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct RegionWithOperators {
        pub id: i32,
        pub name: String,
        pub geometry: serde_json::Value,
        pub center_lat: Option<f64>,
        pub center_lon: Option<f64>,
        pub zoom: Option<f64>,
        pub operators: Vec<operators::Operator>,
    }
}
