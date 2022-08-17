<script>
  import { stops } from "../../cache.js";
  import { createEventDispatcher } from "svelte";
  import {subrouteStops} from "../../context.js";

  const dispatch = createEventDispatcher();

  let selectedStop = 0;

  function onClick(scheduleId, a) {
    dispatch("openschedule", { scheduleId: parseInt(scheduleId) });
  }
</script>

<div class="flex flex-col gap-1">
  <ul class="steps steps-vertical">
    {#if $subrouteStops}
      {#each $subrouteStops.stops as stop, i}
        <li
          class:selected={selectedStop === stop}
          on:click={() => onClick(stop)}
          class="step hover:bg-base-200 rounded-xl cursor-pointer">
          {$stops[stop].short_name}
        </li>
      {/each}
    {/if}
  </ul>
</div>
