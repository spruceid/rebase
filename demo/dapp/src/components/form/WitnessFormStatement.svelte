<script lang="ts">
    import type { Instructions } from "utils";
    import { alert } from "utils";
    import { Button, WitnessFormStepper } from "components";

    export let step: number | string = 1;
    export let totalSteps: number | string = 4;
    export let instructions: Instructions = {};
    export let loading: boolean = false;
    export let handle: string = "";
    export let onChangeValue: Function;
    export let navigate: Function;
    export let getStatement: Function;
    export let advance: Function;
</script>

<WitnessFormStepper
    {step}
    {totalSteps}
    label={instructions.statement_label}
    question={instructions.statement}
    labelFor={`form-step-q-${step}-i-1`}
>
    <input
        class="form-text-input"
        placeholder={instructions.statement_placeholder}
        bind:value={handle}
        on:input={(e) => onChangeValue("handle", e.target.value)}
        name={`form-step-q-${step}-i-1`}
        type="text"
    />
</WitnessFormStepper>
<div
    class="w-full my-[16px] text-center  flex flex-wrap justify-evenly items-center content-end"
>
    <Button
        class="w-2/5"
        onClick={() => navigate("/account")}
        text="Back"
        primary
        disabled={loading}
    />
    <Button
        {loading}
        class="w-2/5"
        disabled={handle.length === 0}
        onClick={async () => {
            try {
                loading = true;
                await getStatement();
                advance();
            } catch (e) {
                alert.set({
                    variant: "error",
                    message: e.message,
                });
            }
            loading = false;
        }}
        text="Next"
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
