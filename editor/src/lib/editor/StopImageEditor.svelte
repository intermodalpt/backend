<script>
  import {picStopRels, stopPicRels, stops} from "../../cache.js";
  import L from "leaflet";
  import {api_server, token} from "../../settings.js";
  import {icons} from "./assets.js";
  import {createEventDispatcher} from "svelte";

  export let image;

  const dispatch = createEventDispatcher();

  let map;
  let marker = null;
  let stopInput;
  let location = {
    lat: $image.lat,
    lon: $image.lon,
  };

  let tags = [];
  let stopIds = [];
  let isSensitive = $image.sensitivity;
  let isPublic = image.public;
  let notes = $image.notes;
  let quality = $image.quality || 0;

  image.subscribe((img) => {
    if (!img) {
      return;
    }
    tags = [...img.tags];
    // stopIds = [...img.stops];
    isSensitive = img.sensitive;
    isPublic = img.public;
    notes = img.notes;
    quality = img.quality || 0;

    location.lat = img.lat;
    location.lon = img.lon;

    let stopRels = $picStopRels[img.id];
    stopIds = stopRels === undefined ? [] : stopRels;
  });

  function createMap(container) {
    let m = L.map(container);

    const lastPos = JSON.parse(sessionStorage.getItem("lastPos"));

    if ($image.lat && $image.lon) {
      m.setView([$image.lat, $image.lon], 16);
    } else if (lastPos) {
      m.setView([lastPos[0], lastPos[1]], lastPos[2]);
    } else {
      m.setView([38.71856, -9.1372], 10);
    }

    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
      maxZoom: 19,
      attribution: "© OpenStreetMap",
    }).addTo(m);

    m.maxBounds = new L.LatLngBounds(new L.LatLng(38.3, -10.0), new L.LatLng(39.35, -8.0));
    m.maxBoundsViscosity = 1.0;
    m.minZoom = 10;

    let markerMoved = (e) => {
      let targetLoc = e.target.getLatLng();
      location.lon = targetLoc.lng;
      location.lat = targetLoc.lat;
    };
    if (location.lat) {
      marker = L.marker([location.lat, location.lon], {draggable: true});
      marker.addTo(m);
      marker.on("moveend", markerMoved);
    }

    m.on("click", function (e) {
      if (marker) {
        marker.removeFrom(map);
      }
      marker = L.marker([e.latlng.lat, e.latlng.lng], {draggable: true});
      location.lon = e.latlng.lng;
      location.lat = e.latlng.lat;
      marker.addTo(map);
      marker.on("moveend", markerMoved);
    });

    m.on("moveend", (e) => {
      sessionStorage.setItem("lastPos", JSON.stringify([e.target.getCenter().lat, e.target.getCenter().lng, e.target.getZoom()]));
    });

    let stopsLayer = L.markerClusterGroup({
      showCoverageOnHover: false,
      disableClusteringAtZoom: 16,
    });

    Object.values($stops).forEach((stop) => {
      if (stop.lat != null && stop.lon != null && stop.source === "osm") {
        let marker = L.marker([stop.lat, stop.lon], Object.assign({}, {icon: icons[stop.source]}));

        marker.stopId = stop.id;

        marker.on("click", (e) => (stopInput.value = e.target.stopId));

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

  function adjustQualityLabel() {
    let label = document.getElementById("quality-label");
    switch (quality) {
      case 0:
        label.textContent = "Sem informação";
        break;
      case 10:
        label.textContent = "Desfocada";
        break;
      case 20:
        label.textContent = "De dentro de um veiculo (visível na imagem)";
        break;
      case 30:
        label.textContent = "De dentro de um veiculo (reflexos ou filto no vidro)";
        break;
      case 40:
        label.textContent = "Mal direccionada";
        break;
      case 50:
        label.textContent = "Noturna";
        break;
      case 60:
        label.textContent = "Excesso ou falta de brilho";
        break;
      case 70:
        label.textContent = "Paragem não é sujeito principal";
        break;
      case 80:
        label.textContent = "Pessoas, veiculos ou lixo";
        break;
      case 90:
        label.textContent = "Imperfeições menores (seria possivel fazer melhor?)";
        break;
      case 100:
        label.textContent = "Absolutamente nada de assinalável";
        break;
      default:
        label.textContent = "?";
    }
  }

  function save() {
    if (stopIds.length === 0 && !confirm("Picture with 0 stops tagged. Do you really want to save it this way?")) {
      return;
    }

    let newMeta = {
      lat: $image.lat,
      lon: $image.lon,
      tags: tags,
      stops: stopIds,
      sensitive: isSensitive,
      public: isPublic,
      notes: notes,
      quality: quality,
    };

    if (location.lat != null) {
      if ($image.lat == null || Math.abs($image.lat - location.lat) > 0.000001) {
        newMeta.lat = location.lat;
      }
    }

    if (location.lon != null) {
      if ($image.lon == null || Math.abs($image.lon - location.lon) > 0.000001) {
        newMeta.lon = location.lon;
      }
    }

    fetch(`${api_server}/upload/stops/${$image.id}`, {
      method: "PATCH",
      body: JSON.stringify(newMeta),
      headers: {
        "Content-Type": "application/json",
        authorization: `Bearer ${$token}`
      },
    })
        .catch((e) => alert("Failed to save the stop meta"))
        .then(() => {
          Object.assign($image, newMeta);
          $image.tagged = true;

          let oldStopIds = $picStopRels[$image.id];
          if (oldStopIds) {
            let removedStopIds = oldStopIds.filter(x => !stopIds.includes(x));
            removedStopIds.forEach((stopId) => {
              const picIds = $stopPicRels[stopId];
              if (picIds.indexOf($image.id)) {
                picIds.push($image.id);
              }
            });
          }

          stopIds.forEach((stopId) => {
            const picIds = $stopPicRels[stopId];
            if (picIds.indexOf($image.id)) {
              picIds.push($image.id);
            }
          });

          dispatch("close");
        });
  }

  function deleteImage() {
    if (confirm("Are you really really sure?")) {
      fetch(`${api_server}/upload/stops/${$image.id}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          authorization: `Bearer ${$token}`
        },
      })
          .catch(() => alert("Failed to delete the image"))
          .then(() => {
            $image.tagged = true;
            dispatch("close");
          });
    }
  }

  function close() {
    dispatch("close");
  }
</script>

<div class="modal modal-bottom sm:modal-middle modal-open">
  <div class="modal-box max-w-full sm:max-w-full w-max">
    <div class="flex flex-col gap-1">
      <div class="flex lg:flex-row flex-col-reverse gap-1 items-center">
        <a target="_blank"
           href="https://intermodal-storage-worker.claudioap.workers.dev/ori/{$image.sha1}/{$image.original_filename}"
           class="block shrink-0">
          <img class="rounded-lg h-96"
               alt="Visualização paragem"
               src="https://intermodal-storage-worker.claudioap.workers.dev/medium/{$image.sha1}/stop" />
        </a>
        <div class="rounded-lg grow-1 h-96 w-full cursor-crosshair" use:mapAction></div>
      </div>
      <div class="flex justify-between space-x-5">
        <div>
          <label class="btn btn-success w-40" class:btn-error={isSensitive} for="is-sensitive">
            {#if isSensitive}Sensitive{:else }Not sensitive{/if}
            <input id="is-sensitive" type="checkbox" class="hidden" bind:checked={isSensitive} />
          </label>
          <label class="btn btn-success w-40" class:btn-error={!isPublic} for="is-public">
            {#if isPublic}Can be public{:else }Private{/if}
            <input id="is-public" type="checkbox" class="hidden" bind:checked={isPublic} />
          </label>
        </div>
        <div class="">
          <span>Location:</span>
          <span>{#if location.lat}{location.lat};{location.lon}{:else}Unset{/if}</span>
        </div>
      </div>
      <div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Quality</span>
            <span class="label-text" id="quality-label">Sem informação</span>
          </label>
          <input
              type="range"
              min="0"
              max="100"
              class="range"
              step="10"
              bind:value={quality}
              on:change={adjustQualityLabel}
          />
          <div class="w-full flex justify-between text-xs px-2">
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
            <span>|</span>
          </div>
        </div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Stops</span>
          </label>
          <div>
            {#each stopIds as stopId}
              <div class="badge badge-outline badge-lg">
                {stopId} - {$stops[stopId].short_name || $stops[stopId].name}
                <div class="btn btn-error btn-circle btn-xs" on:click={() => removeStop(stopId)}>✕</div>
              </div>
            {/each}
            <input
                type="number"
                disabled
                class="input input-bordered"
                id="stop-id"
                placeholder="Select on map"
                bind:this={stopInput}
            />
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
          <div>
            {#each tags as tag}
              <div class="badge badge-outline badge-lg">
                {tag}
                <div class="btn btn-error btn-circle btn-xs" on:click={() => removeTag(tag)}>✕</div>
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
          <textarea
              class="textarea textarea-bordered h-12"
              placeholder="Eg. While not seen properly there's a schedule to that side."
              on:change={(e) => (notes = e.target.value.trim() === "" ? null : e.target.value)}
          />
        </div>
      </div>
    </div>
    <div class="modal-action">
      <button class="btn btn-error" on:click={deleteImage}>Delete</button>
      <span class="grow"></span>
      <button class="btn" on:click={close}>Close without saving</button>
      <button class="btn btn-primary" on:click={save}>Save</button>
    </div>
  </div>
</div>
