<script lang="ts">
    import {
        alert,
        client,
        claims,
        Claim,
        compareQueries,
        displaySignerId,
        displaySignerType,
        lookUp,
        Signer,
        retrieveSignerEntry,
        signerMap,
        SignerMap,
        getSubject,
        disconnect,
        toQuery,
        signerMapAppend,
    } from "src/util";
    import {
        Button,
        KeyIcon,
        StatementSigner,
        WitnessFormHeader,
        WitnessFormStepper,
    } from "src/components";
    import SignerConnect from "./selfsigned/SignerConnect.svelte";
    import WitnessFormComplete from "./WitnessFormComplete.svelte";
    import { onDestroy, onMount } from "svelte";
    import { Writable, writable } from "svelte/store";
    import { useNavigate } from "svelte-navigator";
    import Ajv from "ajv";

    let _lookUp = null;
    lookUp.subscribe((x) => (_lookUp = x));
    let _signerMap: SignerMap = null;
    signerMap.subscribe((x) => (_signerMap = x));
    let _claims: Array<Claim> = [];
    claims.subscribe((x) => (_claims = x));

    const navigate = useNavigate();

    $: lookUp1 = null;
    $: lookUp2 = null;
    $: key1 = null;
    $: key2 = null;
    $: statement = "";
    $: sig1 = "";
    $: sig2 = "";

    let statement_schema = null,
        witness_schema = null;

    const ajv = new Ajv();

    onMount(async () => {
        state.set("connect");
        if (_lookUp) {
            lookUp1 = _lookUp;
        }

        let res = await client.instructions(JSON.stringify({ type: "same" }));
        let instruction_res = JSON.parse(res);
        statement_schema = instruction_res?.statement_schema;
        witness_schema = instruction_res?.witness_schema;
    });

    onDestroy(() => {
        state.set("connect");
        if (lookUp2) {
            disconnect(lookUp2);
        }
        lookUp1 = null;
        lookUp2 = null;
        key1 = null;
        key2 = null;
        statement = "";
        sig1 = "";
        sig2 = "";
    });

    type State = "connect" | "sign" | "witness";
    let state: Writable<State> = writable("connect");
    let _state: State = "connect";
    state.subscribe((x) => {
        _state = x;
    });

    function advance(current: State): State {
        switch (current) {
            case "connect":
                return "sign";
            case "sign":
                return "witness";
            default:
                return current;
        }
    }

    async function getStatement(): Promise<void> {
        if (!lookUp1 || !lookUp2) {
            throw new Error(`Need both signers set to generate statement`);
        }

        let s1 = retrieveSignerEntry(_signerMap, lookUp1);
        let s2 = retrieveSignerEntry(_signerMap, lookUp2);

        if (!s1 || !s1?.signer || !s2 || !s2?.signer) {
            throw new Error(
                `Need have record of both signers to generate statement`
            );
        }

        key1 = getSubject(s1.signer);
        key2 = getSubject(s2.signer);

        let o = {
            opts: {
                same: {
                    id1: key1,
                    id2: key2,
                },
            },
        };

        if (!statement_schema) {
            throw new Error("No JSON Schema found for Statement Request");
        }

        if (!ajv.validate(statement_schema, o.opts.same)) {
            throw new Error("Validation of Statement Request failed");
        }

        const noStatementErr = "Did not find statement in response";
        try {
            let b = JSON.stringify(o);
            let res = await client.statement(b);
            let body = JSON.parse(res);
            if (!body.statement) {
                throw new Error(noStatementErr);
            }

            statement = body.statement;
        } catch (e) {
            if (e.message === noStatementErr) {
                throw new Error(e.message);
            } else {
                throw new Error(
                    "Failed to generate statement, please retry the flow from the start"
                );
            }
        }
    }

    const setNew = (credential: string) => {
        let next: Array<Claim> = [];
        for (let i = 0, n = _claims.length; i < n; i++) {
            let claim = _claims[i];
            if (claim.credential_type === "same") {
                claim.credentials.push(credential);
            }
            next.push(claim);
        }

        claims.set(next);
    };

    const getCredential = async (): Promise<void> => {
        if (!key1 || !key2 || !sig1 || !sig2) {
            throw new Error(
                "Needs two keys, a statement, and two signatures to create credential"
            );
        }

        const proof = {
            same: {
                statement: {
                    id1: key1,
                    id2: key2,
                },
                signature1: sig1,
                signature2: sig2,
            },
        };

        if (!witness_schema) {
            throw new Error("No JSON Schema found for Witness Request");
        }

        if (!ajv.validate(witness_schema, proof.same)) {
            throw new Error("Validation of Witness Request failed");
        }

        try {
            let b = JSON.stringify({ proof });
            let res = await client.jwt(b);

            let { jwt } = JSON.parse(res);

            setNew(jwt);
        } catch (e) {
            throw new Error(
                "Failed to generate credential, please retry the flow from the start"
            );
        }
    };

    function main_handler(signer: Signer) {
        if (_lookUp && compareQueries(_lookUp, signer)) {
            return;
        }

        lookUp1 = signer;

        if (lookUp2 && compareQueries(lookUp1, lookUp2)) {
            throw new Error("Cannot connect the same signer twice");
        }

        signerMap.set(signerMapAppend(signer, _signerMap));
        lookUp.set(toQuery(signer));
        return;
    }

    function second_handler(signer: Signer) {
        if (_lookUp && compareQueries(_lookUp, signer)) {
            throw new Error("Cannot connect the same signer twice");
        }

        lookUp2 = signer;

        if (lookUp1 && compareQueries(lookUp1, lookUp2)) {
            throw new Error("Cannot connect the same signer twice");
        }

        signerMap.set(signerMapAppend(signer, _signerMap));
        return;
    }

    function wrap_handler(
        f: (signer: Signer) => void
    ): (signer: Signer) => void {
        return (signer: Signer) => {
            try {
                f(signer);
            } catch (e) {
                alert.set({
                    message: `${e.message}`,
                    variant: "error",
                });
            }
        };
    }

    function makeValueSetter(n: number): (signer: Signer) => void {
        switch (n) {
            case 1:
                return wrap_handler(main_handler);
            // Special case 2 to respect a pre-existing connection of a primary key.
            case 2:
                return wrap_handler(second_handler);
            default:
                throw new Error(`Value setter must be 1 or 2, got: ${n}`);
        }
    }

    function sigSetter(n: number, val: string): void {
        if (n === 1) {
            sig1 = val;
        } else if (n === 2) {
            sig2 = val;
        } else {
            throw new Error(`Can only set signature for 1 or 2, got ${n}`);
        }
    }

    function makeSigSetter(n: number): (val: string) => void {
        return (val: string) => {
            sigSetter(n, val);
        };
    }
</script>

<WitnessFormHeader
    icon={KeyIcon}
    title={"Two Key Self Signed Verification Workflow"}
    subtitle={"Connect two keys by signing a statement linking them with both"}
/>
{#if _state === "connect"}
    <WitnessFormStepper
        step={1}
        totalSteps={3}
        label=""
        question="Connect two Signers to link"
        labelFor="form-step-q-1-i-1"
    >
        <div class="w-full">
            {#if !lookUp1}
                <div class="flex px-4 text-center">
                    <div class="w-full">
                        <b>Set first signer</b>
                    </div>
                </div>
                <SignerConnect
                    primary
                    class="menu w-full min-w-42 mt-[16px] rounded-xl"
                    signerCallback={makeValueSetter(1)}
                >
                    Connect first signer
                </SignerConnect>
            {:else}
                <div class="flex px-4 text-center">
                    <div class="w-full">
                        <b
                            >First signer is {displaySignerType(
                                lookUp1.signerType
                            )}:</b
                        >
                    </div>
                </div>
                <div class="flex px-4 text-center">
                    <div class="w-full">
                        <Button
                            class="max-w-42 sm:max-w-full my-[8px]"
                            onClick={() => {}}
                            text={displaySignerId(lookUp1)}
                            primary
                        />
                    </div>
                </div>
                {#if !lookUp2}
                    <div class="flex px-4 text-center">
                        <div class="w-full">
                            <b>Set second signer</b>
                        </div>
                    </div>
                    <SignerConnect
                        primary
                        class="menu w-full min-w-42 mt-[8px] rounded-xl"
                        signerCallback={makeValueSetter(2)}
                    >
                        Connect second signer
                    </SignerConnect>
                {:else}
                    <div class="flex px-4 text-center">
                        <div class="w-full">
                            <b
                                >Second signer is {displaySignerType(
                                    lookUp2.signerType
                                )}:</b
                            >
                        </div>
                    </div>
                    <div class="flex px-4 text-center">
                        <div class="w-full">
                            <Button
                                class="max-w-42 sm:max-w-full my-[8px]"
                                onClick={() => {}}
                                text={displaySignerId(lookUp2)}
                                primary
                            />
                        </div>
                    </div>
                {/if}
            {/if}
        </div>
    </WitnessFormStepper>
    <div
        class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center  content-end"
    >
        <Button
            class="w-2/5"
            onClick={() => navigate("/account")}
            text="Back"
            primary
        />
        <Button
            class="w-2/5"
            disabled={!lookUp1 || !lookUp2}
            onClick={async () => {
                try {
                    state.set(advance(_state));
                    await getStatement();
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
{:else if _state === "sign"}
    {#if !statement}
        <WitnessFormStepper
            step={1}
            totalSteps={3}
            label={"Generating statement..."}
            question={"Will use the generated statement as the basis for signatures from each selected key"}
            labelFor="form-step-q-1-i-1"
        >
            <div>Loading....</div>
        </WitnessFormStepper>
        <div
            class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center  content-end"
        >
            <Button
                class="w-2/5"
                onClick={() => navigate("/account")}
                text="Back"
                primary
            />
        </div>
    {:else}
        <WitnessFormStepper
            step={2}
            totalSteps={3}
            label={"Sign the generated statement with both Signers"}
            question={"If you're using the same provider for both keys, you may need to change the connected account the wallet extension's controls"}
            labelFor="form-step-q-2-i-2"
        >
            <div class="w-full flex flex-col">
                <h2 class="w-full text-center text-lg">
                    <b>Sign the first statement</b>
                </h2>
                {#if !sig1}
                    <StatementSigner
                        lookUp={lookUp1}
                        sigSetter={makeSigSetter(1)}
                        {statement}
                    />
                {:else}
                    <div class="flex px-4 text-center">
                        <div class="w-full">
                            <Button
                                disabled
                                class="max-w-42 sm:max-w-full my-[8px]"
                                onClick={() => {}}
                                text={`Signed with: ${displaySignerId(
                                    lookUp1
                                )}`}
                                primary
                            />
                        </div>
                    </div>
                {/if}
                <h2 class="w-full text-center text-lg">
                    <b>Sign the second statement</b>
                </h2>
                {#if !sig2}
                    <StatementSigner
                        lookUp={lookUp2}
                        sigSetter={makeSigSetter(2)}
                        {statement}
                    />
                {:else}
                    <div class="flex px-4 text-center">
                        <div class="w-full">
                            <Button
                                disabled
                                class="max-w-42 sm:max-w-full my-[8px]"
                                onClick={() => {}}
                                text={`Signed with: ${displaySignerId(
                                    lookUp2
                                )}`}
                                primary
                            />
                        </div>
                    </div>
                {/if}
            </div>
        </WitnessFormStepper>
        <div
            class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center  content-end"
        >
            <Button
                class="w-2/5"
                onClick={() => navigate("/account")}
                text="Back"
                primary
            />
            <Button
                class="w-2/5"
                disabled={!sig1 || !sig2}
                onClick={async () => {
                    try {
                        await getCredential();
                        state.set(advance(_state));
                        disconnect(lookUp2);
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
{:else if _state === "witness"}
    <WitnessFormComplete step={3} totalSteps={3} {navigate} />
{/if}
