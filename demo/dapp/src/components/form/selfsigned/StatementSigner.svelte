<script lang="ts">
    import {
        alert,
        SignerQuery,
        signerMap,
        SignerEntry,
        retrieveSignerEntry,
        displaySignerId,
    } from "src/util";

    import { Button } from "src/components";

    export let lookUp: SignerQuery;
    export let sigSetter: (val: string) => void;
    export let statement: string;
    let entry: SignerEntry | false = false;

    signerMap.subscribe((x) => {
        entry = retrieveSignerEntry(x, lookUp);
    });

    async function sign() {
        if (!entry) {
            throw new Error(`No entry found for signer ${lookUp.id}`);
        } else if (!entry.active) {
            throw new Error(
                `Entry ${lookUp.signerType}-${lookUp.providerType}: ${lookUp.id} not actively connected`
            );
        } else {
            sigSetter(await entry.signer.sign(statement));
        }
    }
</script>

<Button
    disabled={!entry || !entry.active}
    onClick={async () => {
        try {
            await sign();
        } catch (e) {
            alert.set({
                message: "Failed to get signature",
                variant: "error",
            });
        }
    }}
    text={`Sign with ${
        entry ? displaySignerId(entry.signer) : "Current Signer"
    }`}
    action
/>
