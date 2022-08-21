<script>
  import { stops } from "../../cache.js";
  import {derived} from "svelte/store";
  let page = 0;
  const pageSize = 100;

  const osmStops = derived(stops, $stops => {
    return Object.values($stops).filter((stop) => stop.source === "osm");
  });

  const stopLen = derived(osmStops, $osmStops => {
    return Object.values($stops).length;
  });
</script>

<div class="flex flex-col items-center">
  <div class="overflow-x-auto">
    <table class="table table-compact w-full">
      <!-- head -->
      <thead>
        <tr>
          <th>OSM</th>
          <th>Oficial</th>
          <th>Nome</th>
          <th>Abrev.</th>
          <th>Loc.</th>
          <th>Via</th>
          <th>Porta</th>
        </tr>
      </thead>
      <tbody>
        {#each Object.values($stops)
          .filter((stop) => stop.source === "osm")
          .slice(page * pageSize, (page + 1) * pageSize) as stop}
          <tr>
            <td> {stop.osm_name} </td>
            <td> {stop.official_name} </td>
            <td>
              <input type="text" class="input input-bordered input-xs w-full" value={stop.name} />
            </td>
            <td>
              <input type="text" class="input input-bordered input-xs w-full" value={stop.short_name} />
            </td>
            <td>
              <input type="text" class="input input-bordered input-xs w-full" value={stop.locality} />
            </td>
            <td>
              <input type="text" class="input input-bordered input-xs w-full" value={stop.street} />
            </td>
            <td>
              <input type="text" class="input input-bordered input-xs w-20" value={stop.door} />
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="flex flex-row gap-2 items-center">
    <button
      class="btn btn-square btn-ghost fill-base-content flex flex-col justify-center items-center"
      on:click={() => (page = Math.max(0, page - 1))}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
        <path
          d="M447.1 256C447.1 273.7 433.7 288 416 288H109.3l105.4 105.4c12.5 12.5 12.5 32.75 0 45.25C208.4 444.9 200.2 448 192 448s-16.38-3.125-22.62-9.375l-160-160c-12.5-12.5-12.5-32.75 0-45.25l160-160c12.5-12.5 32.75-12.5 45.25 0s12.5 32.75 0 45.25L109.3 224H416C433.7 224 447.1 238.3 447.1 256z"
        />
      </svg>
    </button>
    {`${page} / ${Math.floor($stopLen / pageSize)}`}
    <button
      class="btn btn-square btn-ghost fill-base-content flex flex-col justify-center items-center"
      on:click={() =>
        (page = Math.min(
          page + 1,
          Math.floor($stopLen / pageSize)
        ))}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
        <path
          d="M438.6 278.6l-160 160C272.4 444.9 264.2 448 256 448s-16.38-3.125-22.62-9.375c-12.5-12.5-12.5-32.75 0-45.25L338.8 288H32C14.33 288 .0016 273.7 .0016 256S14.33 224 32 224h306.8l-105.4-105.4c-12.5-12.5-12.5-32.75 0-45.25s32.75-12.5 45.25 0l160 160C451.1 245.9 451.1 266.1 438.6 278.6z"
        />
      </svg>
    </button>
  </div>
</div>
