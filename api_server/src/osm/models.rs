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

pub(crate) mod requests {
    use serde::Deserialize;

    use commons::models::osm;

    #[derive(Deserialize)]
    pub struct OsmStop {
        pub id: i64,
        pub history: osm::NodeHistory,
    }

    #[derive(Debug, Deserialize)]
    pub struct OsmFeaturesChange {
        pub features: osm::MapFeatures,
        pub authors: Vec<String>,
    }
}

pub(crate) mod responses {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::Serialize;
    use sqlx::types::Json;

    use commons::models::osm;

    #[derive(Serialize)]
    pub struct OsmStop {
        pub id: i64,
        pub name: Option<String>,
        pub lat: f64,
        pub lon: f64,
        pub pos_author: String,
        pub last_author: String,
        pub creation: DateTime<Utc>,
        pub modification: DateTime<Utc>,
        pub version: i32,
        pub deleted: bool,
    }

    #[derive(Serialize)]
    pub struct FullOsmStop {
        pub id: i64,
        pub name: Option<String>,
        pub lat: f64,
        pub lon: f64,
        pub pos_author: String,
        pub last_author: String,
        pub creation: DateTime<Utc>,
        pub modification: DateTime<Utc>,
        pub version: i32,
        pub deleted: bool,
        pub history: Json<osm::NodeHistory>,
        pub env_features: Json<osm::MapFeatures>,
        pub env_authors: Vec<String>,
        pub env_update: Option<NaiveDateTime>,
    }

    #[derive(Debug, Clone, Serialize, PartialEq)]
    pub struct StopMapFeatures {
        pub id: i32,
        pub osm_id: Option<i64>,
        pub lon: f64,
        pub lat: f64,
        pub env_features: Json<osm::MapFeatures>,
        pub env_authors: Vec<String>,
        pub env_update: Option<NaiveDateTime>,
    }
}
