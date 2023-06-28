<script lang="ts">
    import type { Types } from "@spruceid/rebase-client";
    import type { Signer } from "../util/types";
    import BasicImageAttestation from "./flows/BasicImageAttestation.svelte";
    import BasicPostAttestation from "./flows/BasicPostAttestation.svelte";
    import BasicProfileAttestation from "./flows/BasicProfileAttestation.svelte";
    import BasicTagAttestation from "./flows/BasicTagAttestation.svelte";
    import BookReviewAttestation from "./flows/BookReviewAttestation.svelte";
    import DappPreferencesAttestation from "./flows/DappPreferencesAttestation.svelte";
    import FollowAttestation from "./flows/FollowAttestation.svelte";
    import LikeAttestation from "./flows/LikeAttestation.svelte";
    import ProgressBookLinkAttestation from "./flows/ProgressBookLinkAttestation.svelte";

    export let signer: Signer;
    export let attestationType: Types.AttestationTypes;
    export let issue: (
        statement: Types.AttestationStatement,
        currentSigner: Signer
    ) => Promise<void>;

    const f = async (statement: Types.AttestationStatement) => {
        return await issue(statement, signer);
    };
</script>

{#if attestationType === "BasicImageAttestation"}
    <BasicImageAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "BasicPostAttestation"}
    <BasicPostAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "BasicProfileAttestation"}
    <BasicProfileAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "BasicTagAttestation"}
    <BasicTagAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "BookReviewAttestation"}
    <BookReviewAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "DappPreferencesAttestation"}
    <DappPreferencesAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "FollowAttestation"}
    <FollowAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "LikeAttestation"}
    <LikeAttestation handler={f} subject={signer.subject()} />
{:else if attestationType === "ProgressBookLinkAttestation"}
    <ProgressBookLinkAttestation handler={f} subject={signer.subject()} />
{:else}
    <div>Unknown type: {attestationType}</div>
{/if}
