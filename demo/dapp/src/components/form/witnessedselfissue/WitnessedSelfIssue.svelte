<script lang="ts">
    import {
        client,
        type Claim,
        claims,
        alert,
        retrieveSigner,
        lookUp,
        sign,
        type Signer,
        signerMap,
        signerMapAppend,
        toQuery,
        getSubject,
    } from "src/util";
    import { Button, SignerConnect } from "src/components";
    import { writable } from "svelte/store";
    import { useNavigate } from "svelte-navigator";
    import FormSlot from "./FormSlot.svelte";
    import { Types } from "@rebase-xyz/rebase-client";
    // TODO: Add JSON Schema validation

    let navigate = useNavigate();

    let complete = writable(false);
    let _complete = false;
    complete.subscribe((x) => (_complete = x));

    let _lookUp = null;
    lookUp.subscribe((x) => (_lookUp = x));
    let _signerMap = null;
    signerMap.subscribe((x) => (_signerMap = x));
    let _claims: Array<Claim> = [];
    claims.subscribe((x) => (_claims = x));

    // TODO: Generalize over!
    let description = writable("");
    let _description = "";
    description.subscribe((x) => (_description = x));

    let image = writable("");
    let _image = "";
    image.subscribe((x) => (_image = x));

    let username = writable("");
    let _username = "";
    username.subscribe((x) => (_username = x));

    let website = writable("");
    let _website = "";
    website.subscribe((x) => (_website = x));

    const setNew = (credential: string) => {
        let next: Array<Claim> = [];
        for (let i = 0, n = _claims.length; i < n; i++) {
            let claim = _claims[i];
            if (claim.credential_type === "WitnessedSelfIssued") {
                claim.credentials.push(credential);
            }
            next.push(claim);
        }
        claims.set(next);
    };

    const handleNewSigner = (signer: Signer) => {
        signerMap.set(signerMapAppend(signer, _signerMap));
        lookUp.set(toQuery(signer));
    };

    async function f() {
        let current = retrieveSigner(_signerMap, _lookUp);
        if (!current) {
            throw new Error("No default signer set");
        }

        let stmt: Types.WitnessedBasicProfileStatement = {
            description: _description,
            image: _image,
            username: _username,
            website: _website,
            subject: getSubject(current),
        };

        let req: Types.StatementReq = {
            opts: {
                WitnessedSelfIssued: {
                    WitnessedBasicProfile: stmt,
                },
            },
        };

        // TODO: JSON Schema validation here!
        const badRespErr =
            "Badly formatted witness service response in statement";
        let res = await client.statement(req);

        let statement = res?.statement;
        if (!statement) {
            throw new Error(badRespErr + " missing statement");
        }
        let signature = await sign(statement);

        let proofReq: Types.WitnessReq = {
            proof: {
                WitnessedSelfIssued: {
                    WitnessedBasicProfile: {
                        signature,
                        statement: stmt,
                    },
                },
            },
        };

        // TODO: JSON Schema validation here!
        let proofRes = await client.witness_jwt(proofReq);

        // TODO: Check for missing JWT.
        if (!proofRes.jwt) {
            throw new Error(
                (proofRes as any)?.error ?? "No JWT found in response"
            );
        }
        setNew(proofRes.jwt);
        complete.set(true);
    }

    async function submit() {
        try {
            await f();
        } catch (e) {
            alert.set({
                message: `Failed to submit witnessed credential: ${
                    e?.message ?? `${e}`
                }`,
                variant: "error",
            });
        }
    }
</script>

{#if !_lookUp}
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
{:else if _complete}
    <div
        class="w-full my-[16px] text-center flex flex-wrap justify-evenly items-center content-end"
    >
        <p>Credential Issued!</p>
        <Button
            class="w-fit  my-[16px]"
            onClick={() => navigate("/account#obtained")}
            text="Manage Credentials"
            action
        />
    </div>
{:else}
    <div class="w-full">
        <div
            class="w-full flex flex-wrap justify-center content-between h-full"
        >
            <FormSlot
                label="Enter the username to associate with your profile"
                labelFor="username"
                placeholder="Enter a username here"
                handler={(val) => username.set(val)}
            />
            <FormSlot
                label="Enter a description for your profile"
                labelFor="description"
                placeholder="Enter a description here"
                handler={(val) => description.set(val)}
            />
            <FormSlot
                label="Enter a link to an image to associate with your profile"
                labelFor="image"
                placeholder="Enter an image url here"
                handler={(val) => image.set(val)}
            />
            <FormSlot
                label="Enter a website for your profile"
                labelFor="website"
                placeholder="Enter a website url here"
                handler={(val) => website.set(val)}
            />
            <Button
                class="w-fit  my-[16px]"
                onClick={submit}
                text="Generate Credential"
                action
            />
        </div>
    </div>
{/if}
