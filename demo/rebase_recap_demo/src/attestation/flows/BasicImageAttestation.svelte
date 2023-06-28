<script lang="ts">
    import { Types } from "@spruceid/rebase-client";
    import { Writable, writable } from "svelte/store";
    import FormSlot from "./FormSlot.svelte";

    export let handler = (statement: Types.AttestationStatement) =>
        Promise<void>;
    export let subject: Types.Subjects;

    let src: Writable<string> = writable("");
    let _src: string = "";
    src.subscribe((x) => (_src = x));

    const f = async () => {
        if (!_src) {
            throw new Error("Image source is required");
        }

        let stmt = {
            BasicImageAttestation: {
                src: _src,
                subject,
            },
        };

        await handler(stmt);

        src.set("");
    };
</script>

<!-- TODO: IMPL -->
<div>
    <FormSlot
        label="Image Source"
        labelFor="src"
        placeholder="http://url-to-your-image.jpg"
        handler={(s) => {
            src.set(s);
        }}
    />
    <button on:click={f}>Issue</button>
</div>
