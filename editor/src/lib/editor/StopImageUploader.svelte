<script>
  import {api_server, token} from "../../settings.js";
  import {createEventDispatcher} from "svelte";

  const dispatch = createEventDispatcher();

  let files;
  let uploading = false;

  // function dropFile(i) {
  //   files = files.splice(i, 1);
  // }

  function upload() {
    const formData = new FormData();

    for (let x = 0; x < files.length; x++) {
      formData.append("images[]", files[x]);
    }
    uploading = true;
    fetch(`${api_server}/upload/stops`, {
      method: 'POST',
      body: formData,
      headers: {
        authorization: `Bearer ${$token}`
      }
    }).then(data => {
      alert("Done");
      uploading = false;
    }).catch(() => {
      alert("Something wrong didn't go right");
      uploading = false;
    });
  }
</script>

<input type="checkbox" id="my-modal-6" class="modal-toggle" checked />
<div class="modal modal-bottom sm:modal-middle">
  <div class="modal-box">
    {#if uploading}
      <span>Upload in progress</span><br />
      <div class="radial-progress" style="--value:90; --size:12rem; --thickness: 2rem;">99%</div>
    {:else}
      <div class="flex flex-col gap-1 ">
        {#if files && files[0]}
          {#each files as file, i}
            <div class="flex flex-row justify-between items-center">
              <div class="font-bold">{file.name}</div>
              <!--            <div class="btn btn-error btn-circle btn-sm" on:click={() => { dropFile(i)}}>✕</div>-->
            </div>
          {/each}
        {:else}
          <div class="flex flex-row justify-between items-center">
            <div class="text-base-content text-bold opacity-50 p-2">Select a few files to begin</div>
            <div class="btn btn-error btn-circle btn-sm" on:click={() => dispatch("close")}>✕</div>
          </div>
        {/if}
      </div>
      <div class="btn float-right mt-3 btn-secondary {files && files[0] ? '' : 'btn-disabled'}" on:click={upload}>
        Upload
      </div>

      <label for="dropzone-file" class="float-left mt-3 btn btn-primary">
        Select Files
        <input bind:files multiple accept="image/*" id="dropzone-file" type="file" class="hidden" />
      </label>
    {/if}
  </div>
</div>
