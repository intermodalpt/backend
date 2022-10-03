<script>
  import LineStopsEditor from "./LineStopsEditor.svelte";
  import {api_server, token} from "../../settings.js";
  import {icons} from "./assets.js";
  import {stops, routes, operators} from "../../cache.js";
  import L from "leaflet";
  import {calc_route_multipoly} from "../../utils.js";
  import {derived, writable} from "svelte/store";
  import RouteForm from "./RouteForm.svelte";
  import SubrouteScheduleForm from "./SubrouteScheduleForm.svelte";
  import Select from 'svelte-select';

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
  const selectedSubroute = derived(
      [selectedRoute, selectedSubrouteId],
      ([$selectedRoute, $selectedSubrouteId]) => {
        if (!($selectedRoute || $selectedSubrouteId)) {
          return undefined;
        }
        return $selectedRoute.subroutes.find((subroute) => {
          return subroute.id === $selectedSubrouteId
        })
      });
  const routeSelectorOptions = derived(
      routes,
      ($routes) => {
        return $routes.sort(
            (ra, rb) => {
              if (ra.operator !== rb.operator) {
                return ra.operator - rb.operator;
              }
              if (!ra.code) {
                return -1;
              } else if (!rb.code) {
                return 1;
              } else {
                return (parseInt(ra.code) || 10000) - (parseInt(rb.code) || 10000);
              }
            })
            .map((route) => {
              return {
                value: route.id,
                label: route.code ? `${route.code} : ${route.name}` : route.name,
                group: $operators[route.operator]?.name,
              }
            });
      }
  );

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
          const stops = Object.fromEntries(data.map((subroute) => [subroute.subroute, subroute]));

          for (const subroute of $selectedRoute.subroutes) {
            if (!(subroute.id in stops)) {
              stops[subroute.id] = {
                'stops': [],
                'diffs': [],
              }
            }
          }
          $selectedRouteStops = stops;
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

    let name = info.official_name || info.name || info.short_name || info.osm_name;

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

  function handleSelect(event) {
    $selectedRouteId = event.detail.value;
  }
</script>

{#if !loading}
  <div class="z-[1001] grid grid-cols-1 justify-items-stretch">
    <Select
        items={$routeSelectorOptions}
        on:select={handleSelect}
        isClearable={false}
        groupBy={(item) => item.group}
        placeholder="Route">
    </Select>
    {#if $selectedRouteId}
      <div>
        Variant:
        <select class="select select-bordered select-sm" bind:value={$selectedSubrouteId}>
          {#each $selectedRoute.subroutes as subroute}
            <option value={subroute.id}>{subroute.flag.substring(0, 60)}</option>
          {/each}
        </select>

        <label for="route-edit-modal" class="btn btn-primary btn-xs modal-button text-primary">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-4">
            <path d="M421.7 220.3l-11.3 11.3-22.6 22.6-205 205c-6.6 6.6-14.8 11.5-23.8 14.1L30.8 511c-8.4 2.5-17.5
                  .2-23.7-6.1S-1.5 489.7 1 481.2L38.7 353.1c2.6-9 7.5-17.2 14.1-23.8l205-205 22.6-22.6 11.3-11.3 33.9
                  33.9 62.1 62.1 33.9 33.9zM96 353.9l-9.3 9.3c-.9 .9-1.6 2.1-2 3.4l-25.3 86 86-25.3c1.3-.4 2.5-1.1
                  3.4-2l9.3-9.3H112c-8.8 0-16-7.2-16-16V353.9zM453.3 19.3l39.4 39.4c25 25 25 65.5 0 90.5l-14.5 14.5-22.6
                  22.6-11.3 11.3-33.9-33.9-62.1-62.1L314.3 67.7l11.3-11.3 22.6-22.6 14.5-14.5c25-25 65.5-25 90.5 0z" />
          </svg>
        </label>
        <input type="checkbox" id="route-edit-modal" class="modal-toggle" />
        <label for="route-edit-modal" class="modal cursor-pointer z-[2000]">
          <label class="modal-box lg:w-[60rem] lg:max-w-full" for="">
            <span class="text-lg">A editar {$selectedRoute.code}</span>
            <RouteForm {selectedRoute} />
          </label>
        </label>
        {#if $selectedSubrouteId}
          <label for="schedule-edit-modal" class="btn btn-secondary btn-xs modal-button text-primary">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="w-4">
              <path d="M256 512C114.6 512 0 397.4 0 256S114.6 0 256 0S512 114.6 512 256s-114.6 256-256 256zM232
                  120V256c0 8 4 15.5 10.7 20l96 64c11 7.4 25.9 4.4 33.3-6.7s4.4-25.9-6.7-33.3L280
                  243.2V120c0-13.3-10.7-24-24-24s-24 10.7-24 24z" />
            </svg>
          </label>
          <input type="checkbox" id="schedule-edit-modal" class="modal-toggle" />
          <label for="schedule-edit-modal" class="modal cursor-pointer z-[2000]">
            <label class="modal-box lg:w-[60rem] lg:max-w-full" for="">
              <span class="text-lg">A editar {$selectedSubrouteId.flag}</span>
              <SubrouteScheduleForm />
            </label>
          </label>
        {/if}

        <span>
            Selected stop:
          {#if $selectedStop}
              {$selectedStop.official_name || $selectedStop.osm_name || $selectedStop.name} ({$selectedStop.id})
            {:else}
              None
            {/if}
          </span>
      </div>
    {/if}
  </div>
{/if}
<div class="hwrapper">
  <div class="map" use:mapAction></div>
  <div>
    {#if loading}
      A carregar os dados
    {:else}
      <div class="flex">
        {#if $selectedRouteId}
        {/if}
      </div>
      {#if $selectedRoute}
        <LineStopsEditor
            selectedStop={selectedStop}
            selectedSubrouteStops={selectedSubrouteStops}
            selectedRoute={selectedRoute}
            selectedRouteStops={selectedRouteStops}
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
    flex-grow: 1;
  }

  .hwrapper div:last-child {
    max-width: 500px;
    flex-shrink: 0;
    flex-grow: 0;
  }

  .hwrapper div:nth-of-type(3) {
    margin-left: 10px;
    height: calc(100vh - 80px);
  }

  .map {
    height: calc(100vh - 150px);
    border-top-left-radius: 12px;
    border-bottom-left-radius: 12px;
    cursor: crosshair !important;
  }
</style>
