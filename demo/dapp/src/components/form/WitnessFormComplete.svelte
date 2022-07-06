<script lang="ts">
    import type { Instructions } from "utils";
    import { Button, WitnessFormStepper } from "components";

    export let step: number | string = 4;
    export let totalSteps: number | string = 4;
    export let instructions: Instructions = {
        complete_label: "Complete",
        complete: "Please click the button to manage your credentials."
    };
    export let navigate: Function;
</script>

<WitnessFormStepper
    {step}
    {totalSteps}
    label={instructions.complete_label}
    question={instructions.complete}
    labelFor={`form-q-${step}-i-1`}
/>
<div id={`form-q-${step}-i-1`} class="w-full my-[16px] text-center  flex flex-wrap justify-evenly items-center content-end">
    <Button
        class="w-fit  my-[16px]"
        onClick={() => navigate("/account#obtained")}
        text="Manage Credentials"
        action
    />
</div>
