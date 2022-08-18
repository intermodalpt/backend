<script>
  let schedule = {
    0: [45],
    7: [15, 45],
    8: [0, 15, 30, 45],
    9: [0, 15, 30, 45],
    10: [0, 15, 30, 45],
    11: [0, 15, 30],
    12: [0, 30, 45],
    13: [15, 30],
    14: [0, 15, 30, 45],
    15: [15, 30, 45],
    16: [0, 15, 30, 45],
    17: [30, 45],
    18: [45],
  };
</script>

<div class="p-2 flex flex-col gap-2">
  <div class="flex flex-col gap-2 pt-2">
    {#each Object.keys(schedule) as key}
      <div class="flex flex-row items-center gap-2">
        <span class="text-base-content rounded-full w-10 px-2 text-lg font-bold text-right">{key}</span>
        <span class="text-lg">
          {schedule[key].map((a) => (a < 10 ? "0" + a : a)).join(" ")}
        </span>
      </div>
      <div class="h-[2px] bg-base-300 w-full rounded-full" />
    {/each}
  </div>
</div>
