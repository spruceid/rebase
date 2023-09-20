<script lang="ts">
    import { JWTFMT, Signer } from "src/util/types";
    import { AttestationStatement } from "@spruceid/rebase-client";

    export let postUUID: string;
    export let allCredentials: Array<JWTFMT | false>;
    export let issue: (statement: AttestationStatement) => Promise<void>;
    export let signer: Signer;

    const getProfileUUIDs = (
        credentials: Array<JWTFMT | false>
    ): Array<string> => {
        let a = [];
        if (!credentials) {
            return a;
        }

        for (let i = 0, x = credentials.length; i < x; i++) {
            let next = credentials[i];
            if (next && next.type === "BasicProfileAttestation") {
                a.push(next.uuid);
            }
        }

        return a;
    };

    let user = "";
</script>

<!-- NOTE: This could support many users, but only supports one in the demo for brevity -->
<p>Tag a profile to this picture:</p>
<p>First, add the user you want to tag to this picture!</p>
<p>
    <label for="user">Pick a user:</label>
    <select name="user" bind:value={user}>
        {#each getProfileUUIDs(allCredentials) as uuid}
            <option value={uuid}>{uuid}</option>
        {/each}
    </select>
</p>
{#if user}
    <p>
        Then Click: <button
            on:click={() => {
                issue({
                    BasicTagAttestation: {
                        subject: signer.subject(),
                        post: postUUID,
                        users: [user],
                    },
                });
                user = "";
            }}>Tag!</button
        >
    </p>
{/if}
