<script>
  export let isSelectableZoomLevel;
</script>

{#if $isSelectableZoomLevel}
  <div class="p-4 flex flex-col">
    <span class="text-xl text-center">Seleccione a paragem desejada</span>
    <div class="flex justify-around ml-2 sm:gap-4">
      <label for="select-stop-instruction" class="text-primary btn-ghost cursor-pointer btn-outline rounded-l-lg sm:rounded-lg modal-button flex flex-col grow items-center shrink-0">
        <img src="/gestures/pin-select.svg" alt="Seleccionar paragem" class="h-24" />
        <span class="text-lg">Escolher paragem</span>
      </label>
      <hr />
      <label for="select-area-instruction" class="text-primary btn-ghost cursor-pointer btn-outline rounded-r-lg sm:rounded-lg  modal-button flex flex-col grow items-center shrink-0">
        <img src="/gestures/lasso-select.svg" alt="Escolher zona" class="h-24" />
        <span class="text-lg">Escolher região</span>
      </label>
    </div>
  </div>
{:else }
  <div class="p-4 flex flex-col">
    <span class="text-xl text-center">Seleccione a zona desejada</span>
    <div class="flex justify-around ml-2 sm:gap-4">
      <label for="enter-administrative-instruction" class="text-primary btn-ghost cursor-pointer btn-outline rounded-l-lg sm:rounded-lg  modal-button flex flex-col grow items-center shrink-0">
        <img src="/gestures/region-select.svg" alt="Seleccionar região" class="h-24" />
        <span class="text-lg">Município</span>
      </label>
      <hr />
      <label for="zoom-instruction" class="text-primary btn-ghost cursor-pointer btn-outline rounded-r-lg sm:rounded-lg  modal-button flex flex-col grow items-center shrink-0">
        <img src="/gestures/zoom.svg" alt="Zoom" class="h-24" />
        <span class="text-lg">Aproximar</span>
      </label>
    </div>
  </div>
{/if}

<input type="checkbox" id="enter-administrative-instruction" class="modal-toggle" />
<div class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Municipio</h3>
    <p class="py-4">Pode focar o mapa numa região administrativa com um toque rápido na mesma.</p>
    <div class="modal-action">
      <label for="enter-administrative-instruction" class="btn">Entendi!</label>
    </div>
  </div>
</div>
<input type="checkbox" id="zoom-instruction" class="modal-toggle" />
<div class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Apróximar</h3>
    <p class="py-4">Para apróximar numa zona arbitrária (em dispositivos móveis) deverá de colocar 2 dedos na zona desejada e afasta-los ou (em computadores) de utilizar a roda do rato.</p>
    <div class="modal-action">
      <label for="zoom-instruction" class="btn">Entendi!</label>
    </div>
  </div>
</div>
<input type="checkbox" id="select-stop-instruction" class="modal-toggle" />
<div class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Escolher paragem</h3>
    <p class="py-4">Para escolher uma paragem deverá de encontrar o local desejado. Tocando-lhe, a paragem fica seleccionada e serão mostrados os serviços efetuados.</p>
    <div class="modal-action">
      <label for="select-stop-instruction" class="btn">Entendi!</label>
    </div>
  </div>
</div>
<input type="checkbox" id="select-area-instruction" class="modal-toggle" />
<div class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Escolher região</h3>
    <p class="py-4">Se escolher uma região, verá todos os serviços que param na mesma. Para escolher, seleccione a ferramenta laço na lateral esquerda e desenhe a região pretendida.</p>
    <div class="modal-action">
      <label for="select-area-instruction" class="btn">Entendi!</label>
    </div>
  </div>
</div>