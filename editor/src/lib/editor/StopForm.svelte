<script>
  import { createEventDispatcher } from "svelte";
  import StopCheckbox from "./StopCheckbox.svelte";

  export let stop;

  const simpleCheckboxes = [
    "Tem um passeio",
    "Tem uma passadeira",
    "A paragem está imune a estacionisses",
    "Tem horários",
    "Tem postaletes",
    "Tem informação obsoleta",
    "Tem abrigo",
    "Tem bancos",
    "Danificada",
  ];

  let name = stop.name;
  let short_name = stop.short_name;
  let street = stop.street;
  let door = stop.door;
  let source = stop.source;
  let notes = "";

  $: name = stop.name;
  $: short_name = stop.short_name;
  $: street = stop.street;
  $: door = stop.door;
  $: source = stop.source;
  notes = "";

  function save() {
    dispatch(
      "save",
      Object.assign(stop, {
        name: name,
        short_name: short_name,
        street: street,
        door: door,
        source: source,
      })
    );
  }

  const dispatch = createEventDispatcher();
</script>

<div class="flex flex-col gap-1 p-2 overflow-visible ">
  <div class="form-control w-full max-w-xs">
    <label class="input-group">
      <span class="label-text w-24">Name</span>
      <input type="text" placeholder={name} class="input input-bordered w-full" />
    </label>
  </div>
  <div class="form-control w-full max-w-xs">
    <label class="input-group">
      <span class="label-text w-24">Short name</span>
      <input type="text" placeholder={short_name} class="input input-bordered w-full" />
    </label>
  </div>
  <div class="form-control w-full max-w-xs">
    <label class="input-group">
      <span class="label-text w-24">Street</span>
      <input type="text" placeholder={street} class="input input-bordered w-full" />
    </label>
  </div>
  <div class="form-control w-full max-w-xs">
    <label class="input-group">
      <span class="label-text w-24">Door</span>
      <input type="text" placeholder={door} class="input input-bordered w-full " />
    </label>
  </div>
  <div class="form-control w-full max-w-xs">
    <label class="input-group">
      <span class="label-text w-24">Source</span>
      <input type="text" placeholder={source} class="input input-bordered w-full" />
    </label>
  </div>

  {#each simpleCheckboxes as text}
    <StopCheckbox {text} description="Descricao temporaria" />
  {/each}
  <StopCheckbox text={"Paragem"} icon={"light"} />
  <StopCheckbox text={"Acesso"} icon={"light"} />
  <StopCheckbox text={"Abrigo -> Autocarro"} icon={"eye"} />
  <StopCheckbox text={"Paragem -> Autocarro"} icon={"eye"} />
  <StopCheckbox text={"Autocarro -> Paragem"} icon={"eye"} />
  <button class="btn btn-primary w-20" on:click={save}> Save </button>
</div>

<!--    <textarea bind:notes></textarea><br>-->
