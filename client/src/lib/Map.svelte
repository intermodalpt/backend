<script>
  import L from "leaflet";
  import "leaflet.markercluster";
  import RouteListing from "./components/RouteListing.svelte";
  import RouteStops from "./components/RouteStops.svelte";
  import CompactSchedule from "./components/CompactSchedule.svelte";
  import WHeader from "./components/WidgetHeader.svelte";
  import {api_server} from "../settings.js";
  import {routes, stops} from "../cache.js";
  import {selectedRoute, selectedRouteId, selectedSubrouteId, subrouteStops} from "../context.js";
  import {calc_route_multipoly} from "../utils.js";
  import {tick} from "svelte";

  let map;
  let amlgeo;
  let parishesgeo;
  const touchOriented = window.matchMedia("(pointer: coarse)").matches;

  const color = (b) => `hsl(${getComputedStyle(document.body).getPropertyValue("--" + b)})`;

  let selectionRadius = 750;

  let info = L.control();

  let stopMarkers = {};
  let selectedMarkers = [];
  let selectedPolylines = [];

  let currentSpider;
  let selectedRoutes;

  subrouteStops.subscribe((val) => {
    if (val && map) {
      drawSubroute();
    }
  });

  let mapLayers = {
    parishes: L.layerGroup(),
    municipalities: L.layerGroup(),
    stops: L.markerClusterGroup({
      spiderfyOnMaxZoom: false,
      showCoverageOnHover: false,
      disableClusteringAtZoom: 15,
    }),
    spiderMap: L.layerGroup(),
    selectionArea: L.layerGroup(),
    subrouteLayer: L.layerGroup(),
    legend: L.control({position: "bottomleft"}),
  };

  mapLayers.stops.on("mouseover", () => {
    mapLayers.selectionArea.removeFrom(map);
  });
  mapLayers.stops.on("mouseout", () => {
    mapLayers.selectionArea.addTo(map);
  });

  stops.subscribe((stops) => {
    Object.values(stops).forEach((stop) => {
      if (stop.lat && stop.lon) {
        let marker = L.marker([stop.lat, stop.lon]);
        marker.bindTooltip(`${stop.id} - ${stop.name || stop.short_name}`);
        marker.info = stop;
        marker.stopId = stop.id;

        marker.on("click", (e) => loadSpiderMap(e.target.stopId));
        mapLayers.stops.addLayer(marker);
        stopMarkers[stop.id] = marker;
      }
    });
  });

  function zone_color(zone) {
    switch (zone) {
      case 1:
        return {color: "#f59f00"};
      case 2:
        return {color: "#0ca678"};
      case 3:
        return {color: "#ff6b00"};
      case 4:
        return {color: "#228be6"};
      default:
        return {color: "#6f7479"};
    }
  }

  function onParishFeature(feature, layer) {
    layer.on({
      mouseover: (e) => {
        let layer = e.target;

        layer.setStyle({
          weight: 5,
          color: "#666",
          dashArray: "",
          fillOpacity: 0.7,
        });

        layer.bringToFront();

        info.update(layer.feature.properties);
      },
      mouseout: (e) => {
        parishesgeo.resetStyle(e.target);
        info.update();
      },
      click: (e) => {
        let bounds = e.target.getBounds();
        if (map.getBounds().contains(bounds)) {
          map.setView(bounds.getCenter(), map.getZoom() + 1);
        } else {
          map.zoomIn();
        }
      },
    });
  }

  function onMunicipalityFeature(feature, layer) {
    layer.on({
      mouseover: (e) => {
        let layer = e.target;

        layer.setStyle({
          weight: 5,
          color: "#666",
          dashArray: "",
          fillOpacity: 0.7,
        });

        layer.bringToFront();

        info.update(layer.feature.properties);
      },
      mouseout: (e) => {
        amlgeo.resetStyle(e.target);
        info.update();
      },
      click: (e) => {
        map.fitBounds(e.target.getBounds());
        mapLayers.municipalities.removeFrom(map);
        mapLayers.parishes.addTo(map);
      },
    });
  }

  fetch("/aml.min.geojson")
      .then((r) => r.json())
      .then((obj) => {
        amlgeo = L.geoJSON(obj, {
          style: (feature) => {
            return zone_color(feature.properties.zone);
          },
          onEachFeature: onMunicipalityFeature,
        }).addTo(mapLayers.municipalities);
        if (map) {
          map.fitBounds(amlgeo.getBounds());
        }
      });

  fetch("/freguesias.min.geojson")
      .then((x) => x.json())
      .then((obj) => {
        parishesgeo = L.geoJSON(obj, {
          // style: (feature) => {return zone_color(feature.properties.zone)},
          onEachFeature: onParishFeature,
        }).addTo(mapLayers.parishes);
      });

  function loadSpiderMap(stopId) {
    fetch(`${api_server}/api/stops/${stopId}/spider`)
        .then((x) => x.json())
        .then((spiderMap) => {
          currentSpider = spiderMap;
          selectedRoutes = Object.keys(spiderMap.routes).map((id) => {
            return $routes[id]
          });
          drawSpiderMap(spiderMap);
        });
  }

  function loadAggregateMap(stop_ids) {
    fetch(`${api_server}/api/stops/spider`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(stop_ids),
    })
        .then((x) => x.json())
        .then((spiderMap) => {
          currentSpider = spiderMap;
          selectedRoutes = Object.keys(spiderMap.routes).map((id) => {
            return $routes[id]
          });
          drawSpiderMap(spiderMap);
        });
  }

  function drawSpiderMap(spiderMap) {
    let stops = spiderMap.stops;

    mapLayers.spiderMap.removeFrom(map);
    mapLayers.subrouteLayer.removeFrom(map);
    mapLayers.stops.removeFrom(map);
    mapLayers.spiderMap = L.layerGroup();
    let bounds;

    // used to have a contour
    let innerPolyLines = [];
    Object.values(spiderMap.subroutes).forEach((subroute) => {
      let segments = calc_route_multipoly(stops, subroute.stop_sequence);

      let innerPolyline = L.polyline(segments, {
        color: "white",
        weight: 4,
      });
      innerPolyline.routeId = subroute.route;
      innerPolyLines.push(innerPolyline);
      let outerPolyline = L.polyline(segments, {
        color: "#000",
        weight: 6,
      }).addTo(mapLayers.spiderMap);
      bounds = bounds
          ? bounds.extend(outerPolyline.getBounds())
          : outerPolyline.getBounds();
    });
    innerPolyLines.forEach((polyline) => {
      polyline.addTo(mapLayers.spiderMap);
    });
    mapLayers.spiderMap.addTo(map);
    if (bounds.isValid()) {
      map.fitBounds(bounds);
    }
    selectedPolylines = innerPolyLines;
  }

  function drawSubroute() {
    let cachedStops = $stops;
    mapLayers.spiderMap.removeFrom(map);
    mapLayers.subrouteLayer.removeFrom(map);
    mapLayers.subrouteLayer = L.layerGroup();

    let segments = calc_route_multipoly(cachedStops, $subrouteStops.stops);

    let outerPolyline = L.polyline(segments, {
      color: "black",
      weight: 6,
    }).addTo(mapLayers.subrouteLayer);
    let innerPolyline = L.polyline(segments, {
      color: "white",
      weight: 4,
    }).addTo(mapLayers.subrouteLayer);
    mapLayers.subrouteLayer.addTo(map);
    let bounds = outerPolyline.getBounds();
    if (bounds.isValid()) {
      map.fitBounds(bounds);
    }

    for (let i = 0; i < $subrouteStops.stops.length; i++) {
      let stop = cachedStops[$subrouteStops.stops[i]];
      let diff = $subrouteStops.diffs[i];

      if (stop.lat && stop.lon) {
        let marker = L.marker([stop.lat, stop.lon], {icon: stopIcon});
        marker.on("click", () => {
          // selectStop(stop.id);
        });
        marker.addTo(mapLayers.subrouteLayer);
      }
    }
  }

  async function openRoute(e) {
    $selectedRouteId = e.detail.routeId;
    await tick();
    document.getElementById("route").scrollIntoView(true);
  }

  async function openSchedule(e) {
    $selectedRouteId = e.detail.routeId;
    await tick();
    document.getElementById("schedule").scrollIntoView(true);
  }

  async function openInfo(e) {
    alert("Por fazer");
  }

  function hintRoute(e) {
    let routeId = e.detail.routeId;
    selectedPolylines
        .filter((line) => {
          return line.routeId === routeId;
        })
        .forEach((line) => {
          line.bringToFront();
          line.setStyle({color: color("p")});
        });
  }

  function dropRouteHint(e) {
    let routeId = e.detail.routeId;
    selectedPolylines
        .filter((line) => {
          return line.routeId === routeId;
        })
        .forEach((line) => line.setStyle({color: "white"}));
  }

  function selectArea(center) {
    selectedMarkers = [];

    for (let marker of Object.values(stopMarkers)) {
      let distance = marker.getLatLng().distanceTo(center);
      if (distance <= selectionRadius) {
        selectedMarkers.push(marker);
      }
    }
    if (selectedMarkers.length === 0) {
      alert("A area escolhida não seleccionou nada");
    } else {
      loadAggregateMap(
          selectedMarkers.map((m) => {
            return m.stopId;
          })
      );
    }
  }

  function createMap(container) {
    let m = L.map(container).setView([38.71856, -9.1372], 10);

    let selectorCircle = L.circle([51.508, -0.11], {
      color: color("p"),
      fillColor: color("p"),
      fillOpacity: 0.2,
      radius: 500,
    }).addTo(mapLayers.selectionArea);

    if (touchOriented) {
      selectorCircle.on("click", (e) => {
        selectArea(selectorCircle.getLatLng());
      })
      m.on("move", (e) => {
        selectorCircle.setLatLng(e.target.getCenter());
      });
    } else {
      selectorCircle.on("click", (e) => {
        selectArea(selectorCircle.getLatLng());
      });
      m.on("mousemove", (e) => {
        selectorCircle.setLatLng(e.latlng);
      });
    }


    m.on("zoomend", (e) => {
      let zoomLevel = m.getZoom();
      if (zoomLevel < 14) {
        mapLayers.selectionArea.removeFrom(map);
      } else {
        mapLayers.selectionArea.addTo(map);
        selectionRadius = 750;
        switch (zoomLevel) {
          case 14:
            selectionRadius = 1250;
            break;
          case 15:
            selectionRadius = 750;
            break;
          case 16:
            selectionRadius = 550;
            break;
          case 17:
            selectionRadius = 300;
            break;
          case 18:
            selectionRadius = 150;
            break;
          case 19:
            selectionRadius = 75;
            break;
        }
        selectorCircle.setRadius(selectionRadius);
      }

      if (zoomLevel >= 14) {
        mapLayers.stops.addTo(map);
      } else {
        mapLayers.stops.removeFrom(map);
      }

      if (zoomLevel <= 11 && !selectedRoutes) {
        mapLayers.municipalities.addTo(map);
      } else {
        mapLayers.municipalities.removeFrom(map);
      }
      if (zoomLevel > 11 && zoomLevel <= 13 && !selectedRoutes) {
        mapLayers.parishes.addTo(map);
      } else {
        mapLayers.parishes.removeFrom(map);
      }

      if (zoomLevel < 12) {
        mapLayers.legend.addTo(map);
      } else {
        mapLayers.legend.remove();
      }
    });

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

    L.control.scale().addTo(m);

    info.onAdd = function (map) {
      this._div = L.DomUtil.create("div", "info"); // create a div with a class "info"
      this.update();
      return this._div;
    };

    // method that we will use to update the control based on feature properties passed
    info.update = function (props) {
      if (props) {
        this._div.innerHTML = "<b>" + props.name + "</b><br />";
      }
    };
    info.addTo(m);


    mapLayers.municipalities.addTo(m);

    mapLayers.legend.onAdd = function (map) {
      const div = L.DomUtil.create("div", "info legend");
      div.innerHTML =
          "" +
          '<i style="background:#f59f00"></i>Area 1<br>' +
          '<i style="background:#0ca678"></i>Area 2<br>' +
          '<i style="background:#ff6b00"></i>Area 3<br>' +
          '<i style="background:#228be6"></i>Area 4<br>' +
          '<i style="background:#abb3bb"></i>Independente';

      return div;
    };

    mapLayers.legend.addTo(m);

    return m;
  }

  let stopIcon = L.icon({
    iconUrl: `/markers/bus-minimal.svg`,
    iconSize: [16, 16],
    iconAnchor: [8, 8],
  });

  function mapAction(container) {
    map = createMap(container);

    return {
      destroy: () => {
        map.remove();
        map = null;
      },
    };
  }

  function back(to) {
    document.getElementById(to).scrollIntoView(true);
  }
</script>

<link
    rel="stylesheet"
    href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css"
    integrity="sha512-xwE/Az9zrjBIphAcBb3F6JVqxf46+CDLwfLMHloNu6KEQCAWi6HcDUbeOfBIptF7tcCzusKFjFw2yuvEpDL9wQ=="
    crossorigin=""
/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.css" />
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.Default.css" />
<link rel="stylesheet" href="/map.css" />

<div class="inset-0 fixed">
  <div class="w-full h-screen" use:mapAction />
</div>

<div
    class="fixed right-0 bottom-0 h-2/5 lg:h-3/5 bg-base-100 rounded-t-2xl lg:rounded-t-none lg:rounded-tl-2xl overflow-hidden shadow-xl w-full lg:w-[28rem] flex flex-row"
>
  <div class="carousel w-full overflow-y-hidden">
    <div id="routes" class="carousel-item w-full flex flex-col">
      <WHeader></WHeader>
      <div class="overflow-y-scroll w-full">
        <RouteListing
            bind:selectedRoutes={selectedRoutes}
            on:openroute={openRoute}
            on:openschedule={openSchedule}
            on:openinfo={openInfo}
            on:hint={hintRoute}
            on:drophint={dropRouteHint}
        />
      </div>
    </div>
    {#if $selectedRouteId}
      <div id="route" class="carousel-item w-full flex flex-col gap-1">
        <WHeader back={() => {$selectedRouteId = undefined;}}>
          [{$selectedRoute.code}] {$selectedRoute.name}
        </WHeader>

        <select class="select select-primary select-sm w-[95%] mx-auto" bind:value={$selectedSubrouteId}>
          {#each $selectedRoute.subroutes as subroute}
            <option value={subroute.id}>{subroute.flag}</option>
          {/each}
        </select>
        <div class="overflow-y-scroll w-full">
          <RouteStops />
        </div>
      </div>

      <div id="schedule" class="carousel-item w-full flex flex-col">
        <WHeader
            back={() => {$selectedRouteId = undefined;}}
            fg={$selectedRoute.badge_text}
            bg={$selectedRoute.badge_bg}>
          {$selectedRoute.code}: {$selectedRoute.name}
        </WHeader>
        <div class="overflow-y-scroll w-full">
          <CompactSchedule />
        </div>
      </div>
    {/if}
  </div>
</div>
