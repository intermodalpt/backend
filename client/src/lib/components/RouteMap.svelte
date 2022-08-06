<script>
  import L from "leaflet";
  import { calc_route_multipoly } from "../../utils.js";
  import { stops } from "../../cache.js";

  import { createEventDispatcher } from "svelte";

  export let subrouteStops;

  const dispatch = createEventDispatcher();

  let map;
  let subrouteLayer = L.layerGroup();

  let selectedStop = 0;
  let selectedStopMarker;

  $: map && $subrouteStops && drawSubroute();

  let stopIcon = L.icon({
    iconUrl: `/markers/bus-minimal.svg`,
    iconSize: [16, 16],
    iconAnchor: [8, 8],
  });

  function drawSubroute() {
    let cachedStops = $stops;
    subrouteLayer.removeFrom(map);
    subrouteLayer = L.layerGroup();

    let segments = calc_route_multipoly(cachedStops, $subrouteStops.stops);

    let outerPolyline = L.polyline(segments, {
      color: "black",
      weight: 6,
    }).addTo(subrouteLayer);
    let innerPolyline = L.polyline(segments, {
      color: "white",
      weight: 4,
    }).addTo(subrouteLayer);
    subrouteLayer.addTo(map);
    let bounds = outerPolyline.getBounds();
    if (bounds.isValid()) {
      map.fitBounds(bounds);
    }

    for (let i = 0; i < $subrouteStops.stops.length; i++) {
      let stop = cachedStops[$subrouteStops.stops[i]];
      let diff = $subrouteStops.diffs[i];

      if (stop.lat && stop.lon) {
        let marker = L.marker([stop.lat, stop.lon], { icon: stopIcon });
        marker.on("click", () => {
          selectStop(stop.id);
        });
        marker.addTo(subrouteLayer);
      }
    }
  }

  function selectStop(stopId) {
    let stop = $stops[stopId];
    if (stop.lat && stop.lon) {
      map.once("moveend zoomend", () => {
        selectedStop = stopId;
      });
      map.setView(new L.LatLng(stop.lat, stop.lon), 16);

      if (selectedStopMarker) {
        selectedStopMarker.removeFrom(map);
      }
      selectedStopMarker = L.marker([stop.lat, stop.lon]);
      selectedStopMarker.addTo(subrouteLayer);
    }
    selectedStop = stopId;
  }

  function createMap(container) {
    let m = L.map(container).setView([38.71856, -9.1372], 10);

    L.tileLayer("https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png", {
      maxZoom: 19,
      subdomains: ["a", "b"],
      attribution: "© OpenStreetMap",
    }).addTo(m);

    m.maxBounds = new L.LatLngBounds(
      new L.LatLng(38.3, -10.0),
      new L.LatLng(39.35, -8.0)
    );
    m.maxBoundsViscosity = 1.0;
    m.minZoom = 10;

    m.on("movestart", () => {
      selectedStop = undefined;
      if (selectedStopMarker) {
        selectedStopMarker.removeFrom(map);
      }
    });

    L.control.scale().addTo(m);

    return m;
  }

  function mapAction(container) {
    map = createMap(container);

    return {
      destroy: () => {
        map.remove();
        map = null;
      },
    };
  }
</script>

<div class="w-full" style="height: max(80vh, 20rem)" use:mapAction />

