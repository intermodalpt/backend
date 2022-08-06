<script>
  import { onMount } from "svelte";
  import Issues from "./lib/Issues.svelte";
  import RouteViewer from "./lib/Operators.svelte";
  import GlobalMap from "./lib/Map.svelte";
  import Simulator from "./lib/Simulator.svelte";
  import { initCache } from "./cache.js";

  let page;

  async function hashchange() {
    // the poor man's router!
    const path = window.location.hash.slice(1);

    if (path === "") {
      page = "home";
    } else if (path.startsWith("/issues")) {
      page = "issues";
    } else if (path.startsWith("/map")) {
      page = "map";
    } else if (path.startsWith("/routes")) {
      page = "routes";
    } else if (path.startsWith("/simulator")) {
      page = "simulator";
    } else if (path.startsWith("/editor")) {
      page = "editor";
    } else if (path.startsWith("/about")) {
      page = "about";
    }
  }

  onMount(async () => {
    hashchange();
    await initCache();
  });
</script>

<svelte:window on:hashchange={hashchange} />

<div class="w-[min(920px,100%)] mx-auto z-[3000]">
  <div class="mt-1 xl:mt-4 navbar bg-base-100 shadow-xl rounded-xl">
    <div class="navbar-start">
      <div class="dropdown">
        <label tabindex="0" class="btn btn-ghost lg:hidden">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 6h16M4 12h8m-8 6h16"
            />
          </svg>
        </label>
        <ul
          tabindex="0"
          class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52"
        >
          <li><a href="#/map">Rede</a></li>
          <li><a href="#/routes">Serviços</a></li>
          <li><a href="#/issues">Avisos</a></li>
          <li><a href="#/about">Sobre nós</a></li>
        </ul>
      </div>
      <a
        class="btn btn-ghost normal-case text-lg md:text-3xl text-primary"
        href="/"
      >
        <img src="/logo-blue.o.svg" class="w-8" alt="Início" />
        <span style="color: #59befe">inter</span>
        <span style="color: #006d99">modal</span>
      </a>
    </div>
    <div class="navbar-end hidden lg:flex">
      <ul class="menu menu-horizontal p-0">
        <li><a href="#/map">Rede</a></li>
        <li><a href="#/routes">Serviços</a></li>
        <li><a href="#/issues">Avisos</a></li>
        <li><a href="#/about">Sobre nós</a></li>
      </ul>
    </div>
  </div>
</div>

<div id="content">
  {#if page === "home"}
    <div class="w-[min(920px,100%)] pt-8 mx-auto">
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">O que é o Intermodalis?</h2>
          <p>
            O Intermodal é um agregador e amplificador dos dados de mobilidade
            da area metropolitana de Lisboa. Ambiciona preencher o infeliz vazio
            de funcionalidade, acessibilidade e conveniência que as actuais
            soluções proporcionam. O projeto é desenvolvido por uma comunidade e
            não tem qualquer afiliação a empresas de transportes.
          </p>
          <p>
            ⚠️⚠️⚠️ Esta plataforma é um <b>trabalho em progresso</b> empurrado
            <b>só e apenas por voluntários</b>. Esperamos que goste do que vê
            mas mentalize-se que não está pronto. Não confie demasiado no que
            encontrar e não desate à pancada em motoristas porque o Intermodal
            dizia que algo acontecia e não aconteceu.⚠️⚠️⚠️
          </p>
        </div>
      </div>
    </div>
  {:else if page === "about"}
    <div class="pt-8 w-[min(920px,100%)] mx-auto mb-10">
      <div class="flex flex-col w-full gap-4">
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Quem somos?</h2>
            <p>
              Somos meros voluntários preocupados com o urbanismo das cidades.
              Pessoas não diferentes de si. Acreditamos que a cidade é das
              pessoas e para as pessoas. Queremos espaços agradáveis, queremos
              um futuro sustentável e queremos garantir que todo o cidadão,
              tenha as limitações que tenha, consegue viver em dignidade sem se
              sentir limitado.
            </p>
            <p>
              Acreditamos que os transportes públicos são o melhor meio de
              transporte. Começamos esta nossa iniciativa com vigorando tal
              afirmação enquanto premissa.
            </p>
            <p>
              Com desanimo nos juntamos aos demais na observação que existem
              inúmeras lacunas na atual oferta e operação, e com animo criamos o
              Intermodalis, plataforma que que direciona à mitigação e promoção
              de resolução de problemas na nossa rede de transportes públicos.
            </p>
            <p>
              Não estamos afiliados mas existe uma certa intimidade entre nós e
              a comunidade do projeto
              <a class="link link-primary" href="https://lisboaparapessoas.pt/"
                >Lisboa Para Pessoas</a
              >.
            </p>
          </div>
        </div>
        <div
          class="carousel carousel-center space-x-4 rounded-box  drop-shadow-xl"
          style="overflow: unset"
        >
          <div class="carousel-item">
            <div
              class="card w-72 bg-base-100 "
              style="animation: 1.5s cubic-bezier(.45,.05,.55,.95) 0s infinite alternate none running rotation;"
            >
              <div class="absolute h-20 bg-blue-500 w-full" />
              <div class="card-body items-center text-center">
                <div class="avatar">
                  <div
                    class="w-24 rounded-full ring ring-base-300 ring-offset-base-100 ring-offset-2"
                  >
                    <img src="https://placeimg.com/192/192/people" />
                  </div>
                </div>
                <h2 class="card-title">Tiago Teles</h2>
                <span class="text-sm">Pseudo-programador-designer</span>
                <p class="text-left">
                  Se comprasse autocarros esquecia-me de motoristas, mas seriam
                  lindos autocarros.
                </p>
              </div>
            </div>
          </div>
          <div class="carousel-item">
            <div class="card w-72 bg-base-100 ">
              <div class="absolute h-20 bg-green-500 w-full" />
              <div class="card-body items-left">
                <div class="avatar">
                  <div
                    class="w-24 rounded-full ring ring-base-300 ring-offset-base-100 ring-offset-2"
                  >
                    <img src="https://placeimg.com/192/192/people" />
                  </div>
                </div>
                <div>
                  <span class="card-title">Cláudio Pereira</span>
                  <span class="text-sm">Programador</span>
                </div>
                <p class="text-left">
                  Iniciei este projeto e tento impedir que o Tiago o mate por
                  acidente.
                </p>
              </div>
            </div>
          </div>
        </div>
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">Menções Honrosas</h2>
            <h3 class="text-lg">Diogo Baptista - Cartografia</h3>
            <h3 class="text-lg">Nelson Vassalo - Consultância artistica</h3>
          </div>
        </div>
        <div class="card bg-base-100 shadow-xl">
          <div class="card-body">
            <h2 class="card-title">FAQ</h2>
            <h3 class="text-lg">Representam a Carris Metropolitana?</h3>
            <p>
              Não, não temos qualquer afiliação à marca <b
                >Carris Metropolitana</b
              >
              pertencente à empresa TML que a detem. A nossa plafatorma é não
              lucrativa, desenvolvida por voluntários sem qualquer vínculo à
              TML.<br />
              A página oficial da Carris Metropolitana encontra-se em
              <a
                class="link link-primary"
                href="https://www.carrismetropolitana.pt/"
                >carrismetropolitana.pt</a
              >.
            </p>
            <h3 class="text-lg">
              Porquê é que os sites oficiais são assim e o vosso é assado?
            </h3>
            <p>
              Porque temos de nos cingir a trabalhar neste projeto após os
              nossos empregos e com um orçamento nulo. Para contraste, a página
              da Carris Metropolitana foi produto de um
              <a
                class="link link-primary"
                href="https://www.base.gov.pt/Base4/pt/detalhe/?type=contratos&id=7725440"
              >
                contrato de 18,900€</a
              >
              para com a empresa Flatstudio Lda.<br />
              É natural que esteja muito melhor que o nosso humilde projeto.
            </p>
            <h3 class="text-lg">
              O meu transporte não apareceu/atrasou-se/fez um trajeto
              errado/comeu-me o queijo do frigorifico.
            </h3>
            <p>
              As novas operações vem seguidas de uma fase transitória que
              naturalmente é dada a complexidades. Algumas destas complexidades
              advem de má gestão, outras tantas são naturais. Perfeito é inimigo
              do bom. Se tem uma queixa fundamentada utilize o
              <a
                class="link link-primary"
                href="https://www.livroreclamacoes.pt/">livro de reclamações</a
              >. Não reclame com motoristas ou bilheteiras, não reclame nas
              redes sociais, não utilize plataformas de terceiros, utilize
              <b>só e apenas o livro de reclamações</b>. Procure produzir
              <b>criticas construtivas</b> e não indignação em massa.
            </p>
            <p>
              Queixas no livro de reclamações imputam dever legal à entidade de
              produzir uma justificação. Volumes elevados de queixas levam o
              regulador a tomar medidas.
            </p>
            <h3 class="text-lg">Como é que financiam o projeto?</h3>
            <p>
              Com o nosso próprio dinheiro, pro bono. Não existe qualquer
              financiamento externo, seja por parte de empresas de transportes,
              do governo ou de qualquer outra entidade privada.
            </p>
            <h3 class="text-lg">Posso ajudar?</h3>
            <p>
              Estamos a estudar formas futuras de aceitar contribuições
              externas, como ajuda à verificação dos dados que dispomos. De
              momento a melhor forma de ajudar é divulgar. Ajude-nos retirando
              automóveis da rua.
            </p>
          </div>
        </div>
      </div>
    </div>
  {:else if page === "issues"}
    <div class="pt-8 w-[min(920px,100%)] mx-auto">
      <Issues />
    </div>
  {:else if page === "routes"}
    <div class="pt-8 w-[min(920px,100%)] mx-auto">
      <RouteViewer />
    </div>
  {:else if page === "map"}
    <div class="content-wrapper">
      <GlobalMap />
    </div>
  {:else if page === "simulator"}
    <div class="content-wrapper">
      <Simulator />
    </div>
  {/if}
</div>

<!-- <div id="grass"></div> -->
<!-- <div id="footer"> -->
<!--     <div class="content-wrapper"> -->
<!--         <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/"> -->
<!--             <img alt="Creative Commons Licence" -->
<!--                  style="border-width:0" -->
<!--                  src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png"/></a>. -->
<!--         <ul> -->
<!--             <li>Sobre nós</li> -->
<!--             <li>Contactos</li> -->
<!--         </ul> -->
<!--     </div> -->

<!-- </div> -->
<style>
  :root {
    font-family: "Roboto", sans-serif;
    height: 100%;
    background: #fafafa;
  }
</style>
