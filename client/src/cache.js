import {derived, writable} from 'svelte/store';
import { api_server } from "./settings.js";

export const stops = writable([]);
export const routes = writable([]);
// Maybe replace routes with this later on
export const routeDict = derived(routes, ($routes) => {
  return Object.fromEntries($routes.map(route => [route.id, route]));
});
export const operators = writable({
  1: {id: 1, name: "Carris Metropolitana", tag: "cmet"},
  2: {id: 2, name: "Transportes Colectivos do Barreiro", tag: "tcb"},
  3: {id: 3, name: "Carris", tag: "carris"},
  4: {id: 4, name: "MobiCascais", tag: "mobic"},
  5: {id: 5, name: "Comboios de Portugal", tag: "cp"},
  6: {id: 6, name: "Fertagus", tag: "fert"},
  7: {id: 7, name: "Metro Transportes do Sul", tag: "mts"},
  8: {id: 8, name: "Metro de Lisboa", tag: "ml"},
  9: {id: 9, name: "Transtejo e Soflusa", tag: "ttsl"}
});

export async function initCache() {
  routes.set(await fetch(`${api_server}/api/routes`).then(r => r.json()));

  stops.set(await fetch(`${api_server}/api/stops`).then(r => r.json()).then(stopList => {
    return Object.fromEntries(stopList.map(stop => [stop.id, stop]));
  }));
}
