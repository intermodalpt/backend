<script>
  import Box from "./components/Box.svelte";
  import Operator from "./Operator.svelte";

  let operator = null;

  function choose(e) {
    this.classList.add("selected");
    this.parentElement.classList.add("hide-not-selected");
    operator = this.classList[1];
  }
</script>

<div class="card bg-base-100 shadow-xl mx-2">
  <div class="card-body">
    {#if operator}
      <Operator bind:operator>
        <div class="company {operator}" on:mouseup={choose} />
      </Operator>
    {:else}
      <div id="route-list">
        <div class="company carris unavailable" />
        <div class="company cmet" on:mouseup={choose} />
        <div class="company tcb unavailable" />
        <div class="company mobicascais unavailable" />
        <div class="company ttsl unavailable" />
        <div class="company cp unavailable" />
        <div class="company fertagus unavailable" />
        <div class="company ml unavailable" />
        <div class="company mts unavailable" />
      </div>
    {/if}
  </div>
</div>

<style>
  #route-list {
    display: grid;
    gap: 20px;
    grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr));
  }

  .tcb {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/tcb.svg"),
      bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/tcb.svg");
    background-size: 20%, 60%;
  }

  .cmet {
    background: top 0.7em left 0.7em no-repeat url("/src/assets/logos/cmet.svg"),
      bottom 0.7em right 0.7em no-repeat url("/src/assets/veiculos/cmet.svg");
    background-size: 35%, 60%;
    /*background-color: #aaccaa !important;*/
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
