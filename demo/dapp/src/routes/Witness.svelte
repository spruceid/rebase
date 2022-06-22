<script lang="ts">
    import type { CredentialType } from "../util";
    import { instructions } from "../util";
    import { onMount } from "svelte";
    import { WitnessForm, BasePage, SelfSignedForm } from "components";

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

<BasePage>
    <div class="min-h-[577px] h-full flex flex-wrap">
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
</BasePage>
