<script>
  let files;
</script>

<input type="checkbox" id="my-modal-6" class="modal-toggle" checked />
<div class="modal modal-bottom sm:modal-middle">
  <div class="modal-box">
    <div class="flex flex-col gap-1 ">
      {#if files && files[0]}
        {#each files as file}
          <div class="flex flex-row justify-between items-center">
            <div class="font-bold">{file.name}</div>
            <div class="btn btn-error btn-circle btn-sm">✕</div>
          </div>
        {/each}
      {:else}
        <div class="text-base-content text-bold opacity-50 p-2">
          Select a few files to begin
        </div>
      {/if}
    </div>
    <div
      class="btn float-right mt-3 btn-secondary {files && files[0]
        ? ''
        : 'btn-disabled'}"
    >
      Upload
    </div>

    <label for="dropzone-file" class="float-left mt-3 btn btn-primary">
      Select Files
      <input
        bind:files
        multiple
        accept="image/*"
        id="dropzone-file"
        type="file"
        class="hidden"
      />
    </label>
  </div>
</div>
