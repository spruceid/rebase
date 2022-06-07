<script lang="ts">
    import type {
        CredentialType,
        Instructions,
        Workflow,
        Claim,
    } from "../../util";
    import { Link } from "svelte-navigator";
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
    } from "../../util";
    import { onMount } from "svelte";

    // TODO: Make these an ENV?
    const witnessUrl = "http://localhost:8787";
    const dnsPrefix = "rebase_sig";

    let signer: Signer | false = false;
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

    $: errMsg = "";
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

{#if errMsg}
    <p class="inner-center">{errMsg}</p>
{/if}
{#if signer}
    <h4 class="inner-center">Step 1: Generate a statement</h4>
    <p class="inner-center">{instructions.statement}</p>
    <div class="inner-center">
        <label for={instructions.statement_label} class="inner-center"
            >{instructions.statement_label}</label
        >
        <input
            class="inner-center"
            disabled={state !== "statement"}
            bind:value={handle}
            name={instructions.statement_label}
            type="text"
        />
    </div>
    <div class="inner-center">
        <button
            disabled={state !== "statement"}
            on:click={async () => {
                try {
                    await getStatement();
                    advance();
                } catch (e) {
                    errMsg = e.message;
                }
            }}>Generate Statement</button
        >
    </div>
    {#if state !== "statement"}
        <h4 class="inner-center">Step 2: Sign the statement</h4>
        <p class="inner-center">{instructions.signature}</p>
        <div class="inner-center">
            <textarea
                name="statement_display"
                type="text"
                disabled
                value={statement}
            />
        </div>
        <div class="inner-center">
            <button
                disabled={state !== "signature"}
                on:click={async () => {
                    try {
                        await sign(statement);
                        advance();
                    } catch (e) {
                        errMsg = `${e?.message ? e.message : e}`;
                    }
                }}>Sign Statement</button
            >
        </div>
    {/if}
    {#if state === "witness" || state === "complete"}
        <h4 class="inner-center">Step 3: Show the Witness</h4>
        <p class="inner-center">{instructions.witness}</p>
        <div class="inner-center">
            <label for="post">Post</label>
            <textarea value={post()} name="post" disabled />
        </div>
        {#if type === "twitter" || type === "github" || type === "discord"}
            <div class="inner-center">
                <label for={instructions.witness_label}
                    >{instructions.witness_label}</label
                >
                <input
                    bind:value={proof}
                    name={instructions.witness_label}
                    type="text"
                />
            </div>
        {/if}
        <div class="inner-center">
            <button
                disabled={state !== "witness"}
                on:click={async () => {
                    try {
                        await getCredential();
                        advance();
                    } catch (e) {
                        errMsg = `${e?.message ? e.message : e}`;
                    }
                }}>Generate Credential</button
            >
        </div>
    {/if}
    {#if state === "complete"}
        <div class="inner-center">
            <h4>Step 4: Complete!</h4>
            <p>
                Please return to the <Link to="/account">main page</Link> to download
                your credential
            </p>
        </div>
    {/if}
{:else}
    <div class="inner-center">Connect Signer to Create Credential</div>
{/if}

<style>
    .inner-center {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-left: 5vh;
        margin-right: 5vh;
    }
</style>
