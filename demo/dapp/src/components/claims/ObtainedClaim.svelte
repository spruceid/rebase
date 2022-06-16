<script lang="ts">
    import { Claim, credentialToDisplay } from "util";
    import { DownloadIcon } from "components/icons";
    import { IconLink } from "components";
    export let claim: Claim;

    export const makeDownloadable = (jwt: string): string => {
        let encoded = encodeURIComponent(jwt);
        return `data:application/json;charset=utf-8,${encoded}`;
    };
    // TODO: REMOVE JANKINESS.
</script>

<div class="py-2 w-full flex flex-wrap justify-between">
    <div class="flex flex-wrap w-fit">
        <div class="w-8"><svelte:component this={claim.icon} /></div>
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
                href={makeDownloadable(credential)}
                download={`${claim.credential_type}_${
                    credentialToDisplay(credential).address
                }.jwt`}
            />
        {/each}
        {#if claim.credentials.length == 0}
            No {claim.title} credentials.
        {:else}
            <div>actions</div>
        {/if}
    </div>
</div>
