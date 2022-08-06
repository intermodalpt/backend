<script>
  import { onMount } from "svelte";
  import Issues from "./lib/Issues.svelte";
  import RouteViewer from "./lib/Operators.svelte";
  import GlobalMap from "./lib/Map.svelte";
  import About from "./lib/About.svelte";
  import Home from "./lib/Home.svelte";
  import Simulator from "./lib/Simulator.svelte";
  import { initCache } from "./cache.js";

  let page;

  const pages = {
    issues: Issues,
    map: GlobalMap,
    routes: RouteViewer,
    simulator: Simulator,
    about: About,
  };

  async function hashchange() {
    // the poor man's router!
    const path = window.location.hash.slice(1).replace("/", "");

    page = pages[path];
    if (page === undefined) {
      page = Home;
    }
  }

  onMount(async () => {
    hashchange();
    await initCache();
  });
</script>

<svelte:window on:hashchange={hashchange} />

<div class="w-[min(920px,100%)] mx-auto z-[3000] p-2 xl:pt-4">
  <div class="navbar bg-base-100 shadow-xl rounded-xl">
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
          <li><a href="#/">Início</a></li>
          <li><a href="#/map">Rede</a></li>
          <li><a href="#/routes">Serviços</a></li>
          <li><a href="#/issues">Avisos</a></li>
          <li><a href="#/about">Sobre nós</a></li>
        </ul>
      </div>
      <a
        class="btn btn-ghost normal-case text-xl md:text-3xl text-primary"
        href="/"
      >
        <img
          src="/logo-blue.o.svg"
          class="w-8 pr-1"
          alt="Início,Logo Intermodalis"
        />
        <span class="text-[#59befe]">inter</span>
        <span class="text-[#006d99]">modal</span>
      </a>
    </div>
    <div class="navbar-end hidden lg:flex">
      <ul class="menu menu-horizontal p-0">
        <li><a href="#/">Início</a></li>
        <li><a href="#/map">Rede</a></li>
        <li><a href="#/routes">Serviços</a></li>
        <li><a href="#/issues">Avisos</a></li>
        <li><a href="#/about">Sobre nós</a></li>
      </ul>
    </div>
  </div>
</div>

<div class="w-[min(920px,100%)] mx-auto">
  <svelte:component this={page} />
</div>
