<script>
  import {createEventDispatcher} from "svelte";

  export let routes;
  export let stops;
  export let selectedStop;
  export let selectedSubrouteStops;
  let addAfterIndex = 0;

  let changes = false;
  const dispatch = createEventDispatcher();

  // Deep-ish data copies to leave the original intact. (Sorry for the shitty code)
  let stopList;
  let diffList;

  selectedSubrouteStops.subscribe((value) => {
    if (value) {
      stopList = [...value.stops];
      diffList = [...value.diffs];
    }
  });

  $: stopList = redraw() || stopList;

  function moveUp(i) {
    let aux = stopList[i - 1];
    diffList;
    let aux_diff = diffList[i - 1];
    stopList[i - 1] = stopList[i];
    diffList[i - 1] = diffList[i];
    stopList[i] = aux;
    diffList[i] = aux_diff;
    stopList = stopList;
    diffList = diffList;
    changes = true;
  }

  function moveDown(i) {
    let aux = stopList[i + 1];
    let aux_diff = diffList[i + 1];
    stopList[i + 1] = stopList[i];
    diffList[i + 1] = diffList[i];
    stopList[i] = aux;
    diffList[i] = aux_diff;
    stopList = stopList;
    diffList = diffList;
    changes = true;
  }

  function addStop() {
    if (selectedStop === undefined) {
      alert("Select a stop first...");
      return;
    }

    // stopList.indexOf(addAfterIndex)
    if (
      confirm(`Do you want to add a stop after ${stopList[addAfterIndex]}?`)
    ) {
      stopList.splice(addAfterIndex + 1, 0, selectedStop);
      diffList.splice(addAfterIndex + 1, 0, 0);
      stopList = stopList;
      diffList = diffList;
    }
  }

  function replaceStop(i) {
    if (selectedStop === undefined) {
      alert("Select another stop first...");
      return;
    }

    // if (selectedSubrouteStops.stops.includes(selectedStop)) {
    //     if (!confirm("Route already has this stop. Are you totally sure?")) {
    //         return;
    //     }
    // }

    // if (confirm(`Do you want to replace ${stops[stopList[i]].name} with ${stops[selectedStop].name}?`)) {
    if (
      confirm(
        `"${stops[stopList[i]].short_name}":[["${
          stops[selectedStop].source
        }", "${
          stops[selectedStop].source === "osm"
            ? stops[selectedStop].external_id
            : stops[selectedStop].name
        }"}]],`
      )
    ) {
      stopList[i] = selectedStop;
      stopList = stopList;
      changes = true;
    }
  }

  function removeStop(i) {
    if (
      confirm(
        `Do you want to remove ${stops[stopList[i]].name} from this route?`
      )
    ) {
      stopList.splice(i, 1);
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
      stopList = stopList;
      diffList = diffList;
      changes = true;
    }
  }

  function goTo(i) {
    dispatch("goto", {
      lon: stops[stopList[i]].lon,
      lat: stops[stopList[i]].lat,
    });
  }

  function redraw(i) {
    dispatch("redraw", {stops: stopList});
  }

  function save() {
    dispatch("savesubroutestops", {stops: stopList, diffs: diffList});
  }
</script>

<div class="flex flex-col gap-1">
  {#if stopList}
    {#each stopList as stop, index}
      <div class="flex flex-row justify-between gap-1">
        <!--        <input type="text" on:click={() => goTo(index)} value="({stops[stop].source}{stop}) - {stops[stop].name || stops[stop].short_name}">-->
        <a class="btn btn-xs btn-ghost" on:click={() => goTo(index)}>
          ({stops[stop].source}{stop}) - {stops[stop].name || stops[stop].short_name}
        </a>
        <div class="flex flex-row gap-1">
          ∇
          <input class="input input-bordered input-xs w-12" type="number" maxlength="2" max="99"
                 bind:value={diffList[index]} />
          {#if index > 0}
            <input class="btn btn-xs w-8 cursor-pointer" on:click={() => moveUp(index)} value="🡹" />
          {/if}
          {#if index !== stopList.length - 1}
            <input class="btn btn-xs w-8 cursor-pointer" on:click={() => moveDown(index)} value="🡻" />
          {/if}
          <input class="btn btn-xs w-8 cursor-pointer" on:click={() => replaceStop(index)} value="⮰" />
          <input class="btn btn-xs w-8 cursor-pointer" on:click={() => removeStop(index)} value="❌" />
        </div>
      </div>
    {/each}
    <div class="flex-row bg-base-300">
      <input type="button" class="btn btn-xs" value="Add" on:click={addStop} /> after
      <!--        <input type="number" min="0" max="{stopList.length}" bind:value={addAfterIndex}/>-->
      <select class="select select-bordered select-xs" bind:value={addAfterIndex}>
        {#each stopList as stop, index}
          <option value={index}>{stops[stop].short_name || stops[stop].name || stop}</option>
        {/each}
      </select>
      {#if changes}
        <input type="button" value="Save" on:click={save} />
      {/if}
    </div>
  {/if}
</div>

