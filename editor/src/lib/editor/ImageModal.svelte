<script>
  // import tagger from '@jcubic/tagger';
  import {stops} from '../../cache.js';
  import L from "leaflet";
  import {api_server} from "../../settings.js";
  import {icons} from "./assets.js";
  import {createEventDispatcher} from "svelte";

  export let image;

  const dispatch = createEventDispatcher();

  let map;
  let marker = null;
  let stopInput;
  let location = {
    lat: image.lat,
    lon: image.lon
  }

  let tags = [...image.tags];
  let stopIds = [...image.stops];
  let isSensitive = image.sensitive;
  let isPublic = image.public;
  let notes = image.notes;

  function createMap(container) {
    let m = L.map(container);

    if (image.lat && image.lon) {
      m.setView([image.lat, image.lon], 16);
    } else {
      m.setView([38.71856, -9.1372], 10);
    }

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 19,
      attribution: '© OpenStreetMap'
    }).addTo(m);

    m.maxBounds = new L.LatLngBounds(
      new L.LatLng(38.3, -10.0),
      new L.LatLng(39.35, -8.0)
    );
    m.maxBoundsViscosity = 1.0;
    m.minZoom = 10;

    let markerMoved = (e) => {
      let targetLoc = e.target.getLatLng();
      location.lon = targetLoc.lng;
      location.lat = targetLoc.lat;
    };
    if (location.lat) {
      marker = L.marker([location.lat, location.lng], {"draggable": true});
      marker.addTo(map);
      marker.on("moveend", markerMoved);
    }

    m.on('dblclick', function (e) {
      if (marker) {
        marker.removeFrom(map);
      }
      marker = L.marker([e.latlng.lat, e.latlng.lng], {"draggable": true});
      location.lon = e.latlng.lng;
      location.lat = e.latlng.lat;
      marker.addTo(map);
      marker.on("moveend", markerMoved);
    });


    let stopsLayer = L.markerClusterGroup({
      showCoverageOnHover: false,
      disableClusteringAtZoom: 16,
    });

    Object.values($stops).forEach((stop) => {
      if (stop.lat != null && stop.lon != null && stop.source === "osm") {
        let marker = L.marker(
          [stop.lat, stop.lon],
          Object.assign({}, {icon: icons[stop.source]})
        );

        marker.stopId = stop.id;

        marker.on("click", (e) => stopInput.value = e.target.stopId);

        let name = stop.name || stop.short_name;

        marker.bindTooltip(`${stop.id} - ${name}`);

        stopsLayer.addLayer(marker);
      }
    });

    m.addLayer(stopsLayer);

    return m;
  }

  function mapAction(container) {
    map = createMap(container);

    return {
      destroy: () => {
        map.remove();
        map = null;
      },
    };
  }

  function addTag() {
    let entry = document.getElementById("tag-text");
    let entryValue = entry.value.trim();

    if (entryValue !== "") {
      tags.push(entryValue);
      tags = tags;
    }
    entry.value = "";
  }

  function removeTag(tag) {
    tags.splice(tags.indexOf(tag), 1);
    tags = tags;
  }

  function addStop() {
    let entryValue = parseInt(stopInput.value);

    if (!isNaN(entryValue)) {
      stopIds.push(entryValue);
      stopIds = stopIds;
    }
    stopInput.value = "";
  }

  function removeStop(stopId) {
    stopIds.splice(stopIds.indexOf(stopId), 1);
    stopIds = stopIds;
  }

  function save() {
    let newMeta = {
      lat: image.lat,
      lon: image.lon,
      tags: tags,
      stops: stopIds,
      sensitive: isSensitive,
      public: isPublic,
      notes: notes,
    };

    if (location.lat != null) {
      if (image.lat == null || Math.abs(image.lat - location.lat) > 0.000001) {
        newMeta.lat = location.lat;
      }
    }

    if (location.lon != null) {
      if (image.lon == null || Math.abs(image.lon - location.lon) > 0.000001) {
        newMeta.lon = location.lon;
      }
    }

    fetch(`${api_server}/upload/stops/${image.id}`, {
      method: 'PATCH',
      body: JSON.stringify(newMeta),
      headers: {
        "Content-Type": "application/json",
        // Token: token,
      },
    })
      .catch((e) => alert("Failed to save the stop meta"))
      .then(() => {
        image.tagged = true;
        dispatch("close");
      });
  }

  function close() {
    dispatch("close");
  }
</script>

<div class="modal modal-bottom sm:modal-middle modal-open">
  <div class="modal-box max-w-full sm:max-w-full w-max sm:w-max">
    <div class="flex flex-col">
<div class="flex flex-row gap-1">
  <a target="_blank"
     href="https://intermodal-storage-worker.claudioap.workers.dev/ori/{image.sha1}/{image.original_filename}">
    <img class="rounded-lg h-96"
         src="https://intermodal-storage-worker.claudioap.workers.dev/medium/{image.sha1}/stop"/>
  </a>
  <div class="rounded-lg w-96 h-96 shrink-0" use:mapAction></div>
</div>
      <div class="flex space-x-5">
        <label for="is-sensitive">Sensitive
          <input id="is-sensitive" type="checkbox" bind:checked={isSensitive} /> (eg. faces)
        </label>
        <label for="is-public">Public
          <input id="is-public" type="checkbox" bind:checked={isPublic} /> (after sensitivities removed)
        </label>
        <div>
          <span>Location:</span>
          <span>{#if location.lat}{location.lat};{location.lon}{:else}Unset{/if}</span>
        </div>
      </div>
      <div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Stops</span>
          </label>
          <div>
            {#each stopIds as stopId }
              <div class="badge badge-outline badge-lg">
                {stopId} - {$stops[stopId].short_name || $stops[stopId].name}
                <div class="btn btn-error btn-circle btn-xs" on:click={() => { removeStop(stopId) }}>✕</div>
              </div>
            {/each}
            <input
                type="number"
                disabled
                class="input input-bordered"
                id="stop-id"
                placeholder="Select on map"
                bind:this={stopInput} />
            <select id="stop-pos" class="select select-bordered">
              <option>Foreground</option>
              <option>Background</option>
            </select>
            <input class="btn" type="button" value="Add" on:click={addStop} />
          </div>
        </div>
      </div>
      <div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Tags</span>
          </label>
          <!-- <input type="text" id="tags" placeholder="Insert previous tags here" />-->
          <div>
            {#each tags as tag }
              <div class="badge badge-outline badge-lg">
                {tag}
                <div class="btn btn-error btn-circle btn-xs" on:click={() => { removeTag(tag) }}>✕</div>
              </div>
            {/each}
            <input id="tag-text" type="text" class="input input-bordered" placeholder="Creche ABC123" />
            <input class="btn" type="button" value="Add" on:click={addTag} />
          </div>
        </div>
      </div>
      <div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Notes</span>
          </label>
          <textarea class="textarea textarea-bordered h-24"
                    placeholder="Eg. While not seen properly there's a schedule to that side."
                    on:change={(e) => notes = e.target.value.trim() === "" ? null : e.target.value}></textarea>
        </div>
      </div>
    </div>
    <div class="modal-action">
      <button class="btn" on:click={close}>Close without saving</button>
      <button class="btn" on:click={save}>Save</button>
    </div>
  </div>
</div>