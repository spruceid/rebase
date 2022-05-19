<script lang="ts">
    import type {
        CredentialType,
        Instructions,
        Workflow,
        Claim,
    } from "../util";
    import { Link } from "svelte-navigator";
    import { claims, witnessState } from "../util";
    import { onMount } from "svelte";

    // TODO: Handle this elsewhere?
    import { ethers } from "ethers";
    import Web3Modal from "web3modal";

    // TODO: Make this an ENV?
    const witnessUrl = "http://localhost:8787";

    const providerOptions = {
        /* See Provider Options Section */
    };

    const web3Modal = new Web3Modal({
        network: "mainnet",
        cacheProvider: true,
        providerOptions,
    });

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
    $: address = "";
    $: statement = "";
    $: signature = "";
    $: delimitor = "";
    $: provider = null;
    $: signer = null;

    // TODO: Change to statment + proof types.
    $: handle = "";
    $: proof = "";

    const post = (): string => {
        return `${statement}${delimitor}${signature}`;
    };

    let state: Workflow = "statement";
    witnessState.subscribe((x) => (state = x));

    // TODO: Handle as Signer in store.
    const connect = async () => {
        errMsg = "";
        try {
            const instance = await web3Modal.connect();
            provider = new ethers.providers.Web3Provider(instance);
            signer = provider.getSigner();

            if (signer) {
                let addresses = await provider.listAccounts();
                if (addresses.length > 0) {
                    address = addresses[0];
                } else {
                    errMsg = "User Cancelled Connect";
                    return;
                }
            } else {
                errMsg = "No ethereum provider detected";
            }
        } catch (e) {
            errMsg = `${e.message}`;
        }
    };

    const sign = async () => {
        if (!signer) {
            errMsg = "No signer connected";
            return;
        }
        // TODO: Correctly assign this.
        try {
            signature = await signer.signMessage(statement);
            errMsg = "";
        } catch (e) {
            errMsg = `${e.message}`;
        }
    };

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

    interface KeyType {
        pkh: {
            eip115: {
                address: string;
                chain_id: string;
            };
        };
    }

    const getKeyType = (): KeyType => {
        return {
            pkh: {
                eip115: {
                    address,
                    chain_id: "1",
                },
            },
        };
    };

    const getStatement = async (): Promise<void> => {
        // TODO: Fetch from worker.
        let opts = {};
        opts[type] = {};

        switch (type) {
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
            errMsg = "Did not find statement and delimitor in response.";
            return;
        }

        statement = body.statement;
        delimitor = body.delimitor;
    };

    const getCredential = async (): Promise<void> => {
        // TODO: WORK FROM HERE.
        let opts = {};

        switch (type) {
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
        console.log(b);

        let res = await fetch(`${witnessUrl}/witness?type=${type}`, {
            method: "POST",
            headers: new Headers({
                "Content-Type": "application/json",
            }),
            body: b,
        });

        if (!res.ok || res.status !== 200) {
            throw new Error(`failed in getStatement: ${res.statusText}`);
        }

        let credential = await res.json();

        setNew(JSON.stringify(credential))
    };
</script>

<div>
    {#if errMsg}
        <p>{errMsg}</p>
    {/if}
    {#if signer}
        <div>
            <h4>Step 1: Generate a statement</h4>
            <p>{instructions.statement}</p>
            <label for={instructions.statement_label}
                >{instructions.statement_label}</label
            >
            <input
                disabled={state !== "statement"}
                bind:value={handle}
                name={instructions.statement_label}
                type="text"
            />
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
            <div>
                <h4>Step 2: Sign the statement</h4>
                <p>{instructions.signature}</p>
                <!-- TODO: Fill with statement here -->
                <textarea
                    name="statement_display"
                    type="text"
                    disabled
                    value={statement}
                />
                <button
                    disabled={state !== "signature"}
                    on:click={async () => {
                        await sign();
                        advance();
                    }}>Sign Statement</button
                >
            </div>
        {/if}
        {#if state === "witness" || state === "complete"}
            <div>
                <h4>Step 3: Show the witness</h4>
                <p>{instructions.witness}</p>
                <label for="post">Post</label>
                <textarea value={post()} name="post" disabled />
                <label for={instructions.witness_label}
                    >{instructions.witness_label}</label
                >
                <input
                    bind:value={proof}
                    name={instructions.witness_label}
                    type="text"
                />
                <button
                    disabled={state !== "witness"}
                    on:click={async () => {
                        await getCredential();
                        advance();
                    }}>Generate Credential</button
                >
            </div>
        {/if}
        {#if state === "complete"}
            <div>
                <h4>Step 4: Complete!</h4>
                <p>
                    Please return to the <Link to="/account">main page</Link> to
                    download your credential
                </p>
            </div>
        {/if}
    {:else}
        <div>Connect Signer to Create Credential</div>
        <button on:click={connect}>Connect Provider</button>
    {/if}
</div>
