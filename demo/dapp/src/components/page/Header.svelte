<script lang="ts">
    import {
        _currentType,
        _signerMap,
        currentType,
        connect,
        disconnect,
        signerTypes,
        signerMap,
        Signer,
        SignerType,
    } from "util";
    import { Tooltip, Button, DropdownButton, RebaseLogo } from "components";
    import { scale } from "svelte/transition";
    import { Link, useNavigate } from "svelte-navigator";
    import { alert } from "util/store";

    let moreDropdown;
    const navigate = useNavigate();

    let signer: Signer | false = false;
    currentType.subscribe((x) => (signer = _signerMap[x]));
    signerMap.subscribe((x) => (signer = x[_currentType]));

    const connectNew = async (nextType): Promise<void> => {
        try {
            currentType.set(nextType as SignerType);
            await connect();
        } catch (e) {
            alert.set({
                message: e?.message ? e.message : e,
                variant: "error",
            });
        }
    };

    const capitalizeFirstLetter = (string) => {
        return string.charAt(0).toUpperCase() + string.slice(1);
    };
</script>

<div
    class="min-w-screen px-4 h-[70px] w-full flex items-center justify-between bg-white shadow"
>
    <Link to="/">
        <RebaseLogo class="w-fit flex items-center" xl />
    </Link>
    {#if !signer}
        <DropdownButton
            class="menu focus:outline-none focus:shadow-solid w-full min-w-42 my-[16px] rounded-xl border border-gray-250"
            text="Connect"
        >
            <!-- class="origin-top-right absolute right-0 w-48 py-2 mt-1 bg-white rounded-xl shadow-md" -->
            <div
                in:scale={{ duration: 100, start: 0.95 }}
                out:scale={{ duration: 75, start: 0.95 }}
                class="origin-top-right absolute right-4 w-48 py-0 mt-1 bg-dark-1 rounded-xl shadow-md"
            >
                <div class="px-4 py-3 text-sm text-white text-center">
                    <div>Select Signer Type To Connect</div>
                </div>
                <hr />
                {#each signerTypes as t}
                    <Button
                        class="w-full bg-dark-1 text-white"
                        onClick={() => connectNew(t)}
                        text={capitalizeFirstLetter(t)}
                    />
                {/each}
            </div>
        </DropdownButton>
    {:else}
        <div class="flex flex-wrap">
            <Tooltip tooltip="Currently using {_currentType} signer" bottom>
                <Button
                    class="w-full max-w-42 my-[16px] border border-gray-250"
                    onClick={() => navigate("/account")}
                    text={signer.id()}
                />
            </Tooltip>
            <DropdownButton
                bind:this={moreDropdown}
                class="w-[55px] my-[16px] pl-[16px] rounded-xl border border-gray-250"
                ml
                text="&#8226;&#8226;&#8226;"
            >
                <div
                    in:scale={{ duration: 100, start: 0.95 }}
                    out:scale={{ duration: 75, start: 0.95 }}
                    class="origin-top-right absolute right-4 w-48 py-0 mt-1 bg-dark-1 rounded-xl shadow-md"
                >
                    <Button
                        class="w-full bg-dark-1 text-white"
                        onClick={() => {
                            navigate("/");
                            moreDropdown.closeDropdown();
                        }}
                        text="About"
                    />
                    <Button
                        class="w-full bg-dark-1 text-white"
                        onClick={() => {
                            disconnect();
                            moreDropdown.closeDropdown();
                        }}
                        text="Disconnect"
                    />
                </div>
            </DropdownButton>
        </div>
    {/if}
</div>
