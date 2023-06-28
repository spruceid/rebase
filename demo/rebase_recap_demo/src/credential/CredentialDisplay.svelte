<script lang="ts">
    import { JWTFMT } from "src/util/types";
    import { Writable, writable } from "svelte/store";

    export let credential: JWTFMT;
    export let allCredentials: Array<JWTFMT | false>;

    // Use to avoid warning.
    console.log(allCredentials);

    let showJSON: Writable<boolean> = writable(false);
    let _showJSON = false;
    showJSON.subscribe((x) => (_showJSON = x));
</script>

<div>
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
