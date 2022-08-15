<script>
  import Box from "../components/Box.svelte";
  import LineStopsEditor from "./LineStopsEditor.svelte";
  import {api_server, token} from "../../settings.js";
  import {icons} from "./assets.js";
  import {stops, routes} from "../../cache.js";
  import L from "leaflet";
  import {calc_route_multipoly} from "../../utils.js";
  import {derived, writable} from "svelte/store";

  let map;

  const selectedStop = writable(undefined);
  const selectedRouteId = writable(undefined);
  const selectedRouteStops = writable(undefined);
  const selectedSubrouteId = writable(undefined);
  const selectedSubrouteStops = derived(
      [selectedRouteStops, selectedSubrouteId],
      ([$selectedRouteStops, $selectedSubrouteId]) => {
        return $selectedRouteStops ? $selectedRouteStops[$selectedSubrouteId] : undefined
      }
  );
  const subroutes = derived(
      selectedRouteId,
      $selectedRouteId => {
        return $selectedRouteId
            ? Object.fromEntries($routes[$selectedRouteId].subroutes.map((subroute) => [subroute.id, subroute]))
            : undefined
      }
  );
  const selectedRoute = derived(selectedRouteId, $selectedRouteId => {
    return $routes.find((route) => {
      return route.id === $selectedRouteId
    })
  });

  selectedRoute.subscribe((route) => {
    if (route && route.subroutes.length > 0) {
      $selectedSubrouteId = route.subroutes[0].id;
    }
  });

  let loading = false;

  let mapLayers = {
    stops: L.layerGroup(),
    subrouteDrawing: L.layerGroup(),
  };


  selectedRouteId.subscribe((id) => {
    if (id === undefined) {
      return;
    }
    fetch(`${api_server}/api/routes/${id}/stops?all=true`)
        .then((r) => r.json())
        .then((data) => {
          $selectedRouteStops = Object.fromEntries(data.map((subroute) => [subroute.subroute, subroute]));
        })
        .catch((e) => alert("Failed to load the route stops"));
  })

  selectedSubrouteStops.subscribe((value) => {
    if (value) {
      let polyLine = drawSubroute(value.stops);
      let bounds = polyLine.getBounds();
      if (polyLine && bounds.isValid()) {
        map.fitBounds(bounds);
      }
    }
  });


  function drawSubroute(stop_ids) {
    let segments = calc_route_multipoly($stops, stop_ids);

    mapLayers.subrouteDrawing.removeFrom(map);
    mapLayers.subrouteDrawing = L.layerGroup();
    if (segments) {
      let polyLine = L.polyline(segments, {color: "red"}).addTo(mapLayers.subrouteDrawing);

      mapLayers.subrouteDrawing.addTo(map);
      return polyLine;
    }
  }

  function createStopMarker(info) {
    let marker;
    let markerOptions = {rinseOnHover: true, draggable: true};
    if (icons[info.source] === undefined) {
      marker = L.marker([info.lat, info.lon], markerOptions);
    } else {
      marker = L.marker([info.lat, info.lon], Object.assign({}, markerOptions, {icon: icons[info.source]}));
    }

    marker.stopId = info.id;

    marker.on("click", (e) => {
      $selectedStop = $stops[e.target.stopId];
    });

    let name = info.name || info.short_name;

    marker.bindTooltip(`${info.id} - ${name}`);

    return marker;
  }

  function loadStops() {

    mapLayers.stops = L.markerClusterGroup({
      // spiderfyOnMaxZoom: false,
      showCoverageOnHover: false,
      disableClusteringAtZoom: 16,
    });
    Object.values($stops).forEach((node) => {
      if (node.lat != null && node.lon != null) {
        let marker = createStopMarker(node);
        mapLayers.stops.addLayer(marker);
      }
    });

    map.addLayer(mapLayers.stops);
  }

  function goTo(e) {
    if (e.detail.lat && e.detail.lon) {
      map.setView([e.detail.lat, e.detail.lon], 17);
    }
  }

  function redraw(e) {
    drawSubroute(e.detail.stops);
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

  function saveSubrouteStops(e) {
    let routeStops = $selectedRouteStops[$selectedSubrouteId];
    fetch(`${api_server}/api/routes/${$selectedRouteId}/stops/subroutes/${$selectedSubrouteId}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
        authorization: `Bearer ${$token}`
      },
      body: JSON.stringify({
        from: {
          stops: routeStops.stops,
          diffs: routeStops.diffs,
        },
        to: {
          stops: e.detail.stops,
          diffs: e.detail.diffs,
        },
      }),
    })
        .then((resp) => {
          if (resp.ok) {
            alert("We're good");
            routeStops.stops = e.detail.stops;
            routeStops.diffs = e.detail.diffs;
          } else {
            alert("The server didn't like this data");
          }
        })
        .catch((e) => {
          alert("Error saving");
        });
  }
</script>

<div class="hwrapper">
  <div>
    <Box>
      <div class="map" use:mapAction />
    </Box>
  </div>
  <div>
    {#if loading}
      A carregar os dados
    {:else}
      <label>
        Selected line:
        <select class="select select-bordered select-xs" bind:value={$selectedRouteId}>
          {#each Object.values($routes) as route}
            <option value={route.id}>
              {route.code} - {route.name.substring(0, 60)}
            </option>
          {/each}
        </select>
      </label><br />
      <span>
        Selected stop:
        {#if $selectedStop}
          {$selectedStop.name} ({$selectedStop.id})
        {:else}
          None
        {/if}
      </span>
      {#if $selectedRoute}
        <h2 class="font-bold">
          {$selectedRoute.code} - {$selectedRoute.name} ({$selectedRoute.id})
        </h2>
        <select class="select select-bordered select-xs" bind:value={$selectedSubrouteId}>
          {#each $selectedRoute.subroutes as subroute}
            <option value={subroute.id}>{subroute.flag.substring(0, 60)}</option>
          {/each}
        </select>
        <LineStopsEditor
            selectedStop={selectedStop}
            selectedSubrouteStops={selectedSubrouteStops}
            on:goto={goTo}
            on:redraw={redraw}
            on:savesubroutestops={saveSubrouteStops}
        />
      {/if}
    {/if}
  </div>
</div>

<style>
  .hwrapper {
    display: flex;
  }

  .hwrapper div:first-child {
    flex-basis: max(50vw, 600px);
    flex-shrink: 0;
  }

  .hwrapper div:nth-of-type(2) {
    margin-left: 10px;
    flex-grow: 1;
    overflow: auto;
  }

  .map {
    height: calc(100vh - 80px);
    border-radius: 12px;
    cursor: crosshair !important;
  }
</style>
