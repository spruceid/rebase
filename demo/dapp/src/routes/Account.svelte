<script lang="ts">
    import { Available, Obtained } from "components/claims";
    import { BasePage, ToggleButton, Button } from "components";
    import { accountState, AccountState } from "util";
    import { useNavigate } from "svelte-navigator";
    import { onMount } from "svelte";

    // export let params: any = {};

    const navigate = useNavigate();

    let state: AccountState = "available";
    accountState.subscribe((x) => {
        state = x;
    });

    const changeAccountState = (option) => {
        if (option !== state) {
            accountState.set(option);
        }
    };

    onMount(() => {});
</script>

<BasePage>
    <div class="min-h-[577px] h-full flex flex-wrap">
        <div class="w-full">
            <ToggleButton
                class=""
                onClick={changeAccountState}
                options={["available", "obtained"]}
            />
        </div>
        {#if state == "available"}
            <Available />
        {:else}
            <Obtained />
        {/if}

        <div class="w-full flex justify-center items-center">
            <Button
                class="w-full max-w-42 my-[16px]"
                onClick={() => navigate("/")}
                text="Help"
                primary
            />
        </div>
    </div>
</BasePage>
