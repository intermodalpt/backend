<script>
    import {flip} from 'svelte/animate';
    import {api_server} from "../../settings.js";


    export let routeId;
    export let subrouteId;

    let list = [
        {name: "foo", id: 0},
        {name: "bar", id: 1},
        {name: "bob", id: 2},
        {name: "jean", id: 3}
    ];
    let stopList;

    let hovering = false;

    const drop = (event, target) => {
        event.dataTransfer.dropEffect = 'move';
        const start = parseInt(event.dataTransfer.getData("text/plain"));
        const newStopList = list

        if (start < target) {
            newStopList.splice(target + 1, 0, newStopList[start]);
            newStopList.splice(start, 1);
        } else {
            newStopList.splice(target, 0, newStopList[start]);
            newStopList.splice(start + 1, 1);
        }
        list = newStopList
        hovering = null
    }

    const dragstart = (event, i) => {
        // `i` is the starting index
        event.dataTransfer.effectAllowed = 'move';
        event.dataTransfer.dropEffect = 'move';
        event.dataTransfer.setData('text/plain', i);
    }


    fetch(`${api_server}/api/routes/${routeId}/stops`)
        .then(r => r.json())
        .then(data => {
            stopList = data
        });

</script>

<div class="list">
    {#each list as n, index  (n.name)}
        <div
                class="list-item"
                animate:flip
                draggable={true}
                on:dragstart={event => dragstart(event, index)}
                on:drop|preventDefault={event => drop(event, index)}
                ondragover="return false"
                on:dragenter={() => hovering = index}
                class:is-active={hovering === index}>
            {n.name}
        </div>
    {/each}
</div>

<style>
    .list {
        background-color: white;
        border-radius: 4px;
        box-shadow: 0 2px 3px rgba(10, 10, 10, 0.1), 0 0 0 1px rgba(10, 10, 10, 0.1);
    }

    .list-item {
        display: block;
        padding: 0.5em 1em;
    }

    .list-item:not(:last-child) {
        border-bottom: 1px solid #dbdbdb;
    }

    .list-item.is-active {
        background-color: #3273dc;
        color: #fff;
    }
</style>