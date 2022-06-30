import datetime
import json

HOLIDAYS = (
    (1, 1),
    (4, 15),
    (4, 17),
    (4, 17),
    (4, 25),
    (6, 10),
    (6, 16),
    (8, 15),
    (10, 5),
    (11, 1),
    (12, 1),
    (12, 8),
    (12, 25)
)
SUMMER = ((23, 6), (23, 9))

# Made up FIXME
SCHOOL_PERIODS = [
    # From   To  (month, day)
    ((1, 5), (3, 20)),
    ((3, 27), (6, 10)),
    ((9, 20), (12, 15)),
]

HOLIDAY = 'Holiday'
SUMMER = 'Summer'
RANGE = 'Range'
SCHOOL = 'School'
NTH = 'Nth'

EVERY_DAY = [0, 1, 2, 3, 4, 5, 6]
WEEKDAYS = [0, 1, 2, 3, 4]
WEEKEND = [5, 6]
MONDAYS = [0]
TUESDAYS = [1]
WEDNESDAYS = [2]
THURSDAYS = [3]
FRIDAYS = [4]
SATURDAYS = [5]
SUNDAYS = [6]

WEEKDAY_NAMES = {
    0: 'Segundas',
    1: 'Terças',
    2: 'Quartas',
    3: 'Quintas',
    4: 'Sextas',
    5: 'Sábados',
    6: 'Domingos'
}


def within_dates(date: (int, int), start: (int, int), end: (int, int)):
    [[from_month, from_day], [to_month, to_day]] = start, end
    month, day = date

    if month < from_month or (month == from_month and day < from_day):
        return False

    return not (month > to_month or (month == to_month and day > to_day))


class Calendar:
    def __init__(self, weekdays):
        self.lower_range = None
        self.upper_range = None
        self.weekdays = weekdays
        self.quirks = {
            'only_if': [],
            'also_if': [],
            'except_if': [],
        }

    def set_range(self, from_day, from_month, to_day, to_month):
        self.quirks['only_if'].append(
            {'condition': 'range',
             'start': [from_month, from_day],
             'end': [to_month, to_day]})
        return self

    def only_if(self, condition, **kwargs):
        self.quirks['only_if'].append({'condition': condition, **kwargs})
        return self

    def also_if(self, condition, **kwargs):
        self.quirks['also_if'].append({'condition': condition, **kwargs})
        return self

    def except_if(self, condition, **kwargs):
        self.quirks['except_if'].append({'condition': condition, **kwargs})
        return self

    def export(self):
        return json.dumps({
            'weekdays': self.weekdays,
            **self.quirks
        },
            separators=(',', ':'))

    def happens_in(self, date):
        # Not very optimized but shouldn't matter as this is a run once & cache function
        month, day, weekday = date.month, date.day, date.weekday()
        # This format is convenient, we can repurpose the var
        date = (month, day)
        is_holiday = date in HOLIDAYS
        is_summer = within_dates(date, *SUMMER)
        is_school = any(lambda period: within_dates(date, *period), SCHOOL_PERIODS)

        if (date_range := self.quirks.get('range')):
            start, end = date_range

            if not within_dates((month, day), start, end):
                return False

        if (conditions := self.quirks.get('only_if')):
            for condition in conditions:
                key = condition['condition']

                if key == HOLIDAY:
                    if not is_holiday:
                        return False
                elif key == SUMMER:
                    if not is_summer:
                        return False
                elif key == SCHOOL:
                    if not is_school:
                        return False
                elif key == NTH:
                    if condition['nth'] != (day % 7) + 1:
                        return False
                else:
                    raise Exception("Something wrong is not right")

        if (conditions := self.quirks.get('except_if')):
            for condition in conditions:
                key = condition['condition']

                if key == HOLIDAY:
                    if is_holiday:
                        return False
                elif key == SUMMER:
                    if is_summer:
                        return False
                elif key == SCHOOL:
                    if is_school:
                        return False
                elif key == NTH:
                    if condition['nth'] == (day % 7) + 1:
                        return False
                else:
                    raise Exception("Something wrong is not right")

        if (conditions := self.quirks.get('also_if')):
            for condition in conditions:
                key = condition['condition']

                if key == HOLIDAY:
                    if is_holiday:
                        return True
                elif key == SUMMER:
                    if is_summer:
                        return True
                elif key == SCHOOL:
                    if is_school:
                        return True
                elif key == NTH:
                    if condition['nth'] == (day % 7) + 1:
                        return True
                else:
                    raise Exception("Something wrong is not right")

        if weekday not in self.weekdays:
            return False

        return True

    def __repr__(self):
        if self.weekdays == EVERY_DAY:
            named_weekdays = "Todos os dias"
        elif self.weekdays == WEEKDAYS:
            named_weekdays = "Dias de semana"
        elif self.weekdays == WEEKEND:
            named_weekdays = "Fins de semana"
        else:
            weekdays = self.weekdays.copy()

            # Working days in our list of weekdays
            if all([item in weekdays for item in WEEKDAYS]):
                named_weekdays = ['Dias úteis']
                weekdays = [weekday for weekday in weekdays if weekday not in WEEKDAYS]
            else:
                named_weekdays = []

            named_weekdays = named_weekdays + list(map(lambda weekday: WEEKDAY_NAMES[weekday], weekdays))
            if len(named_weekdays) == 1:
                named_weekdays = named_weekdays[0]
            else:
                named_weekdays = ", ".join(named_weekdays[0:-1]) + " e " + named_weekdays[-1]

        named_conditions = []
        for condition_type, condition_values in self.quirks.items():
            if len(condition_values) == 0:
                continue

            if condition_type == 'only_if':
                named_condition = "que sejam"
            elif condition_type == 'except_if':
                named_condition = "excepto"
            elif condition_type == 'also_if':
                named_condition = "ou"
            elif condition_type == 'range':
                named_conditions.append(f"entre {condition_values[0][1]}/{condition_values[0][0]} e "
                                        f"{condition_values[1][1]}/{condition_values[1][0]}")
                continue
            else:
                named_condition = "???"

            named_quirks = []
            for condition_value in condition_values:
                quirk_type = condition_value['condition']
                if quirk_type == HOLIDAY:
                    named_quirks.append("feriados")
                elif quirk_type == SUMMER:
                    named_quirks.append("verão")
                elif quirk_type == SCHOOL:
                    named_quirks.append("período escolar")
                elif quirk_type == NTH:
                    named_quirks.append(f"{condition_value['nth'] + 1}º do mês")
                else:
                    raise Exception("Something wrong is not right")

            if len(named_quirks) == 1:
                named_quirks = named_quirks[0]
            else:
                named_quirks = ", ".join(named_quirks[0:-1]) + " e " + named_quirks[-1]

            named_conditions.append(f"{named_condition} {named_quirks}")

        if len(named_conditions) == 0:
            return f"{named_weekdays}"
        elif len(named_conditions) == 1:
            named_conditions = named_conditions[0]
        else:
            named_conditions = " ".join(named_conditions)

        result = f"{named_weekdays} {named_conditions}".strip()

        if result == 'Dias de semana excepto feriados':
            return 'Dias úteis'
        if result == 'Dias de semana que sejam período escolar':
            return 'Dias úteis de período escolar'

        return result


CALENDAR_MAPPING = {
    # "Diário"
    1: [Calendar(EVERY_DAY)],
    # "Dias úteis e domingos/feriados todo o ano"
    2: [Calendar(WEEKDAYS + SUNDAYS).also_if(HOLIDAY)],
    # "Dias úteis e Sábados todo o ano"
    3: [Calendar(WEEKDAYS + SATURDAYS).except_if(HOLIDAY)],
    # "Dias úteis todo o ano"
    4: [Calendar(WEEKDAYS).except_if(HOLIDAY)],
    # "Domingos/feriados todo o ano"
    5: [Calendar(WEEKEND)],
    # "Sábados (exceto feriados) todo o ano"
    7: [Calendar(SATURDAYS).except_if(HOLIDAY)],
    # "Sábados, domingos/feriados todo o ano"
    8: [Calendar(WEEKEND).also_if(HOLIDAY)],
    # "Sextas-feiras e Sábados (exceto feriados) todo o ano"
    11: [Calendar(FRIDAYS + SATURDAYS).except_if(HOLIDAY)],
    # "Dias úteis de período escolar"
    15: [Calendar(WEEKDAYS).only_if(SCHOOL)],
    # "Quartas, quintas e sextas-feiras (exceto feriados) de período escolar"
    20: [Calendar(WEDNESDAYS + THURSDAYS + FRIDAYS).only_if(SCHOOL).except_if(HOLIDAY)],
    # "Quartas-feiras (exceto feriados) de período escolar"
    21: [Calendar(WEDNESDAYS).only_if(SCHOOL).except_if(HOLIDAY)],
    # "Segundas e terças-feiras (exceto feriados) de período escolar"
    25: [Calendar(MONDAYS + TUESDAYS).only_if(SCHOOL).except_if(HOLIDAY)],
    # "Segundas, terças, quintas e sextas-feiras (exceto feriados) de período escolar"
    28: [Calendar(MONDAYS + TUESDAYS + THURSDAYS + FRIDAYS).only_if(SCHOOL).except_if(HOLIDAY)],
    # "Sextas-feiras (exceto feriados) de período escolar",
    30: [Calendar(FRIDAYS).only_if(SCHOOL).except_if(HOLIDAY)],
    # "Dias úteis exceto verão"
    36: [Calendar(EVERY_DAY).except_if(SUMMER)],
    # "Sábados (exceto feriados) exceto verão"
    38: [Calendar(SATURDAYS).except_if(SUMMER).except_if(HOLIDAY)],
    # "Sábados, domingos/feriados exceto verão"
    39: [Calendar(WEEKEND).also_if(HOLIDAY).except_if(SUMMER)],
    # "Todos os dias exceto verão"
    40: [Calendar(EVERY_DAY).except_if(SUMMER)],
    # "Dias úteis de férias escolares e verão"
    41: [Calendar(WEEKDAYS).except_if(SCHOOL).only_if(SUMMER)],  # MAYBE WRONG
    # "Dias úteis de férias escolares exceto verão"
    50: [Calendar(WEEKDAYS).except_if(SCHOOL).except_if(SUMMER)],
    # "Dias úteis de verão"
    51: [Calendar(WEEKDAYS).only_if(SUMMER)],
    # "Sábados, domingos/feriados de verão"
    54: [Calendar(WEEKEND).also_if(HOLIDAY).only_if(SUMMER)],
    # "Todos os dias de verão"
    56: [Calendar(EVERY_DAY).only_if(SUMMER)],
    # "Dias úteis de férias escolares e verão e Sábados (exceto feriados) todo o ano"
    60: [
        Calendar(WEEKDAYS).only_if(SCHOOL).except_if(HOLIDAY),
        Calendar(WEEKDAYS).only_if(SUMMER).except_if(HOLIDAY),
        Calendar(SUNDAYS).except_if(HOLIDAY)
    ],
    # "Dias úteis de período escolar e Sábados (exceto feriados) de férias escolares e verão",
    62: [
        Calendar(WEEKDAYS).only_if(SCHOOL),
        Calendar(WEEKDAYS).only_if(SUMMER),
        Calendar(SATURDAYS).except_if(HOLIDAY)
    ],
    # "Dias úteis de período escolar e Sábados (exceto feriados) todo o ano",
    63: [
        Calendar(WEEKDAYS).only_if(SCHOOL),
        Calendar(SATURDAYS).except_if(HOLIDAY)
    ],
    # "Dias úteis de período escolar e segundas-feiras (exceto feriados) de verão"
    66: [
        Calendar(WEEKDAYS).only_if(SCHOOL),
        Calendar(MONDAYS).only_if(SUMMER).except_if(HOLIDAY)
    ],
    # "Segundo domingo de cada mês"
    69: [Calendar(SUNDAYS).except_if(NTH, nth=2)],
    # "Dias úteis todo o ano e Sábados (exceto feriados) de férias escolares e verão"
    77: [
        Calendar(WEEKDAYS).except_if(HOLIDAY),
        Calendar(SATURDAYS).except_if(HOLIDAY).only_if(SUMMER),
        Calendar(SATURDAYS).except_if(HOLIDAY).except_if(SCHOOL)
    ],
    # "Domingos exceto primeiro de cada mês todo o ano"
    87: [Calendar(SUNDAYS).except_if(NTH, nth=1)],
    # "Primeiro domingo de cada mês todo o ano"
    97: [Calendar(SUNDAYS).only_if(NTH, nth=1)],
    # "Sábados (exceto feriados) todo o ano e dias úteis de período escolar",
    100: [
        Calendar(SATURDAYS).except_if(HOLIDAY),
        Calendar(WEEKDAYS).only_if(SCHOOL).except_if(HOLIDAY)
    ],
    # "Sábados (exceto feriados) todo o ano e dias úteis de verão",
    102: [
        Calendar(SATURDAYS).except_if(HOLIDAY),
        Calendar(WEEKDAYS).only_if(SUMMER).except_if(HOLIDAY)
    ],
    # "Domingos/feriados todo o ano e dias úteis de férias escolares e verão",
    109: [
        Calendar(SUNDAYS).also_if(HOLIDAY),
        Calendar(WEEKDAYS).only_if(SUMMER),
        Calendar(WEEKDAYS).except_if(SCHOOL),
    ],
    # "Sábados, domingos/feriados todo o ano e dias úteis de verão",
    111: [
        Calendar(WEEKEND).also_if(HOLIDAY),
        Calendar(WEEKDAYS).only_if(SUMMER),
    ],
    # "Sábados, domingos/feriados todo o ano e dias úteis exceto verão",
    112: [
        Calendar(WEEKEND).also_if(HOLIDAY),
        Calendar(WEEKDAYS).also_if(HOLIDAY).except_if(SUMMER),
    ],
    # "Todos os dias de férias escolares e verão domingos/feriados de período escolar",
    113: [
        Calendar(EVERY_DAY).except_if(SCHOOL),
        Calendar(EVERY_DAY).only_if(SUMMER),
        Calendar(SUNDAYS).also_if(HOLIDAY).only_if(SCHOOL)
    ],
    # "Todos os dias de período escolar e Sábados, domingos/feriados de férias escolares e verão",
    115: [
        Calendar(EVERY_DAY).only_if(SCHOOL),
        Calendar(WEEKEND).also_if(HOLIDAY).except_if(SCHOOL),
        Calendar(WEEKEND).also_if(HOLIDAY).only_if(SUMMER)
    ],
    # "Todos os dias entre 10 de junho e 15 de setembro"
    118: [Calendar(WEEKDAYS).set_range(from_day=10, from_month=7, to_day=15, to_month=9)],
    # "Sábados, domingos/feriados entre 10 de junho e 15 de setembro e dias úteis entre 4 de julho e 15 de setembro"
    119: [
        Calendar(WEEKEND).also_if(HOLIDAY).set_range(from_day=10, from_month=6, to_day=15, to_month=9),
        Calendar(WEEKDAYS).set_range(from_day=4, from_month=7, to_day=15, to_month=9)
    ],
    # "Sábados, domingos/feriados entre 10 de junho e 15 de setembro e dias úteis de agosto"
    120: [
        Calendar(WEEKEND).also_if(HOLIDAY).set_range(from_day=10, from_month=6, to_day=15, to_month=9),
        Calendar(WEEKDAYS).set_range(from_day=1, from_month=7, to_day=31, to_month=8)
    ],
    # "Sábados, domingos/feriados entre 10 de junho e 15 de setembro e dias úteis entre 1 de julho e 15 de setembro"
    121: [
        Calendar(WEEKEND).also_if(HOLIDAY).set_range(from_day=10, from_month=6, to_day=15, to_month=9),
        Calendar(WEEKDAYS).set_range(from_day=1, from_month=7, to_day=15, to_month=9)
    ]
}
