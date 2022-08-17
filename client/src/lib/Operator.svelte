<script>
  import {routes} from "../cache.js";
  import {selectedRouteId} from "../context.js";
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
      {#each $routes as route}
        <tr class="cursor-pointer hover" on:click={() => ($selectedRouteId = route.id)}>
          <th><span class="line-number" style="background-color: #{route.badge_bg}; color: #{route.badge_text}">{route.code}</span></th>
          <td>{route.name}</td>
        </tr>
      {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .line-number {
    color: white;
    background-color: red;
    padding: 0.2em 10px;
    border-radius: 1em;
    font-weight: 900;
    font-size: 1.2rem;
    display: inline-block;
  }
</style>

