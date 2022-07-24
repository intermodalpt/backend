<script>
    import {api_server} from "../settings.js";

    export let operator;

    let routes;

    fetch(`${api_server}/api/routes`)
        .then(r => r.json())
        .then(data => {
            routes = data;
            window.scrollTo(0, 0);
        });
</script>


<div>
    <span on:mouseup={_ => operator = null}>Go bak</span>
    <slot></slot>
</div>

<h2>Linhas</h2>
{#if routes}
    <div class="route-list">
    {#each routes as route}
        <div class="code">
            <span class="line-number">{route.flag}</span>
            <span>{route.flag}</span>
        </div>
    {/each}
    </div>
{:else}
    <p>A carregar...</p>
{/if}


<style>
    .code {

    }
    .line-number {
        background-color: red;
        padding: 0.2em 10px;
        border-radius: 1em;
        font-weight: bold;
        display: inline-block;
    }
</style>