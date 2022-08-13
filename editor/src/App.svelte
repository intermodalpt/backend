<script>
  import Editor from "./lib/editor/Editor.svelte";
  import {initCache} from "./cache.js";
  import {onMount} from "svelte";


  onMount(async () => {
    await initCache();
  });
</script>

<svelte:window />

<div id="content">
  <Editor />
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
