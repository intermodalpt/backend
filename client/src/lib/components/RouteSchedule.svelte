<script>
  import {selectedDay, selectedRoute, selectedSubrouteId, subrouteShedule} from "../../context.js";
</script>

<div class="text-xl flex items-center mb-4 mt-4">
      <span
          class="rounded-full text-info bg-info-content w-8 h-8 flex justify-center items-center text-center shadow-xl mr-4">
        1
      </span>
  Variante a consultar
</div>
<select class="select select-primary select-sm w-full" bind:value={$selectedSubrouteId}>
  {#each $selectedRoute.subroutes as subroute}
    <option value={subroute.id}>{subroute.flag}</option>
  {/each}
</select>
<hr class="mt-2 mb-2">
<div class="text-xl flex items-center mb-4 mt-4">
      <span
          class="rounded-full text-info bg-info-content w-8 h-8 flex justify-center items-center text-center shadow-xl mr-4">
        2
      </span>
  Dia a consultar
</div>
<input type="date" class="input input-lg text-lg -mt-4" bind:value={$selectedDay}>
<hr>
<div class="text-xl flex items-center mb-4 mt-4">
      <span
          class="rounded-full text-info bg-info-content w-8 h-8 flex justify-center items-center text-center shadow-xl mr-4">3</span>
  Horário
</div>
{#if $subrouteShedule}
  <div class="overflow-x-auto">
    <table class="table table-compact w-full">
      <thead>
      <tr>
        {#each $subrouteShedule.schedule_hours as hour}
          <th>{hour}</th>
        {/each}
      </tr>
      </thead>
      <tbody>
      {#each $subrouteShedule.transposed as row}
        <tr>
          {#each row as departure}
            {#if departure !== undefined}
              <td>{departure}</td>
            {:else }
              <td></td>
            {/if}
          {/each}
        </tr>
      {/each}
      </tbody>
    </table>
  </div>
{/if}