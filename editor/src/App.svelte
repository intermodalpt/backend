<script>
  import Editor from "./lib/editor/Editor.svelte";
  import {initCache} from "./cache.js";
  import {token, api_server} from "./settings.js";
  import {onMount} from "svelte";


  onMount(async () => {
    await initCache();
  });

  let ls_token = localStorage.getItem("editor-token");
  if (ls_token != null) {
    $token = ls_token;
    checkToken();
  }

  async function checkToken(token) {
    let res = await fetch(`${api_server}/auth/check`, {
      method: 'POST',
      headers: {
        authorization: `Bearer ${token}`
      }
    });
    console.log(res);
    return res.ok;
  }

  function saveToken() {
    let token_val = document.getElementById("auth-token").value;
    if (checkToken(token_val)) {
      localStorage.setItem("editor-token", token_val);
      $token = token_val;
    }
  }
</script>

<svelte:window />

<div id="content">
  {#if $token}
    <Editor />
  {:else }
    <div>
      Token:
      <input type="text" class="input input-bordered" id="auth-token" />
      <input type="button" value="Save" class="input btn btn-primary" on:click={saveToken} />
    </div>
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
