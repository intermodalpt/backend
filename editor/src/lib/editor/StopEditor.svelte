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

  function stopScore(stop) {
    let score = 10.0;

    // 2 points here
    score -= stop.has_flag === true || stop.has_flag === false ? 0.0 : 0.5;
    score -= stop.has_schedules === true || stop.has_schedules === false ? 0.0 : 0.2;
    score -= stop.has_sidewalk === true || stop.has_sidewalk === false ? 0.0 : 0.2;
    score -= stop.has_shelter === true || stop.has_shelter === false ? 0.0 : 0.5;
    score -= stop.has_bench === true || stop.has_bench === false ? 0.0 : 0.3;
    score -= stop.has_trash_can === true || stop.has_trash_can === false ? 0.0 : 0.3;

    // 0.9 point
    score -= stop.has_abusive_parking === true || stop.has_abusive_parking === false ? 0.0 : 0.2;
    score -= stop.has_outdated_info === true || stop.has_outdated_info === false ? 0.0 : 0.2;
    score -= stop.is_damaged === true || stop.is_damaged === false ? 0.0 : 0.3;
    score -= stop.is_vandalized === true || stop.is_vandalized === false ? 0.0 : 0.2;

    // 0.8 point
    score -= stop.has_crossing === true || stop.has_crossing === false ? 0.0 : 0.2;
    score -= stop.has_accessibility === true || stop.has_accessibility === false ? 0.0 : 0.2;
    score -= stop.is_illuminated === true || stop.is_illuminated === false ? 0.0 : 0.2;
    score -= stop.has_illuminated_path === true || stop.has_illuminated_path === false ? 0.0 : 0.2;

    // 0.8 point here
    score -= stop.has_visibility_from_area === true || stop.has_visibility_from_area === false ? 0.0 : 0.3;
    score -= stop.has_visibility_from_within === true || stop.has_visibility_from_within === false ? 0.0 : 0.2;
    score -= stop.is_visible_from_outside === true || stop.is_visible_from_outside === false ? 0.0 : 0.3;

    // 3 points here
    score -= stop.name != null ? 0.0 : 1.5;
    score -= stop.official_name != null ? 0.0 : 0.7;
    // score -= stop.code  != null ? 0.0 : 0.1;
    score -= stop.abbr != null ? 0.0 : 0.3;
    score -= stop.locality != null ? 0.0 : 0.2;
    score -= stop.street != null ? 0.0 : 0.2;
    // score -= stop.door  != null ? 0.0 : 0.0;

    return score;
  }

  function createStopMarker(info) {
    let marker;
    let markerOptions = {rinseOnHover: true, draggable: true};
    if (!(info.name || info.short_name || info.osm_name || info.official_name)) {
      marker = L.marker([info.lat, info.lon], Object.assign({}, markerOptions, {icon: icons["geoc"]}));
    } else if (info.source === 'osm') {
      const score = Math.round(stopScore(info));
      let color;
      switch (score) {
        case 3:
        case 4:
          color = 'orange';
          break;
        case 5:
        case 6:
          color = 'yellow';
          break;
        case 7:
        case 8:
        case 9:
          color = 'green';
          break;
        case 10:
          color = 'perfect';
          break;
        default:
          color = 'red';
      }
      let icon = L.divIcon({
        html: `${score}`,
        // iconUrl: `/markers/${source}.svg`,
        className: `score-bubble score-bubble-${color}`,
        iconSize: [16, 16],
        tooltipAnchor: [16, 0],
      });
      marker = L.marker([info.lat, info.lon], Object.assign({}, markerOptions, {icon: icon}));
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