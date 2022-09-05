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
    <div class="rounded-full btn btn-ghost -ml-3 -mt-2 -mb-2 sm:mt-0 sm:mb-0 " on:click={goBack}>
      <img src="/icons/arrow-left-solid.svg" class="w-6" alt="Voltar"/>
    </div>
  {/if}
  <div class="text-lg font-bold whitespace-nowrap overflow-hidden flex flex-row gap-1">
    <slot />
  </div>
</div>

