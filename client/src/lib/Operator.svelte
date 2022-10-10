<script>
  import {selectedOperatorRoutes, selectedRouteId} from "../context.js";
  import Route from "./components/Route.svelte";
</script>

<div class="overflow-x-auto">
  {#if $selectedRouteId}
    <Route />
  {:else}
    <table class="table table-zebra table-compact w-full">
      <thead>
      <tr>
        <th></th>
        <th>Linha</th>
      </tr>
      </thead>
      <tbody>
      {#each $selectedOperatorRoutes as route}
        <tr class="cursor-pointer hover" on:click={() => ($selectedRouteId = route.id)}>
          <th>
            {#if route.code}
            <span class="line-number"
                  style="
                    background-color: {route.badge_bg};
                    color: {route.badge_text};
                    border: 2px solid {route.badge_text};">
              {route.code}
            </span>
            {/if}
          </th>
          <td class="w-full">{route.name}</td>
        </tr>
      {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .line-number {
    padding: 0.2em 10px;
    border-radius: 1em;
    font-weight: 900;
    font-size: 1.2rem;
    display: inline-block;
  }
</style>

