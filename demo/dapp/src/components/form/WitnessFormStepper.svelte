<script lang="ts">
    export let step: string | number;
    export let totalSteps: string | number;
    export let label: string;
    export let question: string;
    export let labelFor: string;
</script>

<div>
    <div class="flex flex-wrap justify-center content-between h-full">
        <div class="step-counter">
            {step}
        </div>
        <div class="w-3/4 px-4">
            <label for={labelFor}>
                <p class="font-bold">{label}</p>
                {question}
            </label>
            <slot />
        </div>
        <div class="w-full h-fit font-semibold text-center text-gray-400">
            {step}/{totalSteps}
        </div>
    </div>
</div>

<style>
    .step-counter {
        @apply w-8 h-8 border border-purple-550 rounded-full text-center bg-purple-550 font-bold text-white leading-8 text-[15px];
    }
</style>
