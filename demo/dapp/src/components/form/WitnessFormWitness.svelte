<script lang="ts">
    import type { Instructions, CredentialType } from "src/util";
    import { alert } from "src/util";
    import { Button, WitnessFormStepper, CopyTextArea } from "src/components";

    export let step: number | string = 3;
    export let totalSteps: number | string = 4;
    export let instructions: Instructions;
    export let loading: boolean = false;
    export let verified: boolean = false;
    export let type: CredentialType;
    export let proof: string = "";
    export let onChangeValue: Function;
    export let getCredential: Function;
    export let post: Function;
    export let back: Function;
    export let advance: Function;

    const needsCopyArea = (t: CredentialType): boolean => {
        return (
            t !== "EmailVerification" &&
            t !== "NftOwnershipVerification" &&
            t !== "PoapOwnershipVerification"
        );
    };

    const needsInput = (t: CredentialType): boolean => {
        return (
            t === "TwitterVerification" ||
            t === "GitHubVerification" ||
            t === "EmailVerification"
        );
    };
</script>

<WitnessFormStepper
    {step}
    {totalSteps}
    label={instructions.witness_label}
    question={instructions.witness}
    labelFor={`form-step-q-${step}-i-1`}
>
    <div id={`form-step-q-${step}-i-1`}>
        {#if needsCopyArea(type)}
            <CopyTextArea value={post()} />
        {/if}
        {#if needsInput(type)}
            <div class="w-full">
                <input
                    class="form-text-input w-full"
                    placeholder={instructions.witness_placeholder}
                    bind:value={proof}
                    on:input={(e) => onChangeValue("proof", proof)}
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
                    await getCredential(true);
                    verified = true;
                } catch (e) {
                    alert.set({
                        variant: "error",
                        message: e?.message ? e.message : e,
                    });
                }
                loading = false;
            }}
            text="Issue JWT"
            action
        />
        <Button
            {loading}
            class="w-full"
            disabled={verified}
            onClick={async () => {
                try {
                    loading = true;
                    await getCredential();
                } catch (e) {
                    alert.set({
                        variant: "error",
                        message: e?.message ? e.message : e,
                    });
                }
                loading = false;
            }}
            text="Console Log LD Credential"
            action
        />
    </div>
</WitnessFormStepper>
<div
    class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center content-end"
>
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
