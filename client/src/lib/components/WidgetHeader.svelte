<script>
  export let back = undefined;
  export let fg;
  export let bg;
</script>

{#if (fg)}
  <div class="p-2 flex flex-row items-center gap-1 lg:min-h-16 w-full " style="background-color: #{bg}; color: #{fg}">
    {#if back}
      <div class="rounded-full btn btn-ghost -ml-3" on:click={back}>
        <img src="/icons/arrow-left-solid.svg" class="w-6" alt="Voltar"/>
      </div>
    {/if}
      <div class="text-lg font-bold whitespace-nowrap overflow-hidden flex flex-row gap-1 -ml-3">
        <slot />
      </div>
  </div>
{:else }
  <div class="p-2 bg-primary flex flex-row items-center gap-1 w-full text-primary-content">
    {#if back}
      <div class="rounded-full btn btn-ghost -ml-3" on:click={back}>
        <img src="/icons/arrow-left-solid.svg" alt="Voltar"/>
      </div>
    {/if}
    <div class="text-lg font-bold whitespace-nowrap overflow-hidden flex flex-row gap-1">
      <slot />
    </div>
  </div>
{/if}
