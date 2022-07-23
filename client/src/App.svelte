<script>
    import {onMount} from 'svelte';
    import Issues from './lib/Issues.svelte'
    import RouteViewer from './lib/Operators.svelte'
    import GlobalMap from './lib/Map.svelte'
    import Simulator from './lib/Simulator.svelte'
    import Box from './lib/components/Box.svelte'
    import logo from './assets/logo.svg'
    import {initCache, routes, stops} from "./cache.js";
    import {api_server} from "./settings.js";


    let page;

    async function hashchange() {
        // the poor man's router!
        const path = window.location.hash.slice(1);

        if (path === '') {
            page = "home";
        } else if (path.startsWith('/issues')) {
            page = "issues";
        } else if (path.startsWith('/map')) {
            page = "map";
        } else if (path.startsWith('/routes')) {
            page = "routes";
        } else if (path.startsWith('/simulator')) {
            page = "simulator";
        } else if (path.startsWith('/editor')) {
            page = "editor";
        }
    }

    onMount(async () => {
        hashchange();
        await initCache();
    });
</script>

<svelte:window on:hashchange={hashchange}/>

<div id="header">
    <div class="content-wrapper">
        <a href="#" id="logo">
            <img src={logo} alt="logotipo intermodalis">
            <span>Intermodalis</span>
        </a>
        <ul id="menu">
            <li><a href="#/map">Mapa</a></li>
            <li><a href="#/routes">Operadoras</a></li>
            <li><a href="#/issues">Avisos</a></li>
            <!--            <li><a href="#/simulator">Simulador</a></li>-->
        </ul>
    </div>
</div>

<div id="content">
    {#if page === "home"}
        <div class="content-wrapper">
            <h1>O que é o Intermodalis?</h1>
            <Box padded=true>
                <p>
                    O Intermodalis (<i>intermodális</i> ou <i>inter-muda-lis</i>, pronuncia como quiseres) é um
                    agregador e
                    amplificador dos dados de mobilidade da area metropolitana de Lisboa. Ambiciona preencher o
                    infeliz vazio de funcionalidade, acessibilidade e conveniência que as actuais soluções proporcionam.
                    O projeto é desenvolvido por uma comunidade e não tem qualquer afiliação a empresas de transportes.
                </p>
            </Box>
        </div>
    {:else if page === "issues"}
        <div class="content-wrapper">
            <Issues/>
        </div>
    {:else if page === "routes"}
        <div class="content-wrapper">
            <RouteViewer/>
        </div>
    {:else if page === "map"}
        <div class="content-wrapper">
            <GlobalMap/>
        </div>
    {:else if page === "simulator"}
        <div class="content-wrapper">
            <Simulator/>
        </div>
    {/if}
</div>

<div id="grass"></div>
<div id="footer">
    <div class="content-wrapper">
        <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">
            <img alt="Creative Commons Licence"
                 style="border-width:0"
                 src="https://i.creativecommons.org/l/by-sa/4.0/88x31.png"/></a>.
        <ul>
            <li>Sobre nós</li>
            <li>Contactos</li>
        </ul>
    </div>
</div>

<style>
    :root {
        font-family: 'Roboto', sans-serif;
        height: 100%;
        background: #fafafa;
    }

    #header {
        display: flex;
        flex-direction: row;
        justify-content: center;
        justify-items: center;
        /*background: #ebebeb;*/
        background: #ebf0f2;
        border-bottom: 1px solid #cfcfcf;
        padding: 10px;
    }

    #header a {
        color: #4c566a;
    }

    #header ul, #footer ul {
        margin: 0;
        list-style: none;
        display: flex;
    }


    #header li {
        font-size: 1.5em;
        font-weight: bold;
        padding: 10px;
    }

    #header .content-wrapper {
        align-items: center;
    }

    #logo {
        display: flex;
        align-content: baseline;
    }

    #logo img {
        width: 64px;
    }

    #logo span {
        margin-left: 20px;
        font-size: 3em;
    }

    #content {
        flex-grow: 1;
        display: flex;
        flex-direction: row;
        justify-content: center;
        justify-items: center;
        margin-top: 20px;
        margin-bottom: 40px;
    }

    #header .content-wrapper, #footer .content-wrapper {
        display: flex;
        justify-content: space-between;
    }

    #footer {
        background: #a3be8c;
        padding: 10px;
        display: flex;
        flex-direction: row;
        justify-content: center;
        justify-items: center;
    }

    #footer ul > li {
        font-size: 1.2em;
        font-weight: bold;
        padding: 10px;
        color: rgba(0, 0, 0, 0.5);
    }

    #grass {
        background: url('/src/assets/grass.png') top center repeat-x;
        height: 39px;
        margin-top: -33px;
        width: 100%;
    }
</style>
