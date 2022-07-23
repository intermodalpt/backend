<script>
    import {createEventDispatcher} from "svelte";
    import L from "leaflet";
    import {api_server} from "../settings.js";
    import {calc_route_multipoly} from "../utils.js";

    export let spiderMap;
    export let routeId;
    export let cache;

    let tab = 0;

    let stops;
    let stopIndex;
    let route = cache.routes.find(r => r.id === routeId);

    let selectedSubrouteId = route.subroutes[0].id;
    let subroute = route.subroutes.find(sr => sr.id === selectedSubrouteId);
    let subrouteStops;

    let map;
    let subrouteLayer = L.layerGroup();

    const dispatch = createEventDispatcher();
    let selectedStop = 0;
    let selectedStopMarker;


    $: subrouteStops && map && drawSubroute();


    fetch(`${api_server}/api/routes/${routeId}/stops`)
        .then(r => r.json())
        .then(data => {
            data.forEach(sr => sr.stops.map(stopId => cache.stops[stopId]));
            stops = data;
            subrouteStops = stops.find(stops => {
                return stops.subroute === subroute.id
            });
            stopIndex = Object.fromEntries(subrouteStops.stops.map(stop => [stop, cache.stops[stop]]));
        });


    let stopIcon = L.icon({
        iconUrl: `/public/markers/bus-minimal.svg`,
        iconSize: [16, 16],
        iconAnchor: [8, 8],
    });

    function close() {
        dispatch('close');
    }

    function drawSubroute() {
        let stops = cache.stops;

        subrouteLayer.removeFrom(map);
        subrouteLayer = L.layerGroup();


        let segments = calc_route_multipoly(stops, subrouteStops.stops);

        let outerPolyline = L.polyline(segments, {color: 'black', weight: 6}).addTo(subrouteLayer);
        let innerPolyline = L.polyline(segments, {color: 'white', weight: 4}).addTo(subrouteLayer);
        subrouteLayer.addTo(map);
        let bounds = outerPolyline.getBounds();
        if (bounds.isValid()) {
            map.fitBounds(bounds);
        }


        for (let i = 0; i < subrouteStops.stops.length; i++) {
            let stop = stops[subrouteStops.stops[i]];
            let diff = subrouteStops.diffs[i];

            if (stop.lat && stop.lon) {
                let marker = L.marker([stop.lat, stop.lon], {icon: stopIcon});
                marker.on("click", () => {
                    selectStop(stop.id);
                });
                marker.addTo(subrouteLayer);
            }
        }
    }

    function selectStop(stopId) {
        let stop = cache.stops[stopId];
        if (stop.lat && stop.lon) {
            map.once("moveend zoomend", () => {
                selectedStop = stopId;
            })
            map.setView(new L.LatLng(stop.lat, stop.lon), 16);

            if (selectedStopMarker) {
                selectedStopMarker.removeFrom(map);
            }
            selectedStopMarker = L.marker([stop.lat, stop.lon]);
            selectedStopMarker.addTo(subrouteLayer);
        }
        selectedStop = stopId;

    }

    function changeSubroute(e) {
        subroute = route.subroutes.find(sr => sr.id === selectedSubrouteId);
        subrouteStops = stops.find(stops => {
            return stops.subroute === subroute.id
        });
    }

    function createMap(container) {
        let m = L.map(container).setView([38.71856, -9.13720], 10);

        L.tileLayer('https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png', {
            maxZoom: 19,
            subdomains: ['a', 'b'],
            attribution: '© OpenStreetMap'
        }).addTo(m);

        m.maxBounds = new L.LatLngBounds(new L.LatLng(38.3, -10.0), new L.LatLng(39.35, -8.0));
        m.maxBoundsViscosity = 1.0;
        m.minZoom = 10;

        m.on("movestart", () => {
            selectedStop = undefined;
            if (selectedStopMarker) {
                selectedStopMarker.removeFrom(map);
            }
        });

        L.control.scale().addTo(m);

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

<div class="header">
    <span class="code">{route.code}</span>
    <div class="title-sr-pair">
        <span class="title">{route.name}</span>
        <select bind:value={selectedSubrouteId} on:change={changeSubroute}>
            {#each route.subroutes as subroute}
                <option value={subroute.id}>{subroute.flag}</option>
            {/each}
        </select>
    </div>
    <span style="flex-grow: 1"></span>
    <span class="close" on:click={close}>x</span>
</div>

<div>
    <div class="tabs">
        <span class="tab" class:selected="{tab === 0}" on:click={() => {tab = 0}}>Percurso</span>
        <span class="tab" class:selected="{tab === 1}" on:click={() => {tab = 1}}>Horário</span>
    </div>
    {#if tab === 0}
        <div class="route-map">
            <div class="stops-pane">
                <div class="stops-header">Paragens (⬇)</div>
                <ul class="stop-list">
                    {#if subrouteStops}
                        {#each subrouteStops.stops as stop, i}
                            <li class="stop" class:selected="{selectedStop === stop}"
                                on:click={() => selectStop(stop)}>
                                {cache.stops[stop].short_name}
                            </li>
                        {/each}
                    {/if}
                </ul>
                <div class="stops-footer"></div>
            </div>
            <div class="map" use:mapAction></div>
        </div>
    {:else if tab === 1}
        <div class="schedule">
            <div>Não faças hoje o que podes fazer amanhã</div>
            <table>
                <thead>
                <tr>
                    <td>04</td>
                    <td>05</td>
                    <td>06</td>
                    <td>07</td>
                    <td>08</td>
                    <td>09</td>
                    <td>10</td>
                    <td>11</td>
                    <td>12</td>
                    <td>13</td>
                    <td>14</td>
                    <td>15</td>
                    <td>16</td>
                    <td>17</td>
                    <td>18</td>
                    <td>19</td>
                    <td>20</td>
                    <td>21</td>
                    <td>22</td>
                    <td>23</td>
                    <td>00</td>
                    <td>01</td>
                    <td>02</td>
                </tr>
                </thead>
            </table>
        </div>
    {/if}
</div>


<style>
    .header {
        font-size: 2rem;
        display: flex;
        align-items: start;
        margin-bottom: 20px;
    }

    .title-sr-pair {
        display: flex;
        flex-direction: column;
    }

    .close {
        padding: 5px 15px;
        background-color: #ce5252;
        border-radius: 8px;
        border: 2px solid #ad1717;
    }

    .code {
        border-radius: 32px;
        font-weight: bold;
        color: white;
        background-color: darkred;
        padding: 0.1em 0.3em;
        font-size: 1.2em;
        display: inline-block;
        margin-right: 20px;
    }

    .route-map {
        display: flex;
    }

    .stops-pane {
        width: 300px;
        background-color: #e5e9f0;
        height: 650px;
        border-bottom-left-radius: 12px;
        display: flex;
        flex-direction: column;
        align-content: stretch;
    }

    .stop-list {
        margin: 0;
        padding: 0;
        flex-grow: 1;
        overflow: auto;
    }

    .stop-list li {
        list-style: none;
    }

    .stops-header {
        background-color: #d8dee9;
        font-size: 1.2rem;
        font-weight: bold;
        padding: 0.5rem 1rem;
    }

    .stops-footer {
        background-color: #d8dee9;
        height: 12px;
        min-height: 12px;
        border-bottom-left-radius: 12px;
    }

    .stop {
        display: block;
        background-color: #eceff4;
        padding: 1rem;
        border-top: 2px solid #d8dee9;
        cursor: pointer;
    }

    .stop.selected {
        background-color: #d8dee9;
    }

    .map {
        height: 650px;
        border-top-right-radius: 12px;
        border-bottom-right-radius: 12px;
        flex-grow: 1;
    }

    .tabs {
        display: flex;
        align-items: end;
    }

    .tab {
        font-weight: bold;
        font-size: 1.3rem;
        display: inline-block;
        color: white;
        background-color: #4c566a;
        padding: 0.8rem 3rem;
        border-top-left-radius: 12px;
        border-top-right-radius: 12px;
        border: 2px solid #d8dee9;
        border-bottom: 0;
        margin-right: -12px;
        z-index: 1;
        position: relative;
    }

    .tab.selected {
        color: black;
        background-color: #e5e9f0;
        z-index: 1000;
        padding: 1.0rem 3rem;
    }

    .schedule {
        background-color: #e5e9f0;
        min-height: 300px;
        border-radius: 0 12px 12px 12px;
    }
</style>