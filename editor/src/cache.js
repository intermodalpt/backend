import {derived, writable, get} from 'svelte/store';
import {api_server} from "./settings.js";
import {compressToUTF16, decompressFromUTF16} from 'lz-string'

const ONE_HOUR = 1000 * 3600;

export const stops = writable([]);
export const routes = writable([]);
export const operators = writable({
  1: {name: "Carris Metropolitana", tag: "cmet"},
  2: {name: "Transportes Colectivos do Barreiro", tag: "tcb"},
  3: {name: "Carris", tag: "carris"},
  4: {name: "MobiCascais", tag: "mobic"},
  5: {name: "Comboios de Portugal", tag: "cp"},
  6: {name: "Fertagus", tag: "fert"},
  7: {name: "Metro Transportes do Sul", tag: "mts"},
  8: {name: "Metro de Lisboa", tag: "ml"},
  9: {name: "Transtejo e Soflusa", tag: "ttsl"}
});
export const pictures = writable([]);
export const stopPicRels = writable([]);

export const picStopRels = derived(stopPicRels, $stopPicRels => {
  const reverseRel = {};
  if ($stopPicRels === undefined) {
    return reverseRel;
  }

  Object.entries($stopPicRels).forEach(([stopIdStr, pics]) => {
    let stopId = parseInt(stopIdStr);
    pics.forEach((picId) => {
      if (reverseRel[picId] === undefined) {
        reverseRel[picId] = [stopId];
      } else {
        reverseRel[picId].push(stopId);
      }
    })
  });

  return reverseRel;
})

export async function initCache(token, callback) {
  await fetchData(token, callback)
  // if (!loadCache()) {
  //   await fetchData(token)
  // }
}

export async function refreshCache(token) {
  await fetchData(token, callback);
  saveCache();
}

function loadCache() {
  let cache = localStorage.getItem("cache");
  if (!cache) {
    return false;
  }
  cache = JSON.parse(decompressFromUTF16(cache));
  let now = Date.now();
  let diff = now - cache.timestamp;
  if (diff < ONE_HOUR) {
    routes.set(cache.routes);
    stops.set(cache.stops);
    pictures.set(cache.pictures);
    stopPicRels.set(cache.stopPicRels);
    return true;
  } else {
    return false;
  }
}

async function fetchData(token, callback) {
  await Promise.all([
    fetch(`${api_server}/api/routes`).then(r => r.json()),
    fetch(`${api_server}/api/stops?all=true`).then(r => r.json()).then(stopList => {
      return Object.fromEntries(stopList.map(stop => [stop.id, stop]));
    }),
    fetch(`${api_server}/pictures`, {
      headers: {
        authorization: `Bearer ${token}`
      }
    }).then(r => r.json()).then((pics) => {
      return Object.fromEntries(pics.map(pic => [pic.id, pic]))
    }),
    fetch(`${api_server}/pictures/rels`, {
      headers: {
        authorization: `Bearer ${token}`
      }
    }).then(r => r.json())
  ]).then(([routeList, stopList, pics, rels]) => {
    routes.set(routeList);
    stops.set(stopList);
    pictures.set(pics);
    stopPicRels.set(rels);

    // let cache = {
    //   routes: routeList,
    //   stops: stopList,
    //   pictures: pics,
    //   stopPicRels: rels,
    //   timestamp: Date.now()
    // };
    // localStorage.setItem("cache", compressToUTF16(JSON.stringify(cache)));

  }).catch((e) => console.log(e));
}


export function saveCache() {
  setTimeout(saveCacheForReal, 0)
}

function saveCacheForReal() {
  // let cache = {
  //   routes: get(routes),
  //   stops: get(stops),
  //   pictures: get(pictures),
  //   stopPicRels: get(stopPicRels),
  //   timestamp: Date.now()
  // };
  // localStorage.setItem("cache", compressToUTF16(JSON.stringify(cache)));
}
