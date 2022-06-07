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
    } from "../util";

    $: errMsg = "";
    $: nextType = "ethereum";
    $: showChangeConx = false;

    let signer: Signer | false = false;
    currentType.subscribe((x) => (signer = _signerMap[x]));
    signerMap.subscribe((x) => (signer = x[_currentType]));

    const connectNew = async (): Promise<void> => {
        try {
            currentType.set(nextType as SignerType);
            await connect();
        } catch (e) {
            errMsg = `${e?.message ? e.message : e}`;
        }
    };
</script>

<div class="viewer">
    {#if errMsg}
        <p class="inner-center">
            {errMsg}
        </p>
    {/if}
    <div class="inner-center">
        {#if signer}
            <h5>
                Currently using {_currentType} signer: {signer.id()}
            </h5>
        {:else}
            <h3>No Signer Connected</h3>
        {/if}
    </div>
    {#if signer}
        <div class="inner-center">
            {#if !showChangeConx}
                Change Connection? <button
                    name="show-change-connection"
                    on:click={() => (showChangeConx = true)}>Yes!</button
                >
            {:else}
                Hide Connection Options? <button
                    name="hide-change-connection"
                    on:click={() => (showChangeConx = false)}>Hide</button
                >
            {/if}
        </div>
    {/if}

    {#if signer}
        <div class="inner-center">
            Disconnect Current Signer <button
                name="disconnect-current-signer"
                on:click={disconnect}>Disconnect</button
            >
        </div>
    {/if}

    {#if !signer || showChangeConx}
        <div class="inner-center">
            <label for="signer-type">Select Signer Type To Connect</label>
            <select name="signer-type" bind:value={nextType}>
                {#each signerTypes as t}
                    <option id={t}>{t}</option>
                {/each}
            </select>
            <button name="connect" on:click={connectNew}>Connect</button>
        </div>
    {/if}
</div>

<style>
    .viewer {
        height: 20vh;
        width: 75vh;
        background-color: white;
    }
    .inner-center {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-left: 5vh;
        margin-right: 5vh;
    }
</style>
