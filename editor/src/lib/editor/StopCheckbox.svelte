<script>
  import {onMount} from "svelte";

  export let text = "Missing label";
  export let description = "Missing description";
  export let state;

  let box;

  onMount(() => {
    state.subscribe(() => {
      matchState();
    });
    matchState();
  });

  function matchState() {
    if ($state === null) {
      box.indeterminate = true;
      box.checked = false;
    } else if ($state) {
      box.indeterminate = false;
      box.checked = true;
    } else {
      box.indeterminate = false;
      box.checked = false;
    }
  }

  function rotateState() {
    if ($state === null) {
      box.indeterminate = false;
      box.checked = true;
      $state = true;
    } else if ($state) {
      box.indeterminate = false;
      box.checked = false;
      $state = false;
    } else {
      box.indeterminate = true;
      box.checked = false;
      $state = null;
    }
  }
</script>

<div class="form-control">
  <label class="flex gap-1 cursor-pointer">
    <input
        id="cbox"
        class="checkbox checkbox-accent indeterminate:checkbox-primary"
        type="checkbox"
        bind:this={box}
        on:click={rotateState}
    />
    <span class="label-text flex flex-row gap-1 items-center">
      {text}
      <div class="dropdown relative dropdown-top dropdown-left">
        <label tabindex="0" class="btn btn-circle btn-ghost btn-xs text-info">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-4 h-4 stroke-current">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
        </label>
        <div tabindex="0" class="card compact dropdown-content shadow bg-base-100 rounded-box w-64">
          <div class="card-body">
            <p>{description}</p>
          </div>
        </div>
      </div>
    </span>
  </label>
</div>
