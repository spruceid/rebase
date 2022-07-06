<script lang="ts">
    import { claims, Claim } from "utils";
    import { ObtainedClaim } from "components";

    let bcClaims: Array<Claim> = [];
    let pbClaims: Array<Claim> = [];
    let _claims: Array<Claim> = [];
    let showCredentials: boolean;


    $: $claims, hasCredentials();

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
        hasCredentials();
    };

    const hasCredentials = () => {
        for (let claim of _claims) {
            if (claim.credentials.length > 0) {
                showCredentials = true;
                return;
            }
        }
        showCredentials = false;
    };
</script>

{#if showCredentials}
    <div class="w-full min-h-[24rem] h-auto">
        <h3 class="py-4 px-4">My Credentials</h3>
        <div class="max-h-[350px] overflow-auto px-4">
            {#each pbClaims as claim}
                <ObtainedClaim {claim} {removeClaim} />
            {/each}
            {#each bcClaims as claim}
                <ObtainedClaim {claim} {removeClaim} />
            {/each}
        </div>
    </div>
{:else}
    <div class="w-full text-center">
        <b>You don't have credentials yet</b><br />
        Click on "Available" and start obtaining credentials
    </div>
{/if}
