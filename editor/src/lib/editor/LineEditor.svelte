<script>
    import Box from "../components/Box.svelte";
    import LineStopsEditor from "./LineStopsEditor.svelte";
    import {api_server} from "../../settings.js";
    import {icons} from "./assets.js";
    import L from "leaflet";
    import {calc_route_multipoly} from "../../utils.js";
    import {writable} from "svelte/store";

    let map;

    let stops;
    let routes;
    let subroutes;
    let selectedStop;
    let selectedRoute;
    let selectedRouteStops;
    let selectedSubroute;
    let stopsEditor;

    const selectedSubrouteStops = writable(undefined);

    $: loading = !(stops && routes && map);
    $: subroutes = selectedRoute
        ? Object.fromEntries(
            routes[selectedRoute].subroutes.map((subroute) => [
                subroute.id,
                subroute,
            ])
        )
        : undefined;

    $: selectedRoute && loadRouteStops();

    $: selectedSubroute &&
    selectedRouteStops &&
    selectedSubrouteStops.set(selectedRouteStops[selectedSubroute]);

    let mapLayers = {
        stops: L.layerGroup(),
        subrouteDrawing: L.layerGroup(),
    };

    selectedSubrouteStops.subscribe((value) => {
        if (value) {
            let polyLine = drawSubroute(value.stops);
            let bounds = polyLine.getBounds();
            if (polyLine && bounds.isValid()) {
                map.fitBounds(bounds);
            }
        }
    });

    function loadRoutes() {
        fetch(`${api_server}/api/routes`)
            .then((r) => r.json())
            .then((data) => {
                routes = Object.fromEntries(data.map((line) => [line.id, line]));
                // selectedRoute = Object.keys(routes)[0];
            })
            .catch((e) => alert("Failed to load the routes"));
    }

    loadRoutes();

    function loadRouteStops() {
        fetch(`${api_server}/api/routes/${selectedRoute}/stops?all=true`)
            .then((r) => r.json())
            .then((data) => {
                selectedRouteStops = Object.fromEntries(
                    data.map((subroute) => [subroute.subroute, subroute])
                );
            })
            .catch((e) => alert("Failed to load the route stops"));
    }

    function drawSubroute(stop_ids) {
        let segments = calc_route_multipoly(stops, stop_ids);

        mapLayers.subrouteDrawing.removeFrom(map);
        mapLayers.subrouteDrawing = L.layerGroup();
        if (segments) {
            let polyLine = L.polyline(segments, {color: "red"}).addTo(
                mapLayers.subrouteDrawing
            );

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
            marker = L.marker(
                [info.lat, info.lon],
                Object.assign({}, markerOptions, {icon: icons[info.source]})
            );
        }

        marker.stopId = info.id;
        marker.meta = info;

        marker.on("click", (e) => {
            $: selectedStop = e.target.stopId;
        });

        let name = info.name || info.short_name;

        marker.bindTooltip(`${info.id} - ${name}`);

        return marker;
    }

    function loadStops() {
        fetch(`${api_server}/api/stops?all=true`)
            .then((r) => r.json())
            .catch((e) => {
                alert("Failed to load the stop list");
                console.log(e);
            })
            .then((data) => {
                stops = Object.fromEntries(data.map((stop) => [stop.id, stop]));
                map.removeLayer(mapLayers.stops);
                mapLayers.stops = L.markerClusterGroup({
                    // spiderfyOnMaxZoom: false,
                    showCoverageOnHover: false,
                    disableClusteringAtZoom: 16,
                });
                data.forEach((node) => {
                    if (node.lat != null && node.lon != null) {
                        let marker = createStopMarker(node);
                        mapLayers.stops.addLayer(marker);
                    }
                });

                map.addLayer(mapLayers.stops);
            });
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

            contextmenuWidth: 140
        }).setView([38.71856, -9.13720], 10);

        let osm = L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
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
            }
        };
    }

    function saveSubrouteStops(e) {
        fetch(`${api_server}/api/routes/${selectedRoute}/stops/subroutes/${selectedSubroute}`, {
                method: "PATCH",
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    from: {
                        stops: selectedRouteStops[selectedSubroute].stops,
                        diffs: selectedRouteStops[selectedSubroute].diffs,
                    },
                    to: {
                        stops: e.detail.stops,
                        diffs: e.detail.diffs,
                    }
                })
            }
        ).then(resp => {
            alert("We're good");
        }).catch(
            e => {
                alert("Error saving");
                console.log(e);
            }
        );
    }
</script>

<div class="hwrapper">
    <div>
        <Box>
            <div class="map" use:mapAction/>
        </Box>
    </div>
    <div>
        {#if loading}
            A carregar os dados
        {:else}
            <label>
                Selected line:
                <select bind:value={selectedRoute}>
                    {#each Object.values(routes) as line}
                        <option value={line.id}>
                            {line.code} - {line.name.substring(0, 60)}
                        </option>
                    {/each}
                </select>
            </label><br/>
            <span
            >Selected stop:
                {#if selectedStop}{stops[selectedStop].name} ({stops[selectedStop]
                    .id}){:else}None{/if}</span
            >
            {#if selectedRoute}
                <h2>
                    {routes[selectedRoute].code} - {routes[selectedRoute].name} ({routes[
                    selectedRoute
                    ].id})
                </h2>
                <select bind:value={selectedSubroute}>
                    {#each routes[selectedRoute].subroutes as subroute}
                        <option value={subroute.id}>{subroute.flag.substring(0, 60)}</option
                        >
                    {/each}
                </select>
                {#if selectedSubroute && subroutes[selectedSubroute]}
                    {#if selectedRouteStops}
                        <LineStopsEditor
                                {routes}
                                {stops}
                                bind:this={stopsEditor}
                                bind:selectedStop
                                {selectedSubrouteStops}
                                on:goto={goTo}
                                on:redraw={redraw}
                                on:savesubroutestops={saveSubrouteStops}
                        />
                    {/if}
                {/if}
            {/if}
        {/if}
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
        height: calc(100vh - 50px);
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

