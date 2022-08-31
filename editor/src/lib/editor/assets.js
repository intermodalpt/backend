import L from 'leaflet';

export const sources = ["carris", "cmet", "tcb", "tst", "sf", "mobicascais", "osm", "geoc"];
export const icons = {};
export const picIcon = L.icon({
  iconUrl: `/markers/pic.svg`,
  iconSize: [32, 32],
  iconAnchor: [16, 31],
});

for (let source of sources) {
  icons[source] = L.icon({
    iconUrl: `/markers/${source}.svg`,
    iconSize: [32, 32],
    iconAnchor: [16, 31],
  });
}


