<script>
  import StopImageEditor from "./StopImageEditor.svelte";
  import ImageUploader from "./StopImageUploader.svelte";
  import {api_server, token} from "../../settings.js";
  import {writable} from "svelte/store";

  let uploadModal = false;
  let openedImage = writable(null);
  const pageSize = 20;

  let untaggedStopPictures = [];

  function loadMoreUntaggedStops() {
    let page = untaggedStopPictures.length / pageSize;
    let pages = [];
    if (Math.floor(page) !== page) {
      pages.push(Math.floor(page));
      pages.push(Math.floor(page) + 1);
    } else {
      pages.push(Math.floor(page));
    }

    Promise.all(pages.map((page) => {
      return fetch(`${api_server}/v1/tagging/stops/untagged?p=${page}`, {
        headers: {
          authorization: `Bearer ${$token}`
        }
      })
          .then((r) => r.json())

    }))
        .then(
            (pages) => {
              pages.forEach((results) => {
                results.forEach((image) => {
                  image.stops = [];
                });
                for (let image of results) {
                  if (untaggedStopPictures.find((pic) => {
                    return image.sha1 === pic.sha1
                  }) === undefined) {
                    untaggedStopPictures.push(image);
                  }
                }
                untaggedStopPictures = untaggedStopPictures;
              })
            }
        )
        .catch(() => alert("Unable to load untagged stops"))
  }

  loadMoreUntaggedStops();

  function openPic(id) {
    $openedImage = untaggedStopPictures.find((stop) => {
      return stop.id === id;
    });
  }

  function close() {
    uploadModal = false;
    $openedImage = null;
    untaggedStopPictures = untaggedStopPictures;
  }
</script>

<div class="flex flex-col items-center">
  <div class="w-full flex justify-between p-4 items-center">
    <h2 class="text-lg font-bold md:text-3xl text-base-content">Por Catalogar</h2>
    <button class="btn btn-primary" on:click={() => uploadModal = true}>Upload</button>
  </div>
  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
    {#each untaggedStopPictures as picture}
      {#if !picture.tagged}
        <div class="p-2 flex justify-center items-center cursor-pointer">
          <img
              src="https://intermodal-storage-worker.claudioap.workers.dev/medium/{picture.sha1}/preview"
              class="rounded-box transition-all hover:scale-105"
              on:click={() => {
              openPic(picture.id);
            }}
          />
        </div>
      {/if}
    {/each}
  </div>
  <div class="btn btn-primary" on:click={() => loadMoreUntaggedStops()}>Load more</div>
</div>
{#if uploadModal}
  <ImageUploader on:close={close} />
{/if}
{#if $openedImage}
  <StopImageEditor image={openedImage} on:close={close} />
{/if}
