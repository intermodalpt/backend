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

use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use commons::models::stops;

#[derive(Debug, Eq, Clone, Serialize, Deserialize)]
pub struct TMLTrip {
    pub(crate) id: String,
    pub(crate) headsign: String,
    pub(crate) stops: Vec<u32>,
}

impl PartialEq for TMLTrip {
    fn eq(&self, other: &Self) -> bool {
        // self.id == other.id
        self.stops == other.stops && self.headsign == other.headsign
    }
}

impl Hash for TMLTrip {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.id.hash(state);
        self.headsign.hash(state);
        self.stops.hash(state);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TMLRoute {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) trips: Vec<TMLTrip>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct TMLStop {
    #[serde(flatten)]
    pub stop: stops::Stop,
    pub tml_id_verified: bool,
    pub tml_id: Option<String>,
    pub tml_id_source: String,
    pub deleted_upstream: bool,
    pub verified_position: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MatchVerification {
    #[serde(default)]
    pub(crate) verified: bool,
    pub(crate) source: MatchSource,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MatchSource {
    Unknown,
    Tml,
    Manual,
    OSM,
    Flags,
    H1,
}

impl fmt::Display for MatchSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchSource::Unknown => write!(f, "unknown"),
            MatchSource::Tml => write!(f, "tml"),
            MatchSource::Manual => write!(f, "manual"),
            MatchSource::OSM => write!(f, "osm"),
            MatchSource::Flags => write!(f, "flags"),
            MatchSource::H1 => write!(f, "h1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TMLRoute, TMLTrip};
    use itertools::Itertools;

    #[test]
    fn trip_dedup() {
        let route = TMLRoute {
            id: "4308_0".to_string(),
            name: "Pinhal Novo - Palmela".to_string(),
            trips: vec![
                TMLTrip {
                    id: "p3_306".to_string(),
                    headsign: "Pinhal Novo (Estação)".to_string(),
                    stops: vec![
                        130807, 130028, 130026, 130024, 130021, 130020, 130200,
                        130202, 130204, 130205, 130207, 130210, 130212, 130213,
                        130211, 130216, 130218, 130220, 130222, 130224, 130226,
                        130229,
                    ],
                },
                TMLTrip {
                    id: "p3_307".to_string(),
                    headsign: "Pinhal Novo (Estação)".to_string(),
                    stops: vec![
                        130807, 130028, 130026, 130024, 130021, 130020, 130200,
                        130202, 130204, 130205, 130207, 130210, 130212, 130213,
                        130211, 130216, 130218, 130220, 130222, 130224, 130226,
                        130229,
                    ],
                },
                TMLTrip {
                    id: "p3_308".to_string(),
                    headsign: "Pinhal Novo (Estação)".to_string(),
                    stops: vec![
                        130807, 130028, 130026, 130024, 130021, 130020, 130200,
                        130202, 130204, 130205, 130207, 130210, 130212, 130213,
                        130211, 130216, 130218, 130220, 130222, 130224, 130226,
                        130229,
                    ],
                },
                TMLTrip {
                    id: "p3_309".to_string(),
                    headsign: "Pinhal Novo (Estação)".to_string(),
                    stops: vec![
                        130807, 130028, 130026, 130024, 130021, 130020, 130200,
                        130202, 130204, 130205, 130207, 130210, 130212, 130213,
                        130211, 130216, 130218, 130220, 130222, 130224, 130226,
                        130229,
                    ],
                },
                TMLTrip {
                    id: "p3_310".to_string(),
                    headsign: "Pinhal Novo (Estação)".to_string(),
                    stops: vec![
                        130807, 130028, 130026, 130024, 130021, 130020, 130200,
                        130202, 130204, 130205, 130207, 130210, 130212, 130213,
                        130211, 130216, 130218, 130220, 130222, 130224, 130226,
                        130229,
                    ],
                },
                TMLTrip {
                    id: "p3_311".to_string(),
                    headsign: "Pinhal Novo (Estação)".to_string(),
                    stops: vec![
                        130807, 130028, 130026, 130024, 130021, 130020, 130200,
                        130202, 130204, 130205, 130207, 130210, 130212, 130213,
                        130211, 130216, 130218, 130220, 130222, 130224, 130226,
                        130229,
                    ],
                },
                TMLTrip {
                    id: "p3_312".to_string(),
                    headsign: "Palmela (Terminal)".to_string(),
                    stops: vec![
                        130230, 130227, 130225, 130223, 130221, 130219, 130217,
                        130215, 130212, 130213, 130211, 130209, 130231, 130233,
                        130203, 130201, 130199, 130019, 130022, 130023, 130025,
                        130027, 130807,
                    ],
                },
                TMLTrip {
                    id: "p3_313".to_string(),
                    headsign: "Palmela (Terminal)".to_string(),
                    stops: vec![
                        130230, 130227, 130225, 130223, 130221, 130219, 130217,
                        130215, 130212, 130213, 130211, 130209, 130231, 130233,
                        130203, 130201, 130199, 130019, 130022, 130023, 130025,
                        130027, 130807,
                    ],
                },
                TMLTrip {
                    id: "p3_314".to_string(),
                    headsign: "Palmela (Terminal)".to_string(),
                    stops: vec![
                        130230, 130227, 130225, 130223, 130221, 130219, 130217,
                        130215, 130212, 130213, 130211, 130209, 130231, 130233,
                        130203, 130201, 130199, 130019, 130022, 130023, 130025,
                        130027, 130807,
                    ],
                },
                TMLTrip {
                    id: "p3_315".to_string(),
                    headsign: "Palmela (Terminal)".to_string(),
                    stops: vec![
                        130230, 130227, 130225, 130223, 130221, 130219, 130217,
                        130215, 130212, 130213, 130211, 130209, 130231, 130233,
                        130203, 130201, 130199, 130019, 130022, 130023, 130025,
                        130027, 130807,
                    ],
                },
                TMLTrip {
                    id: "p3_316".to_string(),
                    headsign: "Palmela (Terminal)".to_string(),
                    stops: vec![
                        130230, 130227, 130225, 130223, 130221, 130219, 130217,
                        130215, 130212, 130213, 130211, 130209, 130231, 130233,
                        130203, 130201, 130199, 130019, 130022, 130023, 130025,
                        130027, 130807,
                    ],
                },
                TMLTrip {
                    id: "p3_317".to_string(),
                    headsign: "Palmela (Terminal)".to_string(),
                    stops: vec![
                        130230, 130227, 130225, 130223, 130221, 130219, 130217,
                        130215, 130212, 130213, 130211, 130209, 130231, 130233,
                        130203, 130201, 130199, 130019, 130022, 130023, 130025,
                        130027, 130807,
                    ],
                },
            ],
        };
    }

    #[test]
    fn trips_dedup() {
        assert!(
            TMLTrip {
                id: "p3_306".to_string(),
                headsign: "AAA".to_string(),
                stops: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            } == TMLTrip {
                id: "p3_307".to_string(),
                headsign: "AAA".to_string(),
                stops: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            }
        );

        let trips = vec![
            TMLTrip {
                id: "p3_306".to_string(),
                headsign: "AAA".to_string(),
                stops: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            },
            TMLTrip {
                id: "p3_307".to_string(),
                headsign: "AAA".to_string(),
                stops: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            },
        ]
        .into_iter()
        .unique()
        .collect::<Vec<_>>();
        assert_eq!(trips.len(), 1);
    }
}
