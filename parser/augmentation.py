import json
import sqlite3
from time import sleep
from urllib.parse import quote_plus

import openpyxl
import requests
from bs4 import BeautifulSoup

from consts import MUN_AML, DB_PATH


def update_osm_stops(districts=("Setúbal", "Lisboa")):
    nodes = []

    def collect_district_nodes(district):
        osm_query = \
            f"""
            area
              ["name"="{district}"]
              ["boundary"="administrative"];
            (
              node
                ["highway"="bus_stop"]
                (area);
            );
            out body;
            """

        osm_query_url = "https://overpass-api.de/api/interpreter?data=" + quote_plus(osm_query)

        bus_stops_resp = requests.get(osm_query_url)
        soup = BeautifulSoup(bus_stops_resp.text, 'xml')

        for node in soup.findAll('node'):
            attrs = node.attrs.copy()
            attrs['lat'] = float(attrs['lat'])
            attrs['lon'] = float(attrs['lon'])
            for child in node.findAll('tag'):
                key = child.attrs['k']
                if key in (
                        'highway', 'network:wikidata', 'departures_board', 'lit', 'bin', 'bus', 'mapillary',
                        'network:wikipedia',
                        'survey:date', 'public_transport'):
                    continue

                attrs[key] = child.attrs['v']

            nodes.append(attrs)

    for district in districts:
        sleep(5)
        collect_district_nodes(district)

    upstream_ids = {node['id'] for node in nodes}

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()

    res = cur.execute(
        "SELECT osm_name, lat, lon, external_id FROM Stops "
        "WHERE source='osm' AND external_id NOT IN "
        f"({','.join(['?'] * len(upstream_ids))});",
        (list(upstream_ids)))

    updated = 0
    inserted = 0
    deleted = 0

    for row in res.fetchall():
        name, lat, lon, external_id = row
        deleted += 1
        print(f"{external_id} - {name} ({lon};{lat}) has been deleted. Nuke?")
        ans = input()
        if ans.lower().strip() == 'y':
            res = cur.execute("DELETE FROM Stops WHERE source='osm' AND external_id=?", (external_id,))

    for node in nodes:
        osm_id, name, lat, lon = node['id'], node.get('name', None), node['lat'], node['lon']

        res = cur.execute(
            "SELECT osm_name, lat, lon FROM Stops WHERE source='osm' AND external_id = ?;",
            (osm_id,))

        existing = res.fetchone()

        if existing:
            existing_name, existing_lat, existing_lon = existing
            if existing_name == name and existing_lat == lat and existing_lon == lon:
                continue
            updated += 1
            print(f"Updated: ({osm_id}){name} ({lon} {lat})")
            res = cur.execute(
                "UPDATE Stops SET osm_name=?, lat=?, lon=? WHERE source='osm' AND external_id = ?;",
                (name, lat, lon, osm_id))

        else:
            inserted += 1
            print(f"Inserted: ({osm_id}){name} ({lon} {lat})")
            res = cur.execute(
                "INSERT INTO Stops(osm_name, source, lat, lon, external_id) "
                "VALUES (?, 'osm', ?, ?,?) "
                "ON CONFLICT DO NOTHING;",
                (name, lat, lon, osm_id))

    conn.commit()


def get_geocached_stops():
    with open('data.json', 'r') as f:
        graph = json.load(f)

    labels = [node['label'] for node in graph['nodes']]

    matches = []

    for label in labels[2:]:
        # Make things more pleasant for Nominatim
        original = label
        label = label.replace("R ", "Rua ")
        label = label.replace("Av ", "Avenida ")
        label = label.replace("Pcta ", "Praçeta ")
        label = label.replace("Bº ", "Bairro ")
        label = label.replace("Estr ", "Estrada ")
        label = label.replace("Lgo ", "Largo ")
        label = label.replace("Mte ", "Monte ")
        label = label.replace("Mt ", "Monte ")
        label = label.replace("Qta ", "Quinta ")
        label = label.replace("En ", "Estrada Nacional ")
        label = label.replace("Em ", "Estrada Municipal ")
        label = label.replace("Esc ", "Escola ")

        location = quote_plus(label)
        query = f"https://nominatim.openstreetmap.org/search.php?" \
                f"q={location}&polygon_geojson=1&viewbox=-9.31915%2C38.80118%2C-8.60161%2C38.40032&bounded=1&format=jsonv2"

        request = requests.get(query)

        matches.append((original, label, request.json()))
        sleep(5)  # Be nice to OSM


def store_parishes():
    conn = sqlite3.connect(DB_PATH)
    wookbook = openpyxl.load_workbook("freguesias-metadata.xlsx")

    # Define variable to read the active sheet:
    worksheet = wookbook.active

    # Iterate the loop to read the cell values
    cols = worksheet['C:D']
    cur = conn.cursor()
    sql = '''INSERT INTO Parishes(name, municipality)
               VALUES(?, (SELECT id from Municipalities WHERE name = ?))'''

    for mun, freg in zip(*cols):
        mun = mun.value
        if mun.upper() not in MUN_AML:
            continue

        freg = freg.value

        res = cur.execute(sql, (freg, mun))
        print(mun, end="\t\t")
        print(freg, end="\t\t")
        print('')
    conn.commit()


def store_parish_polygons():
    conn = sqlite3.connect(DB_PATH)

    with open("freguesias.geojson", 'r') as f:
        geojson = json.load(f)

    cur = conn.cursor()
    for feature in geojson['features']:
        properties = feature['properties']
        if 'official_name' not in properties:
            continue

        name = properties['name']
        res = cur.execute("UPDATE Parishes SET polygon = ? WHERE  name LIKE ?", (json.dumps(feature), '%' + name))

        official_name = properties['official_name']
        res = cur.execute("UPDATE Parishes SET polygon = ? WHERE  name LIKE ?",
                          (json.dumps(feature), '%' + official_name))

    conn.commit()
