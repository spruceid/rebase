<script lang="ts">
    import { claims, Claim } from "src/util";
    import { ObtainedClaim } from "src/components";
    import { Writable, writable } from "svelte/store";

    let _claims: Array<Claim> = [];

    claims.subscribe((x) => {
        _claims = x;
    });

    const hasCredentials = (): boolean => {
        for (let claim of _claims) {
            if (claim.credentials.length > 0) {
                return true;
            }
        }
        return false;
    };

    let showCredentials: Writable<boolean> = writable(hasCredentials());
    let _show = hasCredentials();
    showCredentials.subscribe((x) => (_show = x));

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
        showCredentials.set(hasCredentials());
    };
</script>

{#if _show}
    <div class="w-full min-h-[24rem] h-auto">
        <h3 class="py-4 px-4">My Credentials</h3>
        <div class="max-h-[350px] overflow-auto px-4">
            {#each _claims as claim}
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
