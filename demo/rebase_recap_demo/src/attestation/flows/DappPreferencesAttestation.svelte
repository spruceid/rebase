<script lang="ts">
    import { Types } from "@spruceid/rebase-client";

    export let handler = (statement: Types.AttestationStatement) =>
        Promise<void>;
    export let subject: Types.Subjects;

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
