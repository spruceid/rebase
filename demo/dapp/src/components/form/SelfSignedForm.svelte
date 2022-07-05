<script lang="ts">
    import {
        signerMap,
        signerMap2nd,
        signer,
        signer2nd,
        disconnect2nd,
        claims,
        getKeyType,
        getKeyType2nd,
        sign,
        sign2nd,
        Claim,
        KeyType,
        alert,
    } from "utils";
    import { useNavigate } from "svelte-navigator";
    import {
        WitnessFormHeader,
        WitnessFormStepper,
        Button,
        GlobeIcon,
    } from "components";
    import WitnessFormComplete from "./WitnessFormComplete.svelte";
    import { onMount } from "svelte";
    import { ConnectSignerButton } from "components";
    import Connect2ndSignerButton from "../buttons/Connect2ndSignerButton.svelte";

    const navigate = useNavigate();

    $: $signerMap, signerChanged();
    $: $signerMap2nd, signerChanged2nd();

    $: display1 = "";
    $: display2 = "";
    $: statement = "";
    $: signature1 = "";
    $: signature2 = "";

    let key1: KeyType | false = false;
    let key2: KeyType | false = false;
    let loading: boolean = false;

    let c: Array<Claim> = [];
    claims.subscribe((x) => (c = x));

    const witnessUrl = process.env.WITNESS_URL;

    const getKey1 = () => {
        key1 = getKeyType();
        key2 = false;
        if (signer) {
            display1 = `${signer.id()}`;
        }
    };

    const getKey2 = async () => {
        key2 = getKeyType2nd();

        if (JSON.stringify(key1) === JSON.stringify(key2)) {
            key2 = false;
            if (signer.provider.connection.url === "metamask") {
                throw new Error(
                    "Cannot use same signer for both entries. Please change accounts if you want to proceed with MetaMask."
                );
            } else {
                throw new Error("Cannot use same signer for both entries.");
            }
        } else if (signer2nd) {
            display2 = `${signer2nd.id()}`;
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
        if (JSON.stringify(key2) !== JSON.stringify(getKeyType2nd())) {
            throw new Error(`Signer connected is not expected Signer`);
        }

        signature2 = await sign2nd(statement);
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
        disconnect2nd();
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
    };
    const back = () => {
        switch (current as Workflow) {
            case "key2":
                current = "key1";
                return;
            case "sig2":
                current = "key2";
                return;
            case "sig1":
                current = "sig2";
                return;
            case "complete":
                current = "sig1";
                return;
        }
    };

    const signerChanged = () => {
        const connectSignerMessageElem = document.querySelector(
            '[for="form-step-q-1-i-1"] span'
        );
        if (signer) {
            getKey1();
            if (connectSignerMessageElem) {
                connectSignerMessageElem.innerHTML =
                    "We've identified that you already have a signer connected";
            }
        } else {
            key1 = false;
            key2 = false;
            display1 = "none";
            display2 = "none";
            disconnect2nd();
            if (connectSignerMessageElem) {
                connectSignerMessageElem.innerHTML =
                    "Click the button to connect the first of two signers you would like to link";
            }
            current = "key1";
        }
    };

    const signerChanged2nd = () => {
        if (signer && !signer2nd && current !== "sig1") {
            key2 = false;
            display2 = "none";
            current = "key2";
        }
    };

    onMount(() => {
        if (signer) {
            getKey1();
        }
    });
</script>

<WitnessFormHeader
    icon={GlobeIcon}
    title={"Ethereum Account Verification Workflow"}
    subtitle={`First signer: ${display1 ? display1 : "none"}`}
    subsubtitle={`Second signer: ${display2 ? display2 : "none"}`}
/>
{#if current === "key1"}
    <WitnessFormStepper
        step={1}
        totalSteps={5}
        label={"Connect First Key"}
        question={"Click the button to connect the first of two signers you would like to link"}
        labelFor={"form-step-q-1-i-1"}
    >
        <ConnectSignerButton
            class="menu w-full max-w-52.5 mt-[16px] rounded-xl"
            text="Connect First"
            disabled={current !== "key1" || key1 !== false}
            action
        />
    </WitnessFormStepper>
    <div class="w-full my-[16px] text-center">
        <Button
            class="w-2/5"
            onClick={() => navigate("/account")}
            text="Back"
            primary
        />
        <Button
            class="w-2/5"
            disabled={current !== "key1" || !key1}
            onClick={() => {
                try {
                    advance();
                } catch (e) {
                    alert.set({
                        variant: "error",
                        message: e?.message ? e.message : e,
                    });
                }
            }}
            text="Next"
            action
        />
    </div>
{/if}
{#if current === "key2"}
    <WitnessFormStepper
        step={2}
        totalSteps={5}
        label={"Connect Second Key"}
        question={"Click the button to connect the second of two signers you would like to link"}
        labelFor={"form-step-q-2-i-1"}
    >
        <div id="form-step-q-2-i-1">
            <Connect2ndSignerButton
                class="menu w-full max-w-52.5 mt-[16px] rounded-xl"
                text="Connect Second"
                disabled={current !== "key2" || key2 !== false}
                cb={async () => {
                    try {
                        loading = true;
                        await getKey2();
                        await getStatement();
                    } catch (e) {
                        alert.set({
                            variant: "error",
                            message: e?.message ? e.message : e,
                        });
                    }
                    loading = false;
                }}
                action
            />
        </div></WitnessFormStepper
    >
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
            disabled={current !== "key2" || !key2 || loading}
            onClick={() => {
                try {
                    advance();
                } catch (e) {
                    alert.set({
                        variant: "error",
                        message: e?.message ? e.message : e,
                    });
                }
            }}
            text="Next"
            action
        />
    </div>
{/if}
{#if current === "sig2"}
    <WitnessFormStepper
        step={3}
        totalSteps={5}
        label={"Sign with the Second Key"}
        question={"Sign the statement with the second signer"}
        labelFor={"form-step-q-3-i-1"}
    >
        <div id="form-step-q-3-i-1">
            <Button
                {loading}
                class="w-2/5 mt-[16px]"
                disabled={current !== "sig2" || signature2 !== ""}
                onClick={async () => {
                    try {
                        loading = true;
                        await signKey2();
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
            disabled={current !== "sig2" || signature2 === "" || loading}
            onClick={() => {
                try {
                    advance();
                } catch (e) {
                    alert.set({
                        variant: "error",
                        message: e?.message ? e.message : e,
                    });
                }
            }}
            text="Next"
            action
        />
    </div>
{/if}
{#if current === "sig1"}
    <WitnessFormStepper
        step={4}
        totalSteps={5}
        label={"Sign with First Key"}
        question={"Sign the statement with the first signer. The signatures will then be used to generate a credential"}
        labelFor={"form-step-q-4-i-1"}
    >
        <div id="form-step-q-4-i-1">
            <Button
                {loading}
                class="w-2/5 mt-[16px]"
                disabled={current !== "sig1" || signature1 !== ""}
                onClick={async () => {
                    try {
                        loading = true;
                        await signKey1();
                        await getCredential();
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
            disabled={current !== "sig1" || signature1 === "" || loading}
            onClick={() => {
                try {
                    advance();
                } catch (e) {
                    alert.set({
                        variant: "error",
                        message: e?.message ? e.message : e,
                    });
                }
            }}
            text="Next"
            action
        />
    </div>
{/if}
{#if current === "complete"}
    <WitnessFormComplete step={5} totalSteps={5} {navigate} />
{/if}
