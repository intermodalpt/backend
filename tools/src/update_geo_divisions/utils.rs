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

use commons::models::geo::GeojsonGeometry;
use geo::{LineString, MultiPolygon, Polygon};

pub(crate) fn multipoly_from_geometry(
    geometry: GeojsonGeometry,
) -> MultiPolygon<f64> {
    match geometry {
        GeojsonGeometry::Polygon { coordinates } => {
            MultiPolygon::from(vec![poly_from_coords(coordinates)])
        }
        GeojsonGeometry::MultiPolygon { coordinates } => MultiPolygon::from(
            coordinates
                .into_iter()
                .map(poly_from_coords)
                .collect::<Vec<_>>(),
        ),
    }
}

pub(crate) fn poly_from_coords(coordinates: Vec<Vec<Vec<f64>>>) -> Polygon {
    let mut polygons = coordinates.into_iter();
    let outer_coords = polygons.next().unwrap();
    let outer_line = LineString::from(
        outer_coords
            .into_iter()
            .map(|p| (p[0], p[1]))
            .collect::<Vec<_>>(),
    );

    let inner_lines = polygons
        .map(|p| {
            LineString::from(
                p.into_iter().map(|p| (p[0], p[1])).collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    Polygon::new(outer_line, inner_lines)
}
