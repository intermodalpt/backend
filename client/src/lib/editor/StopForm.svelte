<script>
    import Box from '../components/Box.svelte';

    import {createEventDispatcher} from 'svelte';

    export let stop;

    let name = stop.name;
    let short_name = stop.short_name;
    let street = stop.street;
    let door = stop.door;
    let source = stop.source;
    let notes = "";

    $: name = stop.name;
    $: short_name = stop.short_name;
    $: street = stop.street;
    $: door = stop.door;
    $: source = stop.source;
    notes = "";

    function save() {
        dispatch('save', Object.assign(stop, {
            name: name,
            short_name: short_name,
            street: street,
            door: door,
            source: source,
        }));
    }


    const dispatch = createEventDispatcher();
</script>

Name: <input bind:value={name}><br>
Short name: <input bind:value={short_name}><br>
Street: <input bind:value={street}><br>
Door: <input bind:value={door}><br>
Source: <input bind:value={source}><br>
Notes:<br>
<!--    <textarea bind:notes></textarea><br>-->

<button on:click={save}>
    Save
</button>