<script>
    import {createEventDispatcher} from 'svelte';

    export let routes;
    export let stops;
    export let selectedStop;
    export let selectedSubrouteStops;
    let addAfterIndex = 0;

    let changes = false;
    const dispatch = createEventDispatcher();

    // Deep-ish data copies to leave the original intact. (Sorry for the shitty code)
    let stopList;
    let diffList;

    selectedSubrouteStops.subscribe(value => {
        if (value) {
            stopList = [...value.stops];
            diffList = [...value.diffs];
        }
    });

    $: stopList = redraw() || stopList;


    function moveUp(i) {
        let aux = stopList[i - 1];
        diffList
        let aux_diff = diffList[i - 1];
        stopList[i - 1] = stopList[i];
        diffList[i - 1] = diffList[i];
        stopList[i] = aux;
        diffList[i] = aux_diff;
        stopList = stopList;
        diffList = diffList;
        changes = true;
    }

    function moveDown(i) {
        let aux = stopList[i + 1];
        let aux_diff = diffList[i + 1];
        stopList[i + 1] = stopList[i];
        diffList[i + 1] = diffList[i];
        stopList[i] = aux;
        diffList[i] = aux_diff;
        stopList = stopList;
        diffList = diffList;
        changes = true;
    }

    function addStop() {
        if (selectedStop === undefined) {
            alert("Select a stop first...");
            return
        }

        // stopList.indexOf(addAfterIndex)
        if (confirm(`Do you want to add a stop after ${stopList[addAfterIndex]}?`)) {
            stopList.splice(addAfterIndex + 1, 0, selectedStop);
            diffList.splice(addAfterIndex + 1, 0, 0);
            stopList = stopList;
            diffList = diffList;
        }
    }

    function replaceStop(i) {
        if (selectedStop === undefined) {
            alert("Select another stop first...");
            return
        }

        // if (selectedSubrouteStops.stops.includes(selectedStop)) {
        //     if (!confirm("Route already has this stop. Are you totally sure?")) {
        //         return;
        //     }
        // }


        // if (confirm(`Do you want to replace ${stops[stopList[i]].name} with ${stops[selectedStop].name}?`)) {
        if (confirm(`"${stops[stopList[i]].short_name}":[["${stops[selectedStop].source}", "${stops[selectedStop].source === 'osm' ? stops[selectedStop].external_id : stops[selectedStop].name}"}]],`)) {
            stopList[i] = selectedStop;
            stopList = stopList;
            changes = true;
        }
    }

    function removeStop(i) {
        if (confirm(`Do you want to remove ${stops[stopList[i]].name} from this route?`)) {
            stopList.splice(i, 1);
            let removedDiff = diffList.splice(i, 1)[0];
            if (diffList.length > 0) {
                if (i === 0) {
                    diffList[0] += removedDiff;
                } else if (i === diffList.length) {
                    // Discard this diff as there's no dist to the next stop
                    diffList[diffList.length - 1] = null;
                } else {
                    diffList[i - 1] += removedDiff;
                }
            }
            stopList = stopList;
            diffList = diffList;
            changes = true;
        }
    }

    function goTo(i) {
        dispatch('goto', {lon: stops[stopList[i]].lon, lat: stops[stopList[i]].lat});
    }

    function redraw(i) {
        dispatch('redraw', {stops: stopList});
    }

    function save() {
        dispatch('savesubroutestops', {stops: stopList, diffs: diffList});
    }
</script>

<div class="list">
    {#if stopList }
        {#each stopList as stop, index }
            <div class="list-item">
                <a on:click={() => {goTo(index)}}>({stops[stop].source}{stop})
                    - {stops[stop].name || stops[stop].short_name}</a>
                + <input class="time-diff" type="number" maxlength="2" max="99" bind:value={diffList[index]}/>
                <div class="controls">
                    {#if index > 0}<a on:click={() => {moveUp(index)}}>🡹</a>{/if}
                    {#if index !== stopList.length - 1}<a on:click={() => {moveDown(index)}}>🡻</a>{/if}
                    <a on:click={() => {replaceStop(index)}}>⮰</a>
                    <a on:click={() => {removeStop(index)}}>❌</a>
                </div>
            </div>
        {/each}
        <input type="button" value="Add" on:click={addStop}/> after
        <!--        <input type="number" min="0" max="{stopList.length}" bind:value={addAfterIndex}/>-->
        <select bind:value={addAfterIndex}>
            {#each stopList as stop, index }
                <option value="{index}">{stops[stop].short_name || stops[stop].name || stop}</option>
            {/each}
        </select>
        {#if changes}
            <input type="button" value="Save" on:click={save}/>
        {/if}
    {/if}
</div>

<style>
    .list {
        background-color: white;
        border-radius: 4px;
        box-shadow: 0 2px 3px rgba(10, 10, 10, 0.1), 0 0 0 1px rgba(10, 10, 10, 0.1);
    }

    .list-item {
        display: flex;
        justify-content: space-between;
        padding: 2px 5px;
    }

    .list-item a {
        border-radius: 4px;
        padding: 3px;
        border: 1px solid black;
        cursor: pointer;
    }

    .list-item:not(:last-child) {
        border-bottom: 1px solid #dbdbdb;
    }

    .list-item.is-active {
        background-color: #3273dc;
        color: #fff;
    }

    .time-diff {
        width: 40px;
    }
</style>