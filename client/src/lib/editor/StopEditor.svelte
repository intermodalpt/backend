<script>
    import Box from '../components/Box.svelte';
    import StopForm from './StopForm.svelte';

    export let token;
    export let stops;
    export let openedStop;

    let pendingOps = [];

    export function selectStop(stopId) {
        console.log(this);
        console.log(`Showing stop info for ${stopId}`);
        openedStop = stops[stopId];
    }

    export function moveStop(e) {
        let stopId = e.target.stopId;
        let newPos = e.target.getLatLng();
        let existing = pendingOps.find(el => el.op === updateStop && el.stop.id === stopId);
        if (existing) {
            existing.stop.lat = newPos.lat;
            existing.stop.lon = newPos.lng;
        } else {
            let metaCopy = Object.assign({}, e.target.meta);
            metaCopy.lat = newPos.lat;
            metaCopy.lon = newPos.lng;
            pendingOps.push({
                op: updateStop,
                stop: e.target.meta
            });
            pendingOps = pendingOps;
        }
    }

    function saveStopInfo(e) {
        let stopId = e.detail.id;
        let existing = pendingOps.find(el => el.op === updateStop && el.stop.id === stopId);
        if (existing) {
            existing.stop = Object.assign(existing.stop, e.detail);
        } else {
            pendingOps.push({
                op: updateStop,
                stop: e.detail
            });
            pendingOps = pendingOps;
        }

        openedStop = null;
    }

    function updateStop(stop) {
        fetch(`http://0.0.0.0:8080/api/stops/update/${stop.id}`, {
                method: "patch",
                headers: {
                    'Content-Type': 'application/json',
                    'Token': token
                },
                body: JSON.stringify(stop)
            }
        ).then(r => r.json())
            .then(data => {
                //
            });
    }
</script>



{#if openedStop }
    <StopForm bind:stop={openedStop} on:save={saveStopInfo}></StopForm>
{/if}



<h1>Alterações</h1>
<Box padded="true">
    <ul class="pending-changes">
        {#each pendingOps as op, i}
            {#if op.op === updateStop}
                <li>
                    <div class="changes">
                        <span class="title">Atualização de paragem {op.stop.id} - {op.stop.name}</span>
                        <span>
                            {JSON.stringify(op)}
                            <!--{JSON.stringify(op.data)}-->
                        </span>
                    </div>
                    <button on:click={e => {pendingOps.splice(i, 1); pendingOps = pendingOps}}>Call it quitz</button>
                </li>
            {:else }
                <li>???</li>
            {/if}
        {/each}
    </ul>
    <br>
    <input type="button" value="Guardar" disabled/>
</Box>

<style>
    .pending-changes li {
        display: flex;
        justify-content: space-between;
    }

    .pending-changes button {
        width: 60px;
    }

    .pending-changes .changes {
        display: flex;
        flex-direction: column;
    }

    .pending-changes .changes .title {
        font-size: 1.2em;
        font-weight: bold;
    }
</style>