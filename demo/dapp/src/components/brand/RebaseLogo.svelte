<script lang="ts">
    let clazz: string = "";
    export { clazz as class };
    export let xl: boolean = false;
    export let xl4_5: boolean = false;
</script>

<div class={clazz}>
    <img 
    alt="Rebase Logo" 
    src="/assets/rebase_icon.svg" 
    class:h-[30px]={xl} 
    class:h-[60px]={xl4_5} 
    />
    &nbsp;
    <p class="font-base font-bold" 
    class:text-xl={xl}
    class:text-4_5xl={xl4_5}
    >Rebase</p>
</div>
