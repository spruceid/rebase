<script lang="ts">
    export let step: string | number;
    export let totalSteps: string | number;
    export let label: string;
    export let question: string;
    export let labelFor: string;
</script>

<div class="w-full">
    <div class="w-full flex flex-wrap justify-center content-between h-full">
        <div class="w-full mx-6 px-4">
            <label for={labelFor}>
                <p class="font-bold">{label}</p>
                <span>{question}</span>
            </label>
            <slot />
        </div>
        <div class="w-full h-fit font-semibold text-center text-gray-400">
            {step}/{totalSteps}
        </div>
    </div>
</div>
