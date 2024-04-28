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

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapContent {
    data: serde_json::Value,
    #[serde(default)]
    lat: Option<f64>,
    #[serde(default)]
    lon: Option<f64>,
    #[serde(default)]
    zoom: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ContentBlock {
    Md(String),
    Img(String), // SHA1 internally, URL externally
    Map(MapContent),
}

#[derive(Debug, Serialize)]
pub struct NewsItem {
    pub id: i32,
    pub title: String,
    pub summary: String,
    pub content: Vec<ContentBlock>,

    pub publish_datetime: DateTime<Local>,
    pub edit_datetime: Option<DateTime<Local>>,

    pub is_visible: bool,
    pub operator_ids: Vec<i32>,
    pub region_ids: Vec<i32>,
}
