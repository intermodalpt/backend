<script>
  import {createEventDispatcher} from "svelte";
  import {stops, routes} from "../../cache.js";
  import {writable} from "svelte/store";

  export let selectedStop;
  export let selectedSubrouteStops;
  let addAfterIndex = 0;

  let changes = false;
  const dispatch = createEventDispatcher();

  // Deep-ish data copies to leave the original intact. (Sorry for the shitty code)
  let stopList = writable([]);
  let diffList;

  selectedSubrouteStops.subscribe((value) => {
    if (value) {
      $stopList = [...value.stops];
      diffList = [...value.diffs];
    }
  });

  stopList.subscribe((list) => {
    redraw()
  })

  function moveUp(i) {
    let aux = $stopList[i - 1];
    diffList;
    let aux_diff = diffList[i - 1];
    $stopList[i - 1] = $stopList[i];
    diffList[i - 1] = diffList[i];
    $stopList[i] = aux;
    diffList[i] = aux_diff;
    $stopList = $stopList;
    diffList = diffList;
    changes = true;
    closeModal(i);
  }

  function moveDown(i) {
    let aux = $stopList[i + 1];
    let aux_diff = diffList[i + 1];
    $stopList[i + 1] = $stopList[i];
    diffList[i + 1] = diffList[i];
    $stopList[i] = aux;
    diffList[i] = aux_diff;
    $stopList = $stopList;
    diffList = diffList;
    changes = true;
    closeModal(i);
  }

  function addStop() {
    if ($selectedStop === undefined) {
      alert("Select a stop first...");
      return;
    }

    $stopList = [$selectedStop.id];
    diffList = [0];
  }

  function addBefore(index) {
    if ($selectedStop === undefined) {
      alert("Select a stop first...");
      return;
    }

    $stopList.splice(index, 0, $selectedStop.id);
    diffList.splice(index, 0, 0);
    $stopList = $stopList;
    diffList = diffList;
    changes = true;
    closeModal(index);
  }

  function addAfter(index) {
    if ($selectedStop === undefined) {
      alert("Select a stop first...");
      return;
    }

    $stopList.splice(index + 1, 0, $selectedStop.id);
    diffList.splice(index + 1, 0, 0);
    $stopList = $stopList;
    diffList = diffList;
    changes = true;
    closeModal(index);
  }

  function replaceStop(i) {
    if ($selectedStop === undefined) {
      alert("Select another stop first...");
      return;
    }

    // if (selectedSubrouteStops.stops.includes(selectedStop)) {
    //     if (!confirm("Route already has this stop. Are you totally sure?")) {
    //         return;
    //     }
    // }

    if (confirm(`Do you want to replace ${$stops[$stopList[i]].official_name || $stops[$stopList[i]].name || $stops[$stopList[i]].osm_name}
        with ${$selectedStop.official_name || $selectedStop.name || $selectedStop.osm_name}?`)) {
      $stopList[i] = $selectedStop.id;
      // $stopList = $stopList;
      changes = true;
      closeModal(i);
    }
    closeModal(index);
  }

  function removeStop(i) {
    if (confirm(`Do you want to remove ${$stops[$stopList[i]].official_name || $stops[$stopList[i]].name || $stops[$stopList[i]].osm_name} from this route?`)) {
      $stopList.splice(i, 1);
      let removedDiff = diffList.splice(i, 1)[0];
      if (diffList.length > 0) {
        if (i === 0) {
          diffList[0] += removedDiff;
        } else if (i === diffList.length) {
          // Discard this diff as there's no dist to the next stop
          diffList[diffList.length - 1] = null;
        } else {
          diffList[i - 1] += removedDiff;
        }
      }
      $stopList = $stopList;
      diffList = diffList;
      changes = true;
      closeModal(i);
    }
  }

  function goTo(stopId) {
    dispatch("goto", {
      lon: $stops[stopId].lon,
      lat: $stops[stopId].lat,
    });
  }

  function redraw(i) {
    dispatch("redraw", {stops: $stopList});
  }

  function save() {
    dispatch("savesubroutestops", {stops: $stopList, diffs: diffList});
  }

  function closeModal(index) {
    let modalCheckbox = document.getElementById(`index-${index}-modal`);
    if (modalCheckbox) {
      modalCheckbox.checked = false;
    }
  }
</script>

{#if $stopList}
  <div class="max-h-[80vh] min-h-[1px] h-fit overflow-y-scroll">
    {#if $stopList.length === 0}
      <div class="flex flex-col items-center">
        <h2>This subroute has no stops.</h2>
        <input
            type="button"
            value="Add stop"
            class="btn btn-primary"
            disabled={!$selectedStop}
            on:mouseup={addStop}
        >
      </div>
    {/if}

    {#each $stopList as stop, index}
      <div class="flex justify-between">
        <a class="btn btn-xs btn-ghost" on:click={() => goTo(stop)}>
          ({$stops[stop].source}{stop})
          {$stops[stop].official_name || $stops[stop].name || $stops[stop].short_name || $stops[stop].osm_name}
        </a>
        <div class="flex flex-row gap-1">
          <label for="{`index-${index}-modal`}" class="btn btn-xs modal-button">...</label>
          <input type="checkbox" id="{`index-${index}-modal`}" class="modal-toggle" />
          <label for="{`index-${index}-modal`}" class="modal cursor-pointer">
            <label class="modal-box relative max-w-2xl" for="">
                    <span class="text-lg">
                      O que fazer a
                      {$stops[stop].official_name || $stops[stop].name
                      || $stops[stop].short_name || $stops[stop].osm_name}?
                    </span>
              <ul class="menu bg-base-100 w-full rounded-box">
                {#if $selectedStop}
                  <li>
                    <a on:mouseup={() => addBefore(index)}>
                      🡹⁺ Inserir <b>{$selectedStop.official_name || $selectedStop.name || $selectedStop.osm_name}</b>
                      antes
                    </a>
                  </li>
                  <li>
                    <a on:mouseup={() => addAfter(index)}>
                      🡻⁺ Inserir <b>{$selectedStop.official_name || $selectedStop.name || $selectedStop.osm_name}</b>
                      depois
                    </a>
                  </li>
                  <li>
                    <a on:mouseup={() => replaceStop(index)}>
                      ⮰ Substituir por
                      <b>{$selectedStop.official_name || $selectedStop.name || $selectedStop.osm_name}</b>
                    </a>
                  </li>
                {/if}
                {#if index > 0}
                  <li>
                    <a on:mouseup={() => moveUp(index)}>
                      🡹 Mover para cima
                    </a>
                  </li>
                {/if}
                {#if index !== stopList.length - 1}
                  <li>
                    <a on:mouseup={() => moveDown(index)}>
                      🡻 Mover para baixo
                    </a>
                  </li>
                {/if}
                <li><a on:mouseup={() => removeStop(index)}>❌ Remover</a></li>
              </ul>
            </label>
          </label>
        </div>
      </div>
      <hr />
    {/each}
  </div>
  <div class="flex justify-end">
    <input class="btn btn-sm btn-primary" type="button" value="Save" disabled={!changes} on:click={save} />
  </div>
{/if}

