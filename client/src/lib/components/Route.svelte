<script>
  import RouteStops from "./RouteStops.svelte";
  import RouteMap from "./RouteMap.svelte";
  import RouteSchedule from "./RouteSchedule.svelte";
  import {selectedRoute, selectedSubrouteId} from "../../context.js";

  let tab = 1;
</script>

<div id="route-header">
  <div class="title-sr-pair text-3xl">
    <span
        class="code"
        style="background-color: #{$selectedRoute.badge_bg}; color: #{$selectedRoute.badge_text}">
      {$selectedRoute.code}
    </span>
    <span class="title">{$selectedRoute.name}</span>
  </div>

  <div class="tabs mx-auto w-full mb-4">
    <label class={`tab tab-lg tab-bordered ${tab === 0 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={tab} name="mode" value={0} />Informação</label>
    <label class={`tab tab-lg tab-bordered ${tab === 1 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={tab} name="mode" value={1} />Horário</label>
    <label class={`tab tab-lg tab-bordered ${tab === 2 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={tab} name="mode" value={2} />Trajeto</label>
    <label class={`tab tab-lg tab-bordered ${tab === 3 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={tab} name="mode" value={3} />Paragens</label>
    <label class={`tab tab-lg tab-bordered ${tab === 4 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={tab} name="mode" value={4} />Assistência</label>
  </div>

  {#if tab === 0}
    <h2 class="text-xl">Variantes</h2>
    <ul>
      {#each $selectedRoute.subroutes as subroute}
        <li>{subroute.flag}</li>
      {/each}
    </ul>
    <br>
    <h2 class="text-xl">Avisos</h2>
    <p>Sem avisos de momento</p>
  {:else if tab === 1}
    <div class="flex flex-col border-error border-2 p-2 rounded-xl">
      <div class="flex flex-row font-bold items-center text-error-content">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="stroke-current flex-shrink-0 h-6 w-6"
            fill="none"
            viewBox="0 0 24 24">
          <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        Informação potêncialmente incorreta
      </div>
      <div>
        Os horários podem ou não estar a funcionar. Ainda os estamos a validar (manualmente).
      </div>
    </div>
    <RouteSchedule />
  {:else if tab === 2}
    <h2 class="text-xl">Variante a consultar</h2>
    <select class="select select-primary select-sm w-full" bind:value={$selectedSubrouteId}>
      {#each $selectedRoute.subroutes as subroute}
        <option value={subroute.id}>{subroute.flag}</option>
      {/each}
    </select>
    <RouteMap />
  {:else if tab === 3}
    <RouteStops />
  {:else if tab === 4}
    <p class="text-lg">
      Ainda estamos a trabalhar nestas funcionalidades.<br>
      Esperamos tê-las prontas para si e para os seus num futuro próximo.
    </p>
    <div class="flex justify-between">
      <img src="/icons/audio.svg" class="w-64" />
      <img src="/icons/video.svg" class="w-64" />
      <img src="/icons/audiovisual.svg" class="w-64" />
    </div>
  {/if}
</div>

<style>
  .code {
    border-radius: 32px;
    font-weight: bold;
    color: white;
    background-color: darkred;
    padding: 0.1em 0.3em;
    display: inline-block;
  }
</style>

