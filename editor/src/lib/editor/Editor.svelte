<script>
  import StopEditor from './StopEditor.svelte';
  import LineEditor from './LineEditor.svelte';
  import ImageEditor from './StopImageGallery.svelte';
  import "leaflet.markercluster";
  import "leaflet-contextmenu";

  let token = sessionStorage.getItem("authToken");

  let mode = null;

  if (token == null) {
    token = prompt("Insert the authorization token");
    sessionStorage.setItem("authToken", token);
  }
</script>

<div class="flex flex-col flex-1 gap-2 p-2">
  <div class="tabs mx-auto">
    <label class={`tab tab-bordered ${mode == 1 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={mode} name="mode" value={1} />Stops
    </label>
    <label class={`tab tab-bordered ${mode == 2 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={mode} name="mode" value={2} />Lines
    </label>
    <label class={`tab tab-bordered ${mode == 3 && "tab-active"}`}>
      <input class="hidden" type="radio" bind:group={mode} name="mode" value={3} />Images
    </label>
  </div>
  {#if mode === 1}
    <StopEditor />
  {:else if mode === 2}
    <LineEditor />
  {:else if mode === 3}
    <ImageEditor />
  {:else}
    ???
  {/if}
</div>

<link
  rel="stylesheet"
  href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css"
  integrity="sha512-xwE/Az9zrjBIphAcBb3F6JVqxf46+CDLwfLMHloNu6KEQCAWi6HcDUbeOfBIptF7tcCzusKFjFw2yuvEpDL9wQ=="
  crossorigin=""
/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.css" />
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.Default.css" />
<link rel="stylesheet" href="/map.css" />
<link rel="stylesheet" href="/leaflet-contextmenu.css" />
