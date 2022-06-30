import json
import os
import re
import sqlite3

import requests

from consts import DB_PATH
from entities import Subroute, Stop, Route, Departure
from schedules import CALENDAR_MAPPING
from utils import get_timestamp


def extract_subroute(file, route):
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
        Stop(stop_name) if stop_name else Stop(f"Misterio_{route.number}")
        for stop_name, _stop_departures in subroute_info
    ]

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
        print(f"Route {route.number}(variant {file}) changed calendars along the way")

    # Drop routes without time info
    initial_departure_count = len(departures)
    departures = list(filter(lambda departure: departure[0][0] is not None, departures))
    if (lost_departures := initial_departure_count - len(departures)) > 0:
        print(f"Career {route.number} lost {lost_departures} departures due to unfilled times")

    calendars = [departure[0][1] for departure in departures]

    # Drop calendars from departures
    departures = [[time for time, _calendar in departure] for departure in departures]

    # The timestamp one gets for post-midnight buses
    fake_midnight_ts = get_timestamp('1900')

    # Minutes between every pair of stops
    diffs = [0 if next_stop_ts == fake_midnight_ts else next_stop_ts - prev_stop_ts
             for prev_stop_ts, next_stop_ts in zip(departures[0], departures[0][1:])]

    # Validate diffs for every other row (route timings)
    diffs_consistent = all([
        all([
            # a stop is a tuple of (time, calendar)
            next_stop_ts - prev_stop_ts == diffs[diff_index] or next_stop_ts == fake_midnight_ts
            for diff_index, (prev_stop_ts, next_stop_ts) in enumerate(zip(departure, departure[1:]))])
        for departure in departures[1:]])

    if not diffs_consistent:
        print(f"Route {route.number} schedule diffs inconsistent")

    first_stop_departures = [departure[0] for departure in departures]

    departures = []
    for time, calendar in zip(first_stop_departures, calendars):
        for calendar in CALENDAR_MAPPING[calendar]:
            departures.append(Departure(time, calendar, stop=stops[0]))

    return Subroute(route, stops, diffs, departures)


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

    line_schedules_exp = re.compile("(?P<number>\d{4})\s*:\s*{\s*"
                                    "ida\s*:\s*(?P<dir1>\[[ \"_\w\d.,]*\])\s*,\s*"
                                    "volta\s*:\s*(?P<dir2>\[[ \"_\w\d.,]*\])\s*,\s*"
                                    "circular\s*:\s*(?P<circular>\[[\"_\w\d.]*\])\s*}")

    stops = set()
    routes = []

    for match_ in line_schedules_exp.finditer(root_script):
        line_num, dir1, dir2, circular = match_.groups()
        line_num = int(line_num)

        directed = json.loads(dir1) + json.loads(dir2)
        circular = json.loads(circular)

        if len(directed) > 0 and len(circular) > 0:
            raise Exception("Bus with linear and circular routes")

        if len(circular) > 0:
            route = Route(line_num, circular=True)
            for file in circular:
                route.subroutes.append(extract_subroute(file, route))
        else:
            route = Route(line_num)
            for file in directed:
                route.subroutes.append(extract_subroute(file, route))

        for subroute in route.subroutes:
            for stop in subroute.stops:
                stops.add(stop)

        routes.append(route)

    return stops, routes


def save_stops():
    # stops, routes = load_cmet_data()

    with open('geocaching.json', 'r') as f:
        geocaching = json.load(f)

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()

    for short_name, full_name, guesses in geocaching:

        res = cur.execute("INSERT INTO Stops(name, short_name, source) VALUES (?, ?, 'cmet') ON CONFLICT DO NOTHING;",
                          (full_name, short_name))
        original_id = res.lastrowid

        for guess in guesses:
            guess_lat = guess['lat']
            guess_lon = guess['lon']

            res = cur.execute(
                "INSERT INTO Stops(name, source, lat, lon, guess_for) VALUES (?, 'geoc', ?, ?,?) ON CONFLICT DO NOTHING;",
                (full_name, guess_lat, guess_lon, original_id))

    conn.commit()


def upsert_stops(stops):
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()

    for stop in stops:
        res = cur.execute("SELECT id FROM Stops WHERE short_name=? AND source='cmet'", (stop.name,))
        db_stop = cur.fetchone()

        if db_stop is None:
            res = cur.execute("INSERT INTO Stops(short_name, source)  VALUES (?, 'cmet')", (stop.name,))
            stop.id = res.lastrowid
        else:
            stop.id = db_stop[0]

    conn.commit()


def save_routes():
    stops, routes = load_cmet_data()

    upsert_stops(stops)

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()

    for route in routes:

        flag = str(route.number)
        circular = route.circular

        res = cur.execute("INSERT INTO Routes(flag, circular) VALUES (?, ?) ON CONFLICT DO NOTHING;",
                          (flag, circular))

        route_id = res.lastrowid

        for subroute in route.subroutes:
            if len(subroute.stops) == 0:
                print(f"{flag} has a subroute without stops")
                continue

            first_stop = subroute.stops[0]
            last_stop = subroute.stops[-1]

            sql = '''
            INSERT INTO Subroutes(route, cached_from, cached_to)
            VALUES(?, 
                (SELECT id from Stops WHERE source = 'cmet' AND short_name = ?), 
                (SELECT id from Stops WHERE source = 'cmet' AND short_name = ?))'''
            res = cur.execute(sql, (route_id, first_stop.name, last_stop.name))
            subroute_id = res.lastrowid

            for index, (stop, diff) in enumerate(zip_longest(subroute.stops, subroute.diffs)):
                sql = '''
                INSERT INTO SubrouteStops(subroute, stop, 'index', time_to_next)
                VALUES(?, (SELECT id from Stops WHERE source = 'cmet' AND short_name = ?), ?, ?)'''
                res = cur.execute(sql, (subroute_id, stop.name, index, diff))

            departures = []
            [departures.append(departure) for departure in subroute.departures if departure not in departures]

            if (lost_departures := len(subroute.departures) - len(departures)) > 0:
                print(f"Lost {lost_departures} departures from {route.number}")

            for departure in departures:
                sql = '''INSERT INTO Departures(subroute, time, calendar) VALUES(?, ?, ?)'''
                res = cur.execute(sql, (subroute_id, departure.time, departure.calendar.export()))

    conn.commit()
