<script lang="ts">
    import { claims, Claim } from "src/util";
    import { AvailableClaim } from "src/components";

    let _claims: Array<Claim> = [];
    claims.subscribe((x) => (_claims = x));
</script>

<div class="w-full h-96">
    <h3 class="py-4 px-4">Available Credentials</h3>
    <div class="overflow-auto px-4">
        {#each _claims as claim}
            <AvailableClaim {claim} />
        {/each}
    </div>
</div>
