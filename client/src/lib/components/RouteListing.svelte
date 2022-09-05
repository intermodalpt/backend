<script>
  import { createEventDispatcher } from "svelte";
  import RouteLine from "./RouteLine.svelte";

  export let selectedRoutes = undefined;

  const dispatch = createEventDispatcher();
  let focusedRoute;
  let hintedRoute;

  function gotoRoute(routeId) {
    routeId = parseInt(routeId);
    dispatch("openroute", { routeId: routeId });
  }

  function gotoSchedule(routeId) {
    routeId = parseInt(routeId);
    dispatch("openschedule", { routeId: routeId });
  }

  function gotoInfo(routeId) {
    routeId = parseInt(routeId);
    dispatch("openinfo", { routeId: routeId });
  }

  function onEnter(routeId) {
    routeId = parseInt(routeId);
    if (focusedRoute) {
      dispatch("drophint", { routeId: focusedRoute });
    }
    hintedRoute = routeId;
    dispatch("hint", { routeId: routeId });
  }

  function onLeave(routeId) {
    routeId = parseInt(routeId);
    if (focusedRoute !== hintedRoute) {
      dispatch("drophint", { routeId: routeId });
      dispatch("hint", { routeId: focusedRoute });
    }
    hintedRoute = undefined;
  }

  function onFocus(routeId) {
    routeId = parseInt(routeId);
    focusedRoute = routeId;
    if (focusedRoute !== hintedRoute) {
      dispatch("drophint", { routeId: hintedRoute });
      dispatch("hint", { routeId: routeId });
    }
  }

  function onUnfocus(routeId) {
    dispatch("drophint", { routeId: routeId });
    focusedRoute = undefined;
  }
</script>

{#if selectedRoutes}
  <div class="flex flex-col sm:gap-1 sm:p-2">
    {#each selectedRoutes as route}
      <RouteLine {onUnfocus} {onFocus} {onLeave} {onEnter} {gotoSchedule} {gotoInfo} {gotoRoute} {route} />
    {/each}
  </div>
{:else}
  <div class="p-4">Selecione uma região do mapa para visualizar as rotas existentes.</div>
{/if}

<link rel="stylesheet" href="https://unpkg.com/balloon-css/balloon.min.css" />
