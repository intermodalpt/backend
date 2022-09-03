<script>
  import {stops} from "../../cache.js";
  import {createEventDispatcher} from "svelte";
  import {subrouteStops} from "../../context.js";

  const dispatch = createEventDispatcher();

  let selectedStop = 0;

  function onClick(stopId) {
    dispatch("gotoStop", {stopId: parseInt(stopId)});
  }
</script>

<ul class="overflow-y-scroll steps steps-vertical">
  {#if $subrouteStops}
    {#each $subrouteStops.stops as stop, i}
      <li
          class:selected={selectedStop === stop}
          on:click={() => onClick(stop)}
          class="step hover:bg-base-200 rounded-xl cursor-pointer">
        {$stops[stop].short_name || $stops[stop].name || $stops[stop].official_name || $stops[stop].osm_name}
      </li>
    {/each}
  {/if}
</ul>