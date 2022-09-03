<script>
  import {mode} from "../context.js";

  function setMode(newMode) {
    $mode = newMode;
    if (newMode === null){
      localStorage.removeItem("mode");
    } else {
      localStorage.setItem("mode", newMode);
    }
  }
</script>

<div class="flex flex-col gap-4">
  <div class="alert alert-error shadow-lg">
    <div>
    <span>
        Esta plataforma é um <b>trabalho em progresso</b>. Não confie demasiado no que encontrar e
        não assedie motoristas porque o Intermodal dizia que algo acontecia e não aconteceu.
    </span>
    </div>
  </div>
  <div class="card bg-base-100 shadow-xl mx-2">
    <div class="card-body">
      <h2 class="card-title">O que é?</h2>
      <p>
        O Intermodal é um agregador de dados de mobilidade da área metropolitana de Lisboa.<br>
        O projeto é desenvolvido por uma comunidade sem afiliação a empresas de transportes.
      </p>
    </div>
  </div>
  <div class="card lg:card-side bg-base-100 mx-2 shadow-xl">
    <div class="card-body">
      {#if $mode === null }
        <h2 class="card-title">Sê bem vindo</h2>
        <p>
          Temos na ementa uma navegação mais <b>acessível</b> de uso simplificado.<br>
          Alternativamente pode optar por uma navegação <b>avançada</b>.
        </p>
        <div class="card-actions justify-center items-center xs:flex xs:flex-col">
          <button class="btn btn-primary" on:mouseup={() => setMode("simple")}>Simples</button>
          <span>ou</span>
          <button class="btn btn-primary" on:mouseup={() => setMode("advanced")}>Avançada</button>
        </div>
        <div class="card-actions justify-center text-xs">
          (Se não indicar será assumida navegação acessível. Poderá alterar mais tarde.)
        </div>
      {:else }
        <div>
          <span>Navegando em modo <b>{#if $mode === "advanced" }Avançado{:else }Simples{/if}</b></span>
          <button class="btn btn-primary" on:mouseup={() => setMode(null)}>Alterar</button>
        </div>
      {/if}
    </div>
    <figure><img src="/mascot.svg" class="w-24" class:w-64={$mode === null} alt="Mascote"></figure>
  </div>
  <div class="card sm:card-side lg:card-side bg-base-100 mx-2 shadow-xl">
    <figure><img src="/icons/tram.svg" class="w-36 p-4" alt="Elétrico"></figure>
    <div class="card-body">
      <div class="stats">
        <div class="stat">
          <div class="stat-desc">Temos um total de</div>
          <div class="stat-value">255</div>
          <div class="stat-title">Rotas</div>
        </div>
      </div>
      <div class="stats">
        <div class="stat">
          <div class="stat-desc">E as suas</div>
          <div class="stat-value">593</div>
          <div class="stat-title">Variantes</div>
        </div>
      </div>
      <div class="stats">
        <div class="stat">
          <div class="stat-desc">Totalizando</div>
          <div class="stat-value">10.863</div>
          <div class="stat-title">Partidas</div>
        </div>
      </div>
    </div>
    <figure><img src="/icons/navegante.svg" class="w-52 p-4" alt="Passe"></figure>
  </div>
  <div class="card lg:card-side bg-base-100 mx-2 shadow-xl">
    <div class="card lg:card-side">
      <div class="card-body">
        <h2 class="card-title">Pelos demais</h2>
        <p>
          Com esta iniciativa estamos a criar um registo da acessibilidade na via pública.<br>
          Queremos ajuda-lo não só a sim bem como os mais vulneráveis por entre nós: idosos e detentores de
          incapacidades.
        </p>
        <div class="card lg:card-side">
          <figure><img src="/icons/stop.svg" class="w-32 p-4" alt="Paragem"></figure>
          <div class="card-body">
            <div class="stats">
              <div class="stat">
                <div class="stat-desc">Mantemos registo de</div>
                <div class="stat-value">7444</div>
                <div class="stat-title">Paragens</div>
              </div>
              <div class="stat">
                <div class="stat-desc">E um catalogo de</div>
                <div class="stat-value">1511</div>
                <div class="stat-title">Imagens</div>
              </div>
            </div>
            <div>
              Muitos destes dados são doados a iniciativas como o OpenStreetMap e todos eles são abertos à comunidade.
            </div>
            <div class="card-actions justify-end">
              <figure><img src="/icons/child.svg" class="w-24" alt="Repositório"></figure>
              <figure><img src="/icons/elderly.svg" class="w-24" alt="Repositório"></figure>
              <figure><img src="/icons/deaf.svg" class="w-24" alt="Repositório"></figure>
              <figure><img src="/icons/blind.svg" class="w-24" alt="Repositório"></figure>
              <figure><img src="/icons/wheelchair.svg" class="w-24" alt="Repositório"></figure>
              <figure><img src="/icons/guidedog.svg" class="w-24" alt="Repositório"></figure>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="card lg:card-side bg-base-100 mx-2 shadow-xl">
    <div class="card lg:card-side">
      <div class="card-body">
        <h2 class="card-title">Descobre mais</h2>
        <div class="flex">
          <a href="#"><img src="/icons/mastodon.svg" class="w-24" alt="Mastodon"></a>
          <a href="https://gitlab.com/lisboaparapessoas/intermodalis">
            <img src="/icons/gitlab.svg" class="w-24" alt="Repositório">
          </a>
        </div>
        <h2 class="card-title">Projetos irmãos</h2>
        <div class="flex">
          <a href="https://www.openstreetmap.org/about">
            <img src="/icons/osm.svg" class="w-24" alt="Iniciativa OpenStreetMap">
          </a>
          <a href="https://lisboaparapessoas.pt/"><img src="/icons/lpp.webp" class="w-24" alt="Lisboa para Pessoas"></a>
        </div>
        <span class="text-xs">(Estes projetos são mantidos de forma externa ao Intermodal, sem afiliação)</span>
      </div>
    </div>
  </div>
</div>