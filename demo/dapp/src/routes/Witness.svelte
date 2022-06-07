<script lang="ts">
    import type { CredentialType } from "../util";
    import { instructions } from "../util";
    import { onMount } from "svelte";
    import WitnessForm from "../components/claims/WitnessForm.svelte";
    import SelfSignedForm from "../components/claims/SelfSignedForm.svelte";

    export let type: CredentialType;
    $: inst = null;
    $: errMsg = "";
    $: loading = true;

    onMount(async () => {
        try {
            if (type !== "self_signed") {
                inst = await instructions(type);
            }
            loading = false;
        } catch (e) {
            errMsg = `${e.message}`;
        }
    });
</script>

<div class="viewer">
    {#if loading}
        <p class="inner-center">Building workflow...</p>
    {:else if errMsg}
        <p class="inner-center">Error encountered: ${errMsg}</p>
    {:else if type === "self_signed"}
        <SelfSignedForm />
    {:else}
        <WitnessForm {type} instructions={inst} />
    {/if}
</div>


<style>
    .viewer {
        height: 80vh;
        width: 75vh;
        background-color: white;
    }
    .inner-center {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-left: 5vh;
        margin-right: 5vh;
    }
</style>

