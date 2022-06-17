<script lang="ts">
    import type { Claim } from "util";
    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    export let claim: Claim;
</script>

<div class="py-2 w-full flex flex-wrap justify-between border-b border-gray-200">
    <div class="flex flex-wrap w-fit">
        <div class="w-8"><svelte:component this={claim.icon} /></div>
        <span>&nbsp;{claim.title}</span>
    </div>
    <div
        class="w-[30px] h-[30px] rounded-25 border border-purple-550 bg-purple-550 text-center cursor-pointer text-white font-bold text-2xl"
        on:click={() => navigate(`/create/witness/${claim.credential_type}`)}
    >
        +
    </div>
</div>