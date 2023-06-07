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

use crate::{stops, Error, Stop};

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
            external_id: node.id.to_string(),
            refs: vec![],
            notes: None,
            a11y: stops::models::A11yMeta::default(),
            updater: -1,
            update_date: Local::now().to_string(),
            tags: vec![],
            verification_level: 0,
            service_check_date: None,
            infrastructure_check_date: None,
        };

        for tag in node.tags {
            match tag.k.as_str() {
                "name" => res.osm_name = Some(tag.v),
                "official_name" => res.official_name = Some(tag.v),
                "ref" => {
                    res.refs = tag
                        .v
                        .split(';')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>()
                }
                _ => {}
            }
        }

        res
    }
}

pub(crate) async fn import(db_pool: &PgPool) -> Result<(usize, usize), Error> {
    let mut new_stops = vec![];
    let mut updated_stops = vec![];

    let stops = stops::sql::fetch_stops(db_pool, false).await?;

    let stop_index = stops
        .into_iter()
        .map(|stop| (stop.external_id.clone(), stop))
        .collect::<HashMap<String, Stop>>();

    let mut osm_stop_ids = vec![];
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
            osm_stop_ids.push(osm_stop.external_id.clone());
            if let Some(stop) = stop_index.get(&osm_stop.external_id) {
                osm_stop.id = stop.id;
                if (stop.lat.unwrap() - osm_stop.lat.unwrap()).abs()
                    > FLOAT_TOLERANCE
                    || (stop.lon.unwrap() - osm_stop.lon.unwrap()).abs()
                        > FLOAT_TOLERANCE
                    || stop.osm_name != osm_stop.osm_name
                    || (stop.official_name.is_none()
                        && stop.official_name != osm_stop.official_name)
                    || (stop.refs != osm_stop.refs)
                {
                    // Prevent OSM from overriding some of the meta fields
                    if stop.official_name.is_some()
                        && stop.official_name != osm_stop.official_name
                    {
                        osm_stop.official_name = stop.official_name.clone();
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
    tag_missing_stops(db_pool, osm_stop_ids).await?;
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
INSERT INTO Stops(name, osm_name, official_name, lon, lat, source, external_id)
VALUES ($1, $2, $3, $4, $5, $6, $7)
    "#,
            stop.name,
            stop.osm_name,
            stop.official_name,
            stop.lon,
            stop.lat,
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
SET official_name=$1, osm_name=$2, lon=$3, lat=$4, refs=$5
WHERE id=$6 AND external_id=$7
    "#,
            stop.official_name,
            stop.osm_name,
            stop.lon,
            stop.lat,
            &stop.refs,
            stop.id,
            stop.external_id,
        )
        .execute(db_pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }
    Ok(())
}

async fn tag_missing_stops(
    db_pool: &PgPool,
    osm_ids: Vec<String>,
) -> Result<(), Error> {
    let db_ids: Vec<String> =
        sqlx::query!("SELECT external_id FROM Stops WHERE source='osm'")
            .fetch_all(db_pool)
            .await
            .map_err(|err| Error::DatabaseExecution(err.to_string()))?
            .into_iter()
            .map(|s| s.external_id)
            .collect();

    // Check which IDs in the db have disappeared from osm
    let missing_ids: Vec<String> = db_ids
        .iter()
        .filter(|id| !osm_ids.contains(id))
        .map(|id| id.clone())
        .collect();

    for missing_id in missing_ids {
        let _res = sqlx::query!(
            r#"
    UPDATE Stops
    SET deleted_upstream=true
    WHERE external_id=$1
        "#,
            missing_id
        )
        .execute(db_pool)
        .await
        .map_err(|err| Error::DatabaseExecution(err.to_string()))?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::XmlOsm;

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
