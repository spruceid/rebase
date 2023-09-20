<script lang="ts">
    import { AttestationStatement } from "@spruceid/rebase-client";
    import { JWTFMT, Signer } from "src/util/types";
    import { Writable, writable } from "svelte/store";
    import BasicPostAttestation from "src/attestation/flows/BasicPostAttestation.svelte";
    import TagInput from "./TagInput.svelte";

    export let credential: JWTFMT;
    export let allCredentials: Array<JWTFMT | false>;

    export let signer: Signer;
    export let nextPermissions;
    export let issue: (statement: AttestationStatement) => Promise<void>;

    // Use to avoid warning.
    console.log(nextPermissions);

    let showJSON: Writable<boolean> = writable(false);
    let _showJSON = false;
    showJSON.subscribe((x) => (_showJSON = x));

    const makeFilter = (
        comparison: string,
        objectPath: Array<string>
    ): ((credential: JWTFMT) => boolean) => {
        return (credential: JWTFMT): boolean => {
            if (objectPath.length <= 0) {
                return false;
            }

            let pathKeys = Object.keys(objectPath);
            let x = credential;
            for (let i = 0, x = pathKeys.length; i < x; i++) {
                let pathKey = pathKeys[i];
                if (x && x[pathKey]) {
                    x = x[pathKey];
                }
            }

            if (typeof x !== "string") {
                return false;
            } else {
                return x === comparison;
            }
        };
    };
</script>

<div style="border-style:solid;">
    <p>Type: {credential.type}</p>
    <!-- TODO: Make this more dynamic? -->
    {#if credential.type === "BasicImageAttestation"}
        <div>
            <p>
                <img
                    src={credential.details.BasicImageAttestation.src}
                    alt="credential source"
                />
            </p>
            <!-- TODO: Move this to it's own component -->
            {#if nextPermissions.includes("BasicTagAttestation")}
                <TagInput
                    postUUID={credential.uuid}
                    {allCredentials}
                    {issue}
                    {signer}
                />
            {/if}
        </div>
    {:else if credential.type === "BasicPostAttestation"}
        <!-- TODO: Add replies, likes, and tags -->
        <div>
            <p>
                Title: {credential.details.BasicPostAttestation.title}
            </p>
            <p>
                Body: {credential.details.BasicPostAttestation.body}
            </p>
            <p>Post ID: {credential.uuid}</p>
            {#if credential?.details?.BasicPostAttestation?.reply_to}
                <p>
                    Reply To: {credential.details.BasicPostAttestation.reply_to}
                </p>
            {/if}
            {#if nextPermissions.includes("LikeAttestation")}
                <p>
                    Like this post: <button
                        on:click={() => {
                            issue({
                                LikeAttestation: {
                                    subject: signer.subject(),
                                    target: credential.uuid,
                                },
                            });
                        }}>Like!</button
                    >
                </p>
            {/if}
            <div>
                <p>Likes of this post</p>
                {#each allCredentials as reply}
                    {#if reply?.details?.LikeAttestation?.target == credential.uuid}
                        Like ID: {reply.uuid}
                    {/if}
                {/each}
            </div>
            <div>
                <p>Reply to this post?</p>
                <BasicPostAttestation
                    subject={signer.subject()}
                    attestationType="BasicPostAttestation"
                    replyTo={credential.uuid}
                    handler={issue}
                />
            </div>
            <div>
                <p>Replies to this post:</p>
                {#each allCredentials as reply}
                    {#if reply?.details?.BasicPostAttestation?.reply_to == credential.uuid}
                        <p>
                            Title: {reply.details.BasicPostAttestation.title}
                        </p>
                        <p>
                            Body: {reply.details.BasicPostAttestation.body}
                        </p>
                        <p>Post ID: {reply.uuid}</p>
                        {#if reply?.details?.BasicPostAttestation?.reply_to}
                            <p>
                                Reply To: {reply.details.BasicPostAttestation
                                    .reply_to}
                            </p>
                        {/if}
                    {/if}
                {/each}
            </div>
        </div>
    {:else if credential.type === "BasicProfileAttestation"}
        <!-- TODO: Add follows and tags -->
        <div>
            <p>
                Username: {credential.details.BasicProfileAttestation.username}
            </p>
            <p>
                Website: {credential.details.BasicProfileAttestation.website}
            </p>
            {#if credential.details.BasicProfileAttestation.image}
                <p>
                    <img
                        src={credential.details.BasicProfileAttestation.image}
                        alt="credential source"
                    />
                </p>
            {/if}
            {#if credential.details.BasicProfileAttestation.description}
                <p>
                    Description: {credential.details.BasicProfileAttestation
                        .description}
                </p>
            {/if}
            {#if nextPermissions.includes("FollowAttestation")}
                <p>
                    Follow this user: <button
                        on:click={() => {
                            issue({
                                FollowAttestation: {
                                    subject: signer.subject(),
                                    target: credential.uuid,
                                },
                            });
                        }}>Follow!</button
                    >
                </p>
            {/if}
        </div>
    {:else if credential.type === "BasicTagAttestation"}
        <div>
            <p>Post: {credential.details.BasicTagAttestation.post}</p>
            <!-- TODO: Better format users -->
            <p>
                Users: {credential.details.BasicTagAttestation.users.join(", ")}
            </p>
        </div>
    {:else if credential.type === "BookReviewAttestation"}
        <div>
            <!-- TODO: Format this: -->
            <p>Link: {credential.details.BookReviewAttestation.link}</p>
            <p>Rating: {credential.details.BookReviewAttestation.rating}</p>
            <p>Title: {credential.details.BookReviewAttestation.title}</p>
            <p>Review: {credential.details.BookReviewAttestation.review}</p>
        </div>
    {:else if credential.type === "DappPreferencesAttestation"}
        <div>
            <p>
                Using Dark Mode? {credential.details.DappPreferencesAttestation
                    .dark_mode
                    ? "Yes"
                    : "No"}
            </p>
        </div>
    {:else if credential.type === "FollowAttestation"}
        <!-- TODO: Format this: -->
        <div>
            <p>
                Target: {credential.details.FollowAttestation.target}
            </p>
        </div>
    {:else if credential.type === "LikeAttestation"}
        <!-- TODO: Format this: -->
        <div>
            <p>
                Target: {credential.details.LikeAttestation.target}
            </p>
        </div>
    {:else if credential.type === "ProgressBookLinkAttestation"}
        <div>
            <!-- TODO: Format this: -->
            <p>Link: {credential.details.ProgressBookLinkAttestation.link}</p>
            <p>
                Progress: {!credential.details.ProgressBookLinkAttestation
                    .progress
                    ? "In Progress"
                    : credential.details.ProgressBookLinkAttestation === 1
                    ? "Finished"
                    : "Not Started"}
            </p>
        </div>
    {/if}

    <button on:click={() => showJSON.set(!_showJSON)}
        >{_showJSON ? "Hide" : "Show"} JSON</button
    >
    {#if _showJSON}
        <pre style="text-align:left;">{credential.json}</pre>
    {/if}
</div>
