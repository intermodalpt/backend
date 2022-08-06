<script>
  import { createEventDispatcher } from "svelte";

  export let routes = undefined;
  $: routeEntries = routes && Object.entries(routes);

  const dispatch = createEventDispatcher();

  function onClick(routeId, a) {
    dispatch("openroute", { routeId: parseInt(routeId) });
  }

  function onEnter(routeId) {
    dispatch("hint", { routeId: parseInt(routeId) });
  }

  function onLeave(routeId) {
    dispatch("drophint", { routeId: parseInt(routeId) });
  }
</script>

<div class="w-full h-full flex-col flex">
  <h2
    class="p-3 lg:p-4 font-bold text-xl lg:text-2xl text-primary-content bg-primary h-12 lg:h-16"
  >
    Rotas
  </h2>
  {#if routes}
    <div class="overflow-y-scroll">
      <div class="flex flex-col gap-1 p-2">
        {#each routeEntries as [id, info]}
          <div
            class="cursor-pointer flex flex-row items-center p-1 gap-1 sm:gap-2 hover:bg-base-300 rounded-full"
            on:click={() => onClick(id)}
            on:mouseenter={() => onEnter(id)}
            on:mouseleave={() => onLeave(id)}
          >
            <div
              class="flex flex-row items-center rounded-full bg-base-100 border-base-content border-2 shrink-0"
            >
              <img
                class="ml-1 w-7 px-[2px]"
                src="/logos/cmet-min.svg"
                alt="CM"
              />
              <div
                class="rounded-full bg-primary px-2 py-1 -my-[2px] -mr-[2px] font-black text-primary-content text-lg"
                style=""
              >
                {info.code}
              </div>
            </div>
            {#if info.name.split(" - ").length === 2}
              <div>
                <span>{info.name.split(" - ")[0]}</span><br />
                <span>{info.name.split(" - ")[1]}</span>
              </div>
            {:else}
              <div>{info.name}</div>
            {/if}
          </div>
          <hr />
        {/each}
      </div>
    </div>
  {:else}
    <div class="p-4">
      Selecione uma região do mapa para visualizar as rotas existentes.
    </div>
  {/if}
</div>

<link rel="stylesheet" href="https://unpkg.com/balloon-css/balloon.min.css" />
