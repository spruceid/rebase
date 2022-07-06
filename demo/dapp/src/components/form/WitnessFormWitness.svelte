<script lang="ts">
    import type { Instructions, CredentialType } from "utils";
    import { alert } from "utils";
    import { Button, WitnessFormStepper, CopyTextArea } from "components";

    export let step: number | string = 3;
    export let totalSteps: number | string = 4;
    export let instructions: Instructions = {};
    export let loading: boolean = false;
    export let verified: boolean = false;
    export let type: CredentialType;
    export let proof: string = "";
    export let onChangeValue: Function;
    export let getCredential: Function;
    export let post: Function;
    export let back: Function;
    export let advance: Function;
</script>

<WitnessFormStepper
            {step}
            {totalSteps}
            label={instructions.witness_label}
            question={instructions.witness}
            labelFor={`form-step-q-${step}-i-1`}
        >
            <div id={`form-step-q-${step}-i-1`}>
                <CopyTextArea value={post()} />
                {#if type === "twitter" || type === "github" || type === "discord"}
                    <div class="w-full">
                        <input
                            class="form-text-input w-full"
                            placeholder={instructions.witness_placeholder}
                            bind:value={proof}
                            on:input={e => onChangeValue('proof', proof)}
                            name={"form-step-q-3-i-1"}
                            type="text"
                        />
                    </div>
                {/if}
                <Button
                    {loading}
                    class="w-full"
                    disabled={verified}
                    onClick={async () => {
                        try {
                            loading = true;
                            await getCredential();
                            verified = true;
                        } catch (e) {
                            alert.set({
                                variant: "error",
                                message: e?.message ? e.message : e,
                            });
                        }
                        loading = false;
                    }}
                    text="Verify"
                    action
                />
            </div>
        </WitnessFormStepper>
        <div class="w-full my-[16px] text-center  flex flex-wrap justify-evenly items-center content-end">
            <Button
                class="w-2/5"
                onClick={back}
                text="Back"
                primary
                disabled={loading}
            />
            <Button
                class="w-2/5"
                disabled={!verified || loading}
                onClick={advance}
                text="Complete"
                action
            />
        </div>

<style>
    .form-text-input {
        @apply w-full bg-gray-100 rounded-md py-2 px-2 my-4;
    }
    .form-text-input::placeholder {
        @apply font-bold text-gray-350;
    }
</style>
