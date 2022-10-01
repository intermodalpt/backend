/*
    Intermodalis, transportation information aggregator
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

use crate::{middleware, Error, Stop};

use std::collections::HashMap;

use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use urlencoding::encode as urlencode;

const FLOAT_TOLERANCE: f64 = 0.000_001;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XmlOsm {
    #[serde(rename = "$value")]
    nodes: Vec<XmlNodeTypes>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum XmlNodeTypes {
    Meta(XmlMeta),
    Note(XmlNote),
    Node(XmlNode),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "meta")]
struct XmlMeta {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "note")]
struct XmlNote {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "node")]
struct XmlNode {
    id: i64,
    lon: f64,
    lat: f64,
    #[serde(rename = "$value")]
    tags: Vec<XMLTag>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "tag")]
struct XMLTag {
    k: String,
    v: String,
}

impl From<XmlNode> for Stop {
    fn from(node: XmlNode) -> Self {
        let mut res = Self {
            id: -1,
            source: "osm".to_string(),
            name: None,
            official_name: None,
            osm_name: None,
            short_name: None,
            locality: None,
            street: None,
            door: None,
            parish: None,
            lat: Some(node.lat),
            lon: Some(node.lon),
            external_id: Some(node.id.to_string()),
            succeeded_by: None,
            notes: None,
            has_crossing: None,
            has_accessibility: None,
            has_abusive_parking: None,
            has_outdated_info: None,
            is_damaged: None,
            is_vandalized: None,
            has_flag: None,
            has_schedules: None,
            has_sidewalk: None,
            has_shelter: None,
            has_bench: None,
            has_trash_can: None,
            is_illuminated: None,
            has_illuminated_path: None,
            has_visibility_from_within: None,
            has_visibility_from_area: None,
            is_visible_from_outside: None,
            updater: -1,
            update_date: Local::now().to_string(),
            tags: vec![],
        };

        for tag in node.tags {
            match tag.k.as_str() {
                "name" => res.osm_name = Some(tag.v),
                "official_name" => res.official_name = Some(tag.v),
                "shelter" => match tag.v.as_str() {
                    "yes" => res.has_shelter = Some(true),
                    "no" => res.has_shelter = Some(false),
                    _ => {}
                },
                "bench" => match tag.v.as_str() {
                    "yes" => res.has_bench = Some(true),
                    "no" => res.has_bench = Some(false),
                    _ => {}
                },
                "bin" => match tag.v.as_str() {
                    "yes" => res.has_trash_can = Some(true),
                    "no" => res.has_trash_can = Some(false),
                    _ => {}
                },
                "lit" => match tag.v.as_str() {
                    "yes" => res.is_illuminated = Some(true),
                    "no" => res.is_illuminated = Some(false),
                    _ => {}
                },
                _ => {}
            }
        }

        res
    }
}

pub(crate) async fn import(db_pool: &PgPool) -> Result<(usize, usize), Error> {
    let mut new_stops = vec![];
    let mut updated_stops = vec![];

    let stops = middleware::get_stops(db_pool).await?;

    let stop_index = stops
        .into_iter()
        .filter_map(|stop| {
            stop.external_id.clone().map(|external_id| (external_id, stop))
        })
        .collect::<HashMap<String, Stop>>();

    fetch_osm_stops()
        .await?
        .nodes
        .into_iter()
        .filter_map(|node| {
            if let XmlNodeTypes::Node(node) = node {
                Some(Stop::from(node))
            } else {
                None
            }
        })
        .for_each(|mut osm_stop| {
            if let Some(stop) =
                stop_index.get(osm_stop.external_id.as_ref().unwrap())
            {
                osm_stop.id = stop.id;
                if (stop.lat.unwrap() - osm_stop.lat.unwrap()).abs()
                    > FLOAT_TOLERANCE
                    || (stop.lon.unwrap() - osm_stop.lon.unwrap()).abs()
                        > FLOAT_TOLERANCE
                    || stop.osm_name != osm_stop.osm_name
                    || (stop.official_name.is_none()
                        && stop.official_name != osm_stop.official_name)
                    || (stop.has_shelter.is_none()
                        && stop.has_shelter != osm_stop.has_shelter)
                    || (stop.has_trash_can.is_none()
                        && stop.has_trash_can != osm_stop.has_trash_can)
                    || (stop.is_illuminated.is_none()
                        && stop.is_illuminated != osm_stop.is_illuminated)
                {
                    // Prevent OSM from overriding some of the meta fields
                    if stop.official_name.is_some()
                        && stop.official_name != osm_stop.official_name
                    {
                        osm_stop.official_name = stop.official_name.clone();
                    }
                    if stop.has_shelter.is_some()
                        && stop.has_shelter != osm_stop.has_shelter
                    {
                        osm_stop.has_shelter = stop.has_shelter;
                    }
                    if stop.has_trash_can.is_some()
                        && stop.has_trash_can != osm_stop.has_trash_can
                    {
                        osm_stop.has_trash_can = stop.has_trash_can;
                    }
                    if stop.is_illuminated.is_some()
                        && stop.is_illuminated != osm_stop.is_illuminated
                    {
                        osm_stop.is_illuminated = stop.is_illuminated;
                    }
                    updated_stops.push(osm_stop);
                }
            } else {
                new_stops.push(osm_stop);
            }
        });

    let counts = (new_stops.len(), updated_stops.len());

    update_stops(db_pool, updated_stops).await?;
    insert_stops(db_pool, new_stops).await?;
    Ok(counts)
}

async fn fetch_osm_stops() -> Result<XmlOsm, Error> {
    let query = r#"
    area[name="Lisboa"][admin_level=6];
        node["highway"="bus_stop"](area)->.a;
    area[name="Vendas Novas"][admin_level=7];
        node["highway"="bus_stop"](area)->.b;
    area[name="Setúbal"][admin_level=6];
        node["highway"="bus_stop"](area)->.c;
    (.a;.b;.c;);
    out;"#;

    let osm_query_url = format!(
        "https://overpass-api.de/api/interpreter?data={}",
        urlencode(query)
    );

    // TODO wrong errors
    let xml = reqwest::get(&osm_query_url)
        .await
        .map_err(|e| Error::Processing(e.to_string()))?
        .text()
        .await
        .map_err(|e| Error::Processing(e.to_string()))?;
    serde_xml_rs::from_str(&xml).map_err(|e| Error::Processing(e.to_string()))
}

async fn insert_stops(db_pool: &PgPool, stops: Vec<Stop>) -> Result<(), Error> {
    for stop in stops {
        let _res = sqlx::query!(
            r#"
INSERT INTO Stops(name, osm_name, official_name, lon, lat, has_shelter,
    has_bench, has_trash_can, is_illuminated, source, external_id)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    "#,
            stop.name,
            stop.osm_name,
            stop.official_name,
            stop.lon,
            stop.lat,
            stop.has_shelter,
            stop.has_bench,
            stop.has_trash_can,
            stop.is_illuminated,
            stop.source,
            stop.external_id,
        )
        .execute(db_pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }
    Ok(())
}

async fn update_stops(db_pool: &PgPool, stops: Vec<Stop>) -> Result<(), Error> {
    for stop in stops {
        let _res = sqlx::query!(
            r#"
UPDATE Stops
SET official_name=$1, osm_name=$2, lon=$3, lat=$4, has_shelter=$5, has_bench=$6,
    has_trash_can=$7, is_illuminated=$8
WHERE id=$9 AND external_id=$10
    "#,
            stop.official_name,
            stop.osm_name,
            stop.lon,
            stop.lat,
            stop.has_shelter,
            stop.has_bench,
            stop.has_trash_can,
            stop.is_illuminated,
            stop.id,
            stop.external_id,
        )
        .execute(db_pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::osm::XmlOsm;

    #[test]
    fn test_deserialization() {
        let data = r#"
<?xml version="1.0" encoding="UTF-8"?>
<osm version="0.6" generator="Overpass API 0.7.58.5 b0c4acbb">
  <note>The data included in this document is from www.openstreetmap.org. The data is made available under ODbL.</note>
  <meta osm_base="2022-08-30T10:53:17Z" areas="2022-07-20T10:48:09Z"/>
  <node id="9986914942" lat="38.6618776" lon="-9.0514656">
    <tag k="bus" v="yes"/>
    <tag k="highway" v="bus_stop"/>
    <tag k="name" v="Quinta da Várzea (Cemitério)"/>
    <tag k="network" v="Carris Metropolitana"/>
    <tag k="network:wikidata" v="Q111611112"/>
    <tag k="public_transport" v="platform"/>
    <tag k="shelter" v="no"/>
  </node>
</osm>
    "#;
        let _xml_root: XmlOsm = serde_xml_rs::from_str(data).unwrap();
    }
}
