<script>
  export let onEnter, onUnfocus, onFocus, onLeave, gotoRoute, gotoInfo, gotoSchedule;
  export let route;
  let open = false;
</script>

<div
  tabindex="0"
  on:focusin={() => {
    open = true;
    console.log("fox");
    onFocus(route.id);
  }}
  on:focusout={() => {
    open = false;
    console.log("unfox");
    onUnfocus(route.id);
  }}
>
  <div
    class="cursor-pointer flex flex-row items-center p-1 gap-1 sm:gap-2  bg-base-100 hover:bg-base-300 rounded-full"
    on:mouseenter={() => onEnter(route.id)}
    on:mouseleave={() => onLeave(route.id)}
  >
    <div class="flex flex-row items-center rounded-full  border-base-content border-2 shrink-0">
      <img class="ml-1 w-7 px-[2px]" src="/logos/cmet-min.svg" alt="CM" />
      <div
        class="rounded-full px-2 py-1 -my-[2px] -mr-[2px] text-lg"
        style:background-color="#{route.badge_bg}"
        style:color="#{route.badge_text}"
      >
        {route.code}
      </div>
    </div>
    {#if route.name.split(" - ").length === 2}
      <div>
        <span>{route.name.split(" - ")[0]}</span><br />
        <span>{route.name.split(" - ")[1]}</span>
      </div>
    {:else}
      <div>{route.name}</div>
    {/if}
  </div>
  <div class="overflow-hidden max-w-xs mx-auto">
    <div class="transition-all flex justify-between max-w-lg gap-4 h-20 {open ? '' : '-mt-20'}">
      <div class="flex flex-col items-center cursor-pointer" on:click={() => gotoRoute(route.id)}>
        <img class="w-10" src="/icons/route.svg" alt="Percurso" />
        <span class="text-lg">Percurso</span>
      </div>
      <div class="flex flex-col items-center cursor-pointer" on:click={() => gotoSchedule(route.id)}>
        <img class="w-10" src="/icons/time.svg" alt="Horário" />
        <span class="text-lg">Horário</span>
      </div>
      <div class="flex flex-col items-center cursor-pointer" on:click={() => gotoInfo(route.id)}>
        <img class="w-10" src="/icons/info.svg" alt="Informação" />
        <span class="text-lg">Informação</span>
      </div>
    </div>
  </div>
</div>
<hr />
