import pathlib

from parser.extraction import load_cmet_data

import csv
import json
import sqlite3
from rapidfuzz import fuzz
from entities import Stop


def load_stop_mapping():
    if pathlib.Path("data/stop_mapping.json").is_file():
        with open("data/stop_mapping.json", "r") as f:
            return json.load(f)
    return {}


def save_stop_mapping(mapping):
    with open("data/stop_mapping.json", "w") as f:
        return json.dump(mapping, f, indent=2, ensure_ascii=False)


def load_db_stops():
    conn = sqlite3.connect("db.sqlite")
    cur = conn.cursor()

    stops = []
    res = cur.execute("SELECT id, name, short_name, lon, lat, external_id, source, succeeded_by FROM Stops")
    for row in cur.fetchall():
        id, name, short_name, lon, lat, external_id, source, succeeded_by = row

        stops.append(
            Stop(
                id=id,
                name=name,
                short_name=short_name,
                lon=lon,
                lat=lat,
                external_id=external_id,
                source=source,
                succeeded_by=succeeded_by,
            ))

    return stops


def load_gtfs_stops(company):
    stops = []

    with open(f"data/GTFS/{company}/stops.txt", newline='') as csvfile:
        csv_stops = csv.reader(csvfile, delimiter=',', quotechar='|')
        csv_stops.__next__()
        for row in csv_stops:
            stop_id, stop_code, name, stop_desc, lat, lon, zone_id, stop_url, location_type, parent_station = row
            stops.append(Stop(name=name, short_name=name, source=company, lon=lon, lat=lat, external_id=stop_id))

    return stops


def save_stops(stops: [Stop]):
    conn = sqlite3.connect("db.sqlite")

    def upsert_stop(stop: Stop):
        cur = conn.cursor()

        _res = cur.execute("SELECT id FROM Stops WHERE (name=? OR short_name=?) AND source=?",
                           (stop.name, stop.short_name, stop.source))
        db_stop = cur.fetchone()

        if db_stop is None:
            res = cur.execute(
                "INSERT INTO Stops(name, short_name, source, lat, lon, external_id)  "
                "VALUES (?, ?, ?, ?, ?, ?)",
                (stop.name, stop.short_name, stop.source, stop.lat, stop.lon, stop.external_id))

            conn.commit()

            stop.id = res.lastrowid
        else:
            stop.id = db_stop[0]

    for stop in stops:
        if stop.id is None:
            upsert_stop(stop)


def fix_name(name):
    name = name.replace("R ", "Rua ")
    name = name.replace("Av ", "Avenida ")
    name = name.replace("Pcta ", "Praçeta ")
    name = name.replace("Bº ", "Bairro ")
    name = name.replace("Estr ", "Estrada ")
    name = name.replace("Lgo ", "Largo ")
    name = name.replace("Mte ", "Monte ")
    name = name.replace("Mt ", "Monte ")
    name = name.replace("Qta ", "Quinta ")
    name = name.replace("En ", "Estrada Nacional ")
    name = name.replace("Em ", "Estrada Municipal ")
    name = name.replace("Esc ", "Escola ")
    name = name.replace("Cabo E ", "Cabo Espichel ")
    name = name.replace("Sto ", "Santo ")

    return name


def aided_stop_matcher(company="tst"):
    gtfs_stops = load_gtfs_stops(company)
    cmet_stops, _ = load_cmet_data()

    # mapping = Name -> (source, name)
    current_mapping = load_stop_mapping()

    gtfs_stop_names = set(map(lambda stop: stop.name, gtfs_stops))
    missing_cmet_stop_names = set(
        filter(lambda stop: stop not in current_mapping, map(lambda stop: stop.name, cmet_stops)))

    for cmet_stop_name in missing_cmet_stop_names:
        if cmet_stop_name in gtfs_stop_names:
            print(f"Exact lmatch for {cmet_stop_name}")
            missing_cmet_stop_names.remove(cmet_stop_name)
            current_mapping.setdefault(cmet_stop_name, []).append([company, cmet_stop_name])
            continue


    for missing_stop_name in missing_cmet_stop_names:
        fixed_missing_name = fix_name(missing_stop_name)
        matches = list(
            sorted(
                map(lambda gtfs_name: (
                    max(fuzz.ratio(gtfs_name, missing_stop_name), fuzz.ratio(gtfs_name, fixed_missing_name)),
                    gtfs_name,
                    company
                ), gtfs_stop_names),
                key=lambda res: res[0],
                reverse=True))[:20]
        if matches[0][0] < 85 and matches[0][0] > 88:
            continue

        print(f"Matches for {missing_stop_name}")
        for i, match_ in enumerate(matches):
            print(f"-------->{i + 1}: {match_[1]}, {match_[0]}, {match_[2]}")

        decision = input(">")

        if decision.strip() == '':
            print("Empty answer, skipping")
            continue

        if '~' in decision:
            decision = decision.lstrip('~')
            close_enough = True
        else:
            close_enough = False

        try:
            decision = list(map(lambda index: int(index) - 1, decision.split(",")))
        except ValueError:
            print("Invalid decision, skipping")
            continue

        if close_enough:
            print(f"You partially matched {missing_stop_name} with {[matches[stop] for stop in decision]}")
            current_mapping[missing_stop_name] = [("partial", matches[stop][2], matches[stop][1]) for stop in decision]
        else:
            print(f"You matched {missing_stop_name} with {[matches[stop] for stop in decision]}")
            current_mapping[missing_stop_name] = [(matches[stop][2], matches[stop][1]) for stop in decision]
        save_stop_mapping(current_mapping)
        print()


def remap_stops():
    current_mapping = load_stop_mapping()
    db_stops = load_db_stops()

    cmet_stops = {stop.short_name: stop for stop in filter(lambda stop: stop.source == 'cmet', db_stops)}
    tst_stops = {stop.short_name: stop for stop in filter(lambda stop: stop.source == 'tst', db_stops)}
    # iml_stops = {stop.name: stop for stop in filter(lambda stop: stop.source == 'iml', db_stops)}

    id_remap = dict()

    for stop_name, replacement_names in current_mapping.items():
        if stop_name in cmet_stops:
            if len(replacement_names[0]) == 3:
                continue

            if len(replacement_names) == 1:
                chosen_replacement = tst_stops.get(replacement_names[0][0])
            else:
                chosen_replacement = tst_stops.get(replacement_names[0][0], None)
                # Default for first but end up preferring something not stating "fte" (aka. opsite side of the road)
                for replacement_name in reversed(replacement_names):
                    if 'fte' not in replacement_name[0].lower():
                        chosen_replacement = tst_stops.get(replacement_name[0], None)

            if chosen_replacement is None:
                continue

            id_remap[cmet_stops[stop_name].id] = chosen_replacement.id

    conn = sqlite3.connect("db.sqlite")
    cur = conn.cursor()

    stops = []
    res = cur.execute(
        f"SELECT DISTINCT stop FROM SubrouteStops "
        f"WHERE stop IN ({','.join(['?'] * len(id_remap))})", list(id_remap.keys()))
    for stop_id, in cur.fetchall():

        res = cur.execute("UPDATE SubrouteStops SET  stop = ? WHERE stop = ?", (id_remap[stop_id], stop_id))

    conn.commit()


remap_stops()
