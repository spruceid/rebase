<script lang="ts">
    import type { CredentialType } from "../util";
    import { instructions } from "../util";
    import { onMount } from "svelte";
    import WitnessForm from "./WitnessForm.svelte";

    export let type: CredentialType;
    $: inst = null;
    $: errMsg = "";
    $: loading = true;

    onMount(async () => {
        try {
            inst = await instructions(type);
            loading = false;
        } catch (e) {
            errMsg = `${e.message}`;
        }
    });
</script>

<div>
    {#if loading}
        <p>Building workflow...</p>
    {:else if errMsg}
        <p>Error encountered: ${errMsg}</p>
    {:else}
        <WitnessForm {type} instructions={inst} />
    {/if}
</div>
