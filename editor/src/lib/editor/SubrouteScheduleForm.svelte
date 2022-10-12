<script>
  import { calendarStr, isDeepEqual, timestampToTime } from "../../utils.js";
  import { derived, writable } from "svelte/store";
  import { api_server, token } from "../../settings.js";

  let creatingDeparture = false;

  export let selectedRouteId;
  export let selectedSubrouteId ;

  const newDepartures = writable([]);
  const patchedDepartures = writable({});
  const deletedDepartures = writable(new Set());

  const editingId = writable(undefined);


  export const schedule = derived([selectedRouteId], async ([$selectedRouteId, $selectedDay], set) => {
    console.log($selectedRouteId);
    if ($selectedRouteId) {
      await fetch(`${api_server}/v1/routes/${$selectedRouteId}/schedule`)
          .catch(() => {
            alert("Failed to load the schedule")
          })
          .then((r) => r.json())
          .catch(() => {
            alert("Failed to parse the schedule")
          })
          .then((data) => set(data));
    }
  });

  const subrouteSchedule = derived(
      [selectedSubrouteId, schedule, newDepartures, patchedDepartures, deletedDepartures],
      ([$selectedSubrouteId, $schedule, $newDepartures, $patchedDepartures, $deletedDepartures]) => {

        if ($schedule) {
          let sourceDeparture = $schedule.find((departure) => departure.id === $selectedSubrouteId);
          let currentSchedule = $schedule.filter((departure) => {
            return departure.subroute === $selectedSubrouteId && !$deletedDepartures.has(departure.id);
          }).map((departure) => {
            let patch = $patchedDepartures[departure.id];
            if (patch) {
              departure.time = patch.time;
              departure.calendar = patch.calendar;
            }
            return departure;
          }).concat($newDepartures);
          return currentSchedule;
        }
      }
  );


  const editingDeparture = derived([subrouteSchedule, editingId], ([$subrouteSchedule, $editingId]) => {
    if ($editingId) {
      let departure = $subrouteSchedule?.find((e) => e.id === $editingId);
      console.log(departure);
      formTime = timestampToTime(departure.time);
      formWeekdays = departure.calendar.weekdays;
      formCalendar = departure.calendar;

      return departure;
    }
  });

  const departuresAndCalendars = derived(subrouteSchedule, ($subrouteSchedule) => {
    if ($schedule) {
      let scheduleMat = {};
      let calendars = [];
      for (let e of $subrouteSchedule) {
        let hour = Math.floor(e.time / 60);
        let minute = String(Math.floor(e.time % 60)).padStart(2, "0");
        if (!scheduleMat[hour]) scheduleMat[hour] = [];

        let calendarIndex = calendars.findIndex((calendar) => isDeepEqual(calendar, e.calendar))

        if (calendarIndex === -1) {
          calendarIndex = calendars.length;
          calendars.push(e.calendar);
        }

        scheduleMat[hour].push({id: e.id, minute: minute, calendarIndex: calendarIndex});
      }
      let departures = {};
      for (let hour of Object.keys(scheduleMat).sort()) {
        departures[hour] = scheduleMat[hour];
      }
      return {departures: departures, calendars: calendars};
    }
  });

  let formTime = null;
  let formNewConditionType = null;
  let formNewConditionPeriod = null;
  $: newConditionPeriodReady =
      formNewConditionPeriod != null &&
      ((formNewConditionPeriod === "Range" &&
              rangeStart != null &&
              rangeStart !== "" &&
              rangeEnd != null &&
              rangeEnd !== "") ||
          (formNewConditionPeriod === "Nth" && nth != null) ||
          !(formNewConditionPeriod === "Range" || formNewConditionPeriod === "Nth"));
  let formNewCondition = null;
  let formWeekdays = [0, 1, 2, 3, 4];
  let formCalendar = {
    only_if: [],
    also_if: [],
    except_if: [],
  };
  let nth = null;
  let rangeStart = null;
  let rangeEnd = null;
  $: formCalendar.weekdays = formWeekdays;

  function clearForm() {
    formTime = null;
    // formNewConditionType = null;
    // formNewConditionPeriod = null;
    // formNewCondition = null;
    // formCalendar = {
    //   only_if: [],
    //   also_if: [],
    //   except_if: [],
    // };
    // formWeekdays = [0, 1, 2, 3, 4];
    // nth = null;
    // rangeStart = null;
    // rangeEnd = null;
  }

  function addModifier() {
    let modifier;
    if (formNewConditionPeriod === "Range") {
      console.log(rangeStart);
      const startParts = rangeStart.split("-");
      const endParts = rangeEnd.split("-");
      const start = [parseInt(startParts[1]), parseInt(startParts[2])];
      const end = [parseInt(endParts[1]), parseInt(endParts[2])];
      modifier = {condition: "Range", start: start, end: end};
    } else if (formNewConditionPeriod === "Nth") {
      modifier = {condition: "Nth", Nth: nth};
    } else {
      modifier = {condition: formNewConditionPeriod};
    }
    formCalendar[formNewConditionType].push(modifier);
    formCalendar = formCalendar;
  }

  function indexToChar(index) {
    const firstLetterOffset = 96;
    return String.fromCharCode(index + firstLetterOffset + 1)
  }

  function editEntry(id) {
    console.log(id);
    creatingDeparture = false;
    $editingId = id;
  }

  async function deleteEntry(id) {
    await fetch(`${api_server}/v1/schedules/${$selectedSubrouteId}/${id}`, {
      method: "DELETE",
      headers: {authorization: `Bearer ${$token}`},
    });
    $deletedDepartures.add(id);
    $deletedDepartures = $deletedDepartures;
  }

  async function saveDeparture() {
    let calendar = formCalendar;
    if (!formTime) {
      alert("Time is unset");
      return;
    }

    let hour = parseInt(formTime.split(':')[0]);
    if (hour < 4) {
      hour += 24;
    }
    let minute = parseInt(formTime.split(':')[1]);

    let timestamp = hour * 60 + minute;
    console.log(timestamp);


    let departure = {
      time: timestamp,
      calendar: calendar,
    };

    if (creatingDeparture) {
      await fetch(`${api_server}/v1/schedules/${$selectedSubrouteId}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          authorization: `Bearer ${$token}`
        },
        body: JSON.stringify(departure),
      })
          .then((x) => x.json())
          .then((resp) => {
            departure.id = resp.id;
            departure.subroute = $selectedSubrouteId;
            $newDepartures.push(departure);
            $newDepartures = $newDepartures;
            clearForm();
            creatingDeparture = false;
          });
    } else {
      await fetch(`${api_server}/v1/schedules/${$selectedSubrouteId}/${$editingId}`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
          authorization: `Bearer ${$token}`
        },
        body: JSON.stringify(departure),
      })
          .then(() => {
            $patchedDepartures[$editingId] = departure;
            $patchedDepartures = $patchedDepartures;
            clearForm();
            $editingId = undefined;
          });
    }

  }
</script>

<span class="text-lg">Current departures</span><br />

{#if $departuresAndCalendars}
  <div class="flex flex-row gap-1 bg-base-200 p-1 rounded-xl w-min mx-auto">
    {#each Object.entries($departuresAndCalendars.departures) as [hour, departures]}
      <div class="bg-base-100 rounded-lg flex flex-col min-w-[1.0rem] items-start p-1  group">
        <div class="font-bold">{hour}</div>
        {#each departures as departure}
          <div class="whitespace-nowrap">
            <a class="cursor-pointer hover:bg-base-300"
               on:mouseup={() => {editEntry(departure.id)}}>{departure.minute}</a>
            <sup>{indexToChar(departure.calendarIndex)}</sup>
            <div
                on:mouseup={() => deleteEntry(departure.id)}
                class="btn btn-circle btn-xs btn-ghost hover:bg-error opacity-0 group-hover:opacity-100 -ml-7
                group-hover:-ml-1 transition-all -z-40">
              ✕
            </div>
          </div>
        {/each}
      </div>
    {/each}
  </div>
  <div class="flex flex-col">
    {#each $departuresAndCalendars.calendars as calendar, i}
      <div>
        {String.fromCharCode(i + 96 + 1)} - {calendarStr(calendar)}
      </div>
    {/each}
  </div>
{/if}

{#if !(creatingDeparture || $editingDeparture)}
  <div class="flex justify-end">
    <button class="btn btn-primary" on:mouseup={() => creatingDeparture=true}>New departure</button>
  </div>
{/if}

{#if creatingDeparture || $editingDeparture}
  <div class="border-2 rounded-lg p-2">
    <div class="flex justify-between">
      {#if creatingDeparture}
        <span class="text-lg">Add another departure</span><br />
      {:else if $editingDeparture}
        <span class="text-lg">Edit departure</span><br />
      {/if}
      <div
          on:mouseup={() => {creatingDeparture = false; $editingId = undefined;}}
          class="btn btn-circle btn-xs btn-error bg-error">✕
      </div>
    </div>

    <div class="border-2 rounded-lg p-2">
      <div class="form-control w-full max-w-xs">
        <label class="input-group">
          <span class="label-text w-24">Time</span>
          <input type="time" class="input input-bordered input-sm w-fit" bind:value={formTime} />
        </label>
      </div>
      <span class="text-md">In the weekdays</span>
      <div class="flex gap-4">
        <label><input class="checkbox" name="weekdays" type="checkbox" value={0} bind:group={formWeekdays} /> Mo</label>
        <label><input class="checkbox" name="weekdays" type="checkbox" value={1} bind:group={formWeekdays} /> Tu</label>
        <label><input class="checkbox" name="weekdays" type="checkbox" value={2} bind:group={formWeekdays} /> We</label>
        <label><input class="checkbox" name="weekdays" type="checkbox" value={3} bind:group={formWeekdays} /> Th</label>
        <label><input class="checkbox" name="weekdays" type="checkbox" value={4} bind:group={formWeekdays} /> Fr</label>
        <label><input class="checkbox" name="weekdays" type="checkbox" value={5} bind:group={formWeekdays} /> Sa</label>
        <label><input class="checkbox" name="weekdays" type="checkbox" value={6} bind:group={formWeekdays} /> Su</label>
      </div>

      <div class="border-2 rounded-lg p-2">
        <div class="flex gap-8">
          <span class="text-md">That applies</span>
          <label class="flex gap-1 items-center">
            <input class="radio" name="exception-type" type="radio" value="only_if" bind:group={formNewConditionType} />
            Only if
          </label>
          <label class="flex gap-1 items-center">
            <input class="radio" name="exception-type" type="radio" value="except_if"
                   bind:group={formNewConditionType} />
            Except if
          </label>
          <label class="flex gap-1 items-center">
            <input class="radio" name="exception-type" type="radio" value="also_if" bind:group={formNewConditionType} />
            Also if
          </label>
        </div>
        <div class="flex gap-12">
          <span class="text-md">By</span>
          <label class="flex gap-1 items-center">
            <input class="radio" name="period" type="radio" value="Summer" bind:group={formNewConditionPeriod} />
            Summer
          </label>
          <label class="flex gap-1 items-center">
            <input class="radio" name="period" type="radio" value="School" bind:group={formNewConditionPeriod} />
            School
          </label>
          <label class="flex gap-1 items-center">
            <input class="radio" name="period" type="radio" value="Holiday" bind:group={formNewConditionPeriod} />
            Holiday
          </label>
          <div class="flex flex-col gap-2">
            <label class="flex gap-1 items-center">
              <input
                  class="radio"
                  name="period"
                  type="radio"
                  value="Nth"
                  bind:group={formNewConditionPeriod} />
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
                    disabled={formNewConditionPeriod !== "Nth"}
                    bind:value={nth} />
              </label>
            </div>
          </div>
          <div class="flex flex-col gap-2">
            <label class="flex gap-1 items-center">
              <input class="radio" name="period" type="radio" value="Range" bind:group={formNewConditionPeriod} />
              Range
            </label>
            <div class="form-control">
              <label class="input-group">
                <span class="label-text w-16">From</span>
                <input
                    type="date"
                    class="input input-bordered input-xs w-fit"
                    disabled={formNewConditionPeriod !== "Range"}
                    bind:value={rangeStart} />
              </label>
            </div>
            <div class="form-control">
              <label class="input-group">
                <span class="label-text w-16">To</span>
                <input
                    type="date"
                    class="input input-bordered input-xs w-fit"
                    disabled={formNewConditionPeriod !== "Range"}
                    bind:value={rangeEnd} />
              </label>
            </div>
          </div>
        </div>
        <div class="flex justify-end">
          <button
              class="btn btn-info btn-xs"
              disabled={!(newConditionPeriodReady && formNewConditionType)}
              on:mouseup={addModifier}>
            Add modifier
          </button>
        </div>
      </div>
      <span>Current calendar: {calendarStr(formCalendar)}</span>
      <div class="flex justify-end">
        <button class="btn btn-success btn-sm" disabled={!formTime} on:mouseup={saveDeparture}>
          {#if creatingDeparture}Create{:else }Change{/if}
        </button>
      </div>
    </div>
  </div>
{/if}