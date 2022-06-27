<script lang="ts">
    export let icon: any;
    export let title: string;
    export let subtitle: string;
    export let subsubtitle: string = "";
</script>

<div class="w-full text-center">
    <div class="w-8 mx-auto icon-container py-4">
        <svelte:component this={icon} />
    </div>
    <p class="font-bold text-2xl text-gray-850">{title}</p>
    <p class="text-sm px-4">{subtitle}</p>
    <p class="text-sm px-4">{subsubtitle}</p>
</div>

<style>
    .icon-container :global(svg) {
        width: 30px;
        height: auto;
    }
</style>
