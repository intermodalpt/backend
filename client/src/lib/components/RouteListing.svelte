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
    <h2 style="margin-top: 0.9rem; margin-bottom: 0.6rem">Rotas</h2>
    {#if routes}
        <div class="grid">
            {#each routeEntries as [id, info]}
                <div>
                    <span class="route"
                          on:click={() => onClick(id)}
                          on:mouseenter={() => onEnter(id)}
                          on:mouseleave={() => onLeave(id)}>
                        <span class="operator"><img src="/logos/cmet-min.svg" alt="CM"/></span>
                        <span class="code">{info.code}</span>
                    </span>
                    <span class="name">{info.name}</span>
                </div>
            {/each}
        </div>
    {:else }
        Selecione uma região do mapa para visualizar as rotas existentes.
    {/if}
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(25rem, 1fr));
        grid-gap: 0.2rem;
        max-height: 200px;
        overflow-y: scroll;


        background: linear-gradient(white 30%, rgba(255, 255, 255, 0)) center top,
        linear-gradient(rgba(255, 255, 255, 0), white 70%) center bottom,
        radial-gradient(farthest-side at 50% 0, rgba(0, 0, 0, 0.2), rgba(0, 0, 0, 0)) center top,
        radial-gradient(farthest-side at 50% 100%, rgba(0, 0, 0, 0.2), rgba(0, 0, 0, 0)) center bottom;

        background-repeat: no-repeat;
        background-size: 100% 40px, 100% 40px, 100% 14px, 100% 14px;
        background-attachment: local, local, scroll, scroll;

    }

    .route {
        border-radius: 12px 16px 16px 12px;
        margin: 2px 2px 5px 5px;
        cursor: pointer;
        min-width: 0;
        display: inline-block;
        box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.03), 0 1px 3px 1px rgba(0, 0, 0, 0.07), 0 2px 6px 2px rgba(0, 0, 0, 0.03);
    }

    .code {
        border-radius: 14px;
        font-weight: bold;
        color: white;
        background-color: darkred;
        padding: 4px 3px;
        font-size: 1.2rem;
        display: inline-block;
    }

    .name {
        display: inline-block;
        overflow: auto;
    }

    .operator img {
        max-height: 16px;
    }
</style>
<link rel="stylesheet" href="https://unpkg.com/balloon-css/balloon.min.css">