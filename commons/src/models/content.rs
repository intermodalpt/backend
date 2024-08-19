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

#[derive(Serialize, Deserialize)]
pub struct MapContent {
    pub layers: Vec<MapLayer>,
    #[serde(default)]
    pub camera: Option<CameraSettings>,
    #[serde(default)]
    pub bounding: Option<Vec<[f64; 2]>>,
    pub version: usize,
}

impl MapContent {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.layers.is_empty() {
            return Err("MapContent must have at least one layer");
        }

        for layer in &self.layers {
            layer.validate()?;
        }

        if let Some(camera) = &self.camera {
            camera.validate()?;
        }

        if let Some(bounding) = &self.bounding {
            if bounding.len() != 2 {
                return Err("Bounding box must have two points");
            }

            if bounding.iter().any(|[lon, lat]| {
                !(-180.0..=180.0).contains(lon) || !(-90.0..=90.0).contains(lat)
            }) {
                return Err("Bounding box must have valid coordinates");
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for MapContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapContent")
            .field("layers", &self.layers.len())
            .field("camera", &self.camera)
            .field("bounding", &self.bounding)
            .field("version", &self.version)
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraSettings {
    pub center: [f64; 2],
    #[serde(default)]
    pub zoom: Option<f64>,
    #[serde(default)]
    pub bearing: Option<f64>,
    #[serde(default)]
    pub pitch: Option<f64>,
}

impl CameraSettings {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(-180.0..=180.0).contains(&self.center[0])
            || !(-90.0..=90.0).contains(&self.center[1])
        {
            return Err("Invalid camera center");
        }

        if let Some(zoom) = self.zoom {
            if !(0.0..=20.0).contains(&zoom) {
                return Err("Invalid camera zoom");
            }
        }

        if let Some(bearing) = self.bearing {
            if !(-180.0..=180.0).contains(&bearing) {
                return Err("Invalid camera bearing");
            }
        }

        if let Some(pitch) = self.pitch {
            if !(0.0..=60.0).contains(&pitch) {
                return Err("Invalid camera pitch");
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapLayer {
    pub name: String,
    pub features: Vec<Feature>,
    pub spec: LayerSpec,
    // pub z: i32,
}

impl MapLayer {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.features.is_empty() {
            return Err("MapLayer must have at least one feature");
        }

        for feature in &self.features {
            feature.validate()?;
        }

        self.spec.validate()?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Feature {
    #[serde(rename = "point")]
    PointFeature { loc: [f64; 2] },
    #[serde(rename = "line")]
    LineFeature { line: Vec<[f64; 2]> },
    #[serde(rename = "route")]
    RouteFeature { edges: Vec<RouteEdge> },
    #[serde(rename = "poly")]
    PolyFeature {
        incl: Vec<[f64; 2]>,
        excl: Vec<Vec<[f64; 2]>>,
    },
}

impl Feature {
    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            Feature::PointFeature { loc } => {
                if !(-180.0..=180.0).contains(&loc[0])
                    || !(-90.0..=90.0).contains(&loc[1])
                {
                    return Err("PointFeature must have valid coordinates");
                }

                Ok(())
            }
            Feature::LineFeature { line } => {
                if line.len() < 2 {
                    return Err("LineFeature must have at least two points");
                }

                if line.iter().any(|[lon, lat]| {
                    !(-180.0..=180.0).contains(lon)
                        || !(-90.0..=90.0).contains(lat)
                }) {
                    return Err("LineFeature must have valid coordinates");
                }

                Ok(())
            }
            Feature::RouteFeature { edges } => {
                if edges.is_empty() {
                    return Err("RouteFeature must have at least one edge");
                }

                for edge in edges {
                    edge.validate()?;
                }

                Ok(())
            }
            Feature::PolyFeature { incl, excl } => {
                if incl.len() < 3 {
                    return Err("PolyFeature must have at least three points");
                }

                if incl.iter().any(|[lon, lat]| {
                    !(-180.0..=180.0).contains(lon)
                        || !(-90.0..=90.0).contains(lat)
                }) {
                    return Err("PolyFeature must have valid coordinates");
                }

                for poly in excl {
                    if poly.len() < 3 {
                        return Err(
                            "PolyFeature must have at least three points",
                        );
                    }

                    if poly.iter().any(|[lon, lat]| {
                        !(-180.0..=180.0).contains(lon)
                            || !(-90.0..=90.0).contains(lat)
                    }) {
                        return Err("PolyFeature must have valid coordinates");
                    }
                }

                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RouteEdge {
    #[serde(rename = "string")]
    String { line: Vec<[f64; 2]> },
    #[serde(rename = "snapped")]
    Snapped {
        waypoints: Vec<[f64; 2]>,
        polyline: String,
    },
}

impl RouteEdge {
    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            RouteEdge::String { line } => {
                if line.len() < 2 {
                    return Err(
                        "RouteEdge::String must have at least two points",
                    );
                }

                if line.iter().any(|[lon, lat]| {
                    !(-180.0..=180.0).contains(lon)
                        || !(-90.0..=90.0).contains(lat)
                }) {
                    return Err(
                        "RouteEdge::String must have valid coordinates",
                    );
                }

                Ok(())
            }
            RouteEdge::Snapped {
                waypoints,
                polyline,
            } => {
                if waypoints.len() < 2 {
                    return Err(
                        "RouteEdge::Snapped must have at least two waypoints",
                    );
                }

                if waypoints.iter().any(|[lon, lat]| {
                    !(-180.0..=180.0).contains(lon)
                        || !(-90.0..=90.0).contains(lat)
                }) {
                    return Err("RouteEdge::Snapped must have valid waypoints");
                }

                if polyline.is_empty() {
                    return Err(
                        "RouteEdge::Snapped must have a non-empty polyline",
                    );
                }

                polyline::decode_polyline(polyline, 6).map_err(|_| {
                    "RouteEdge::Snapped has an invalid polyline"
                })?;

                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LayerSpec {
    pub points: PointRendering,
    pub lines: LineRendering,
    pub polys: PolyRendering,
    pub effects: RenderingEffects,
}

impl LayerSpec {
    pub fn validate(&self) -> Result<(), &'static str> {
        self.points.validate()?;
        self.lines.validate()?;
        self.polys.validate()?;

        if let Some(blink) = &self.effects.blink {
            if !blink.points && !blink.lines && !blink.polys {
                return Err("At least one effect target must be enabled");
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct RenderingEffects {
    #[serde(default)]
    pub blink: Option<BlinkSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct BlinkSettings {
    pub points: bool,
    pub lines: bool,
    pub polys: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PointRendering {
    pub size: f32,
    pub color: String,
    pub opacity: i32,
    #[serde(default)]
    pub outline: Option<OutlineRendering>,
    pub pulse: Option<PulseSettings>,
}

impl PointRendering {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(0.0..=10.0).contains(&self.size) {
            return Err("Invalid point size");
        }

        if !self.color.len() == 7 {
            return Err("Invalid point color");
        }

        if !self.color.starts_with('#') {
            return Err("Invalid point color");
        }

        if !self.color.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid point color");
        }

        if !(0..=1).contains(&self.opacity) {
            return Err("Invalid point opacity");
        }

        if let Some(outline) = &self.outline {
            outline.validate()?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LineRendering {
    pub size: f32,
    pub color: String,
    pub opacity: f32,
    #[serde(default)]
    pub dash_array: Option<Vec<i32>>,
    #[serde(default)]
    pub outline: Option<OutlineRendering>,
}

impl LineRendering {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(0.0..=10.0).contains(&self.size) {
            return Err("Invalid line size");
        }

        if !self.color.starts_with('#') {
            return Err("Invalid line color");
        }

        if !self.color.starts_with('#') {
            return Err("Invalid line color");
        }

        if !self.color.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid line color");
        }

        if !(0.0..=1.0).contains(&self.opacity) {
            return Err("Invalid line opacity");
        }

        if let Some(outline) = &self.outline {
            outline.validate()?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct PolyRendering {
    pub color: String,
    pub opacity: f32,
    #[serde(default)]
    pub dash_array: Option<Vec<i32>>,
    #[serde(default)]
    pub outline: Option<OutlineRendering>,
}

impl PolyRendering {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.color.len() == 7 {
            return Err("Invalid poly color");
        }

        if !self.color.starts_with('#') {
            return Err("Invalid poly color");
        }

        if !self.color.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid poly color");
        }

        if !(0.0..=1.0).contains(&self.opacity) {
            return Err("Invalid poly opacity");
        }

        if let Some(outline) = &self.outline {
            outline.validate()?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct OutlineRendering {
    pub color: String,
    pub opacity: f32,
    pub size: f32,
}

impl OutlineRendering {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.color.len() == 7 {
            return Err("Invalid outline color");
        }

        if !self.color.starts_with('#') {
            return Err("Invalid outline color");
        }

        if !self.color.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid outline color");
        }

        if !(0.0..=1.0).contains(&self.opacity) {
            return Err("Invalid outline opacity");
        }

        if !(0.0..=10.0).contains(&self.size) {
            return Err("Invalid outline size");
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct PulseSettings {
    pub expansion: f64,
    pub speed: f64,
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
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
            ContentBlock::Map(map) => map.validate(),
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
