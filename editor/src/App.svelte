<script>
  import Editor from "./lib/editor/Editor.svelte";
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
