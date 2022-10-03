<script>
  import { createEventDispatcher } from "svelte";

  export let backBtn = false;
  export let fg = "hsl(var(--pc))";
  export let bg = "hsl(var(--p))";

  const dispatch = createEventDispatcher();

  function goBack() {
    dispatch("back");
  }
</script>

<div class="p-2 flex flex-row items-center gap-1 lg:min-h-16 w-full " style="color: {fg}; background-color: {bg}">
  {#if backBtn}
    <div class="btn btn-ghost btn-circle sm:mt-0 sm:mb-0" on:click={goBack} style="border: 2px solid {fg}33">
      <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" style="fill: {fg}">
        <path
          d="M447.1 256C447.1 273.7 433.7 288 416 288H109.3l105.4 105.4c12.5 12.5 12.5 32.75 0 45.25C208.4 444.9 200.2 448 192 448s-16.38-3.125-22.62-9.375l-160-160c-12.5-12.5-12.5-32.75 0-45.25l160-160c12.5-12.5 32.75-12.5 45.25 0s12.5 32.75 0 45.25L109.3 224H416C433.7 224 447.1 238.3 447.1 256z"
        />
      </svg>
    </div>
  {/if}
  <div class="text-lg font-bold whitespace-nowrap overflow-hidden flex flex-row gap-1">
    <slot />
  </div>
</div>

