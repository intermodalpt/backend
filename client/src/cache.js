import {derived, writable} from 'svelte/store';
import { api_server } from "./settings.js";

export const stops = writable([]);
export const routes = writable([]);
// Maybe replace routes with this later on
export const routeDict = derived(routes, ($routes) => {
  return Object.fromEntries($routes.map(route => [route.id, route]));
});


export async function initCache() {
  routes.set(await fetch(`${api_server}/api/routes`).then(r => r.json()));

  stops.set(await fetch(`${api_server}/api/stops`).then(r => r.json()).then(stopList => {
    return Object.fromEntries(stopList.map(stop => [stop.id, stop]));
  }));
}
