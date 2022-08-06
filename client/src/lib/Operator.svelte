<script>
    import {routes, stops} from "../cache.js";
    import RouteMap from "./components/Route.svelte";
    import {writable} from "svelte/store";

    export let operator;

    const selectedRoute = writable(undefined);
</script>


<div>
  <slot></slot>
</div>

<h2 class="text-2xl">Linhas</h2>
<div class="route-list">
  {#if ($selectedRoute)}
    <RouteMap routeId={selectedRoute}/>
  {:else}
    {#each $routes as route}
      <div class="code" on:click={() => $selectedRoute = route.id}>
        <span class="line-number">{route.code}</span>
        <span>{route.name}</span>
      </div>
    {/each}
  {/if}
</div>


<style>
    .line-number {
        background-color: red;
        padding: 0.2em 10px;
        border-radius: 1em;
        font-weight: 900;
        font-size: 1.2rem;
        display: inline-block;
    }
</style>