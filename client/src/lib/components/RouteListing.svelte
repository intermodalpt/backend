<script>
  import {createEventDispatcher} from "svelte";

  export let selectedRoutes = undefined;

  const dispatch = createEventDispatcher();
  let focusedRoute;
  let hintedRoute;

  function gotoRoute(routeId) {
    routeId = parseInt(routeId);
    dispatch("openroute", {routeId: routeId});
  }


  function gotoSchedule(routeId) {
    routeId = parseInt(routeId);
    dispatch("openschedule", {routeId: routeId});
  }


  function gotoInfo(routeId) {
    routeId = parseInt(routeId);
    dispatch("openinfo", {routeId: routeId});
  }

  function onEnter(routeId) {
    routeId = parseInt(routeId);
    if (focusedRoute) {
      dispatch("drophint", {routeId: focusedRoute});
    }
    hintedRoute = routeId;
    dispatch("hint", {routeId: routeId});
  }

  function onLeave(routeId) {
    routeId = parseInt(routeId);
    if (focusedRoute !== hintedRoute) {
      dispatch("drophint", {routeId: routeId});
      dispatch("hint", {routeId: focusedRoute});
    }
    hintedRoute = undefined;
  }

  function onFocus(routeId) {
    routeId = parseInt(routeId);
    focusedRoute = routeId;
    if (focusedRoute !== hintedRoute) {
      dispatch("drophint", {routeId: hintedRoute});
      dispatch("hint", {routeId: routeId});
    }
  }

  function onUnfocus(routeId) {
    dispatch("drophint", {routeId: routeId});
    focusedRoute = undefined;
  }
</script>

{#if selectedRoutes}
  <div class="flex flex-col sm:gap-1 sm:p-2">
    {#each selectedRoutes as route}
      <div tabindex="0" class="collapse" on:focusin={() =>  onFocus(route.id)} on:focusout={() =>  onUnfocus(route.id)}>
        <div
            class="collapse-title cursor-pointer flex flex-row items-center p-1 gap-1 sm:gap-2  bg-base-100 hover:bg-base-300 rounded-full"
            on:mouseenter={() => onEnter(route.id)}
            on:mouseleave={() => onLeave(route.id)}>
          <div class="flex flex-row items-center rounded-full  border-base-content border-2 shrink-0">
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
        <div class="collapse-content">
          <div class="flex justify-between max-w-lg gap-4">
            <a class="flex flex-col items-center cursor-pointer" on:click={() => gotoRoute(route.id)}>
              <img class="w-14" src="/icons/route.svg" alt="Percurso" />
              <span class="text-lg">Percurso</span>
            </a>
            <a class="flex flex-col items-center cursor-pointer" on:click={() => gotoSchedule(route.id)}>
              <img class="w-14" src="/icons/time.svg" alt="Horário" />
              <span class="text-lg">Horário</span>
            </a>
            <a class="flex flex-col items-center cursor-pointer" on:click={() => gotoInfo(route.id)}>
              <img class="w-14" src="/icons/info.svg" alt="Informação" />
              <span class="text-lg">Informação</span>
            </a>
          </div>
        </div>
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
