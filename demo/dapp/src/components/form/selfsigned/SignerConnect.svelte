<script lang="ts">
    import { onMount } from "svelte";
    import { connectSignerType, signerTypes, SignerQuery } from "src/util";
    import { Button, SpinnerIcon } from "src/components";
    import { scale } from "svelte/transition";

    import "../../buttons/button.scss";
    // export let signerName: string;

    export let signerCallback: (newSigner: SignerQuery) => void;
    let clazz: string = "";
    export { clazz as class };
    export let primary: boolean = false;
    export let secondary: boolean = false;
    export let title: string = "";
    export let disabled: boolean = false;
    export let small: boolean = false;
    export let ml: boolean = false;
    export let loading: boolean = false;
    const capitalizeFirstLetter = (string) => {
        return string.charAt(0).toUpperCase() + string.slice(1);
    };

    let showDropdown = false;
    let dropdownRef = null;

    export const closeDropdown = () => (showDropdown = false);

    onMount(() => {
        const handleESC = (e) => {
            if (showDropdown && e.key === "Escape") {
                showDropdown = false;
            }
        };

        document.addEventListener("keyup", handleESC, false);

        return () => {
            document.removeEventListener("keyup", handleESC, false);
        };
    });
</script>

<div class="" class:ml-1={ml} bind:this={dropdownRef}>
    <button
        {disabled}
        class={`${clazz} button-container text-ellipsis overflow-hidden `}
        class:py-4={!small}
        class:py-3={small}
        class:opacity-50={disabled}
        class:cursor-not-allowed={disabled}
        class:primary-button-container={primary}
        class:secondary-button-container={secondary}
        on:click|preventDefault={() => (showDropdown = !showDropdown)}
        aria-label={title}
        {title}
    >
        {#if loading}
            <div class="flex flex-wrap items-center justify-between">
                <SpinnerIcon class="w-6 h-6 mr-2 animate-spin" />
                <slot />
                <div>⌄</div>
            </div>
        {:else if showDropdown}
            <div class="flex flex-wrap items-center justify-between">
                <div>Close</div>
                <div>⌃</div>
            </div>
        {:else}
            <div class="flex flex-wrap items-center justify-between">
                <slot />
                <div>⌄</div>
            </div>
        {/if}
    </button>
    {#if showDropdown}
        <div
            in:scale={{ duration: 100, start: 0.95 }}
            out:scale={{ duration: 75, start: 0.95 }}
            class="w-full bg-dark-1 rounded-xl shadow-md"
        >
            {#each signerTypes as t}
                <Button
                    class="w-full bg-dark-1 text-white py-4"
                    onClick={async () => {
                        loading = true;
                        // TODO: Add provider type.
                        let signer = await connectSignerType(
                            t,
                            t === "ethereum" ? "metamask" : "phantom"
                        );
                        loading = false;
                        return signerCallback(signer);
                    }}
                    text={capitalizeFirstLetter(t)}
                />
            {/each}
        </div>
    {/if}
</div>

<!--
<div>
    <DropdownButton
        class="menu w-full min-w-42 mt-[16px] rounded-xl"
        text={`Set ${signerName ? signerName : "Signer"}`}
        primary
        {loading}
    >

    </DropdownButton>
</div>
 -->
