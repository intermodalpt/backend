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

use urlencoding::encode as urlencode;

use crate::models;

pub(crate) async fn fetch_osm_stops(
) -> Result<models::XmlOsm, Box<dyn std::error::Error>> {
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
    let xml = reqwest::get(&osm_query_url).await?.text().await?;
    Ok(serde_xml_rs::from_str(&xml)?)
}

#[cfg(test)]
mod test {
    use crate::models::XmlOsm;

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
