<script lang="ts">
    import { claims, Claim } from "util";
    import AvailableClaim from "./AvailableClaim.svelte";

    let bcClaims: Array<Claim> = [];
    let pbClaims: Array<Claim> = [];

    claims.subscribe((x) => {
        pbClaims = x.filter((claim) => claim.type === "public");
    });
</script>

<div class="w-full">
    <h3>Public Accounts</h3>
    <div class="max-h-[100px] overflow-auto px-4">
        {#each pbClaims as claim}
            <AvailableClaim {claim} />
        {/each}
    </div>
</div>

<div class="w-full">
    <h3>Blockchain Accounts</h3>
    <div class="max-h-[100px] overflow-auto px-4">
        {#each bcClaims as claim}
            <AvailableClaim {claim} />
        {/each}
    </div>
</div>