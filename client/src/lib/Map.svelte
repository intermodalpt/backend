<script>
    import L from 'leaflet';
    import 'leaflet.markercluster';
    import {api_server} from "../settings.js";
    import {calc_route_multipoly, randomHexColor} from "../utils.js";
    import RouteListing from "./components/RouteListing.svelte";
    import Route from "./Route.svelte";

    let map;
    let amlgeo;
    let parishesgeo;
    let cache;

    function loadCache() {
        Promise.all([
            fetch(`${api_server}/api/routes`).then(r => r.json()),
            fetch(`${api_server}/api/stops`).then(r => r.json())
        ]).then(([routes, stops]) => {

            cache = {
                routes: routes,
                stops: Object.fromEntries(stops.map(stop => [stop.id, stop]))
            }
        });
    }
    loadCache();


    let mapLayers = {
        parishes: L.layerGroup(),
        municipalities: L.layerGroup(),
        stops: L.layerGroup(),
        spiderMap: L.layerGroup(),
        selectionArea: L.layerGroup(),
    };

    let selectionRadius = 750;

    let info = L.control();
    let zoom = 0;
    let stopMarkers = {};
    let selectedMarkers = [];
    let selectedPolylines = [];

    let currentSpider;
    let selectedRoutes;
    let selectedRoute;


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


    function loadStops() {
        let bounds = map.getBounds();
        let x0 = bounds.getWest();
        let y0 = bounds.getNorth();
        let x1 = bounds.getEast();
        let y1 = bounds.getSouth();
        fetch(`${api_server}/api/stops/within_boundary/${x0}/${y0}/${x1}/${y1}`)
            .then(r => r.json())
            .then(nodes => {
                map.removeLayer(mapLayers.stops);
                mapLayers.stops = L.markerClusterGroup({
                    spiderfyOnMaxZoom: false,
                    showCoverageOnHover: false,
                    disableClusteringAtZoom: 14
                });
                mapLayers.stops.on('mouseover', () => {
                    mapLayers.selectionArea.removeFrom(map);
                });
                mapLayers.stops.on('mouseout', () => {
                    mapLayers.selectionArea.addTo(map);
                });
                nodes.forEach(node => {
                    let marker = L.marker([node.lat, node.lon]);
                    marker.bindTooltip(`${node.id} - ${node.name || node.short_name}`);
                    marker.info = node;
                    marker.stopId = node.id;

                    marker.on('click', (e) => loadSpiderMap(e.target.stopId));
                    mapLayers.stops.addLayer(marker);
                    stopMarkers[node.id] = marker;
                });

                map.addLayer(mapLayers.stops);
            });
    }

    function onParishFeature(feature, layer) {
        layer.on({
            mouseover: (e) => {
                var layer = e.target;

                layer.setStyle({
                    weight: 5,
                    color: '#666',
                    dashArray: '',
                    fillOpacity: 0.7
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
                map.fitBounds(bounds);
                loadStops();
                mapLayers.parishes.removeFrom(map);
            }
        });
    }

    function onMunicipalityFeature(feature, layer) {
        layer.on({
            mouseover: (e) => {
                let layer = e.target;

                layer.setStyle({
                    weight: 5,
                    color: '#666',
                    dashArray: '',
                    fillOpacity: 0.7
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
            }
        });
    }

    fetch('/aml.min.geojson')
        .then(r => r.json())
        .then(obj => {
            amlgeo = L.geoJSON(obj, {
                style: (feature) => {
                    return zone_color(feature.properties.zone)
                },
                onEachFeature: onMunicipalityFeature
            }).addTo(mapLayers.municipalities);
            mapLayers.municipalities.addTo(map);
            map.fitBounds(amlgeo.getBounds());
        });

    fetch("/freguesias.min.geojson")
        .then(x => x.json()).then(obj => {
        parishesgeo = L.geoJSON(obj, {
            // style: (feature) => {return zone_color(feature.properties.zone)},
            onEachFeature: onParishFeature
        }).addTo(mapLayers.parishes);
    });

    function loadSpiderMap(stopId) {
        fetch(`${api_server}/api/stops/${stopId}/spider`)
            .then(x => x.json())
            .then(spiderMap => {
                currentSpider = spiderMap;
                selectedRoutes = spiderMap.routes;
                drawSpiderMap(spiderMap);
            });

    }

    function loadAggregateMap(stop_ids) {
        fetch(`${api_server}/api/stops/spider`, {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(stop_ids)
        })
            .then(x => x.json())
            .then(spiderMap => {
                currentSpider = spiderMap;
                selectedRoutes = spiderMap.routes;
                drawSpiderMap(spiderMap);
            });

    }

    function drawSpiderMap(spiderMap) {
        let stops = spiderMap.stops;

        mapLayers.spiderMap.removeFrom(map);
        mapLayers.stops.removeFrom(map);
        mapLayers.spiderMap = L.layerGroup();
        let bounds;

        // used to have a contour
        let innerPolyLines = [];
        Object.values(spiderMap.subroutes).forEach((subroute) => {
                let segments = calc_route_multipoly(stops, subroute.stop_sequence);

                let innerPolyline = L.polyline(segments, {color: 'white', weight: 4});
                innerPolyline.routeId = subroute.route;
                innerPolyLines.push(innerPolyline);
                let outerPolyline = L.polyline(segments, {color: 'black', weight: 6}).addTo(mapLayers.spiderMap);
                bounds = bounds ? bounds.extend(outerPolyline.getBounds()) : outerPolyline.getBounds();
            }
        );
        innerPolyLines.forEach(polyline => {
            polyline.addTo(mapLayers.spiderMap)
        });
        mapLayers.spiderMap.addTo(map);
        map.fitBounds(bounds);
        selectedPolylines = innerPolyLines;
    }

    function openRoute(e) {
        let routeId = e.detail.routeId;

        for (const [key, subroute] of Object.entries(currentSpider.subroutes)) {
            subroute.id = parseInt(key);
        }
        for (const [key, stop] of Object.entries(currentSpider.stops)) {
            stop.id = parseInt(key);
        }

        selectedRoute = routeId;
    }

    function hintRoute(e) {
        let routeId = e.detail.routeId;
        selectedPolylines.filter((line) => {
            return line.routeId === routeId
        }).forEach((line) => {
            line.bringToFront();
            line.setStyle({'color': 'green'});
        });
    }

    function dropRouteHint(e) {
        let routeId = e.detail.routeId;
        selectedPolylines.filter((line) => {
            return line.routeId === routeId
        }).forEach((line) => line.setStyle({'color': 'white'}));
    }

    function createMap(container) {
        let m = L.map(container).setView([38.71856, -9.13720], 10);

        let selectorCircle = L.circle([51.508, -0.11], {
            color: 'green',
            fillColor: '#12ff00',
            fillOpacity: 0.2,
            radius: 500
        }).addTo(mapLayers.selectionArea);

        m.on('mousemove', (e) => {
            selectorCircle.setLatLng(e.latlng);
        });

        m.on('zoomend', (e) => {
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
        });

        selectorCircle.on('click', (e) => {
            let center = selectorCircle.getLatLng();

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
                loadAggregateMap(selectedMarkers.map(m => {
                    return m.stopId
                }))
            }

        });


        // L.tileLayer('https://tile.openstreetmap.bzh/br/{z}/{x}/{y}.png', {
        L.tileLayer('https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png', {
            maxZoom: 19,
            subdomains: ['a', 'b'],
            attribution: '© OpenStreetMap'
        }).addTo(m);

        m.maxBounds = new L.LatLngBounds(new L.LatLng(38.3, -10.0), new L.LatLng(39.35, -8.0));
        m.maxBoundsViscosity = 1.0;
        m.minZoom = 10;

        L.control.scale().addTo(m);


        info.onAdd = function (map) {
            this._div = L.DomUtil.create('div', 'info'); // create a div with a class "info"
            this.update();
            return this._div;
        };

        // method that we will use to update the control based on feature properties passed
        info.update = function (props) {
            if (props) {
                this._div.innerHTML = '<b>' + props.name + '</b><br />';
            }
        };
        info.addTo(m);


        var legend = L.control({position: 'bottomleft'});

        legend.onAdd = function (map) {
            const div = L.DomUtil.create('div', 'info legend');
            div.innerHTML = '' +
                '<i style="background:#f59f00"></i>Area 1<br>' +
                '<i style="background:#0ca678"></i>Area 2<br>' +
                '<i style="background:#ff6b00"></i>Area 3<br>' +
                '<i style="background:#228be6"></i>Area 4<br>' +
                '<i style="background:#abb3bb"></i>Independente';

            return div;
        };

        legend.addTo(m);

        return m;
    }

    function mapAction(container) {
        map = createMap(container);

        return {
            destroy: () => {
                map.remove();
                map = null;
            }
        };
    }
</script>


<link rel="stylesheet" href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css"
      integrity="sha512-xwE/Az9zrjBIphAcBb3F6JVqxf46+CDLwfLMHloNu6KEQCAWi6HcDUbeOfBIptF7tcCzusKFjFw2yuvEpDL9wQ=="
      crossorigin=""/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.css"/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.Default.css"/>
<link rel="stylesheet" href="/map.css"/>


<div>

</div>
<div class="{'map ' + (selectedRoute ? 'hide' : '')}" use:mapAction></div>

{#if selectedRoute}
    <Route bind:routeId={selectedRoute}
           bind:spiderMap={currentSpider}
           bind:cache={cache}
           on:close={() => {selectedRoute = undefined}}/>
{:else }
    <RouteListing
            bind:routes={selectedRoutes}
            on:openroute={openRoute}
            on:hint={hintRoute}
            on:drophint={dropRouteHint}/>
{/if}

<style>
    .map {
        height: 650px;
        border-radius: 12px;
    }

    .hide {
        display: none
    }
</style>