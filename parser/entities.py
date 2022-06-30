import sqlite3


class Zone:
    def __init__(self, id):
        self.id = id
        self.municipalities = []

    def add_municipality(self, municipality):
        self.municipalities.append(municipality)


class Municipality:
    def __init__(self):
        pass


class Stop:
    def __init__(self, name, id=None, source='cmet', lat=None, lon=None):
        if name.strip() == '':
            print("Wtf")
        self.id = id
        self.name = name
        self.source = source
        self.lat = lat
        self.lon = lon

    def __repr__(self):
        return f"({self.id if self.id else '-'}) {self.name}"

    def __eq__(self, other):
        if self.source == 'cmet':
            return self.name == other.name
        else:
            return self.id == other.id

    def __hash__(self):
        if self.source == 'cmet':
            return self.name.__hash__()
        else:
            return self.id


class Subroute:
    def __init__(self, route, stops, diffs, departures):
        self.route = route
        self.stops = stops
        self.diffs = diffs
        self.departures = departures

        if len(diffs) != len(stops) - 1:
            raise Exception("Number of diffs does not match number of stops")

    def __repr__(self):
        return f"{self.stops[0].name} - {self.stops[-1].name}"


class Route:
    def __init__(self, number, circular=False):
        self.number = number
        self.circular = circular
        self.subroutes = []

    def add_subroute(self, subroute):
        self.subroutes.append(subroute)

    def __repr__(self):
        return str(self.number)


class Departure:
    def __init__(self, time, calendar, stop):
        self.time = time
        self.calendar = calendar
        self.stop = stop

    def __repr__(self):
        return f"{self.time // 60}:{self.time % 60 :02}"

    def __eq__(self, other):
        return self.time == other.time \
               and self.calendar == other.calendar \
               and self.stop == other.stop
