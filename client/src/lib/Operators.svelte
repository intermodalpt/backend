<script>
  import Operator from "./Operator.svelte";
  import {selectedOperatorId, selectedRouteId} from "../context.js";

  const operatorIdTagPairs = [
    {id: 1, name: "Carris Metropolitana", tag: "cmet"},
    {id: 3, name: "Carris", tag: "carris"},
    {id: 2, name: "Transportes Colectivos do Barreiro", tag: "tcb"},
    {id: 4, name: "MobiCascais", tag: "mobic"},
    {id: 5, name: "Comboios de Portugal", tag: "cp"},
    {id: 6, name: "Fertagus", tag: "fert"},
    {id: 7, name: "Metro Transportes do Sul", tag: "mts"},
    {id: 8, name: "Metro de Lisboa", tag: "ml"},
    {id: 9, name: "Transtejo e Soflusa", tag: "ttsl"}
  ];

</script>

{#if $selectedOperatorId}
  <div class="card bg-base-100 shadow-xl mx-2 z-[5000]">
    <div class="card-body -mt-4">
      <div class="flex gap-2">
        <a on:mouseup={() => $selectedOperatorId = undefined} class="btn btn-ghost btn-xs text-primary">Outras operadoras</a>
        {#if $selectedRouteId}
          <a on:mouseup={() => $selectedRouteId = undefined} class="btn btn-ghost btn-xs text-primary">Outras rotas</a>
        {/if}
      </div>
      <div class="company compact {operatorIdTagPairs.find((op) => {return op.id === $selectedOperatorId})?.tag} bg-base-200"></div>
      <Operator />
    </div>
  </div>
{:else}
  <div class="grid gap-3 grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] mx-2">
    {#each operatorIdTagPairs as op}
      <div class="card bg-base-100 shadow-xl cursor-pointer z-[5000]" on:mouseup={() => {$selectedOperatorId = op.id}}>
        <div class="rounded-xl aspect-[3.6] {op.tag}"></div>
      </div>
    {/each}
  </div>
{/if}

<style>
  .tcb {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/tcb.svg"),
    bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/tcb.svg");
    background-size: 20%, 60%;
  }

  .cmet {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/cmet.svg"),
    bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/cmet.svg");
    background-size: 35%, 60%;
  }

  .cmet.compact {
    background-size: 40%, 50%;
  }

  .mobic {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/mobic.svg"),
    bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/mobic.svg");
    background-size: 35%, 60%;
  }

  .cmet.compact {
    background-size: 40%, 50%;
  }

  .carris {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/carris.svg"),
    bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/carris.svg");
    background-size: 45%, 60%;
  }

  .carris.compact {
    background-size: 45%, 50%;
  }

  .ml {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/ml.svg");
    background-size: 15%;
  }

  .mts {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/mts.svg");
    background-size: 22%;
  }

  .mts.compact {
    background-size: 28%;
  }

  .cp {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/cp.svg"),
    bottom 0 right 0.7em no-repeat url("/src/assets/veiculos/cp.svg");
    background-size: 75%, 55%;
  }

  .cp.compact {
    background-size: 75%, 40%;
  }

  .fert {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/fertagus.svg"),
    bottom 0 right 0.7em no-repeat url("/src/assets/veiculos/fertagus.svg");
    background-size: 40%, 55%;
  }

  .fert.compact {
    background-size: 40%, 50%;
  }

  .ttsl {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/ttsl.svg"),
    bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/ttsl.svg");
    background-size: 30%, 70%;
  }

  .company {
    border-radius: 12px;
    background-color: hsl(var(--b2));
    aspect-ratio: 3.6;
  }

  .company.compact {
    border-radius: 12px;
    background-color: hsl(var(--b2));
    aspect-ratio: 5;
  }
</style>
