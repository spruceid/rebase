<script lang="ts">
    import { Claim, credentialToDisplay, client, alert } from "src/util";
    import {
        Button,
        IconButton,
        DownloadIcon,
        DeleteIcon,
        CheckmarkIcon,
        IconLink,
    } from "src/components";

    export let claim: Claim;
    export let removeClaim: Function;

    export const makeDownloadable = (jwt: string): string => {
        let encoded = encodeURIComponent(jwt);
        return `data:application/json;charset=utf-8,${encoded}`;
    };

    export const verify = async (jwt: string) => {
        try {
            let res = await client.verify_jwt(JSON.stringify({ jwt }));
            console.log(res);
            alert.set({
                message: "Credential verified!",
                variant: "success",
            });
        } catch (e) {
            console.error(e);
            alert.set({
                variant: "error",
                message: "Failed in verification",
            });
        }
    };
</script>

{#if claim.credentials.length !== 0}
    <div class="obtained-claim py-2 w-full flex flex-wrap justify-between ">
        <div class="flex flex-wrap w-full">
            <div class="flex flex-wrap justify-center items-center">
                <div class="w-8 h-fit">
                    <svelte:component this={claim.icon} />
                </div>
                <div class="font-semibold">
                    {claim.title}
                </div>
            </div>
            {#each claim.credentials as credential}
                <div
                    class="w-full px-4 py-1 flex flex-wrap justify-between items-center"
                >
                    {#if credentialToDisplay(credential).type === "basic_public"}
                        <div
                            class="w-4/5 whitespace-nowrap overflow-hidden text-ellipsis"
                        >
                            {credentialToDisplay(credential).handle}
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
                        <div class="w-2" />
                        <div
                            class="obtained-claim-action border border-gray-250 w-8 h-8 rounded-full flex flex-wrap align-center justify-center"
                        >
                            <IconButton
                                class="block w-4"
                                onClick={() => verify(credential)}
                                icon={CheckmarkIcon}
                                color="#b3b3b3"
                            />
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </div>
{/if}

<style>
    .obtained-claim:nth-child(n + 2) {
        @apply border-t border-gray-200;
    }

    .obtained-claim-action :global(a) {
        margin: auto;
    }
    .obtained-claim-action :global(svg) {
        width: 15px;
    }
</style>
