<script>
  import {createEventDispatcher} from "svelte";

  export let selectedRoutes = undefined;

  const dispatch = createEventDispatcher();

  function onClick(routeId, a) {
    dispatch("openroute", {routeId: parseInt(routeId)});
  }

  function onEnter(routeId) {
    dispatch("hint", {routeId: parseInt(routeId)});
  }

  function onLeave(routeId) {
    dispatch("drophint", {routeId: parseInt(routeId)});
  }
</script>

{#if selectedRoutes}
  <div class="flex flex-col gap-1 p-2">
    {#each selectedRoutes as route}
      <div
          class="cursor-pointer flex flex-row items-center p-1 gap-1 sm:gap-2 hover:bg-base-300 rounded-full"
          on:click={() => onClick(route.id)}
          on:mouseenter={() => onEnter(route.id)}
          on:mouseleave={() => onLeave(route.id)}>
        <div class="flex flex-row items-center rounded-full bg-base-100 border-base-content border-2 shrink-0">
          <img class="ml-1 w-7 px-[2px]" src="/logos/cmet-min.svg" alt="CM" />
          <div
              class="rounded-full px-2 py-1 -my-[2px] -mr-[2px]  text-lg"
              style="background-color: #{route.badge_bg}; color: #{route.badge_text}">
            {route.code}
          </div>
        </div>
        {#if route.name.split(" - ").length === 2}
          <div>
            <span>{route.name.split(" - ")[0]}</span><br />
            <span>{route.name.split(" - ")[1]}</span>
          </div>
        {:else}
          <div>{route.name}</div>
        {/if}
      </div>
      <hr />
    {/each}
  </div>
{:else}
  <div class="p-4">
    Selecione uma região do mapa para visualizar as rotas existentes.
  </div>
{/if}

<link rel="stylesheet" href="https://unpkg.com/balloon-css/balloon.min.css" />
