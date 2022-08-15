<script>
  import {createEventDispatcher, tick} from "svelte";
  import StopCheckbox from "./StopCheckbox.svelte";
  import {writable} from "svelte/store";

  export let stop;

  let id = $stop.id;
  let name = $stop.name;
  let short_name = $stop.short_name;
  let official_name = $stop.official_name;
  let street = $stop.street;
  let door = $stop.door;
  let notes = $stop.notes;

  let has_crossing = writable($stop.has_crossing);
  let has_accessibility = writable($stop.has_accessibility);
  let has_abusive_parking = writable($stop.has_abusive_parking);
  let has_outdated_info = writable($stop.has_outdated_info);
  let is_damaged = writable($stop.is_damaged);
  let is_vandalized = writable($stop.is_vandalized);
  let has_flag = writable($stop.has_flag);
  let has_schedules = writable($stop.has_schedules);
  let has_sidewalk = writable($stop.has_sidewalk);
  let has_shelter = writable($stop.has_shelter);
  let has_bench = writable($stop.has_bench);
  let has_trash_can = writable($stop.has_trash_can);
  let is_illuminated = writable($stop.is_illuminated);
  let has_illuminated_path = writable($stop.has_illuminated_path);
  let has_visibility_from_within = writable($stop.has_visibility_from_within);
  let has_visibility_from_area = writable($stop.has_visibility_from_area);
  let is_visible_from_outside = writable($stop.is_visible_from_outside);

  const dispatch = createEventDispatcher();




  stop.subscribe((selectedStop) => {
    if (selectedStop == null) {
      return;
    }

    id = selectedStop.id;
    name = selectedStop.name;
    short_name = selectedStop.short_name;
    street = selectedStop.street;
    door = selectedStop.door;
    notes = selectedStop.notes;

    $has_crossing = selectedStop.has_crossing;
    $has_accessibility = selectedStop.has_accessibility;
    $has_abusive_parking = selectedStop.has_abusive_parking;
    $has_outdated_info = selectedStop.has_outdated_info;
    $is_damaged = selectedStop.is_damaged;
    $is_vandalized = selectedStop.is_vandalized;
    $has_flag = selectedStop.has_flag;
    $has_schedules = selectedStop.has_schedules;
    $has_sidewalk = selectedStop.has_sidewalk;
    $has_shelter = selectedStop.has_shelter;
    $has_bench = selectedStop.has_bench;
    $has_trash_can = selectedStop.has_trash_can;
    $is_illuminated = selectedStop.is_illuminated;
    $has_illuminated_path = selectedStop.has_illuminated_path;
    $has_visibility_from_within = selectedStop.has_visibility_from_within;
    $has_visibility_from_area = selectedStop.has_visibility_from_area;
    $is_visible_from_outside = selectedStop.is_visible_from_outside;
  });

  has_visibility_from_within.subscribe((visibility_from_within) => {
    if (visibility_from_within) {
      $has_shelter = true;
      $has_visibility_from_area = true;
    }
  });

  has_visibility_from_area.subscribe((visibility_from_area) => {
    if (visibility_from_area == null) {
      $has_visibility_from_within = null;
    } else if (!visibility_from_area) {
      $has_visibility_from_within = false;
    }
  });

  has_shelter.subscribe((shelter) => {
    if (shelter == null && !shelter) {
      $has_visibility_from_within = null;
    }
  });

  function save() {
    let newMeta = {
      id: id,
      name: name,
      short_name: short_name,
      official_name: official_name,
      street: street,
      door: door,
      notes: !notes || notes.trim() === "" ? null : notes.trim(),

      has_crossing: $has_crossing,
      has_accessibility: $has_accessibility,
      has_abusive_parking: $has_abusive_parking,
      has_outdated_info: $has_outdated_info,
      is_damaged: $is_damaged,
      is_vandalized: $is_vandalized,
      has_flag: $has_flag,
      has_schedules: $has_schedules,
      has_sidewalk: $has_sidewalk,
      has_shelter: $has_shelter,
      has_bench: $has_bench,
      has_trash_can: $has_trash_can,
      is_illuminated: $is_illuminated,
      has_illuminated_path: $has_illuminated_path,
      has_visibility_from_within: $has_visibility_from_within,
      has_visibility_from_area: $has_visibility_from_area,
      is_visible_from_outside: $is_visible_from_outside
    };
    dispatch("save", newMeta);
  }
</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
  <div class="flex flex-col gap-1 p-2 overflow-visible w-full">
    <div class="form-control w-full max-w-xs">
      <label class="input-group">
        <span class="label-text w-24">Fonte</span>
        <input type="text" value={`${$stop.id} - ${$stop.source}`} class="input input-bordered w-full h-8" disabled />
      </label>
    </div>
    <div class="form-control w-full max-w-xs">
      <label class="input-group">
        <span class="label-text w-24">OSM</span>
        <input type="text" bind:value={$stop.osm_name} class="input input-bordered w-full h-8" disabled />
      </label>
    </div>
    <div class="form-control w-full max-w-xs">
      <label class="input-group">
        <span class="label-text w-24">Oficial</span>
        <input type="text" bind:value={official_name} class="input input-bordered w-full h-10" />
      </label>
    </div>
    <div class="form-control w-full max-w-xs">
      <label class="input-group">
        <span class="label-text w-24">Nome</span>
        <input type="text" bind:value={name} placeholder={name} class="input input-bordered w-full h-10" />
      </label>
    </div>
    <div class="form-control w-full max-w-xs">
      <label class="input-group">
        <span class="label-text w-24">Abrev.</span>
        <input type="text" bind:value={short_name} placeholder={short_name} class="input input-bordered w-full h-10" />
      </label>
    </div>
    <div class="form-control w-full max-w-xs">
      <label class="input-group">
        <span class="label-text w-24">Via</span>
        <input type="text" bind:value={street} placeholder={street} class="input input-bordered w-full h-10" />
      </label>
    </div>
    <div class="form-control w-full max-w-xs">
      <label class="input-group">
        <span class="label-text w-24">Porta</span>
        <input type="text" bind:value={door} placeholder={door} class="input input-bordered w-full h-10" />
      </label>
    </div>
  </div>
  <div>
    <label class="label"><span class="label-text">Carateristicas</span></label>
    <StopCheckbox text="Postaletes" description="O poste ou abrigo da paragem tem um postalete" state={has_flag} />
    <StopCheckbox text="Horários" description="A paragem tem horários atualizados" state={has_schedules} />
    <StopCheckbox
        text="Passeio"
        description="A paragem encontra-se fora da via de rodagem, berma ou de terreno"
        state={has_sidewalk} />
    <StopCheckbox
        text="Abrigo"
        description="A paragem encontra-se inserida num abrigo que resguarde da chuva e do vento"
        state={has_shelter} />
    <StopCheckbox
        text="Banco"
        description="A paragem tem bancos onde os passageiros se possam sentar"
        state={has_bench} />
    <StopCheckbox
        text="Caixote do lixo"
        description="A paragem dispõe de um caixote do lixo a menos de 20 metros"
        state={has_trash_can} />
  </div>
  <div>
    <label class="label"><span class="label-text">Acesso</span></label>
    <StopCheckbox
        text="Atravessamento de via"
        description="Existem infraestruturas ou sinalizações que permitam o atravessamento de via"
        state={has_crossing} />
    <StopCheckbox
        text="Acesso mobilidade reduzida"
        description="A paragem dispõe de acesso para pessoas com mobilidade reduzida"
        state={has_accessibility} />
    <label class="label"><span class="label-text">Defeitos</span></label>
    <StopCheckbox
        text="Estacionamento abusivo"
        description="Alvo recorrente de estacionamento abusivo impeditivo ao bom funcionamento"
        state={has_abusive_parking} />
    <StopCheckbox
        text="Informação obsoleta"
        description="A informação prestada na paragem (horários/postaletes) encontra-se obsoleta"
        state={has_outdated_info} />
    <StopCheckbox
        text="Danificada"
        description="A infraestrutura encontra-se danificada (ex. banco partido)"
        state={is_damaged} />
    <StopCheckbox
        text="Vandalizada"
        description="Existe uma quantidade substâncial de vandalismo (eg. graffitti)"
        state={is_vandalized} />
  </div>
  <div>
    <label class="label"><span class="label-text">Iluminação</span></label>
    <StopCheckbox
        text="Na paragem"
        description="A paragem encontra-se bem iluminada durante todas as 24 horas"
        state={is_illuminated} />
    <StopCheckbox
        text="No acesso"
        description="O acesso para a paragem encontra-se bem iluminado todas as 24 horas"
        state={has_illuminated_path} />
    <label class="label"><span class="label-text">Visibilidade</span></label>
    <StopCheckbox
        text="Da paragem para autocarro"
        description="Estando na paragem (+-5 metros) é possível ver autocarros atempadamente"
        state={has_visibility_from_area} />
    <StopCheckbox
        text="Do abrigo para autocarro"
        description="Estando sentado no abrigo é possível ver autocarros atempadamente"
        state={has_visibility_from_within} />
    <StopCheckbox
        text="Do autocarro para paragem"
        description="Enquanto motorista, é possível ver devidamente a paragem sem abrandar"
        state={is_visible_from_outside} />
  </div>
</div>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-2">
  <div class="form-control">
    <label class="label">
      <span class="label-text">Notas</span>
    </label>
    <textarea
        class="textarea textarea-bordered h-12"
        placeholder="Falta obter-se uma foto que mostre que a paragem se encontra frente a xyz"
        bind:value={notes}></textarea>
  </div>
  <div class="form-control">
    <label class="label">
      <span class="label-text">Fotos</span>
    </label>
    <span>Sem fotos</span>
  </div>
</div>
<div class="flex w-full justify-end">
  <button class="btn btn-primary w-20 float-right" on:click={save}>Guardar</button>
</div>