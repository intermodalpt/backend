<script>
  import { onMount, tick } from "svelte";
  import Issues from "./lib/Issues.svelte";
  import RouteViewer from "./lib/Operators.svelte";
  import GlobalMap from "./lib/Map.svelte";
  import About from "./lib/About.svelte";
  import Home from "./lib/Home.svelte";
  import Simulator from "./lib/Simulator.svelte";
  import { initCache } from "./cache.js";
  import { writable } from "svelte/store";

  let page;
  let path;

  const pages = {
    issues: Issues,
    map: GlobalMap,
    routes: RouteViewer,
    simulator: Simulator,
    about: About,
  };

  let links = [
    ["", "Início"],
    ["map", "Rede"],
    ["routes", "Serviços"],
    ["issues", "Avisos"],
    ["about", "Sobre nós"],
  ];

  async function hashchange() {
    // the poor man's router!
    path = window.location.hash.slice(1).replace("/", "");
    page = pages[path];
    if (page === undefined) {
      page = Home;
    }
  }

  onMount(async () => {
    hashchange();
    await initCache();
  });

  let ddopen = writable(false);
  let ddClasses = writable("hidden");
  function toggleDD(delay = 0) {
    function doToggle() {
      $ddopen = !$ddopen;
      $ddClasses = "absolute scale-95 opacity-10";
      tick();
      if (!$ddopen) {
        setTimeout(() => ($ddClasses = "hidden"), 150);
      } else {
        setTimeout(
          () => ($ddClasses = $ddopen ? "absolute" : "absolute scale-95"),
          1
        );
      }
    }
    if (delay === 0) {
      doToggle();
    } else {
      setTimeout(() => {
        doToggle();
      }, delay);
    }
  }
</script>

<svelte:window on:hashchange={hashchange} />

<div class="w-[min(920px,100%)] mx-auto z-[3000] p-2 xl:pt-4">
  <div class="navbar bg-base-100 shadow-xl rounded-xl">
    <div class="navbar-start">
      <div class="">
        <span class="btn btn-ghost lg:hidden" on:click={() => toggleDD(0)}>
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
        </span>
        <ul
          class="menu menu-compact mt-3 p-2 dropdown-content shadow bg-base-100 rounded-box w-52 duration-200 transition-all {$ddClasses}"
          on:click={() => toggleDD(150)}
        >
          {#each links as k}
            <li>
              <a
                class={k[0] === path ? "bg-primary text-primary-content" : ""}
                href="#/{k[0]}">{k[1]}</a
              >
            </li>
          {/each}
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
        {#each links as k}
          <li>
            <a class={k[0] === path ? "bg-base-300" : ""} href="#/{k[0]}"
              >{k[1]}</a
            >
          </li>
        {/each}
      </ul>
    </div>
  </div>
</div>

<div class="w-[min(920px,100%)] mx-auto">
  <svelte:component this={page} />
</div>
