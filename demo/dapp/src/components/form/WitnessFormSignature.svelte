<script lang="ts">
    import type { Instructions } from "utils";
    import { alert } from "utils";
    import { Button, WitnessFormStepper } from "components";

    export let step: number | string = 2;
    export let totalSteps: number | string = 4;
    export let instructions: Instructions = {};
    export let loading: boolean = false;
    export let statement: string = "";
    export let onChangeValue: Function;
    export let signature: string = "";
    export let sign: Function;
    export let back: Function;
    export let advance: Function;
</script>

<WitnessFormStepper
    {step}
    {totalSteps}
    label={instructions.signature_label}
    question={instructions.signature}
    labelFor={`form-step-q-${step}-i-1`}
>
    <div id={`form-step-q-${step}-i-1`}>
        <Button
            {loading}
            class="w-2/5 mt-[16px]"
            disabled={signature !== ""}
            onClick={async () => {
                try {
                    loading = true;
                    signature = await sign(statement);
                    onChangeValue("signature", signature);
                } catch (e) {
                    alert.set({
                        variant: "error",
                        message: e?.message ? e.message : e,
                    });
                }
                loading = false;
            }}
            text="Sign"
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
        disabled={signature === "" || loading}
        onClick={advance}
        text="Next"
        action
    />
</div>
