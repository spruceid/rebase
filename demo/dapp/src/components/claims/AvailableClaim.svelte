<script lang="ts">
    import type { Claim } from "util";
    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    export let claim: Claim;
</script>

<div class="py-2 w-full flex flex-wrap justify-between">
    <div class="flex flex-wrap w-fit">
        <div class="w-8"><svelte:component this={claim.icon} /></div>
        <span>&nbsp;{claim.title}</span>
    </div>
    <div
        class="w-6 h-6 rounded-25 border border-purple-550 text-center cursor-pointer text-black font-bold"
        on:click={() => navigate(`/create/witness/${claim.credential_type}`)}
    >
        +
    </div>
</div>