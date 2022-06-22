<script lang="ts">
    import {
        _currentType,
        _signerMap,
        currentType,
        signerMap,
        signer,
        claims,
        getKeyType,
        sign,
        Claim,
        KeyType,
        Signer,
    } from "../../util";
    import { Link } from "svelte-navigator";

    $: errMsg = "";
    $: display1 = "";
    $: display2 = "";
    $: statement = "";
    $: signature1 = "";
    $: signature2 = "";

    let key1: KeyType | false = false;
    let key2: KeyType | false = false;

    let c: Array<Claim> = [];
    claims.subscribe((x) => (c = x));

    const witnessUrl = "https://rebasedemo.spruceid.workers.dev";

    const getKey1 = () => {
        key1 = getKeyType();
        if (signer) {
            display1 = `First signer is ${_currentType} signer: ${signer.id()}`;
        }
    };

    const getKey2 = () => {
        key2 = getKeyType();

        if (JSON.stringify(key1) === JSON.stringify(key2)) {
            throw new Error("Cannot use same signer for both entries");
        }

        if (signer) {
            display2 = `Second signer is ${_currentType} signer: ${signer.id()}`;
        }
    };

    const getStatement = async (): Promise<void> => {
        if (!key1 || !key2) {
            throw new Error(`Need two keys set to use cross signed credential`);
        }

        let res = await fetch(`${witnessUrl}/statement`, {
            method: "POST",
            headers: new Headers({
                "Content-Type": "application/json",
            }),
            body: JSON.stringify({
                opts: {
                    self_signed: {
                        key_1: key1,
                        key_2: key2,
                    },
                },
            }),
        });

        if (!res.ok || res.status !== 200) {
            throw new Error(`failed in getStatement: ${res.statusText}`);
        }
        let body = await res.json();

        if (!body.statement) {
            throw new Error("Did not find statement in response.");
        }

        statement = body.statement;
    };

    const signKey2 = async () => {
        if (JSON.stringify(key2) !== JSON.stringify(getKeyType())) {
            throw new Error(`Signer connected is not expected Signer`);
        }

        signature2 = await sign(statement);
    };

    const signKey1 = async () => {
        if (JSON.stringify(key1) !== JSON.stringify(getKeyType())) {
            throw new Error(`Signer connected is not expected Signer`);
        }

        signature1 = await sign(statement);
    };

    const setNew = (credential: string) => {
        let next: Array<Claim> = [];
        for (let i = 0, n = c.length; i < n; i++) {
            let claim = c[i];
            if (claim.credential_type === "self_signed") {
                claim.credentials.push(credential);
            }
            next.push(claim);
        }

        claims.set(next);
    };

    const getCredential = async (): Promise<void> => {
        if (!key1 || !key2 || !signature1 || !signature2) {
            throw new Error(
                "Needs two keys, a statement, and two signatures to create credential"
            );
        }

        const proof = {
            self_signed: {
                statement_opts: {
                    key_1: key1,
                    key_2: key2,
                },
                signature_1: signature1,
                signature_2: signature2,
            },
        };

        let b = JSON.stringify({ proof });
        let res = await fetch(`${witnessUrl}/witness?type=self_signed`, {
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

    type Workflow = "key1" | "key2" | "sig2" | "sig1" | "complete";
    $: current = "key1";

    const advance = () => {
        switch (current as Workflow) {
            case "key1":
                current = "key2";
                return;
            case "key2":
                current = "sig2";
                return;
            case "sig2":
                current = "sig1";
                return;
            case "sig1":
            case "complete":
                current = "complete";
                return;
        }

        throw new Error(`Unknown workflow state: ${current}`);
    };
</script>

{#if errMsg}
    <p class="inner-center">{errMsg}</p>
{/if}

<p class="inner-center">{display1 ? display1 : "No first signer set"}</p>
<p class="inner-center">{display2 ? display2 : "No second signer set"}</p>

<h4 class="inner-center">Step 1: Connect First Key</h4>
<p class="inner-center">
    Using the connection controls in the header, select the first of two signers
    you would like to link.
</p>
<div class="inner-center">
    <label for="connect-key-1"> Is the first signer connected? </label>
    <button
        name="connect-key-1"
        disabled={current !== "key1"}
        on:click={() => {
            try {
                getKey1();
                advance();
            } catch (e) {
                errMsg = `${e?.message ? e.message : e}`;
            }
        }}>Confirm</button
    >
</div>
{#if current !== "key1"}
    <h4 class="inner-center">Step 2: Connect Second Key</h4>
    <p class="inner-center">
        Using the connection controls in the header, select the second
        (DIFFERENT than the key used in the last step) of two signers you would
        like to link.
    </p>
    <div class="inner-center">
        <label for="connect-key-2"> Is the second signer connected? </label>
        <button
            name="connect-key-2"
            disabled={current !== "key2"}
            on:click={async () => {
                try {
                    getKey2();
                    await getStatement();
                    advance();
                } catch (e) {
                    errMsg = `${e?.message ? e.message : e}`;
                }
            }}>Confirm</button
        >
    </div>
{/if}
{#if current !== "key1" && current !== "key2"}
    <h4 class="inner-center">Step 3: Sign with the Second Key</h4>
    <p class="inner-center">
        Keeping the same signer as was connected in the last step, sign the
        statement.
    </p>
    <div class="inner-center">
        <button
            name="connect-key-1"
            disabled={current !== "sig2"}
            on:click={async () => {
                try {
                    await signKey2();
                    advance();
                } catch (e) {
                    errMsg = `${e?.message ? e.message : e}`;
                }
            }}>Sign with Second Key</button
        >
    </div>
{/if}
{#if current !== "key1" && current !== "key2" && current !== "sig2"}
    <h4 class="inner-center">Step 4: Sign with First Key</h4>
    <p class="inner-center">
        Using the controls in the header, reconnect the signer that was
        connected for the first step, then sign the statement. The signatures
        will then be used to generate a credential.
    </p>
    <div class="inner-center">
        <button
            name="connect-key-1"
            disabled={current !== "sig1"}
            on:click={async () => {
                try {
                    await signKey1();
                    await getCredential();
                    advance();
                } catch (e) {
                    errMsg = `${e?.message ? e.message : e}`;
                }
            }}>Sign with First Key</button
        >
    </div>
{/if}
{#if current === "complete"}
    <div class="inner-center">
        <h4>Step 4: Complete!</h4>
        <p>
            Please return to the <Link to="/account">main page</Link> to download
            your credential
        </p>
    </div>
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
