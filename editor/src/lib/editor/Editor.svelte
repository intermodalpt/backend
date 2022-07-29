<script>
    import StopEditor from './StopEditor.svelte';
    import LineEditor from './LineEditor.svelte';

    import 'leaflet.markercluster';
    import 'leaflet-contextmenu';

    let token = sessionStorage.getItem("authToken");

    let mode = 2;

    if (token == null) {
        token = prompt("Insert the authorization token");
        sessionStorage.setItem("authToken", token);
    }
</script>

<div class="vwrapper">
    <div>
        <label><input type=radio bind:group={mode} name="mode" value={1}>Stops</label>
        <label><input type=radio bind:group={mode} name="mode" value={2}>Lines</label>
    </div>
    {#if (mode === 1)}
        <StopEditor></StopEditor>
    {:else if (mode === 2) }
        <LineEditor></LineEditor>
    {:else }
        ???
    {/if}
</div>


<style>
    .vwrapper {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
        padding: 0 20px;
        margin-bottom: 40px;
    }
</style>


<link rel="stylesheet" href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css"
      integrity="sha512-xwE/Az9zrjBIphAcBb3F6JVqxf46+CDLwfLMHloNu6KEQCAWi6HcDUbeOfBIptF7tcCzusKFjFw2yuvEpDL9wQ=="
      crossorigin=""/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.css"/>
<link rel="stylesheet" href="https://unpkg.com/leaflet.markercluster@1.5.3/dist/MarkerCluster.Default.css"/>
<link rel="stylesheet" href="/map.css"/>
<link rel="stylesheet" href="/leaflet-contextmenu.css"/>