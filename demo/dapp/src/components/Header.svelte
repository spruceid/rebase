<script lang="ts">
    import {
        connectNewSigner,
        SignerType,
        signerMap,
        SignerMap,
        Signer,
        currentSigner,
    } from "../util";

    const newOpts: Array<SignerType> = ["ethereum"];
    const currentOpts: Array<SignerType> = ["ethereum"];

    $: newSigner = "";
    $: currentType = "";
    $: currentId = "";
    $: errMsg = "";

    let _signerMap: SignerMap = { ethereum: {} };
    signerMap.subscribe((x) => (_signerMap = x));

    let _currentSigner: [SignerType, Signer] = null;
    currentSigner.subscribe((x) => (_currentSigner = x));

    const connectNew = async (): Promise<void> => {
        try {
            await connectNewSigner(newSigner as SignerType);
        } catch (e) {
            errMsg = `${e?.message ? e.message : e}`;
        }
    };

    const useExisting = async () => {
        let t = _signerMap[currentType];
        if (!t) {
            errMsg = `No signers for ${currentType} found in existing.`
            return;
        }
        let s = t[currentId];
        if (!s) {
            errMsg = `No signers with id ${currentId} found in existing.`
            return;
        }

        currentSigner.set([currentType as SignerType, s]);
    };
</script>

<div class="viewer">
    {#if errMsg}
        <p class="inner-center">
            ERROR: {errMsg}
        </p>
    {/if}
    <div class="inner-center">
        <label for="connect-new">Connect New Signer: </label>
        <select name="connect-new" bind:value={newSigner}>
            {#each newOpts as opt}
                <option value={opt}>{opt}</option>
            {/each}
        </select>
        <button on:click={connectNew}>Connect</button>
    </div>
    <div class="inner-center">
        {#if _currentSigner}
            <h5>
                Currently using {_currentSigner[0]} signer: {_currentSigner[1].id()}
            </h5>
        {:else}
            <h3>No Signer Connected</h3>
        {/if}
    </div>
    <div class="inner-center">
        Use connected <select name="signer-type" bind:value={currentType}>
            {#each currentOpts as opt}
                <option value={opt}>{opt}</option>
            {/each}
        </select>
        signer:
        {#if !_signerMap || !_signerMap[currentType] || Object.keys(_signerMap[currentType]).length <= 0}
            -- No signers connected.
        {:else}
            <select name="signer-id" bind:value={currentId}>
                {#each Object.keys(_signerMap[currentType]) as id}
                    <option value={id}>{id}</option>
                {/each}
            </select>
        {/if}
        {#if currentId && currentType}
            <button name="use-existing-signer" on:click={useExisting}
                >Use This Signer</button
            >
        {/if}
    </div>
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
