<script>
    import Box from '../components/Box.svelte';
    import LineStopsEditor from './LineStopsEditor.svelte';
    import {api_server} from "../../settings.js";

    export let token;
    export let stops;
    let lines;
    let selectedLine;
    let selectedStop;

    let pendingOps = [];


    export function selectStop(stopId) {
        console.log(stopId);
        $: selectedStop = stopId;
    }

    function loadLines() {
        fetch(`${api_server}/api/routes`)
            .then(r => r.json())
            .then(data => {
                console.log(data);
                lines = Object.fromEntries(data.map(line => [line.id, line]));
                // data.forEach(node => {
                //     if (node.lat != null && node.lon != null) {
                //         let marker = createStopMarker(node);
                //         mapLayers.stops.addLayer(marker);
                //     }
                // });
                //
                // map.addLayer(mapLayers.stops);
            });
    }

    loadLines();
</script>

<Box padded="true">

    {#if lines}
        <label>
            Selected line:
            <select bind:value={selectedLine}>
                {#each Object.values(lines) as line}
                    <option value={line.id}>
                        {line.flag}
                    </option>
                {/each}
            </select>
        </label>

        <span style="float:right">
            Selected stop:
            {#if selectedStop}
                {stops[selectedStop].name} ({stops[selectedStop].id})
            {:else}
                None
            {/if}
        </span>

        {#if selectedLine}
            <h2>{lines[selectedLine].flag} ({lines[selectedLine].id})</h2>
            <h3>Subroutes</h3>
            <ul>
                {#each lines[selectedLine].subroutes as subline}
                    <li>
                        <h4>{subline.flag} ({subline.id})</h4>
                    </li>
                {/each}
            </ul>
        {/if}
    {/if}
</Box>

<style>
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
</style>