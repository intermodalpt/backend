import json
import os
import re
import sqlite3
from itertools import zip_longest

import requests
import ftfy

from consts import DB_PATH
from entities import Subroute, Stop, Route, Departure
from schedules import CALENDAR_MAPPING
from utils import get_timestamp


def extract_subroute(name, file):
    if os.path.isfile(f'./data/schedules/{file}'):
        with open(f'./data/schedules/{file}', 'r') as f:
            subroute_info = json.load(f)
    else:
        subroute_info = requests.get(f"https://www.carrismetropolitana.pt/images/horarios/{file}")

        with open(f'./data/schedules/{file}', 'w') as f:
            f.write(subroute_info.text)

        if subroute_info.status_code != 200:
            print(f"Failed to get {file}")
            return None

        subroute_info = json.loads(subroute_info.text)

    if len(subroute_info) == 0 or len(subroute_info[1]) == 0:
        raise Exception("Dumbass bus without stops or departures")

    stops = [
        Stop(ftfy.fix_text(stop_name).strip()) if stop_name else Stop(f"Misterio_{file.split('_')[0]}")
        for stop_name, _stop_departures in subroute_info
    ]

    if len({len(stop[1]) for stop in subroute_info}) != 1:
        # This route doesn't have the same amount of passes through each stop
        # We call it a mess and quit
        print(f"Subroute variant {file} has an inconsistent stop pass count")
        return Subroute(name, stops, [], [])

    # One entry per departure from the first stop (index 0)
    departures = [[] for _ in range(len(subroute_info[0][1]))]
    for _stop_name, stop_departures in subroute_info:
        for departure_index, departure in enumerate(stop_departures):
            departures[departure_index].append((get_timestamp(departure[0]), departure[1]))

    # Validate whether the calendar changes as the bus goes (eg. a daily suddenly becomes a weekend bus)
    calendar_consistent = all([
        all([
            # stop[1] is the calendar identifier
            next_stop[1] == prev_stop[1]
            for prev_stop, next_stop in zip(departure, departure[1:])])
        for departure in departures])

    if not calendar_consistent:
        print(f"Subroute variant {file} changed calendars along the way")

    # Drop routes without time info
    initial_departure_count = len(departures)
    departures = list(filter(lambda departure: departure[0][0] is not None, departures))
    if (lost_departures := initial_departure_count - len(departures)) > 0:
        print(f"Subroute {file} lost {lost_departures} departures due to unfilled times")

    calendars = [int(departure[0][1]) for departure in departures]

    # Drop calendars from departures
    departures = [[time for time, _calendar in departure] for departure in departures]

    # The timestamp one gets for post-midnight buses
    fake_midnight_ts = get_timestamp('1900')

    # Minutes between every pair of stops
    diffs = [0 if next_stop_ts == fake_midnight_ts else next_stop_ts - prev_stop_ts
             for prev_stop_ts, next_stop_ts in zip(departures[0], departures[0][1:])]

    # Validate diffs for every other row (route timings)
    # diffs_consistent = all([
    #     all([
    #         # a stop is a tuple of (time, calendar)
    #         next_stop_ts - prev_stop_ts == diffs[diff_index] or next_stop_ts == fake_midnight_ts
    #         for diff_index, (prev_stop_ts, next_stop_ts) in enumerate(zip(departure, departure[1:]))])
    #     for departure in departures[1:]])
    #
    # if not diffs_consistent:
    #     print(f"Subroute {file} schedule diffs inconsistent")

    first_stop_departures = [departure[0] for departure in departures]

    departures = []
    for time, calendar in zip(first_stop_departures, calendars):
        for calendar in CALENDAR_MAPPING[calendar]:
            departures.append(Departure(time, calendar, stop=stops[0]))

    return Subroute(name, stops, diffs, departures)


def load_cmet_data():
    if os.path.isfile(f'./data/scripts.js'):
        with open(f'./data/scripts.js', 'r') as f:
            root_script = str(f.read())
    else:
        response = requests.get('https://www.carrismetropolitana.pt/js/scripts.js')

        with open(f'./data/scripts.js', 'w') as f:
            f.write(response.text)

        if response.status_code != 200:
            print(f"Failed to get scripts.js")
            return None

        root_script = response.text

    script_subroutes_file_exp = re.compile('"(?P<subroute_name>[^"]*)":\s*"(?P<file>\d{4}_\d_\d.json)"')
    script_pdf_file_exp = re.compile('\"(?P<file>[\w\d\-_]+.pdf)\"')
    script_line_types_exp = re.compile(
        '[{\s,](?P<line_number>\d{4})\s*:\s*\[\"(?P<line_name>[^\"]*)\",\s*\"(?P<route_type>[^\"])*\"\]')

    stops = set()
    routes = {}
    route_names = {}
    route_types = {}

    for pdf_schedule in script_pdf_file_exp.findall(root_script):
        file_name = pdf_schedule
        if not os.path.isfile(f'./data/pdf-schedules/{file_name}'):
            response = requests.get(f'https://www.carrismetropolitana.pt/images/horarios_pdf/{file_name}')

            with open(f'./data/pdf-schedules/{file_name}', 'wb') as f:
                f.write(response.content)

    for route in script_line_types_exp.findall(root_script):
        number, name, route_type = route
        number = int(number)
        route_names[number] = ftfy.fix_text(name).strip()
        route_types[number] = ftfy.fix_text(route_type).strip()

    for match_ in script_subroutes_file_exp.finditer(root_script):
        subroute_name, file = match_.groups()
        subroute_name = ftfy.fix_text(subroute_name).strip()
        route_number = int(file.split("_")[0])
        route_name = route_names[route_number]
        route_type = route_types[route_number]

        route = routes.setdefault(route_number, Route(number=route_number, name=route_name, rtype=route_type))

        route.subroutes.append(extract_subroute(subroute_name, file))

        for subroute in route.subroutes:
            for stop in subroute.stops:
                stops.add(stop)

        # routes.append(route)

    return stops, routes.values()


def save_cmet_data(cmet_data):
    stops, routes = cmet_data

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()

    def upsert_stops(stops):
        for stop in stops:
            res = cur.execute("SELECT id FROM Stops WHERE short_name=? AND source='cmet'", (stop.name,))
            db_stop = cur.fetchone()

            if db_stop is None:
                res = cur.execute("INSERT INTO Stops(short_name, source)  VALUES (?, 'cmet')", (stop.name,))
                stop.id = res.lastrowid
            else:
                stop.id = db_stop[0]

        conn.commit()

    # def remap_stops(stops):
    #     mapping = load_stop_mapping()
    #
    #     for stop in stops:
    #         if stop.short_name in mapping:
    #             destination_stops = mapping[stop.short_name]
    #
    #             replacements = []
    #             for destination in destination_stops:
    #                 if len(destination) != 2:
    #                     continue
    #                 dst_short_name, dst_company = destination
    #                 res = cur.execute("SELECT id, name, lon, lat, external_id, succeeded_by "
    #                                   "FROM Stops "
    #                                   "WHERE short_name=? AND source=?", (dst_short_name, dst_company))
    #                 for row in cur.fetchall():
    #                     id, name, lon, lat, external_id, succeeded_by = row
    #
    #                     replacements.append(
    #                         Stop(
    #                             id=id,
    #                             name=name,
    #                             short_name=dst_short_name,
    #                             lon=lon,
    #                             lat=lat,
    #                             external_id=external_id,
    #                             source=dst_company,
    #                             succeeded_by=succeeded_by,
    #                         ))
    #
    #             if len(replacements) == 0:
    #                 continue
    #
    #             stops[stop] = replacements

    # upsert_stops(stops)

    # remap_stops(stops)

    def upsert_route(route):
        cur = conn.cursor()

        code = str(route.number)
        circular = route.circular

        res = cur.execute("SELECT id FROM Routes WHERE code=?", (code,))
        db_route = cur.fetchone()

        if db_route is None:
            res = cur.execute("INSERT INTO Routes(code, name, operator) VALUES (?, ?, 1) ON CONFLICT DO NOTHING;",
                              (code, route.name))
            route.id = res.lastrowid
        else:
            route.id = db_route[0]

    def upsert_subroutes(route, subroutes):
        cur = conn.cursor()

        res = cur.execute("SELECT id, flag FROM Subroutes WHERE route=?", (route.id,))
        db_subroutes = cur.fetchall()

        subroute_flags = {subroute[1]: subroute[0] for subroute in db_subroutes}

        for subroute in subroutes:
            if subroute.name in subroute_flags:
                subroute.id = subroute_flags.pop(subroute.name)
            else:
                sql = '''
                INSERT INTO Subroutes(route, flag, circular)
                VALUES(?, ?, ?)'''
                res = cur.execute(sql, (route.id, subroute.name, 1 if 'circ' in subroute.name.lower() else 0))
                subroute.id = res.lastrowid

        sql = f"DELETE FROM Subroutes WHERE id IN ({','.join(['?'] * len(subroute_flags))})"
        res = cur.execute(sql, list(subroute_flags.keys()))

    for route in routes:
        upsert_route(route)

        upsert_subroutes(route, route.subroutes)
        for subroute in route.subroutes:
            if len(subroute.stops) == 0:
                print(f"{route.number} has a subroute without stops")
                continue

            upsert_stops(subroute.stops)
            for index, (stop, diff) in enumerate(zip_longest(subroute.stops, subroute.diffs)):
                sql = '''
                SELECT Stops.id, Stops.source FROM SubrouteStops
                JOIN Stops ON SubrouteStops.stop = Stops.id
                WHERE SubrouteStops.subroute = ? AND SubrouteStops.idx = ?'''
                res = cur.execute(sql, (subroute.id, index))

                existing = res.fetchone()

                if existing is not None:
                    db_stop_id, db_stop_source = existing
                    if stop.id == db_stop_id:
                        continue

                    if db_stop_source != 'cmet':
                        # Maaaaybe not the best handling
                        continue

                    # TODO diffs

                    sql = '''
                    UPDATE SubrouteStops SET SubrouteStops.stop = ?
                    WHERE SubrouteStops.subroute = ? AND SubrouteStops.idx = ?'''
                    res = cur.execute(sql, (stop.id, subroute.id, index))
                    if res.rowcount != 1:
                        print("Huh?")

                    continue

                sql = '''
                INSERT INTO SubrouteStops(subroute, stop, idx, time_to_next)
                VALUES(?, ?, ?, ?)'''
                res = cur.execute(sql, (subroute.id, stop.id, index, diff))

            sql = '''
            DELETE FROM SubrouteStops WHERE subroute=? AND idx>=?'''
            res = cur.execute(sql, (subroute.id, len(subroute.stops)))

            departures = []
            [departures.append(departure) for departure in subroute.departures if departure not in departures]

            if (lost_departures := len(subroute.departures) - len(departures)) > 0:
                print(f"Lost {lost_departures} departures from {route.number}")

            for departure in departures:
                sql = '''INSERT INTO Departures(subroute, time, calendar) VALUES(?, ?, ?) ON CONFLICT DO NOTHING;'''
                res = cur.execute(sql, (subroute.id, departure.time, departure.calendar.export()))

    conn.commit()
