import {derived, writable} from "svelte/store";
import {api_server} from "./settings.js";
import {routes, stops} from "./cache.js";

export const mode = writable(localStorage.getItem("mode"));

export const selectedOperatorTag = writable(undefined);

export const selectedRouteId = writable(undefined);

export const selectedRoute = derived(
    [routes, selectedRouteId],
    ([$routes, $selectedRouteId]) => {

      if ($selectedRouteId === undefined) {
        return;
      }
      return $routes.find((r) => {
        return r.id === $selectedRouteId;
      });
    }
);

export const selectedRouteStops = derived(
    [selectedRouteId, stops],
    ([$selectedRouteId, $stops], set) => {
      if ($selectedRouteId) {
        fetch(`${api_server}/api/routes/${$selectedRouteId}/stops`)
            .then((r) => r.json())
            .then((data) => {
              data.forEach((sr) => sr.stops.map((stopId) => $stops[stopId]));
              set(data);
            });
      } else {
        return [];
      }
    }
);

export const selectedSubrouteId = writable(undefined);
const selectedSubroute = derived(selectedSubrouteId, $selectedSubrouteId => {
  if ($selectedRoute && $selectedSubrouteId) {
    $selectedRoute.find((sr) => {
      return sr.id === $selectedSubrouteId;
    });
  }
});


export const subrouteStops = derived(
    ([selectedRouteStops, selectedSubrouteId]),
    ([$selectedRouteStops, $selectedSubrouteId]) => {
      if ($selectedRouteStops) {
        return $selectedRouteStops.find((stops) => {
          return stops.subroute === $selectedSubrouteId;
        });
      }
    }
);

export const selectedDay = writable(new Date().toISOString().split("T")[0]);
export const schedule = derived(
    [selectedRouteId, selectedDay],
    async ([$selectedRouteId, $selectedDay], set) => {

      if ($selectedDay) {
        await fetch(`${api_server}/api/routes/${$selectedRouteId}/schedule/${$selectedDay}`)
            .catch(() => {
            })
            .then((r) => r.json())
            .catch(() => {
            })
            .then((data) => set(data));
      }
    }
);

export const scheduleBySubroute = derived(schedule, $schedule => {
  if ($schedule === undefined) {
    return;
  }
  const subroutes = {};
  for (let departure of $schedule) {
    if (subroutes[departure.subroute] === undefined) {
      subroutes[departure.subroute] = {};
    }

    let hour = Math.floor(departure.time / 60) % 24;
    let minute = departure.time % 60 < 10 ? '0' + departure.time % 60 : '' + departure.time % 60;

    if (subroutes[departure.subroute].schedule === undefined) {
      subroutes[departure.subroute].schedule = [[], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], [], []];
    }
    subroutes[departure.subroute].schedule[hour].push(minute);
  }

  for (let subroute of Object.values(subroutes)) {
    let min_hour = 4; // 4AM
    let max_hour = 2; // 2AM

    for (let i = 4; i <= 26; i++) {
      if (subroute.schedule[i % 24].length !== 0) {
        min_hour = i;
        break;
      }
    }
    for (let i = 26; i >= 4; i--) {
      if (subroute.schedule[i % 24].length !== 0) {
        max_hour = i;
        break;
      }
    }

    subroute.min_hour = min_hour;
    subroute.max_hour = max_hour;

    // Shift hours so that the day starts at 4AM
    subroute.schedule_hours = [
      ...Array(max_hour - min_hour + 1).keys()
    ].map((offset) => {
      return (min_hour + offset) % 24
    });
    subroute.schedule = (max_hour < 24) ?
        subroute.schedule.slice(min_hour, max_hour + 1)
        : subroute.schedule.slice(min_hour, 24).concat(subroute.schedule.slice(0, max_hour % 24 + 1));


    subroute.depth = Math.max.apply(0, subroute.schedule.map((hour) => {
      return hour.length
    }));

    subroute.transposed = [...Array(subroute.depth).keys()].map(() => []);

    for (let i = 0; i < subroute.depth; i++) {
      for (let j = 0; j < subroute.schedule.length; j++) {
        subroute.transposed[i][j] = subroute.schedule[j][i];
      }
    }
  }

  return subroutes;
});

export const subrouteShedule = derived([selectedSubrouteId, scheduleBySubroute], ([$selectedSubrouteId, $scheduleBySubroute]) => {
  if ($scheduleBySubroute) {
    return $scheduleBySubroute[$selectedSubrouteId];
  }
});