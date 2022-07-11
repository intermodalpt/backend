<script>
    import Box from '../components/Box.svelte';
    import StopEditor from './StopEditor.svelte';
    import LineEditor from './LineEditor.svelte';

    import L from 'leaflet';
    import 'leaflet.markercluster';
    import 'leaflet-contextmenu';
    import {api_server} from "../../settings.js";

    let map;
    let stops;
    let stopEditor;
    let lineEditor;

    let sources = ["carris", "cmet", "tcb", "tst", "mobicascais", "osm", "geoc"];
    let icons = {};
    let token = sessionStorage.getItem("authToken");

    let mode = 1;

    if (token == null) {
        token = prompt("Insert the authorization token");
        sessionStorage.setItem("authToken", token);
    }

    for (let source of sources) {
        icons[source] = L.icon({
            iconUrl: `/src/assets/markers/${source}.svg`,
            iconSize: [32, 32],
            iconAnchor: [16, 31],
        });
    }


    let mapLayers = {
        parishes: L.layerGroup(),
        municipalities: L.layerGroup(),
        stops: L.layerGroup()
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

        marker.on('dragend', (e) => {
            if (mode === 1) {
                stopEditor.moveStop(e);
            } else if (mode === 2) {

            }
        });

        marker.on('click', (e) => {
            if (mode === 1) {
                stopEditor.selectStop(e.target.stopId);
            } else if (mode === 2) {
                lineEditor.selectStop(e.target.stopId);
            }
        });

        let name = info.name || info.short_name;

        marker.bindTooltip(`${info.id} - ${name}`);

        return marker;
    }

    function loadStops() {
        fetch(`${api_server}/api/stops`)
            .then(r => r.json())
            .then(data => {
                stops = Object.fromEntries(data.map(stop => [stop.id, stop]));
                map.removeLayer(mapLayers.stops);
                mapLayers.stops = L.markerClusterGroup({
                    // spiderfyOnMaxZoom: false,
                    showCoverageOnHover: false,
                    disableClusteringAtZoom: 16
                })
                data.forEach(node => {
                    if (node.lat != null && node.lon != null) {
                        let marker = createStopMarker(node);
                        mapLayers.stops.addLayer(marker);
                    }
                });

                map.addLayer(mapLayers.stops);
            });
    }


    function createStop(e) {
        let stop = {
            "source": "iml",
            "lat": e.latlng.lat,
            "lon": e.latlng.lng
        };
        fetch(`${api_server}/api/stops/create`, {
                method: "post",
                headers: {
                    'Content-Type': 'application/json',
                    'Token': token
                },
                body: JSON.stringify(stop)
            }
        ).then(r => r.json())
            .then(data => {
                stop.id = data.id;
                let marker = createStopMarker(stop);
                mapLayers.stops.addLayer(marker);
            });
    }

    function createMap(container) {
        let m = L.map(container, {
            contextmenu: true,

            contextmenuWidth: 140,
            contextmenuItems: [{
                text: 'Create a stop',
                callback: createStop
            }]
        }).setView([38.71856, -9.13720], 10);

        let osm = L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(m);
        let satellite = L.tileLayer('https://{s}.google.com/vt/lyrs=s,h&x={x}&y={y}&z={z}', {
            maxZoom: 19,
            attribution: '© Google; Do not use this',
            subdomains: ['mt0', 'mt1', 'mt2', 'mt3']
        }).addTo(m);

        let baseMaps = {
            "OSM": osm,
            "Satellite": satellite
        };
        L.control.layers(baseMaps).addTo(m);


        m.maxBounds = new L.LatLngBounds(new L.LatLng(38.3, -10.0), new L.LatLng(39.35, -8.0));
        m.maxBoundsViscosity = 1.0;
        m.minZoom = 10;
        m.setView([38.605, -9.0], 0);
        m.setZoom(11);

        loadStops();

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

<h1>Mapa</h1>
<Box>
    <div class="map" use:mapAction></div>
</Box>

<div>
    <label><input type=radio bind:group={mode} name="mode" value={1}>Stops</label>
    <label><input type=radio bind:group={mode} name="mode" value={2}>Lines</label>
</div>

{#if (mode === 1)}
    <StopEditor bind:this={stopEditor} bind:stops={stops} bind:token={token}></StopEditor>
{:else if (mode === 2) }
    <LineEditor bind:this={lineEditor} bind:stops={stops} bind:token={token}></LineEditor>
{:else }
    ???
{/if}


<style>
    .map {
        height: 650px;
        border-radius: 12px;
        cursor: crosshair !important;
    }
</style>

<link rel="stylesheet" href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css"
      integrity="sha512-xwE/Az9zrjBIphAcBb3F6JVqxf46+CDLwfLMHloNu6KEQCAWi6HcDUbeOfBIptF7tcCzusKFjFw2yuvEpDL9wQ=="
      crossorigin=""/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.css"/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.Default.css"/>
<link rel="stylesheet" href="/src/assets/css.css"/>
<link rel="stylesheet" href="/src/assets/leaflet-contextmenu.css"/>