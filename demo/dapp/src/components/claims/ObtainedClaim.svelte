<script lang="ts">
    import { Claim, credentialToDisplay } from "util";
    import { IconButton, DownloadIcon, DeleteIcon, IconLink } from "components";

    export let claim: Claim;
    export let removeClaim: Function;

    export const makeDownloadable = (jwt: string): string => {
        let encoded = encodeURIComponent(jwt);
        return `data:application/json;charset=utf-8,${encoded}`;
    };
</script>

<div
    class="py-2 w-full flex flex-wrap justify-between border-b border-gray-200"
>
    <div class="flex flex-wrap w-full">
        <div class="flex flex-wrap justify-center items-center">
            <div class="w-8 h-fit"><svelte:component this={claim.icon} /></div>
            <div class="font-semibold">
                {claim.title}
            </div>
        </div>
        {#if claim.credentials.length == 0}
            <div class="w-full  px-4">
                No {claim.title} credentials
            </div>
        {:else}
            {#each claim.credentials as credential}
                <div
                    class="w-full px-4 flex flex-wrap justify-between items-center"
                >
                    {#if credentialToDisplay(credential).type === "basic_public"}
                        <div
                            class="w-4/5 whitespace-nowrap overflow-hidden text-ellipsis"
                        >
                            Handle: {credentialToDisplay(credential).handle}
                        </div>
                    {:else if credentialToDisplay(credential).type === "basic_blockchain"}
                        Address: {credentialToDisplay(credential).address}
                    {/if}
                    <div class="flex flex-wrap justify-center">
                        <div
                            class="obtained-claim-action border border-gray-250 w-8 h-8 rounded-full flex flex-wrap align-center justify-center transition-all"
                        >
                            <IconLink
                                class="block w-4"
                                icon={DownloadIcon}
                                href={makeDownloadable(credential)}
                                download={`${claim.credential_type}_${
                                    credentialToDisplay(credential).address
                                }.jwt`}
                            />
                        </div>
                        <div class="w-2" />
                        <div
                            class="obtained-claim-action border border-gray-250 w-8 h-8 rounded-full flex flex-wrap align-center justify-center"
                        >
                            <IconButton
                                class="block w-4"
                                onClick={() => removeClaim(claim, credential)}
                                icon={DeleteIcon}
                                color="#b3b3b3"
                            />
                        </div>
                    </div>
                </div>
            {/each}
        {/if}
    </div>
</div>

<style>
    .obtained-claim-action :global(a) {
        margin: auto;
    }
    .obtained-claim-action :global(svg) {
        width: 15px;
    }
</style>
