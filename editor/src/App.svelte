<script>
  import Editor from "./lib/editor/Editor.svelte";
  import {initCache} from "./cache.js";
  import {token, api_server} from "./settings.js";
  import {onMount} from "svelte";

  let loading = true;

  onMount(async () => {
    let ls_token = localStorage.getItem("editor-token");
    if (ls_token != null && await checkToken(ls_token)) {
      $token = ls_token;

      await initCache(ls_token);
    }

    loading = false;
  });

  async function checkToken(token) {
    let res = await fetch(`${api_server}/auth/check`, {
      method: 'POST',
      headers: {
        authorization: `Bearer ${token}`
      }
    });
    return res.ok;
  }

  async function saveToken() {
    let token_val = document.getElementById("auth-token").value;
    if (await checkToken(token_val)) {
      localStorage.setItem("editor-token", token_val);
      $token = token_val;

      await initCache(token_val);
    } else {
      alert("Auth failed");
    }
  }
</script>

<svelte:window />

<div id="content">
  {#if loading}
    Pensando com muita força...
  {:else }
    {#if $token}
      <Editor />
    {:else }
      <div>
        Token:
        <input type="text" class="input input-bordered" id="auth-token" />
        <input type="button" value="Save" class="input btn btn-primary" on:click={saveToken} />
      </div>
    {/if}
  {/if}
</div>

<style>
  :root {
    font-family: "Roboto", sans-serif;
    height: 100%;
    background: #fafafa;
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
</style>
