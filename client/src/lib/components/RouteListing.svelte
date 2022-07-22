<script>
    import {createEventDispatcher} from "svelte";

    export let routes = undefined;
    $: routeEntries = routes && Object.entries(routes);

    const dispatch = createEventDispatcher();

    function onClick(routeId) {
        dispatch('openroute', {routeId: parseInt(routeId)});
    }

    function onEnter(routeId) {
        dispatch('hint', {routeId: parseInt(routeId)});
    }

    function onLeave(routeId) {
        dispatch('drophint', {routeId: parseInt(routeId)});
    }
</script>


<div class="content-wrapper">
    <h2>Rotas</h2>
    {#if routes}
        {#each routeEntries as [id, info]}
            <span class="route"
                  data-route-id="{id}"
                  aria-label="{info.name}"
                  data-balloon-pos="up"
                  on:click={() => onClick(id)}
                  on:mouseenter={() => onEnter(id)}
                  on:mouseleave={() => onLeave(id)}>
                {info.code}
            </span>
        {/each}
    {:else }
        Selecione uma região do mapa para visualizar as rotas existentes.
    {/if}
</div>

<style>
    .route {
        border-radius: 12px;
        font-weight: bold;
        color: white;
        background-color: darkred;
        padding: 4px 3px;
        margin: 2px 2px 5px 5px;
        cursor: pointer;
        font-size: 1.2em;
        display: inline-block;
    }

    .route:hover {
        border: 2px solid gray;
        margin: 0 0 3px 3px;
    }
</style>
<link rel="stylesheet" href="https://unpkg.com/balloon-css/balloon.min.css">