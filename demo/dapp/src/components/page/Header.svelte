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
    import { useNavigate } from "svelte-navigator";
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
</script>

<div
    class="min-w-screen px-4 h-[70px] w-full flex items-center justify-between bg"
>
    <RebaseLogo class="w-fit flex items-center" />
    {#if !signer}
        <DropdownButton
            class="menu focus:outline-none focus:shadow-solid w-full min-w-42 my-[16px] rounded-xl"
            text="Connect"
        >
            <div
                in:scale={{ duration: 100, start: 0.95 }}
                out:scale={{ duration: 75, start: 0.95 }}
                class="origin-top-right absolute right-0 w-48 py-2 mt-1 bg-white rounded shadow-md"
            >
                <div class="px-4 py-3 text-sm text-gray-900 dark:text-white">
                    <div>Select Signer Type To Connect</div>
                </div>
                <hr />
                {#each signerTypes as t}
                    <Button
                        class="w-full my-[4px]"
                        onClick={() => connectNew(t)}
                        text={t}
                    />
                {/each}
            </div>
        </DropdownButton>
    {:else}
        <div class="flex flex-wrap">
            <Tooltip tooltip="Currently using {_currentType} signer" bottom>
                <Button
                    class="w-full max-w-42 my-[16px]"
                    onClick={() => navigate("/account")}
                    text={signer.id()}
                />
            </Tooltip>
            <DropdownButton
                bind:this={moreDropdown}
                class="w-[50px] my-[16px] pl-[16px] rounded-xl"
                ml
                text="&#8226;&#8226;&#8226;"
            >
                <div
                    in:scale={{ duration: 100, start: 0.95 }}
                    out:scale={{ duration: 75, start: 0.95 }}
                    class="origin-top-right absolute right-0 w-48 py-2 mt-1 bg-white rounded shadow-md"
                >
                    <Button
                        class="w-full my-[4px]"
                        onClick={() => {
                            navigate("/");
                            moreDropdown.closeDropdown();
                        }}
                        text="About"
                    />
                    <Button
                        class="w-full my-[4px]"
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
