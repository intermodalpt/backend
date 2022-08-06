<script>
  export let back = undefined;
</script>

<div
  class="p-2 bg-primary flex flex-row items-center gap-1 min-h-12 lg:min-h-16 w-full text-primary-content"
>
  {#if back}
    <div class="rounded-full btn btn-ghost w-12" on:click={back}>
      <span class="text-2xl">🡸</span>
    </div>
  {/if}
  <div
    class={`${
      back ? "text-lg" : "text-2xl"
    } font-bold whitespace-nowrap overflow-hidden`}
  >
    <slot />
  </div>
</div>
