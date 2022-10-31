<script lang="ts">
    import {
        _signerMap,
        alert,
        connect,
        disconnect,
        disconnectAll,
        displaySignerId,
        displaySignerType,
        displayProviderType,
        lookUp,
        signerTypes,
        getAllConnected,
        retrieveSigner,
        SignerType,
    } from "src/util";

    import {
        Tooltip,
        Button,
        DropdownButton,
        RebaseLogo,
    } from "src/components";

    import { scale } from "svelte/transition";
    import { Link, useNavigate } from "svelte-navigator";

    $: connectNext = false;

    let _lookUp = null;
    let _signer = null;

    lookUp.subscribe((x) => {
        _lookUp = x;
        if (_lookUp && _signerMap) {
            _signer = retrieveSigner(_signerMap, _lookUp);
        }
    });

    const navigate = useNavigate();

    let loading: boolean = false;

    const capitalizeFirstLetter = (string) => {
        return string.charAt(0).toUpperCase() + string.slice(1);
    };

    // This is dumbed down compared to the eventual support needed by
    // multiple providers. Suggested the usage of the implementation in the
    // comments below.
    const conn = (signerType: SignerType) => {
        switch (signerType) {
            case "ethereum":
                // NOTE: Only support web3 modal, so if the user
                // selects a non-MetaMask wallet through
                // web3 modal, it will get correctly set.
                // Only WalletConnect and Metamask supported.
                return connect(signerType, "metamask");
            case "solana":
                return connect(signerType, "phantom");
            default:
                alert.set({
                    message: `Unknown signer type: ${signerType}`,
                    variant: "error",
                });
        }
    };

    // TODO: Use this when supporting multiple providers
    // Adds section to dropdown below
    /* $: signerType = null;

    const conn = async (providerType: ProviderType) => {
        if (!providerType) {
            alert.set({
                message: "Provider Type must be set.",
                variant: "error",
            });
        } else if (!signerType) {
            alert.set({
                message: "Signer Type must be set.",
                variant: "error",
            });
        } else {
            await connect(signerType, providerType);
            signerType = null;
        }
    };
    */
</script>

<div
    class="min-w-screen px-4 h-[70px] w-full flex items-center justify-between bg-white shadow"
>
    <Link to="/account">
        <RebaseLogo class="w-fit flex items-center" xl />
    </Link>

    <div class="flex flex-wrap">
        {#if _lookUp}
            <Tooltip
                tooltip="Using {displaySignerType(
                    _lookUp.signerType
                )} with {displayProviderType(_lookUp.providerType)}"
                bottom
            >
                <Button
                    class="max-w-42 sm:max-w-full my-[16px]"
                    onClick={() => navigate("/account")}
                    text={`${_signer?.ens?.name ?? displaySignerId(_signer)}`}
                    primary
                    avatar={_signer?.ens?.avatar ?? false}
                />
            </Tooltip>
        {/if}
        <DropdownButton
            class="menu w-full my-[16px] rounded-xl px-1000"
            ml={!!_lookUp}
            text={_lookUp ? "..." : "Connect Wallet"}
            primary
            {loading}
        >
            <div
                in:scale={{ duration: 100, start: 0.95 }}
                out:scale={{ duration: 75, start: 0.95 }}
                class="origin-top-right absolute right-4 w-48 py-0 mt-1 bg-dark-1 rounded-xl shadow-md"
            >
                {#if _lookUp && !connectNext}
                    <Button
                        class="w-full bg-dark-1 text-white py-4"
                        onClick={() => {
                            navigate("/");
                        }}
                        text="About Rebase"
                    />
                    <Button
                        class="w-full bg-dark-1 text-white py-4"
                        onClick={() => {
                            disconnectAll();
                            lookUp.set(null);
                        }}
                        text="Disconnect"
                    />
                {:else}
                    <div class="px-4 py-3 text-sm text-white text-center">
                        <div>Select Signer Type To Connect</div>
                    </div>
                    {#if connectNext}
                        <Button
                            class="w-full bg-dark-1 text-white py-4"
                            onClick={() => {
                                connectNext = false;
                            }}
                            text="Cancel New Connection"
                        />
                    {/if}
                    <hr />

                    {#each signerTypes as t}
                        <Button
                            class="w-full bg-dark-1 text-white py-4"
                            onClick={async () => {
                                loading = true;
                                await conn(t);
                                loading = false;
                                connectNext = false;
                            }}
                            text={capitalizeFirstLetter(t)}
                        />
                    {/each}

                    <!-- TODO: Use this when supporting multiple providers
                {:else if signerType === "solana"}
                    <div class="px-4 py-3 text-sm text-white text-center">
                        <div>
                            Select Provider Type To Connect For {capitalizeFirstLetter(
                                signerType
                            )}
                        </div>
                    </div>
                    <Button
                        class="w-full bg-dark-1 text-white py-4"
                        onClick={() => {
                            signerType = null;
                        }}
                        text="Change Signer Type"
                    />
                    <hr />
                    {#each providerTypes[signerType] as t}
                        <Button
                            class="w-full bg-dark-1 text-white py-4"
                            onClick={() => {
                                conn(t);
                            }}
                            text={capitalizeFirstLetter(t)}
                        />
                    {/each}
                -->
                {/if}
            </div>
        </DropdownButton>
    </div>
</div>
