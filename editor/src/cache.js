import {derived, writable} from 'svelte/store';
import {api_server, token} from "./settings.js";

export const stops = writable([]);
export const routes = writable([]);

// TODO get rid of this, see below
export const pictures = derived(token, ($token, set) => {
  if ($token === undefined) {
    return [];
  }
  fetch(`${api_server}/pictures`, {
    headers: {
      authorization: `Bearer ${$token}`
    }
  })
      .then(r => r.json())
      .then((pics) => {
        set(Object.fromEntries(pics.map(pic => [pic.id, pic])));
      });
});

export const stopPicRels = derived([stops, token], ([$stops, $token], set) => {
  if ($stops === undefined || $token === undefined) {
    return [];
  }

  fetch(`${api_server}/pictures/rels`, {
    headers: {
      authorization: `Bearer ${$token}`
    }
  })
      .then(r => r.json())
      .then((rels) => {
        set(rels);
      });
});

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


export async function initCache() {
  routes.set(await fetch(`${api_server}/api/routes`).then(r => r.json()));

  stops.set(await fetch(`${api_server}/api/stops?all=true`).then(r => r.json()).then(stopList => {
    return Object.fromEntries(stopList.map(stop => [stop.id, stop]));
  }));

  // TODO do this once it is figured why token isn't set by now
  // Promise.all([
  //   fetch(`${api_server}/pictures`, {
  //     headers: {
  //       authorization: `Bearer ${$token}`
  //     }
  //   }).then(r => r.json()),
  //   fetch(`${api_server}/pictures/rels`, {
  //     headers: {
  //       authorization: `Bearer ${$token}`
  //     }
  //   }).then(r => r.json()),
  // ]).then(([picList, rels]) => {
  //   pictures.set(Object.fromEntries(picList.map(pic => [pic.id, pic])));
  //   stopPicRels.set(rels);
  // }).catch(() => console.log("Shit went kaboom"));
}
