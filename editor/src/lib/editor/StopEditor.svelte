<script>
  import StopForm from "./StopForm.svelte";
  import L from "leaflet";
  import {api_server, token} from "../../settings.js";
  import {icons} from "./assets.js";
  import {writable} from "svelte/store";
  import {stops} from "../../cache.js";

  let map;
  let selectedStop = writable(undefined);

  export function selectStop(stopId) {
    $selectedStop = $stops[stopId];
  }

  function saveStopMeta(e) {
    let newMeta = Object.assign(Object.assign({}, $selectedStop), e.detail);

    updateStop(newMeta);
    $selectedStop = null;
  }

  function updateStop(stop) {
    fetch(`${api_server}/api/stops/update/${stop.id}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
        authorization: `Bearer ${$token}`
      },
      body: JSON.stringify(stop),
    }).then((data) => {
      alert("Done");
      Object.assign($stops[stop.id], stop);
    }).catch(() => {
      alert("Error updating")
    });
  }

  let mapLayers = {
    parishes: L.layerGroup(),
    municipalities: L.layerGroup(),
    stops: L.layerGroup(),
    lineSeq: L.layerGroup(),
  };

  let info = L.control();
  let zoom = 0;

  function createStopMarker(info) {
    let marker;
    let markerOptions = {rinseOnHover: true, draggable: true};
    if (icons[info.source] === undefined) {
      marker = L.marker([info.lat, info.lon], markerOptions);
    } else {
      marker = L.marker([info.lat, info.lon], Object.assign({}, markerOptions, {icon: icons[info.source]}));
    }

    marker.stopId = info.id;
    marker.meta = info;

    marker.on("click", (e) => selectStop(e.target.stopId));

    let name = info.name || info.short_name;

    marker.bindTooltip(`${info.id} - ${name}`);

    return marker;
  }

  function loadStops() {
    mapLayers.stops = L.markerClusterGroup({
      spiderfyOnMaxZoom: false,
      showCoverageOnHover: false,
      disableClusteringAtZoom: 16,
    });

    Object.values($stops).forEach((stop) => {
      if (stop.lat != null && stop.lon != null) {
        let marker = createStopMarker(stop);
        mapLayers.stops.addLayer(marker);
      }
    });

    map.addLayer(mapLayers.stops);
  }

  function createStop(e) {
    let stop = {
      source: "iml",
      lat: e.latlng.lat,
      lon: e.latlng.lng,
    };
    fetch(`${api_server}/api/stops/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(stop),
    })
        .then((r) => r.json())
        .then((data) => {
          stop.id = data.id;
          let marker = createStopMarker(stop);
          mapLayers.stops.addLayer(marker);
        });
  }

  function createMap(container) {
    let m = L.map(container, {
      contextmenu: true,

      contextmenuWidth: 140,
      contextmenuItems: [
        {
          text: "Create a stop",
          callback: createStop,
        },
      ],
    }).setView([38.71856, -9.1372], 10);

    let osm = L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
      maxZoom: 19,
      attribution: "© OpenStreetMap",
    }).addTo(m);

    let baseMaps = {
      OSM: osm,
      // "Satellite": satellite
    };
    L.control.layers(baseMaps).addTo(m);

    m.maxBounds = new L.LatLngBounds(new L.LatLng(38.3, -10.0), new L.LatLng(39.35, -8.0));
    m.maxBoundsViscosity = 1.0;
    m.minZoom = 10;
    m.setView([38.605, -9.0], 0);
    m.setZoom(11);

    return m;
  }

  function mapAction(container) {
    map = createMap(container);
    loadStops();

    return {
      destroy: () => {
        map.remove();
        map = null;
      },
    };
  }
</script>

<div class="flex flex-col">
  <div class="map h-96 cursor-crosshair" use:mapAction />
  <div>
    {#if $selectedStop}
      <StopForm stop={selectedStop} on:save={saveStopMeta} />
    {:else}
      <p>Select a stop to edit
        <it></it>
        .
      </p>
    {/if}
  </div>
</div>

<style>
  .pending-changes li {
    display: flex;
    justify-content: space-between;
    overflow: auto;
  }

  .pending-changes button {
    width: 60px;
  }

  .pending-changes .changes {
    display: flex;
    flex-direction: column;
    overflow: auto;
  }

  .pending-changes .changes .title {
    font-size: 1.2em;
    font-weight: bold;
  }
</style>
