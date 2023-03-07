<script lang="ts">
    import {
        CredentialType,
        Instructions,
        Workflow,
        Claim,
        retrieveSigner,
        signerMap,
        claims,
        getSubject,
        witnessState,
        sign,
        client,
        Signer,
        lookUp,
        alert,
        needsDelimiter,
        signerMapAppend,
        toQuery,
    } from "src/util";
    import Ajv from "ajv";
    import addFormats from "ajv-formats";
    import { onDestroy, onMount } from "svelte";
    import { useNavigate } from "svelte-navigator";
    import {
        WitnessFormHeader,
        Button,
        // NewSignerSelector,
        SignerConnect,
    } from "src/components";
    import WitnessFormStatement from "./WitnessFormStatement.svelte";
    import WitnessFormSignature from "./WitnessFormSignature.svelte";
    import WitnessFormWitness from "./WitnessFormWitness.svelte";
    import WitnessFormComplete from "./WitnessFormComplete.svelte";
    import { Writable, writable } from "svelte/store";

    // TODO: Make this a drop-down and include polygon?
    const NFT_NETWORK = "eth-mainnet";
    const DNS_PREFIX = "rebase_sig=";

    let _lookUp = null;
    lookUp.subscribe((x) => (_lookUp = x));
    let _signerMap = null;
    signerMap.subscribe((x) => (_signerMap = x));

    const navigate = useNavigate();
    const ajv = new Ajv();
    addFormats(ajv);

    let statement_schema = null,
        witness_schema = null;

    let verified: boolean = false;
    let loading: boolean = false;

    let c: Array<Claim> = [];
    claims.subscribe((x) => (c = x));

    const handleNewSigner = (signer: Signer) => {
        signerMap.set(signerMapAppend(signer, _signerMap));
        lookUp.set(toQuery(signer));
        advance();
    };

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

    let statement: Writable<string> = writable("");
    let _statement: string = "";
    statement.subscribe((x) => (_statement = x));

    let signature: Writable<string> = writable("");
    let _signature: string = "";
    signature.subscribe((x) => (_signature = x));

    let delimiter: Writable<string> = writable("");
    let _delimiter: string = "";
    delimiter.subscribe((x) => (_delimiter = x));

    let handle: Writable<string> = writable("");
    let _handle: string = "";
    handle.subscribe((x) => (_handle = x));

    let proof: Writable<string> = writable("");
    let _proof: string = "";
    proof.subscribe((x) => (_proof = x));

    let issuedAt: Writable<string> = writable("");
    let _issuedAt: string = "";
    issuedAt.subscribe((x) => (_issuedAt = x));

    const onChangeValue = (name, value) => {
        switch (name) {
            case "handle":
                handle.set(value);
                break;
            case "signature":
                signature.set(value);
                break;
            case "proof":
                proof.set(value);
                break;
            case "verified":
                verified = value;
                break;
        }
    };

    const post = (): string => {
        switch (type) {
            case "GitHubVerification":
            case "TwitterVerification":
                return `${_statement}${_delimiter}${_signature}`;
            case "DnsVerification":
                return `${DNS_PREFIX}${_signature}`;
            case "SoundCloudVerification":
            case "RedditVerification":
                return `${_signature}`;
            case "EmailVerification":
            case "NftOwnershipVerification":
            case "PoapOwnershipVerification":
                return;
        }
    };

    let state: Workflow = "signer";
    witnessState.subscribe((x) => (state = x));

    onMount(async () => {
        if (_lookUp) {
            witnessState.set("statement");
        } else {
            witnessState.set("signer");
        }

        try {
            console.log("Here?");
            let res = await client.instructions(JSON.stringify({ type }));
            let instruction_res = JSON.parse(res);
            console.log("Here??");
            statement_schema = instruction_res?.statement_schema;
            witness_schema = instruction_res?.witness_schema;
            issuedAt.set(new Date().toISOString());
        } catch (e) {
            console.error(e);
            alert.set({
                message: "Failed to retrieve instructions from witness service",
                variant: "error",
            });
        }
    });

    onDestroy(() => {
        witnessState.set("signer");
    });

    const advance = () => {
        if (!_lookUp) {
            return witnessState.set("signer");
        }
        switch (state) {
            case "signer":
                return witnessState.set("statement");
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
            case "statement":
                return witnessState.set("signer");
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
        let current = retrieveSigner(_signerMap, _lookUp);
        if (!current) {
            throw new Error("No default signer set");
        }

        switch (type) {
            case "DnsVerification":
                opts[type]["domain"] = _handle;
                opts[type]["prefix"] = DNS_PREFIX;
                opts[type]["subject"] = getSubject(current);
                break;
            case "GitHubVerification":
            case "TwitterVerification":
            case "RedditVerification":
                opts[type]["handle"] = _handle;
                opts[type]["subject"] = getSubject(current);
                break;
            case "SoundCloudVerification":
                opts[type]["permalink"] =
                    _handle.split("/")[_handle.split("/").length - 1];
                opts[type]["subject"] = getSubject(current);
                break;
            case "EmailVerification":
                opts[type]["email"] = _handle;
                opts[type]["subject"] = getSubject(current);
                break;
            case "NftOwnershipVerification":
                opts[type]["contract_address"] = _handle;
                opts[type]["network"] = NFT_NETWORK;
                opts[type]["issued_at"] = _issuedAt;
                opts[type]["subject"] = getSubject(current);
                break;
            case "PoapOwnershipVerification":
                let next_id = parseInt(_handle);
                if (isNaN(next_id) || !next_id) {
                    throw new Error(
                        "Invalid event id, expected a number greater than 0"
                    );
                }

                opts[type]["event_id"] = next_id;
                opts[type]["issued_at"] = _issuedAt;
                opts[type]["subject"] = getSubject(current);
                break;
            default:
                throw new Error(`${type} flow is currently unsupported`);
        }

        if (!statement_schema) {
            throw new Error("No JSON Schema found for Statement Request");
        }

        if (!ajv.validate(statement_schema, opts[type])) {
            throw new Error("Validation of Statement Request failed");
        }

        const badRespErr = "Badly formatted witness service response";
        try {
            let res = await client.statement(JSON.stringify({ opts }));

            let body = JSON.parse(res);
            if (!body.statement) {
                throw new Error(badRespErr + " missing statement");
            }

            if (needsDelimiter(type) && !body.delimiter) {
                throw new Error(badRespErr + " missing delimiter");
            }

            statement.set(body.statement);
            delimiter.set(body.delimiter);
        } catch (e) {
            if (e.message === badRespErr) {
                throw e;
            } else {
                throw new Error("Failed in request for statement to witness");
            }
        }
    };

    const getCredential = async (): Promise<void> => {
        let opts = {};
        opts[type] = {};

        let current = retrieveSigner(_signerMap, _lookUp);
        if (!current) {
            throw new Error("No default signer set");
        }

        switch (type) {
            case "DnsVerification":
                opts[type]["domain"] = _handle;
                opts[type]["prefix"] = DNS_PREFIX;
                opts[type]["subject"] = getSubject(current);
                break;
            case "RedditVerification":
                opts[type]["handle"] = _handle;
                opts[type]["subject"] = getSubject(current);
                break;
            case "SoundCloudVerification":
                opts[type]["permalink"] =
                    _handle.split("/")[_handle.split("/").length - 1];
                opts[type]["subject"] = getSubject(current);
                break;
            case "GitHubVerification":
                opts[type]["statement"] = {};
                opts[type]["statement"]["handle"] = _handle;
                opts[type]["statement"]["subject"] = getSubject(current);
                opts[type]["gist_id"] = _proof.split("/").pop();
                break;
            case "TwitterVerification":
                opts[type]["statement"] = {};
                opts[type]["statement"]["handle"] = _handle;
                opts[type]["statement"]["subject"] = getSubject(current);
                opts[type]["tweet_url"] = _proof.split("?")[0];
                break;
            case "EmailVerification":
                opts[type]["statement"] = {};
                opts[type]["statement"]["email"] = _handle;
                opts[type]["statement"]["subject"] = getSubject(current);
                opts[type]["challenge"] = _proof.trim();
                opts[type]["signature"] = _signature;
                break;
            case "NftOwnershipVerification":
                opts[type]["statement"] = {};
                opts[type]["statement"]["contract_address"] = _handle;
                opts[type]["statement"]["network"] = NFT_NETWORK;
                opts[type]["statement"]["issued_at"] = _issuedAt;
                opts[type]["statement"]["subject"] = getSubject(current);
                opts[type]["signature"] = _signature;
                break;
            case "PoapOwnershipVerification":
                let next_id = parseInt(_handle);
                if (isNaN(next_id) || !next_id) {
                    throw new Error(
                        "Invalid event id, expected a number greater than 0"
                    );
                }
                opts[type]["statement"] = {};
                opts[type]["statement"]["event_id"] = next_id;
                opts[type]["statement"]["issued_at"] = _issuedAt;
                opts[type]["statement"]["subject"] = getSubject(current);
                opts[type]["signature"] = _signature;
                break;
            default:
                throw new Error(`${type} flow is currently unsupported`);
        }

        if (!witness_schema) {
            throw new Error("No JSON Schema found for Witness Request");
        }

        if (!ajv.validate(witness_schema, opts[type])) {
            throw new Error("Validation of Witness Request failed");
        }

        try {
            let b = JSON.stringify({ proof: opts });

            let res = await client.jwt(b);
            let { jwt } = JSON.parse(res);

            setNew(jwt);
        } catch (e) {
            console.error(e);
            throw new Error(
                "Failed to issue credential, please retry the flow"
            );
        }
    };
</script>

<WitnessFormHeader
    icon={instructions.icon}
    title={instructions.title}
    subtitle={instructions.subtitle}
/>

{#if _lookUp && (type === "NftOwnershipVerification" || type === "PoapOwnershipVerification") && _lookUp?.signerType !== "ethereum"}
    <div class="w-full">
        <div class="flex px-4 text-center">
            <div class="w-full">
                <p>Currently this flow only supports Ethereum subjects.</p>
                <div
                    class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center content-end"
                >
                    <Button
                        class="w-fit  my-[16px]"
                        onClick={() => navigate("/account#obtained")}
                        text="Manage Credentials"
                        action
                    />
                </div>
            </div>
        </div>
    </div>
{:else}
    {#if state === "signer"}
        <div class="w-full">
            <div class="flex px-4 text-center">
                <div class="w-full">
                    <h4>Please connect a signer to link</h4>
                    <SignerConnect
                        primary
                        class="menu w-full min-w-42 mt-[8px] rounded-xl"
                        signerCallback={handleNewSigner}
                    >
                        Connect signer
                    </SignerConnect>
                </div>
            </div>
        </div>
        <div
            class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center content-end"
        >
            <Button
                class="w-2/5"
                disabled={!_lookUp}
                onClick={advance}
                text="Next"
                action
            />
        </div>
    {/if}
    {#if state === "statement"}
        <WitnessFormStatement
            {instructions}
            {loading}
            handle={_handle}
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
            statement={_statement}
            signature={_signature}
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
            proof={_proof}
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
{/if}
