<script lang="ts">
    export let step: string | number;
    export let totalSteps: string | number;
    export let label: string;
    export let question: string;
    export let labelFor: string;
</script>

<div class="flex flex-wrap justify-center">
    <div class="step-counter">
        {step}
    </div>
    <div class="w-3/4 px-4">
        <label for={labelFor}>
            <p class="font-bold">{label}</p>
            {question}
        </label>
        <slot />
        {step}/{totalSteps}
    </div>
</div>

<style>
    .step-counter {
        @apply w-8 h-8 border border-purple-550 rounded-full text-center bg-purple-550 font-bold text-white leading-8 text-[15px] ;
    }
</style>
