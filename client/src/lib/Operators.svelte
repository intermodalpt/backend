<script>
  import Operator from "./Operator.svelte";

  let operator = null;
  const operators = [
    "carris",
    "cmet",
    "tcb",
    "mobicascais",
    "ttsl",
    "cp",
    "fertagus",
    "ml",
    "mts",
  ];

  function choose(e) {
    this.classList.add("selected");
    this.parentElement.classList.add("hide-not-selected");
    operator = this.classList[1];
  }
</script>

{#if operator}
  <div class="card bg-base-100 shadow-xl mx-2">
    <div class="card-body">
      <Operator bind:operator>
        <div class="company {operator}" on:mouseup={choose} />
      </Operator>
    </div>
  </div>
{:else}
  <div class="grid gap-3 grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] mx-2">
    {#each operators as op}
      <div
        class="card bg-base-100 shadow-xl cursor-pointer"
        onmouseup={() => op === "cmet" && choose}
      >
        <div
          class="rounded-xl aspect-[3.6] {op !== 'cmet' && 'blur-sm'} {op}"
        />
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

  .mobicascais {
    background: top 0.7em left 0.7em no-repeat
        url("/src/assets/logos/mobic.svg"),
      bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/mobic.svg");
    background-size: 35%, 60%;
  }

  .carris {
    background: top 0.7em left 0.7em no-repeat
        url("/src/assets/logos/carris.svg"),
      bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/carris.svg");
    background-size: 45%, 60%;
  }

  .ml {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/ml.svg");
    background-size: 15%;
  }

  .mts {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/mts.svg");
    background-size: 22%;
  }

  .cp {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/cp.svg"),
      bottom 0 right 0.7em no-repeat url("/src/assets/veiculos/cp.svg");
    background-size: 75%, 55%;
  }

  .fertagus {
    background: top 0.7em left 0.7em no-repeat
        url("/src/assets/logos/fertagus.svg"),
      bottom 0 right 0.7em no-repeat url("/src/assets/veiculos/fertagus.svg");
    background-size: 40%, 55%;
  }

  .ttsl {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/ttsl.svg"),
      bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/ttsl.svg");
    background-size: 30%, 70%;
  }

  .company {
    border-radius: 12px;
    background-color: #efefef;
    aspect-ratio: 3.6;
  }

  .company.unavailable {
    filter: blur(5px) grayscale(50%);
  }
</style>
