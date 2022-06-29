<script lang="ts">
    let clazz: string = "";
    export { clazz as class };
    export let xl: boolean = false;
    export let logo: boolean = false;
</script>

<div class={clazz}>
    <img
        alt="Rebase Logo"
        src="/assets/rebase_icon.svg"
        class:h-[30px]={xl}
        class:h-[60px]={logo}
    />
    {#if !logo}
        &nbsp;
        <p
            class="font-base font-bold"
            class:text-xl={xl}
        >
            Rebase
        </p>
    {/if}
</div>
