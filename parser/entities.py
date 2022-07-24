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
    def __init__(self, name, short_name=None, id=None, external_id=None, source='cmet', lat=None, lon=None,
                 succeeded_by=None):

        self.id = id
        self.external_id = external_id
        self.name = name
        self.short_name = short_name
        self.source = source
        self.lat = lat
        self.lon = lon
        self.succeeded_by = succeeded_by

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
    def __init__(self, name, stops, diffs, departures):
        self.name = name
        self.stops = stops
        self.diffs = diffs
        self.departures = departures

    def __repr__(self):
        return f"{self.stops[0].name} - {self.stops[-1].name}"


class Route:
    def __init__(self, number, name, rtype, circular=False):
        self.number = number
        self.name = name
        self.rtype = rtype
        self.circular = circular
        self.subroutes = []

    def add_subroute(self, subroute):
        self.subroutes.append(subroute)

    def __repr__(self):
        return f"{self.number} - {self.name}"


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
