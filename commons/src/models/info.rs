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

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgRef {
    pub id: i32,
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub transcript: Option<String>,
    #[serde(default)]
    pub attribution: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentRef {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ContentBlock {
    Md(String),
    Img(ImgRef),
    Map(MapContent),
    Ref(ContentRef),
}

impl ContentBlock {
    pub fn validate(&self) -> Result<(), &'static str> {
        // There's a lot of room for improvement here.
        match self {
            ContentBlock::Md(_) => Ok(()),
            ContentBlock::Img(_) => {
                // TODO: Validate URL
                Ok(())
            }
            ContentBlock::Map(map) => {
                if let Some(lat) = map.lat {
                    if map.lon.is_none() || !(-90.0..=90.0).contains(&lat) {
                        return Err("Invalid latitude");
                    }
                }

                if let Some(lon) = map.lon {
                    if map.lat.is_none() || !(-180.0..=180.0).contains(&lon) {
                        return Err("Invalid longitude");
                    }
                }

                Ok(())
            }
            ContentBlock::Ref(content) => {
                if content.name.is_some() || content.url.is_some() {
                    Ok(())
                } else {
                    Err("ContentRef must have at least one of name or url")
                }
            }
        }
    }
}
