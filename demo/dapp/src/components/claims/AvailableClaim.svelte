<script lang="ts">
    import type { Claim } from "utils";
    import { useNavigate } from "svelte-navigator";
    import { Tooltip } from "components";

    const navigate = useNavigate();

    export let claim: Claim;
</script>

<div
    class="py-2 w-full flex flex-wrap items-center justify-between border-b border-gray-200"
>
    <div class="flex flex-wrap items-center w-fit">
        <div><svelte:component this={claim.icon} /></div>
        <span>&nbsp;{claim.title}</span>
    </div>
    {#if claim.available}
        <div
            class="w-[30px] h-[30px] rounded-25 border border-purple-550 bg-purple-550 text-center cursor-pointer text-white font-bold text-2xl"
            on:click={() =>
                navigate(`/create/witness/${claim.credential_type}`)}
        >
            +
        </div>
    {:else}
        <Tooltip tooltip="Unavailable" left>
            <div
                class="w-[30px] h-[30px] rounded-25 border border-purple-550 bg-purple-550 text-center text-white font-bold text-2xl opacity-40 cursor-not-allowed"
                on:click={() => {}}
            >
                +
            </div>
        </Tooltip>
    {/if}
</div>
