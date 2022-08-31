<script>
  import {picStopRels, pictures, routes, stopPicRels, stops} from "../../cache.js";
  import {api_server, token} from "../../settings.js";

  let isDebug = false;
  let osmSyncing = false;

  function osmSync() {
    osmSyncing = true;
    fetch(`${api_server}/actions/import_osm`, {
      headers: {
        authorization: `Bearer ${$token}`
      }
    })
        .then(r => r.json())
        .then((result) => {
          alert(JSON.stringify(result));
        }).catch((e) => alert(`Error occurred: ${e}`));
  }
</script>


<div class="flex flex-col">
  <button type="button" class="btn btn-info" on:click={() => {osmSync()}} disabled={osmSyncing}>
    {#if osmSyncing}
      <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
    {/if}
    Sync with OSM
  </button>
  <span class="justify-center text-center text-lg font-bold">Debug info</span>
  <input class="btn btn-error" type="button" on:click={() => {isDebug = true}} value="Show debug info" />
  {#if isDebug}
    <span class="justify-center text-center">$stops</span>
    <textarea>{JSON.stringify($stops)}</textarea>
    <span class="justify-center text-center">$routes</span>
    <textarea>{JSON.stringify($routes)}</textarea>
    <span class="justify-center text-center">$pictures</span>
    <textarea>{JSON.stringify($pictures)}</textarea>
    <span class="justify-center text-center">$picStopRels</span>
    <textarea>{JSON.stringify($picStopRels)}</textarea>
    <span class="justify-center text-center">$stopPicRels</span>
    <textarea>{JSON.stringify($stopPicRels)}</textarea>
  {/if}
</div>