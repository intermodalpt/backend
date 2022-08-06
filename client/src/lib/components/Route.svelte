<script>
  import { api_server } from "../../settings.js";
  import { routes, stops } from "../../cache.js";
  import RouteStops from "./RouteStops.svelte";
  import RouteMap from "./RouteMap.svelte";

  import { createEventDispatcher } from "svelte";
  import { derived, writable } from "svelte/store";

  export let routeId;

  const dispatch = createEventDispatcher();

  let routeStops = [];
  let route;

  const selectedSubrouteId = writable(undefined);
  let subroute;

  const subrouteStops = derived(selectedSubrouteId, ($selectedSubrouteId) => {
    return routeStops.find((stops) => {
      return stops.subroute === $selectedSubrouteId;
    });
  });

  let selectedStop = 0;

  let tab = 0;

  routeId.subscribe((routeId) => {
    if (routeId === undefined) {
      return;
    }
    // TODO make a derived store from this
    route = $routes.find((r) => {
      return r.id === routeId;
    });
    if (!route) {
      return;
    }

    fetch(`${api_server}/api/routes/${routeId}/stops`)
      .then((r) => r.json())
      .then((data) => {
        data.forEach((sr) => sr.stops.map((stopId) => stops[stopId]));
        routeStops = data;
        $selectedSubrouteId = route.subroutes[0].id;
      });
  });

  selectedSubrouteId.subscribe((subrouteId) => {
    if (!route) {
      return;
    }

    subroute = route.subroutes.find((sr) => {
      return sr.id === subrouteId;
    });
  });

  function onClick(scheduleId, a) {
    dispatch("openschedule", { scheduleId: parseInt(scheduleId) });
  }
</script>

<div id="route-header">
  <div class="title-sr-pair text-3xl">
    <span class="code">{route.code}</span>
    <span class="title">{route.name}</span>
  </div>

  <div class="tabs mx-auto w-full">
    <label class={`tab tab-bordered ${tab === 0 && "tab-active"}`}>
      <input
        class="hidden"
        type="radio"
        bind:group={tab}
        name="mode"
        value={0}
      />Informação</label
    >
    <label class={`tab tab-bordered ${tab === 1 && "tab-active"}`}>
      <input
        class="hidden"
        type="radio"
        bind:group={tab}
        name="mode"
        value={1}
      />Horário</label
    >
    <label class={`tab tab-bordered ${tab === 2 && "tab-active"}`}>
      <input
        class="hidden"
        type="radio"
        bind:group={tab}
        name="mode"
        value={2}
      />Trajeto</label
    >
    <label class={`tab tab-bordered ${tab === 3 && "tab-active"}`}>
      <input
        class="hidden"
        type="radio"
        bind:group={tab}
        name="mode"
        value={3}
      />Paragens</label
    >
    <label class={`tab tab-bordered ${tab === 4 && "tab-active"}`}>
      <input
        class="hidden"
        type="radio"
        bind:group={tab}
        name="mode"
        value={4}
      />Assistência</label
    >
  </div>

  {#if tab === 0}
    <h2 class="text-xl">Variantes</h2>
    <ul>
      {#each route.subroutes as subroute}
        <li>{subroute.flag}</li>
      {/each}
    </ul>
    <h2 class="text-xl">Avisos</h2>
    <p>Sem avisos de momento</p>
  {:else if tab === 1}{:else if tab === 2}
    <span class="text-xl">Variante a consultar</span>
    <select
      class="select select-primary select-sm w-full"
      bind:value={$selectedSubrouteId}
    >
      {#each route.subroutes as subroute}
        <option value={subroute.id}>{subroute.flag}</option>
      {/each}
    </select>
    <RouteMap {subrouteStops} />
  {:else if tab === 3}
    <RouteStops {subrouteStops} />
  {:else if tab === 4}
    Por fazer
  {/if}
</div>

<style>
  .code {
    border-radius: 32px;
    font-weight: bold;
    color: white;
    background-color: darkred;
    padding: 0.1em 0.3em;
    font-size: 2rem;
    display: inline-block;
    margin-right: 20px;
  }
</style>

