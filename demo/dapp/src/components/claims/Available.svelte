<script lang="ts">
    import { claims, Claim } from "../../util";
    import AvailableClaim from "./AvailableClaim.svelte";

    let bcClaims: Array<Claim> = [];
    let pbClaims: Array<Claim> = [];
    // let sfClaims: Array<Claim> = [];

    claims.subscribe((x) => {
        bcClaims = x.filter((claim) => claim.type === "blockchain");
        pbClaims = x.filter((claim) => claim.type === "public");
        // sfClaims = x.filter((claim) => claim.type === "self_attested");
    });
</script>

<div>
    <h3>Public Accounts</h3>
    <div class="account">
        {#each pbClaims as claim}
            <AvailableClaim {claim} />
        {/each}
    </div>
    <h3>Blockchain Accounts</h3>
    <div class="account">
        {#each bcClaims as claim}
            <AvailableClaim {claim} />
        {/each}
    </div>
    <!-- <h3>Self Attested Claims</h3> -->
    <!-- <div class="account"> -->
    <!-- </div> -->
</div>
