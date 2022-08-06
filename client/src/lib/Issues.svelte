<script>
  import Box from "./components/Box.svelte";
</script>

<!--This still is a mockup-->

<div class="flex flex-col gap-2">
  <div class="card card-compact bg-base-100 shadow-xl mx-2">
    <div class="collapse collapse-arrow rounded-box">
      <input type="checkbox" class="cursor-pointer" />
      <div
        class="collapse-title text-xl font-medium flex flex-row justify-between"
      >
        Carris Metropolitana
        <div>
          <span class="badge badge-error">1</span>
          <span class="badge badge-info">1</span>
        </div>
      </div>
      <div class="collapse-content !p-0">
        <div class="p-1 text-sm flex flex-col gap-2">
          <div class="flex flex-col border-info border-2 p-2 rounded-xl">
            <div class="flex flex-row font-bold items-center text-info-content">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                class="stroke-current flex-shrink-0 w-6 h-6"
                ><path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                /></svg
              >
              Alteração temporária de horários
            </div>
            <div>
              Durante o mês de Junho a operação da zona 4 decorre com os
              horários prévios à Carris Metropolitana.
            </div>
          </div>

          <div class="flex flex-col border-error border-2 p-2 rounded-xl">
            <div
              class="flex flex-row font-bold items-center text-error-content"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="stroke-current flex-shrink-0 h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                ><path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                /></svg
              >
              Supressão de linhas
            </div>
            <div>
              As novas linhas na númeração 4xxx poderão estar a funcionar, bem
              como poderão não estar.<br />
              Vá de comboio se o tiver!
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="card card-compact bg-base-100 shadow-xl mx-2">
    <div class="collapse collapse-arrow rounded-box">
      <input type="checkbox" class="cursor-pointer" />
      <div
        class="collapse-title text-xl font-medium flex flex-row justify-between"
      >
        Transtejo/Softlusa
        <div>
          <div class="badge badge-warning">1</div>
        </div>
      </div>
      <div class="collapse-content !p-0">
        <div class="p-1 text-sm flex flex-col gap-2">
          <div class="flex flex-col border-warning border-2 p-2 rounded-xl">
            <div
              class="flex flex-row font-bold items-center text-warning-content"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="stroke-current flex-shrink-0 h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                ><path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                /></svg
              >
              Barreiro - Terreiro do Paço
            </div>
            <div>Perturbações de serviço entre 30 de Maio e 30 de Junho</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .issue-list {
    display: flex;
    flex-direction: column;
    list-style: none;
    padding: 0;
  }

  .issue-list > li {
    border-bottom: 2px solid #cfcfcf;
  }

  .issue-list > li:first-child {
    border-top-left-radius: 12px;
  }

  .issue-list > li:last-child {
    border-bottom-left-radius: 12px;
    border-bottom: none;
  }

  .issue-list > li.error {
    border-left: 5px solid #bf616a;
  }

  .issue-list > li.warning {
    border-left: 5px solid #ebcb8b;
  }

  .issue-list > li {
    display: flex;
    flex-direction: row;
  }

  .issue-list .text {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    padding: 10px;
  }

  .issue-list .title {
    font-size: 1.1em;
    font-weight: bold;
  }

  .issue-list .actions {
    display: flex;
  }

  .issue-list .actions > .link {
    background: red;
  }
</style>
