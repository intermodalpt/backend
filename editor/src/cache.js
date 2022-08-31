import {derived, writable} from 'svelte/store';
import {api_server, token} from "./settings.js";

export const stops = writable([]);
export const routes = writable([]);
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


export async function initCache(token) {
  routes.set(await fetch(`${api_server}/api/routes`).then(r => r.json()));

  stops.set(await fetch(`${api_server}/api/stops?all=true`).then(r => r.json()).then(stopList => {
    return Object.fromEntries(stopList.map(stop => [stop.id, stop]));
  }));

  Promise.all([
    fetch(`${api_server}/pictures`, {
      headers: {
        authorization: `Bearer ${token}`
      }
    }).then(r => r.json()),
    fetch(`${api_server}/pictures/rels`, {
      headers: {
        authorization: `Bearer ${token}`
      }
    }).then(r => r.json()),
  ]).then(([pics, rels]) => {
    pictures.set(Object.fromEntries(pics.map(pic => [pic.id, pic])));
    stopPicRels.set(rels);
  }).catch(() => console.log("Shit went kaboom"));
}
