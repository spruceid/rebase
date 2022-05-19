<script lang="ts">
    import { Claim, credentialToDisplay } from "../../util";
    import DownloadIcon from "../icons/DownloadIcon.svelte";
    import IconLink from "../icons/IconLink.svelte";
    export let claim: Claim;

    export const makeDownloadable = (obj: any): string => {
        let stringify = JSON.stringify(obj, null, 2);
        let encoded = encodeURIComponent(stringify);
        return `data:application/json;charset=utf-8,${encoded}`;
    };
    // TODO: REMOVE JANKINESS.
</script>

<div>
    <p>
        <span><svelte:component this={claim.icon} /></span>
        {#each claim.credentials as credential}
            {#if credentialToDisplay(credential).type === "basic_public"}
                Handle: {credentialToDisplay(credential).handle}
                Address: {credentialToDisplay(credential).address}
            {:else if credentialToDisplay(credential).type === "basic_blockchain"}
                Address: {credentialToDisplay(credential).address}
            {/if}

            <IconLink
                class="block w-4 h-4"
                icon={DownloadIcon}
                href={makeDownloadable(JSON.parse(credential))}
                download={`${claim.credential_type}_${
                    credentialToDisplay(credential).address
                }.json`}
            />
        {/each}
        {#if claim.credentials.length == 0}
            No {claim.title} credentials.
        {/if}
    </p>
</div>
