<script>
  import ImageModal from "./ImageModal.svelte";
  import ImageUpload from "./ImageUpload.svelte";
  import Notification from "./ImageNotif.svelte";
</script>

<div class="flex flex-col">
  <div class="w-full flex justify-between p-4 items-center">
    <h2 class="text-lg font-bold md:text-3xl text-base-content">
      Por Catalogar
    </h2>
    <button class="btn btn-primary">Upload</button>
  </div>
  <div
    class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5"
  >
    <div class="p-2 flex justify-center items-center cursor-pointer ">
      <img
        src="https://placeimg.com/250/180/arch"
        class="rounded-box transition-all hover:scale-105"
      />
    </div>
  </div>
</div>
<!-- <ImageUpload/> -->
<!-- <ImageModal /> -->
<Notification />
