<script>
  import {stops} from "../../cache.js";
  import {derived, writable} from "svelte/store";

  let page = 0;
  const pageSize = 100;

  let osmFilter = writable(null);
  let officialNameFilter = writable(null);
  let nameFilter = writable(null);
  let abbrFilter = writable(null);

  let changes = [];

  const osmStops = derived([stops, osmFilter], ([$stops, $osmFilter]) => {
    let vals = Object.values($stops).filter((stop) => stop.source === "osm");
    if ($osmFilter) {
      vals = vals.filter((stop) => {
        return stop.osm_name && stop.osm_name.includes($osmFilter)
      });
    }
    console.log(vals);
    return vals;
  });

  const stopLen = derived(osmStops, $osmStops => {
    return Object.values($stops).length;
  });

  function applyOfficialName(id, officialName) {
    let existing = changes.find((stop) => {
      return stop.id === id
    });
    if (existing) {
      existing.official_name = officialName;
    } else {
      changes.push(Object.assign(Object.assign({}, $stops[id]), {official_name: officialName}));
      changes = changes;
    }
  }

  function applyName(id, name) {
    let existing = changes.find((stop) => {
      return stop.id === id
    });
    if (existing) {
      existing.name = name;
    } else {
      changes.push(Object.assign(Object.assign({}, $stops[id]), {name: name}));
      changes = changes;
    }
  }

  function applyShortName(id, shortName) {
    let existing = changes.find((stop) => {
      return stop.id === id
    });
    if (existing) {
      existing.short_name = shortName;
    } else {
      changes.push(Object.assign(Object.assign({}, $stops[id]), {short_name: shortName}));
      changes = changes;
    }
  }

  function applyLocality(id, locality) {
    let existing = changes.find((stop) => {
      return stop.id === id
    });
    if (existing) {
      existing.locality = locality;
    } else {
      changes.push(Object.assign(Object.assign({}, $stops[id]), {locality: locality}));
      changes = changes;
    }
  }

  function applyStreet(id, street) {
    let existing = changes.find((stop) => {
      return stop.id === id
    });
    if (existing) {
      existing.street = street;
    } else {
      changes.push(Object.assign(Object.assign({}, $stops[id]), {street: street}));
      changes = changes;
    }
  }

  function applyDoor(id, door) {
    console.log("Yo");
    let existing = changes.find((stop) => {
      return stop.id === id
    });
    if (existing) {
      existing.door = door;
    } else {
      changes.push(Object.assign(Object.assign({}, $stops[id]), {door: door}));
      changes = changes;
    }
  }

  function stopDiff(changed) {
    const diff = {};
    const keys = Object.keys($stops[changed.id]);

    console.log("yo");

    for (const key of keys) {
      if ($stops[changed.id][key] !== changed[key]) {
        diff[key] = changed[key];
      }
    }
    return diff;
  }

  function save() {

  }
</script>

<div class="flex flex-col items-stretch">
  <div class="overflow-x-auto">
    <table class="table table-compact w-full">
      <!-- head -->
      <thead>
      <tr>
        <th>Id</th>
        <th>
          <input type="text" class="input input-bordered input-xs w-full" placeholder="OSM" bind:value={$osmFilter} />
        </th>
        <th>
          <input
              type="text" class="input input-bordered input-xs w-full"
              placeholder="Official name"
              bind:value={$officialNameFilter} />
        </th>
        <th>
          <input type="text" class="input input-bordered input-xs w-full" placeholder="Name" bind:value={$nameFilter} />
        </th>
        <th>
          <input
              type="text" class="input input-bordered input-xs w-full"
              placeholder="Abbreviation"
              bind:value={$abbrFilter} />
        </th>
        <th>Locality</th>
        <th>Way</th>
        <th>Door</th>
      </tr>
      </thead>
      <tbody>
      {#each $osmStops.slice(page * pageSize, (page + 1) * pageSize) as stop}
        <tr>
          <td> {stop.id}</td>
          <td> {stop.osm_name}</td>
          <td>
            <input
                type="text"
                class="input input-bordered input-xs w-full"
                value={stop.official_name}
                on:change={(e) => {
                  const val = e.target.value.trim();
                  applyOfficialName(stop.id, val === "" ? null : val);
                }} />
          </td>
          <td>
            <input
                type="text"
                class="input input-bordered input-xs w-full"
                value={stop.name}
                on:change={(e) => {
                  const val = e.target.value.trim();
                  applyName(stop.id, val === "" ? null : val);
                }} />
          </td>
          <td>
            <input
                type="text"
                class="input input-bordered input-xs w-full"
                value={stop.short_name}
                on:change={(e) => {
                  const val = e.target.value.trim();
                  applyShortName(stop.id, val === "" ? null : val);
                }} />
          </td>
          <td>
            <input
                type="text"
                class="input input-bordered input-xs w-36"
                value={stop.locality}
                on:change={(e) => {
                  const val = e.target.value.trim();
                  applyLocality(stop.id, val === "" ? null : val);
                }} />
          </td>
          <td>
            <input
                type="text"
                class="input input-bordered input-xs w-36"
                value={stop.street}
                on:change={(e) => {
                  const val = e.target.value.trim();
                  applyStreet(stop.id, val === "" ? null : val);
                }} />
          </td>
          <td>
            <input
                type="text"
                class="input input-bordered input-xs w-20"
                value={stop.door}
                on:change={(e) => {
                  const val = e.target.value.trim();
                  applyDoor(stop.id, val === "" ? null : val);
                }} />
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

<ul>
  {#each changes as change}
    <li>
      <span>{change.id}</span>
      <ul class="ml-2">
        {#each Object.keys(stopDiff(change)) as key}
          <li>
            {key} from {$stops[change.id][key]} to {change[key]}
          </li>
        {/each}
      </ul>
    </li>
  {/each}
</ul>

<input type="button" value="Save" class="input btn btn-primary" on:click={save} />