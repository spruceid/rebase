<script lang="ts">
    import {
        Available,
        Obtained,
        BasePage,
        ToggleButton,
        Button,
    } from "components";
    import { AccountState } from "utils";
    import { useNavigate } from "svelte-navigator";
    import { onMount } from "svelte";

    export let location: any = {};

    const navigate = useNavigate();

    let state: AccountState;

    const changeAccountState = (option) => {
        if (option !== state) {
            state = option;
        }
    };

    onMount(() => {
        if (location.hash.includes("obtained")) {
            state = "obtained";
        } else {
            state = "available";
        }
    });
</script>

<BasePage>
    <div class="min-h-[577px] h-full flex flex-wrap">
        <div class="w-full">
            {#if state}
                <ToggleButton
                    class=""
                    selected={state}
                    onClick={changeAccountState}
                    options={["available", "obtained"]}
                />
            {/if}
        </div>
        {#if state == "available"}
            <Available />
        {/if}
        {#if state == "obtained"}
            <Obtained />
        {/if}
    </div>
</BasePage>
