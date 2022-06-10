<script lang="ts">
    import type { Claim } from "../../util";
    import { Link } from "svelte-navigator";

    export let claim: Claim;
</script>

<div>
    <p>
        <span><svelte:component this={claim.icon} /></span>
        <span>{claim.title}</span>
        <span><Link to={`/create/witness/${claim.credential_type}`}>Obtain</Link></span>
    </p>
</div>
