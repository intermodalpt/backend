<script>
  import ImageModal from "./StopImageEditor.svelte";
  import ImageUpload from "./StopImageUploader.svelte";
  import {api_server, token} from "../../settings.js";

  let uploadModal = false;
  let openedImage = null;

  let untaggedStopPictures = [];

  function loadUntaggedStops() {
    fetch(`${api_server}/tagging/stops/untagged`, {
      headers: {
        authorization: `Bearer ${$token}`
      }
    })
      .then((r) => r.json())
      .then((data) => {
        data.forEach((image) => {
          image.stops = [];
        });
        untaggedStopPictures = data;
      })
      .catch((e) => alert("Failed to load the untagged stops"));
  }

  loadUntaggedStops();

  function openPic(id) {
    openedImage = untaggedStopPictures.find((stop) => {
      return stop.id === id;
    });
  }

  function close() {
    uploadModal = false;
    openedImage = null;
    untaggedStopPictures = untaggedStopPictures;
  }
</script>

<div class="flex flex-col">
  <div class="w-full flex justify-between p-4 items-center">
    <h2 class="text-lg font-bold md:text-3xl text-base-content">Por Catalogar</h2>
    <button class="btn btn-primary" on:click={() => uploadModal = true;}>Upload</button>
  </div>
  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
    {#each untaggedStopPictures as picture}
      {#if !picture.tagged}
        <div class="p-2 flex justify-center items-center cursor-pointer">
          <img
            src="https://intermodal-storage-worker.claudioap.workers.dev/thumb/{picture.sha1}/preview"
            class="rounded-box transition-all hover:scale-105"
            on:click={() => {
              openPic(picture.id);
            }}
          />
        </div>
      {/if}
    {/each}
  </div>
</div>
{#if uploadModal}
  <ImageUpload on:close={close} />
{/if}
{#if openedImage}
  <ImageModal bind:image={openedImage} on:close={close} />
{/if}
