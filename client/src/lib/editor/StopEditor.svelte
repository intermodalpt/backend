<script>
    import Box from '../components/Box.svelte';
    import StopForm from './StopForm.svelte';
    import L from "leaflet";
    import {api_server} from "../../settings.js";
    import {icons} from "./assets.js";

    export let token;

    let map;
    let stops;
    let selectedStop;

    let pendingOps = [];

    export function selectStop(stopId) {
        selectedStop = stops[stopId];
    }

    export function moveStop(e) {
        let stopId = e.target.stopId;
        let newPos = e.target.getLatLng();
        let existing = pendingOps.find(el => el.op === updateStop && el.stop.id === stopId);
        if (existing) {
            existing.stop.lat = newPos.lat;
            existing.stop.lon = newPos.lng;
        } else {
            let metaCopy = Object.assign({}, e.target.meta);
            metaCopy.lat = newPos.lat;
            metaCopy.lon = newPos.lng;
            pendingOps.push({
                op: updateStop,
                stop: e.target.meta
            });
            pendingOps = pendingOps;
        }
    }

    function saveStopInfo(e) {
        let stopId = e.detail.id;
        let existing = pendingOps.find(el => el.op === updateStop && el.stop.id === stopId);
        if (existing) {
            existing.stop = Object.assign(existing.stop, e.detail);
        } else {
            pendingOps.push({
                op: updateStop,
                stop: e.detail
            });
            pendingOps = pendingOps;
        }

        selectedStop = null;
    }

    function updateStop(stop) {
        fetch(`http://0.0.0.0:8080/api/stops/update/${stop.id}`, {
                method: "patch",
                headers: {
                    'Content-Type': 'application/json',
                    'Token': token
                },
                body: JSON.stringify(stop)
            }
        ).then(r => r.json())
            .then(data => {
                //
            });
    }


    let mapLayers = {
        parishes: L.layerGroup(),
        municipalities: L.layerGroup(),
        stops: L.layerGroup(),
        lineSeq: L.layerGroup()
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

        marker.on('dragend', (e) => moveStop(e));
        marker.on('click', (e) => selectStop(e.target.stopId));

        let name = info.name || info.short_name;

        marker.bindTooltip(`${info.id} - ${name}`);

        return marker;
    }

    function loadStops() {
        fetch(`${api_server}/api/stops`)
            .then(r => r.json())
            .then(data => {
                $: stops = Object.fromEntries(data.map(stop => [stop.id, stop]));
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
</script>


<div class="hwrapper">
    <div>
        <Box>
            <div class="map" use:mapAction></div>
        </Box>
    </div>
    <div>
        {#if selectedStop }
            <StopForm bind:stop={selectedStop} on:save={saveStopInfo}></StopForm>
        {:else }
            <p>Carregue ou arraste uma paragem para a editar.</p>
        {/if}

        <h2>Alterações</h2>
        <ul class="pending-changes">
            {#each pendingOps as op, i}
                {#if op.op === updateStop}
                    <li>
                        <div class="changes">
                            <span class="title">Atualização de paragem {op.stop.id} - {op.stop.name}</span>
                            <span>
                            {JSON.stringify(op)}
                        </span>
                        </div>
                        <button on:click={e => {pendingOps.splice(i, 1); pendingOps = pendingOps}}>Call it quitz
                        </button>
                    </li>
                {:else }
                    <li>???</li>
                {/if}
            {/each}
        </ul>
        <br>
        <input type="button" value="Guardar" disabled/>
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