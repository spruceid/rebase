<script lang="ts">
    import type { Instructions } from "../util";
    import { instructions, alert } from "../util";
    import { onMount } from "svelte";
    import {
        WitnessForm,
        BasePage,
        SameForm,
        WitnessedSelfIssue,
    } from "src/components";
    import { writable, type Writable } from "svelte/store";
    import { FlowType } from "@spruceid/rebase-client";

    export let type: FlowType;

    let inst: Writable<Instructions> = writable(null);
    let _inst: Instructions = null;
    inst.subscribe((x) => (_inst = x));

    onMount(async () => {
        console.log("TYPE:", type);
        try {
            if (type !== "SameControllerAssertion" && type !== "Attestation") {
                let i = await instructions(type);
                inst.set(i as Instructions);
            }
        } catch (e) {
            alert.set({
                message: e?.message ?? `${e}`,
                variant: "error",
            });
        }
    });
</script>

<BasePage>
    <div class="min-h-[577px] h-full flex flex-wrap">
        {#if type === "SameControllerAssertion"}
            <SameForm />
        {:else if type === "Attestation"}
            <WitnessedSelfIssue />
        {:else if !_inst}
            <p class="inner-center">Building workflow...</p>
        {:else}
            <WitnessForm {type} instructions={_inst} />
        {/if}
    </div>
</BasePage>
