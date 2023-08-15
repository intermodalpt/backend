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

pub(crate) mod responses {
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(Serialize, Clone, ToSchema)]
    pub struct Stats {
        pub stop_count: i64,
        pub route_count: i64,
        pub subroute_count: i64,
        pub departure_count: i64,
        pub picture_count: i64,
    }
}
