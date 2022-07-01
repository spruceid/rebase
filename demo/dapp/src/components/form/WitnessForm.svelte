<script lang="ts">
    import type { CredentialType, Instructions, Workflow, Claim } from "utils";
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
    } from "utils";
    import { onMount } from "svelte";
    import { useNavigate } from "svelte-navigator";
    import { WitnessFormHeader, ConnectSignerButton } from "components";
    import WitnessFormStatement from "./WitnessFormStatement.svelte";
    import WitnessFormSignature from "./WitnessFormSignature.svelte";
    import WitnessFormWitness from "./WitnessFormWitness.svelte";
    import WitnessFormComplete from "./WitnessFormComplete.svelte";

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
    $: handle = "";
    $: proof = "";

    const onChangeValue = (name, value) => {
        switch (name) {
            case "handle":
                handle = value;
                break;
            case "signature":
                signature = value;
                break;
            case "proof":
                proof = value;
                break;
            case "verified":
                verified = value;
                break;
        }
    };

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
                opts["github"]["gist_id"] = proof.split("/").pop();
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
        <WitnessFormStatement
            {instructions}
            {loading}
            {handle}
            {onChangeValue}
            {navigate}
            {getStatement}
            {advance}
        />
    {/if}
    {#if state === "signature"}
        <WitnessFormSignature
            {instructions}
            {loading}
            {statement}
            {signature}
            {onChangeValue}
            {sign}
            {back}
            {advance}
        />
    {/if}
    {#if state === "witness"}
        <WitnessFormWitness
            {instructions}
            {loading}
            {verified}
            {type}
            {proof}
            {onChangeValue}
            {getCredential}
            {post}
            {back}
            {advance}
        />
    {/if}
    {#if state === "complete"}
        <WitnessFormComplete {navigate} />
    {/if}
{:else}
    <div class="w-full text-center">
        Please connect your wallet
        <ConnectSignerButton
            class="menu w-full max-w-52.5 my-[16px] rounded-xl"
            text="Connect Wallet"
            action
        />
    </div>
{/if}
