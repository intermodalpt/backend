<script>
  import { calendarStr, randomHexColor, weekdayName } from "../../utils.js";
  import { derived, writable } from "svelte/store";
  import { api_server } from "../../settings.js";

  let departures = [];
  randomHexColor();
  let calendarColors = {};

  for (const departure of departures) {
  }

  export const selectedRouteId = writable(223);
  export const selectedSubrouteId = writable(513);
  export const schedule = derived([selectedRouteId], async ([$selectedRouteId, $selectedDay], set) => {
    if ($selectedRouteId) {
      await fetch(`${api_server}/v1/routes/${$selectedRouteId}/schedule`)
          .catch(() => {
          })
          .then((r) => r.json())
          .catch(() => {
          })
          .then((data) => set(data));
    }
  });

  export const subrouteSchedule = derived([selectedSubrouteId, schedule], ([$selectedSubrouteId, $schedule]) => {
    if ($schedule) {
      let currentSchedule = $schedule.filter((entry) => {
        return entry.subroute === $selectedSubrouteId;
      });

      let schMatrix = {};
      let schTypes = [];
      for (let e of currentSchedule) {
        let h = Math.floor(e.time / 60);
        let m = String(Math.floor(e.time % 60)).padStart(2, "0");
        let schId = JSON.stringify(e.calendar);
        if (!schMatrix[h]) schMatrix[h] = [];
        if (!schTypes.includes(schId)) schTypes.push(schId);
        schMatrix[h].push({ minute: m, conditions: schId });
      }
      let schArray = [];
      for (let k of Object.keys(schMatrix).sort()) {
        schArray.push({ k: k, minutes: schMatrix[k] });
      }
      return [schArray, schTypes];
    }
  });

  let newTime = null;
  let newConditionType = null;
  let newConditionPeriod = null;
  $: newConditionPeriodReady =
      newConditionPeriod != null &&
      ((newConditionPeriod === "Range" &&
              rangeStart != null &&
              rangeStart !== "" &&
              rangeEnd != null &&
              rangeEnd !== "") ||
          (newConditionPeriod === "Nth" && nth != null) ||
          !(newConditionPeriod === "Range" || newConditionPeriod === "Nth"));
  let newCondition = null;
  let newWeekdays = [0, 1, 2, 3, 4];
  let newCalendar = {
    only_if: [],
    also_if: [],
    except_if: [],
  };
  let nth = null;
  let rangeStart = null;
  let rangeEnd = null;
  $: newCalendar.weekdays = newWeekdays;

  function addModifier() {
    let modifier;
    if (newConditionPeriod === "Range") {
      console.log(rangeStart);
      const startParts = rangeStart.split("-");
      const endParts = rangeEnd.split("-");
      const start = [parseInt(startParts[1]), parseInt(startParts[2])];
      const end = [parseInt(endParts[1]), parseInt(endParts[2])];
      modifier = {condition: "Range", start: start, end: end};
    } else if (newConditionPeriod === "Nth") {
      modifier = {condition: "Nth", Nth: nth};
    } else {
      modifier = {condition: newConditionPeriod};
    }
    newCalendar[newConditionType].push(modifier);
    newCalendar = newCalendar;
  }
</script>

<span class="text-lg">Current departures</span><br />

{#if $subrouteSchedule}
  <div class="flex flex-row gap-1 bg-base-200 p-1 rounded-xl w-min mx-auto">
    {#each $subrouteSchedule[0] as scheduleEntry}
      <div class="bg-base-100 rounded-lg flex flex-col min-w-[1.0rem] items-start p-1">
        <div class="font-bold">{scheduleEntry.k}</div>
        {#each scheduleEntry.minutes as min}
          <div class="whitespace-nowrap">
            {min.minute}<sup>{String.fromCharCode($subrouteSchedule[1].indexOf(min.conditions) + 96 + 1)}</sup>
            <class
              on:click={() =>
                alert("Olha aqui dei delete da coisa, se ainda está é impressão tua isto está totalmente implementado")}
              class="btn btn-circle btn-xs btn-ghost hover:bg-error">✕</class>
          </div>
        {/each}
      </div>
    {/each}
  </div>
  <div class="flex flex-col">
    {#each $subrouteSchedule[1] as schType, i}
      <div>
        {String.fromCharCode(i + 96 + 1)}
        {calendarStr(JSON.parse(schType))}
      </div>
    {/each}
  </div>
{/if}

<!-- <ul class="flex flex-col gap-2"> -->
<!--   {#if $subrouteShedule} -->
<!--     {#each $subrouteShedule as scheduleEntry} -->
<!--       <li class="bg-base-200 rounded-lg">{JSON.stringify(scheduleEntry)}</li> -->
<!--     {/each} -->
<!--   {/if} -->
<!-- </ul> -->

<hr />
<hr />
<span class="text-lg">Add another departure</span><br />
<div class="border-2 rounded-lg p-2">
  <div class="form-control w-full max-w-xs">
    <label class="input-group">
      <span class="label-text w-24">Time</span>
      <input type="time" class="input input-bordered input-sm w-fit" bind:value={newTime} />
    </label>
  </div>
  <span class="text-md">In the weekdays</span>
  <div class="flex gap-4">
    <label><input class="checkbox" name="weekdays" type="checkbox" value={0} bind:group={newWeekdays} /> Mo</label>
    <label><input class="checkbox" name="weekdays" type="checkbox" value={1} bind:group={newWeekdays} /> Tu</label>
    <label><input class="checkbox" name="weekdays" type="checkbox" value={2} bind:group={newWeekdays} /> We</label>
    <label><input class="checkbox" name="weekdays" type="checkbox" value={3} bind:group={newWeekdays} /> Th</label>
    <label><input class="checkbox" name="weekdays" type="checkbox" value={4} bind:group={newWeekdays} /> Fr</label>
    <label><input class="checkbox" name="weekdays" type="checkbox" value={5} bind:group={newWeekdays} /> Sa</label>
    <label><input class="checkbox" name="weekdays" type="checkbox" value={6} bind:group={newWeekdays} /> Su</label>
  </div>

  <div class="border-2 rounded-lg p-2">
    <div class="flex gap-8">
      <span class="text-md">That applies</span>
      <label>
        <input class="radio" name="exception-type" type="radio" value="only_if" bind:group={newConditionType} />
        Only if
      </label>
      <label>
        <input class="radio" name="exception-type" type="radio" value="except_if" bind:group={newConditionType} />
        Except if
      </label>
      <label>
        <input class="radio" name="exception-type" type="radio" value="also_if" bind:group={newConditionType} />
        Also if
      </label>
    </div>
    <div class="flex gap-12">
      <span class="text-md">By</span>
      <label>
        <input class="radio" name="period" type="radio" value="Summer" bind:group={newConditionPeriod} />
        Summer
      </label>
      <label>
        <input class="radio" name="period" type="radio" value="School" bind:group={newConditionPeriod} />
        School
      </label>
      <label>
        <input class="radio" name="period" type="radio" value="Holiday" bind:group={newConditionPeriod} />
        Holiday
      </label>
      <div class="flex flex-col gap-2">
        <label>
          <input
              class="radio"
              name="period"
              type="radio"
              value="Nth"
              bind:group={newConditionPeriod} />
          Occurence#
        </label>
        <div class="form-control">
          <label class="input-group">
            <span class="label-text w-16">Nth</span>
            <input
                type="number"
                min="1"
                max="5"
                class="input input-bordered input-xs w-16 h-10"
                disabled={newConditionPeriod !== "Nth"}
                bind:value={nth} />
          </label>
        </div>
      </div>
      <div class="flex flex-col gap-2">
        <label>
          <input class="radio" name="period" type="radio" value="Range" bind:group={newConditionPeriod} />
          Range
        </label>
        <div class="form-control">
          <label class="input-group">
            <span class="label-text w-16">From</span>
            <input
                type="date"
                class="input input-bordered input-xs w-fit"
                disabled={newConditionPeriod !== "Range"}
                bind:value={rangeStart} />
          </label>
        </div>
        <div class="form-control">
          <label class="input-group">
            <span class="label-text w-16">To</span>
            <input
                type="date"
                class="input input-bordered input-xs w-fit"
                disabled={newConditionPeriod !== "Range"}
                bind:value={rangeEnd} />
          </label>
        </div>
      </div>
    </div>
    <div class="flex justify-end">
      <button
          class="btn btn-info btn-xs"
          disabled={!(newConditionPeriodReady && newConditionType)}
          on:mouseup={addModifier}>
        Add modifier
      </button>
    </div>
  </div>
  <span>Current addition: {calendarStr(newCalendar)}</span>
  <div class="flex justify-end">
    <button class="btn btn-success btn-sm" disabled={!newTime} on:mouseup={addModifier}> Confirm</button>
  </div>
</div>
<div class="flex justify-end">
  <button class="btn btn-primary">Save</button>
</div>