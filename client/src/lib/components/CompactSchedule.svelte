<script>
  import {
    scheduleBySubroute,
    selectedDay,
    selectedRoute,
    selectedSubrouteId,
    subrouteShedule,
    schedule,
    selectedRouteId
  } from "../../context.js";

</script>

<div class="overflow-y-scroll w-full flex flex-col">
  <select class="select select-primary select-sm w-full" bind:value={$selectedSubrouteId}>
    {#each $selectedRoute.subroutes as subroute}
      <option value={subroute.id}>{subroute.flag}</option>
    {/each}
  </select>
  <input type="date" class="input  input-bordered text-lg" bind:value={$selectedDay}>
  <div class="flex flex-col gap-2 p-2 overflow-y-scroll">
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
        Os horários podem ou não estar a funcionar. Ainda os estamos a copiar e validar.
      </div>
    </div>

    {#if $schedule === undefined || $schedule.length === 0 || $subrouteShedule.length === 0 }
      <span class="text-lg">Sem partidas no dia selecionado</span>
    {/if}
    {#if $subrouteShedule !== undefined}
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
          <hr />
        {/if}
      {/each}
    {/if}
  </div>
</div>