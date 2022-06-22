<script lang="ts">
    import type { CredentialType, Instructions, Workflow, Claim } from "util";
    import {
        _currentType,
        _signerMap,
        currentType,
        signerMap,
        claims,
        getKeyType,
        witnessState,
        sign,
        Signer,
        alert,
    } from "../../util";
    import { onMount } from "svelte";
    import WitnessFormHeader from "./WitnessFormHeader.svelte";
    import WitnessFormStepper from "./WitnessFormStepper.svelte";
    import { Button } from "components";
    import { useNavigate } from "svelte-navigator";
    import CopyTextArea from "../form/CopyTextArea.svelte";

    const navigate = useNavigate();

    const witnessUrl = process.env.WITNESS_URL;

    const dnsPrefix = "rebase_sig";

    let signer: Signer | false = false;
    let verified: boolean = false;
    let loading: boolean = false;
    currentType.subscribe((x) => (signer = _signerMap[x]));
    signerMap.subscribe((x) => (signer = x[_currentType]));

    let c: Array<Claim> = [];
    claims.subscribe((x) => (c = x));

    const setNew = (credential: string) => {
        let next: Array<Claim> = [];
        for (let i = 0, n = c.length; i < n; i++) {
            let claim = c[i];
            if (claim.credential_type === type) {
                claim.credentials.push(credential);
            }
            next.push(claim);
        }

        claims.set(next);
    };

    export let type: CredentialType;
    export let instructions: Instructions;

    $: statement = "";
    $: signature = "";
    $: delimitor = "";

    // TODO: Change to statment + proof types.
    $: handle = "";
    $: proof = "";

    const post = (): string => {
        switch (type) {
            case "discord":
            case "github":
            case "twitter":
                return `${statement}${delimitor}${signature}`;
            case "dns":
                return `${dnsPrefix}${delimitor}${signature}`;
        }
    };

    let state: Workflow = "statement";
    witnessState.subscribe((x) => (state = x));

    onMount(async () => {
        witnessState.set("statement");
    });

    const advance = () => {
        switch (state) {
            case "statement":
                return witnessState.set("signature");
            case "signature":
                return witnessState.set("witness");
            case "witness":
                return witnessState.set("complete");
            case "complete":
                return;
        }
    };

    const back = () => {
        switch (state) {
            case "signature":
                return witnessState.set("statement");
            case "witness":
                return witnessState.set("signature");
            case "complete":
                return witnessState.set("witness");
        }
    };

    const getStatement = async (): Promise<void> => {
        let opts = {};
        opts[type] = {};

        switch (type) {
            case "dns":
                opts[type]["domain"] = handle;
                opts[type]["prefix"] = dnsPrefix;
                opts[type]["key_type"] = getKeyType();
                break;
            case "github":
            case "twitter":
                opts[type]["handle"] = handle;
                opts[type]["key_type"] = getKeyType();
                break;
            default:
                throw new Error(`${type} flow is currently unsupported`);
        }

        let res = await fetch(`${witnessUrl}/statement`, {
            method: "POST",
            headers: new Headers({
                "Content-Type": "application/json",
            }),
            body: JSON.stringify({
                opts,
            }),
        });

        if (!res.ok || res.status !== 200) {
            throw new Error(`failed in getStatement: ${res.statusText}`);
        }

        let body = await res.json();
        if (!body.statement || !body.delimitor) {
            throw new Error(
                "Did not find statement and delimitor in response."
            );
        }

        statement = body.statement;
        delimitor = body.delimitor;
    };

    const getCredential = async (): Promise<void> => {
        let opts = {};

        switch (type) {
            case "dns":
                opts["dns"] = {};
                opts["dns"]["domain"] = handle;
                opts["dns"]["prefix"] = dnsPrefix;
                opts["dns"]["key_type"] = getKeyType();
                break;
            case "github":
                opts["github"] = {};
                opts["github"]["statement_opts"] = {};
                opts["github"]["statement_opts"]["handle"] = handle;
                opts["github"]["statement_opts"]["key_type"] = getKeyType();
                opts["github"]["gist_id"] = proof;
                break;
            case "twitter":
                opts["twitter"] = {};
                opts["twitter"]["statement_opts"] = {};
                opts["twitter"]["statement_opts"]["handle"] = handle;
                opts["twitter"]["statement_opts"]["key_type"] = getKeyType();
                opts["twitter"]["tweet_url"] = proof.split("?")[0];
                break;
            default:
                throw new Error(`${type} flow is currently unsupported`);
        }

        let b = JSON.stringify({ proof: opts });
        let res = await fetch(`${witnessUrl}/witness?type=${type}`, {
            method: "POST",
            headers: new Headers({
                "Content-Type": "application/json",
            }),
            body: b,
        });

        if (!res.ok || res.status !== 200) {
            throw new Error(`failed in getCredential: ${res.statusText}`);
        }

        let { jwt } = await res.json();

        setNew(jwt);
    };
</script>

<WitnessFormHeader
    icon={instructions.icon}
    title={instructions.title}
    subtitle={instructions.subtitle}
/>
{#if signer}
    {#if state === "statement"}
        <WitnessFormStepper
            step={1}
            totalSteps={4}
            label={instructions.statement_label}
            question={instructions.statement}
            labelFor={"form-step-q-1-i-1"}
        >
            <input
                class="form-text-input"
                placeholder={instructions.statement_placeholder}
                disabled={state !== "statement"}
                bind:value={handle}
                name={"form-step-q-1-i-1"}
                type="text"
            />
        </WitnessFormStepper>
        <div
            class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center"
        >
            <Button
                class="w-2/5"
                onClick={() => navigate("/account")}
                text="Back"
                primary
                disabled={loading}
            />
            <Button
                {loading}
                class="w-2/5"
                disabled={state !== "statement" || handle.length === 0}
                onClick={async () => {
                    try {
                        loading = true;
                        await getStatement();
                        advance();
                    } catch (e) {
                        alert.set({
                            variant: "error",
                            message: e.message,
                        });
                    }
                    loading = false;
                }}
                text="Next"
                action
            />
        </div>
    {/if}
    {#if state === "signature"}
        <WitnessFormStepper
            step={2}
            totalSteps={4}
            label={instructions.signature_label}
            question={instructions.signature}
            labelFor={"form-step-q-2-i-1"}
        >
            <div id="form-step-q-2-i-1">
                <Button
                    {loading}
                    class="w-2/5 mt-[16px]"
                    disabled={state !== "signature" || signature !== ""}
                    onClick={async () => {
                        try {
                            loading = true;
                            signature = await sign(statement);
                        } catch (e) {
                            alert.set({
                                variant: "error",
                                message: e?.message ? e.message : e,
                            });
                        }
                        loading = false;
                    }}
                    text="Sign"
                    action
                />
            </div>
        </WitnessFormStepper>
        <div class="w-full my-[16px] text-center">
            <Button
                class="w-2/5"
                onClick={back}
                text="Back"
                primary
                disabled={loading}
            />
            <Button
                class="w-2/5"
                disabled={signature === "" || loading}
                onClick={advance}
                text="Next"
                action
            />
        </div>
    {/if}
    {#if state === "witness"}
        <WitnessFormStepper
            step={3}
            totalSteps={4}
            label={instructions.witness_label}
            question={instructions.witness}
            labelFor={"form-step-q-3-i-1"}
        >
            <div id="form-step-q-3-i-1">
                <CopyTextArea value={post()} />
                {#if type === "twitter" || type === "github" || type === "discord"}
                    <div class="w-full">
                        <input
                            class="form-text-input w-full"
                            placeholder={instructions.witness_placeholder}
                            bind:value={proof}
                            name={"form-step-q-3-i-1"}
                            type="text"
                        />
                    </div>
                {/if}
                <Button
                    {loading}
                    class="w-full"
                    disabled={state !== "witness" || verified}
                    onClick={async () => {
                        try {
                            loading = true;
                            await getCredential();
                            verified = true;
                        } catch (e) {
                            alert.set({
                                variant: "error",
                                message: e?.message ? e.message : e,
                            });
                        }
                        loading = false;
                    }}
                    text="Verify"
                    action
                />
            </div>
        </WitnessFormStepper>
        <div class="w-full my-[16px] text-center">
            <Button
                class="w-2/5"
                onClick={back}
                text="Back"
                primary
                disabled={loading}
            />
            <Button
                class="w-2/5"
                disabled={!verified || loading}
                onClick={advance}
                text="Complete"
                action
            />
        </div>
    {/if}
    {#if state === "complete"}
        <WitnessFormStepper
            step={4}
            totalSteps={4}
            label={"Complete"}
            question={"Please click the button to manage your credentials"}
            labelFor={""}
        />
        <div class="w-full my-[16px] text-center">
            <Button
                class="w-fit  my-[16px]"
                onClick={() => navigate("/account")}
                text="Manage Credentials"
                action
            />
        </div>
    {/if}
{:else}
    <div class="w-full text-center">
        <b>No Signer connected</b><br />
        Please, connect a Signer to Create Credentials
    </div>
{/if}

<style>
    .form-text-input {
        @apply w-full bg-gray-100 rounded-md py-2 px-2 my-4;
    }
    /* .form-text-input-action {
        @apply bg-gray-100 rounded-md py-4 px-2 my-4;
    }
    .form-text-input-action::placeholder, */
    .form-text-input::placeholder {
        @apply font-bold text-gray-350;
    }
</style>
