<script>
    import {flip} from 'svelte/animate';
    import {api_server} from "../../settings.js";
    import {createEventDispatcher} from 'svelte';

    export let routes;
    export let stops;
    export let selectedSubrouteStops;
    export let token;

    let result;

    $: stopList = selectedSubrouteStops ? selectedSubrouteStops.stops : [];


    const dispatch = createEventDispatcher();

    function moveUp(i) {

    }

    function moveDown(i) {

    }

    function replaceStop() {
        if (confirm("O rly?")) {
            // TODO
        }
    }

    function goTo(i) {
        dispatch('goto', {lon: stops[stopList[i]].lon, lat: stops[stopList[i]].lat});
    }
</script>

<div class="list">
    {#if stopList }
        {#each stopList as stop, index }
            <div class="list-item">
                <a on:click={() => {goTo(index)}}>({stops[stop].source}{stop})
                    - {stops[stop].name || stops[stop].short_name}</a>
                <div class="controls">
                    {#if index > 0}<a>🡹</a>{/if}
                    {#if index !== stopList.length - 1}<a>🡻</a>{/if}
                    <a>⮰</a>
                    <a>❌</a>
                </div>
            </div>
        {/each}
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
</style>