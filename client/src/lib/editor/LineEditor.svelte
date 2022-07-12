<script>
    import Box from '../components/Box.svelte';
    import LineStopsEditor from './LineStopsEditor.svelte';
    import {api_server} from "../../settings.js";
    import {icons} from "./assets.js";
    import {createEventDispatcher} from 'svelte';
    import L from "leaflet";
    import {calc_route_multipoly} from "../../utils.js";

    let map;
    let token;

    let stops;
    let routes;
    let subroutes;
    let selectedStop;
    let selectedRoute;
    let selectedSubroute;

    $: loading = stops == undefined || routes == undefined || map == undefined;
    $: subroutes = selectedRoute ? Object.fromEntries(routes[selectedRoute].subroutes.map(subroute => [subroute.id, subroute])) : undefined;

    $: selectedRoute && loadRouteStops();

    let selectedRouteStops;
    $: selectedSubrouteStops = selectedSubroute && selectedRouteStops && selectedRouteStops[selectedSubroute];


    $: selectedSubroute && selectedSubrouteStops && selectedRouteStops[selectedSubroute] && drawSubroute();

    let mapLayers = {
        stops: L.layerGroup(),
        subrouteDrawing: L.layerGroup()
    };


    function loadRoutes() {
        fetch(`${api_server}/api/routes`)
            .then(r => r.json())
            .then(data => {
                routes = Object.fromEntries(data.map(line => [line.id, line]));
                // selectedRoute = Object.keys(routes)[0];
            });
    }

    loadRoutes();

    function loadRouteStops() {
        fetch(`${api_server}/api/routes/${selectedRoute}/stops`)
            .then(r => r.json())
            .then(data => {
                selectedRouteStops = Object.fromEntries(data.map(subroute => [subroute.subroute, subroute]));
            });
    }

    function drawSubroute() {
        let segments = calc_route_multipoly(stops, selectedRouteStops[selectedSubroute].stops);

        mapLayers.subrouteDrawing.removeFrom(map);
        mapLayers.subrouteDrawing = L.layerGroup()
        let polyLine = L.polyline(segments, {color: 'red'}).addTo(mapLayers.subrouteDrawing);

        mapLayers.subrouteDrawing.addTo(map);
        map.fitBounds(polyLine.getBounds());
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
        marker.meta = info;

        marker.on('click', (e) => {
            $: selectedStop = e.target.stopId;
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
        // let satellite = L.tileLayer('https://{s}.google.com/vt/lyrs=s,h&x={x}&y={y}&z={z}', {
        //     maxZoom: 19,
        //     attribution: '© Google; Do not use this',
        //     subdomains: ['mt0', 'mt1', 'mt2', 'mt3']
        // }).addTo(m);

        let baseMaps = {
            "OSM": osm,
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
            }
        };
    }

    function goTo(e) {
        if (e.detail.lat && e.detail.lon) {
            map.setView([e.detail.lat, e.detail.lon], 17)
        }
    }
</script>


<div class="hwrapper">
    <div>
        <Box>
            <div class="map" use:mapAction></div>
        </Box>
    </div>
    <div>
        <Box padded="true">
            {#if loading}
                A carregar os dados
            {:else }
                <label>
                    Selected line:
                    <select bind:value={selectedRoute}>
                        {#each Object.values(routes) as line}
                            <option value={line.id}>
                                {line.code} - {line.name.substring(0, 60)}
                            </option>
                        {/each}
                    </select>
                </label><br>
                <span>Selected stop:
                    {#if selectedStop}{stops[selectedStop].name} ({stops[selectedStop].id}){:else}None{/if}</span>
                {#if selectedRoute}
                    <h2>{routes[selectedRoute].code} - {routes[selectedRoute].name} ({routes[selectedRoute].id})</h2>
                    <select bind:value={selectedSubroute}>
                        {#each routes[selectedRoute].subroutes as subroute}
                            <option value={subroute.id}>{subroute.flag.substring(0, 60)}</option>
                        {/each}
                    </select>
                    <!--                    <pre>{selectedRoute}</pre>-->
                    <!--                    <pre>{selectedSubroute}</pre>-->
                    <!--                    <textarea>{JSON.stringify(subroutes)}</textarea>-->
                    {#if selectedSubroute && subroutes[selectedSubroute]}
                        <!--    (id {selectedSubroute})<br>-->
                        <!--    <h4>-->
                        <!--        {subroutes[selectedSubroute].flag} ({subroutes[selectedSubroute].id})-->
                        <!--    </h4>-->

                        {#if selectedRouteStops}
                            <LineStopsEditor
                                    routes={routes} stops={stops}
                                    bind:selectedSubrouteStops={selectedSubrouteStops}
                                    token={token}
                                    on:goto={goTo}/>
                        {/if}
                    {/if}
                {/if}
            {/if}
        </Box>
    </div>
</div>

<style>
    .hwrapper {
        display: flex;
    }

    .hwrapper div:first-child {
        flex-basis: max(60vw, 600px);
        flex-shrink: 0;
    }

    .hwrapper div:nth-of-type(2) {
        margin-left: 10px;
        flex-grow: 1;
        overflow: auto;
    }

    .map {
        height: 650px;
        border-radius: 12px;
        cursor: crosshair !important;
    }

    .pending-changes li {
        display: flex;
        justify-content: space-between;
    }

    .pending-changes button {
        width: 60px;
    }

    .pending-changes .changes {
        display: flex;
        flex-direction: column;
    }

    .pending-changes .changes .title {
        font-size: 1.2em;
        font-weight: bold;
    }

    .subroute {
        list-style: none;
    }
</style>