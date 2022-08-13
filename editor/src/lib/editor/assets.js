import L from 'leaflet';

export const sources = ["carris", "cmet", "tcb", "tst", "sf", "mobicascais", "osm", "geoc"];
export const icons = {};

for (let source of sources) {
  icons[source] = L.icon({
    iconUrl: `/markers/${source}.svg`,
    iconSize: [32, 32],
    iconAnchor: [16, 31],
  });
}


