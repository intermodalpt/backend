<script>
  import StopForm from "./StopForm.svelte";
  import L from "leaflet";
  import "leaflet.featuregroup.subgroup";
  import { api_server, token } from "../../settings.js";
  import { icons, picIcon } from "./assets.js";
  import { writable } from "svelte/store";
  import { pictures, stops } from "../../cache.js";
  import StopMassEditor from "./StopMassEditor.svelte";

  let map;
  let control = L.control.layers(null, null, {collapsed: false});
  let selectedStop = writable(undefined);
  let previewedPic = undefined;

  let pickingFilers = false;
  let filterOnlyNoName = false;
  let filterOnlyNoOfficialName = false;
  let filterOnlyNoOSM = false;
  let filterOnlyNoAttrs = false;
  let filterOnlyNoPics = false;

  export function selectStop(stopId) {
    $selectedStop = $stops[stopId];
  }

  function saveStopMeta(e) {
    let newMeta = Object.assign($selectedStop, e.detail);

    updateStop(newMeta);
    // saveCache();

    $selectedStop = null;
  }

  function updateStop(stop) {
    fetch(`${api_server}/v1/stops/update/${stop.id}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
        authorization: `Bearer ${$token}`,
      },
      body: JSON.stringify(stop),
    })
        .then((data) => {
          alert("Done");
          Object.assign($stops[stop.id], stop);
        })
        .catch(() => {
          alert("Error updating");
        });
  }

  let mapLayers = {
    stops: L.layerGroup(),
    osmStops: L.layerGroup(),
    otherStops: L.layerGroup(),
    stopPics: L.layerGroup(),
  };

  let info = L.control();
  let zoom = 0;

  function createStopMarker(info) {
    let marker;
    let markerOptions = {rinseOnHover: true, draggable: true};
    if (!(info.name || info.short_name || info.osm_name || info.official_name)) {
      marker = L.marker([info.lat, info.lon], Object.assign({}, markerOptions, {icon: icons["geoc"]}));
    } else if (icons[info.source] === undefined) {
      marker = L.marker([info.lat, info.lon], markerOptions);
    } else {
      marker = L.marker([info.lat, info.lon], Object.assign({}, markerOptions, {icon: icons[info.source]}));
    }

    marker.stopId = info.id;
    marker.meta = info;

    marker.on("click", (e) => selectStop(e.target.stopId));

    let name = info.name || info.short_name || info.official_name || info.osm_name;

    marker.bindTooltip(`${info.id} - ${name}`);

    return marker;
  }

  function createPicMarker(pic) {
    let marker = L.marker([pic.lat, pic.lon], {rinseOnHover: true, icon: picIcon});

    marker.picId = pic.id;

    marker.on("click", (e) => previewedPic = $pictures[pic.id]);
    return marker;
  }

  function loadStops() {
    mapLayers.stops.removeFrom(map);
    mapLayers.stops = L.markerClusterGroup({
      spiderfyOnMaxZoom: true,
      showCoverageOnHover: true,
      disableClusteringAtZoom: 14,
      // iconCreateFunction: function(cluster) {
      //   return L.divIcon({ html: '<b>' + cluster.getChildCount() + console.log(cluster) + '</b>' });
      // }
    });

    let osmMarkers = [];
    let otherMarkers = [];
    let picMarkers = [];

    Object.values($stops).forEach((stop) => {
      if (stop.lat != null && stop.lon != null) {
        let marker = createStopMarker(stop);
        if (stop.source === "osm") {
          if (filterOnlyNoName && stop.name) {
            return;
          }

          if (filterOnlyNoOfficialName && stop.official_name) {
            return;
          }

          if (filterOnlyNoOSM && stop.osm_name) {
            return;
          }

          if (filterOnlyNoAttrs && stop.locality && stop.street) {
            return;
          }

          if (filterOnlyNoPics && stop.locality && stop.street) {
            return;
          }
          osmMarkers.push(marker);
        } else {
          otherMarkers.push(marker);
        }
      }
    });

    Object.values($pictures).forEach((pic) => {
      if (pic.lat != null && pic.lon != null) {
        let marker = createPicMarker(pic);
        picMarkers.push(marker);
      }
    })


    control.removeLayer(mapLayers.stopPics);
    control.removeLayer(mapLayers.osmStops);
    control.removeLayer(mapLayers.otherStops);

    mapLayers.stopPics = L.featureGroup.subGroup(mapLayers.stops, picMarkers);
    mapLayers.osmStops = L.featureGroup.subGroup(mapLayers.stops, osmMarkers);
    mapLayers.otherStops = L.featureGroup.subGroup(mapLayers.stops, otherMarkers);
    control.addOverlay(mapLayers.osmStops, "OSM");
    control.addOverlay(mapLayers.otherStops, "GTFS");
    control.addOverlay(mapLayers.stopPics, "Pics");

    map.addLayer(mapLayers.stops);
    map.addLayer(mapLayers.osmStops);
    // map.addLayer(mapLayers.other_stops);
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
    }).setView([38.71856, -9.1372], 10);

    let osm = L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
      maxZoom: 19,
      attribution: "© OpenStreetMap",
    }).addTo(m);

    control.addTo(m);

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

  let massEditing = false;

  function openFilterPicker() {
    pickingFilers = true;
  }

  function applyFilters() {
    loadStops();
  }
</script>

<div class="absolute right-8 top-2">
  <div class="form-control">
    <label class="label cursor-pointer">
      <span class="label-text mr-2">Mass Edit</span>
      <input type="checkbox" class="toggle toggle-primary" bind:checked={massEditing} />
    </label>
  </div>
</div>
{#if massEditing}
  <StopMassEditor />
{:else}
  <div class="flex flex-col">
    <div class="map h-96 cursor-crosshair" use:mapAction />
    <div>
      {#if $selectedStop}
        <StopForm stop={selectedStop} on:save={saveStopMeta} />
      {:else}
        <p>Select a stop to edit.</p>
      {/if}
    </div>
  </div>

  <input type="button" class="input input-info" value="Filtros" on:click={openFilterPicker} />
  {#if previewedPic}
    <input type="checkbox" id="pic-preview" class="modal-toggle" checked />
    <div class="modal">
      <div class="modal-box w-11/12 max-w-5xl">
        <a>
          <a target="_blank"
             href="https://intermodal-storage-worker.claudioap.workers.dev/ori/{previewedPic.sha1}/{previewedPic.original_filename}">
            <img
                src="https://intermodal-storage-worker.claudioap.workers.dev/medium/{previewedPic.sha1}/preview"
                class="rounded-box w-full"
            />
          </a>
        </a>
        <div class="modal-action">
          <label for="pic-preview" class="btn" on:click="{() => {previewedPic = undefined;}}">Close</label>
        </div>
      </div>
    </div>
  {/if}
{/if}

{#if pickingFilers}
  <input type="checkbox" id="filter-picker" class="modal-toggle" checked />
  <div class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Filtros</h3>
      <p class="py-4 flex flex-col">
        <label><input type="checkbox" class="mr-2" bind:value={filterOnlyNoName}>Só sem nome (nosso)</label>
        <label><input type="checkbox" class="mr-2" bind:value={filterOnlyNoOfficialName}>Só sem nome oficial</label>
        <label><input type="checkbox" class="mr-2" bind:value={filterOnlyNoOSM}>Só sem nome osm</label>
        <label><input type="checkbox" class="mr-2" bind:value={filterOnlyNoAttrs}>Só com atributos em falta</label>
        <label><input type="checkbox" class="mr-2" bind:value={filterOnlyNoPics}>Só sem fotos</label>
      </p>
      <div class="modal-action">
        <label for="filter-picker" class="btn" on:mouseup={applyFilters}>Aplicar</label>
      </div>
    </div>
  </div>
{/if}