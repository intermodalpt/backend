<script>
  import {selectedRoute, selectedSubrouteId, subrouteShedule} from "../../context.js";

</script>

<select class="select select-primary select-sm w-full" bind:value={$selectedSubrouteId}>
  {#each $selectedRoute.subroutes as subroute}
    <option value={subroute.id}>{subroute.flag}</option>
  {/each}
</select>
{#if $subrouteShedule}
  <div class="p-2 flex flex-col gap-2">
    <div class="flex flex-col gap-2 pt-2">
      {#each $subrouteShedule.schedule_hours as hour, i }
        {#if $subrouteShedule.schedule[i].length > 0}
          <div class="flex flex-row flex-wrap items-center gap-2">
          {#each $subrouteShedule.schedule[i] as minutes}
              <span class="mr-2">
                <span class="text-base-content rounded-full text-lg font-bold text-right">{hour}h</span>
                <span class="text-lg">{minutes}</span>
              </span>
          {/each}
          </div>
          <div class="h-[2px] bg-base-300 w-full rounded-full"></div>
        {/if}
      {/each}
    </div>
  </div>
{/if}