<h1>Avisos</h1>
<h2>Carris metropolitana</h2>
<div class="content">
  <ul class="issue-list">
    <li class="warning">
      <div class="text">
        <span class="title">Alteração temporária de horários</span>
        <span class="summary"
          >Durante o mês de Junho a operação da zona 4 decorre com os horários
          prévios à Carris Metropolitana.</span
        >
      </div>
      <div class="actions">
        <a href="#" class="link">Ler mais</a>
      </div>
    </li>
    <li class="error">
      <div class="text">
        <span class="title">Supressão de linhas</span>
        <span class="summary">
          As novas linhas na númeração 4xxx poderão estar a funcionar, bem como
          poderão não estar.<br />
          Vá de comboio se o tiver!
        </span>
      </div>
      <div class="actions">
        <a href="#" class="link">Ler mais</a>
      </div>
    </li>
  </ul>
</div>

<h2>Transtejo/Soflusa</h2>
<div class="content">
  <ul class="issue-list">
    <li class="warning">
      <div class="text">
        <span class="title">Barreiro – Terreiro do Paço</span>
        <span class="summary"
          >Perturbações de serviço entre 30 de Maio e 30 de Junho</span
        >
      </div>
      <div class="actions">
        <a href="#" class="link">Ler mais</a>
      </div>
    </li>
  </ul>
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

