<script lang="ts">
    import { AttestationStatement, Subjects } from "@spruceid/rebase-client";

    export let handler = (statement: AttestationStatement) => Promise<void>;
    export let subject: Subjects;

    let dark_mode = false;

    const f = async () => {
        let stmt = {
            DappPreferencesAttestation: {
                dark_mode,
                subject,
            },
        };

        await handler(stmt);

        dark_mode = false;
    };
</script>

<div>
    <p>Enable darkmode? <input type="checkbox" bind:checked={dark_mode} /></p>
    <button on:click={f}>Issue</button>
</div>
