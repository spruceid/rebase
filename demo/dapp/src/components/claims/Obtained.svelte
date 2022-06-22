<script lang="ts">
    import { claims, Claim } from "util";
    import ObtainClaim from "./ObtainedClaim.svelte";

    let bcClaims: Array<Claim> = [];
    let pbClaims: Array<Claim> = [];
    let _claims: Array<Claim> = [];

    claims.subscribe((x) => {
        _claims = x;
        bcClaims = x.filter((claim) => claim.type === "blockchain");
        pbClaims = x.filter((claim) => claim.type === "public");
    });

    const removeClaim = (claim, credential) => {
        let newClaims: Array<Claim> = [];
        _claims.forEach((c) => {
            if (c.credential_type === claim.credential_type) {
                c.credentials = c.credentials.filter(
                    (cred) => cred !== credential
                );
            }
            newClaims.push(c);
        });
        claims.set(newClaims);
    };
</script>

<div class="w-full">
    <h3 class="py-4 px-4">My Credentials</h3>
    <div class="max-h-full overflow-auto px-4">
        {#each pbClaims as claim}
            <ObtainClaim {claim} {removeClaim} />
        {/each}
        {#each bcClaims as claim}
            <ObtainClaim {claim} {removeClaim} />
        {/each}
    </div>
</div>
