<script lang="ts">
    import { claims, Claim } from "utils";
    import { AvailableClaim } from "components";

    let bcClaims: Array<Claim> = [];
    let pbClaims: Array<Claim> = [];

    claims.subscribe((x) => {
        pbClaims = x.filter((claim) => claim.type === "public");
        bcClaims = x.filter((claim) => claim.type === "blockchain");
    });
</script>

<div class="w-full h-96">
    <h3 class="py-4 px-4">Social Media Credentials</h3>
    <div class="overflow-auto px-4">
        {#each pbClaims as claim}
            <AvailableClaim {claim} />
        {/each}
    </div>
</div>

{#if bcClaims?.length > 0}
    <div class="w-full">
        <h3 class="py-4 px-4">Blockchain Accounts</h3>
        <div class="max-h-40 overflow-auto px-4">
            {#each bcClaims as claim}
                <AvailableClaim {claim} />
            {/each}
        </div>
    </div>
{/if}
