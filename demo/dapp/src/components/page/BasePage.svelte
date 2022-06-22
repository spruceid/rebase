<script lang="ts">
    import { Alert, Header } from "components";
    let clazz: string = "";
    export { clazz as class };
</script>

<Header />
<div
    class={`${clazz} flex flex-row flex-wrap items-center justify-center w-full min-h-[calc(100vh-70px)]`}
>
    <div class="w-full max-w-[500px] card min-h-[577px] h-auto mt-2.5 sm:mt-0">
        <div class="px-[42px] md:px-[16px] md:p-0 min-h-[577px] h-full">
            <slot />
        </div>
    </div>
</div>
<Alert />
