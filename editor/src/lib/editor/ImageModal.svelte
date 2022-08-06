
<input type="checkbox" id="my-modal-6" class="modal-toggle" checked/>
<div class="modal modal-bottom sm:modal-middle">
  <div class="modal-box max-w-full sm:max-w-full w-max sm:w-max">
    <div class="flex flex-col">
      <div class="flex flex-row gap-4">
        <img class="rounded-lg h-64" src="https://placeimg.com/300/250/a" alt="Lol cego"/>
        <img class="rounded-lg h-64" src="https://placeimg.com/300/250/map" alt="Lol cego"/>
      </div>
      <div class="">
        <div class="form-control">
          <label class="label">
            <span class="label-text">Tags</span>
          </label> 
          <input type="text" id="tags" placeholder="Insert previous tags here"/>
        </div>
      </div>
      <div class="">
        <div class="form-control">
          <label class="label">
            <span class="label-text">Notes</span>
          </label> 
          <textarea class="textarea textarea-bordered h-24" placeholder="Insert previous notes here"></textarea>
        </div>
      </div>
    </div>
    <div class="modal-action">
      <label for="my-modal-6" class="btn">Save</label>
    </div>
  </div>
</div>

<script>
  // your script goes here
  import tagger from '@jcubic/tagger';
  import {tick} from 'svelte';
  async function hue(){
    await tick();
    tagger(document.querySelector('#tags'), {allow_spaces: false});
  }
  hue();
</script>
