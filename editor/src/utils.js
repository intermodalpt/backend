export function calc_route_multipoly(stops, route_stops) {
  let segments = [];

  let current_segment = [];
  for (let i = 0; i < route_stops.length - 1; i++) {
    let firstStopId = route_stops[i];
    let secondStopId = route_stops[i + 1];
    if (stops[firstStopId].lon && stops[secondStopId].lon) {
      if (current_segment.length === 0) {
        current_segment.push([stops[firstStopId].lat, stops[firstStopId].lon]);
      }
      current_segment.push([stops[secondStopId].lat, stops[secondStopId].lon]);
    } else {
      if (current_segment.length !== 0) {
        segments.push(current_segment);
        current_segment = [];
      }
    }
  }

  if (current_segment.length !== 0) {
    segments.push(current_segment);
  }

  return segments;
}


function randomInteger(max) {
  return Math.floor(Math.random() * (max + 1));
}

function randomRgbColor() {
  let r = randomInteger(255);
  let g = randomInteger(255);
  let b = randomInteger(255);
  return [r, g, b];
}

export function randomHexColor() {
  let [r, g, b] = randomRgbColor();

  let hr = r.toString(16).padStart(2, '0');
  let hg = g.toString(16).padStart(2, '0');
  let hb = b.toString(16).padStart(2, '0');

  return "#" + hr + hg + hb;
}


export function weekdayName(weekday) {
  switch (weekday) {
    case 0:
      return "Mon";
    case 1:
      return "Tue";
    case 2:
      return "Wed";
    case 3:
      return "Thu";
    case 4:
      return "Fri";
    case 5:
      return "Sat";
    case 6:
      return "Sun";
    default:
      return "???";
  }
}


function conditionName(condition) {
  switch (condition.condition) {
    case "Holiday":
      return "holidays";
    case "Summer":
      return "summer";
    case "School":
      return "school";
    case "Nth":
      switch (condition.nth) {
        case 1:
          return "(month's) 1st";
        case 2:
          return "(month's) 2nd";
        case 3:
          return "(month's) 3rd";
        case 4:
          return "(month's) 4th";
        case 5:
          return "(month's) 5th";
        default:
          return "???";
      }
    case "Range":
      return `between ${condition.start} and ${condition.end}`;
    default:
      return "???";
  }
}

const EVERY_DAY = [0, 1, 2, 3, 4, 5, 6];
const WEEKDAYS = [0, 1, 2, 3, 4];
const WEEKEND = [5, 6];

export function calendarStr(calendar) {
  if (calendar === {weekdays: WEEKDAYS, except_if: [{condition: "Holiday"}]}) {
    return "Workdays"
  }
  if (calendar === {
    weekdays: WEEKDAYS,
    except_if: [{condition: "Holiday"}],
    only_if: [{condition: "School"}]
  }) {
    return "School workdays"
  }

  let namedWeekdays;
  if (calendar.weekdays === EVERY_DAY) {
    namedWeekdays = "Everyday";
  } else if (calendar.weekdays === WEEKDAYS) {
    namedWeekdays = "Business weekdays";
  } else if (calendar.weekdays === WEEKEND) {
    namedWeekdays = "Weekend";
  } else {
    namedWeekdays = calendar.weekdays.map((day) => weekdayName(day)).join(", ");
  }

  let conditions = [];
  if (calendar.only_if.length > 0) {
    conditions.push(
        "that are " + calendar.only_if.map((condition) => {
          return conditionName(condition)
        }).join(", ")
    );
  }
  if (calendar.also_if.length > 0) {
    conditions.push(
        "plus " + calendar.also_if.map((condition) => {
          return conditionName(condition)
        }).join(", ")
    );
  }
  if (calendar.except_if.length > 0) {
    conditions.push(
        "except " + calendar.except_if.map((condition) => {
          return conditionName(condition)
        }).join(", ")
    );
  }

  return `${namedWeekdays} ${conditions.join(", ")}`
}