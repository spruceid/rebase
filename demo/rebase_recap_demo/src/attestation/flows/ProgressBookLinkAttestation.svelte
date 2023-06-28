<script lang="ts">
    import { Types } from "@spruceid/rebase-client";
    import { Writable, writable } from "svelte/store";
    import FormSlot from "./FormSlot.svelte";

    export let handler = (statement: Types.AttestationStatement) =>
        Promise<void>;
    export let subject: Types.Subjects;

    let link: Writable<string> = writable("");
    let _link: string = "";
    link.subscribe((x) => (_link = x));

    let progress: Writable<string> = writable("-1");
    let _progress: string = "-1";
    progress.subscribe((x) => (_progress = x));

    const reset = () => {
        link.set("");
        progress.set("-1");
    };

    const f = async () => {
        if (!_link) {
            throw new Error("Book link is required");
        }

        if (
            !_progress ||
            (_progress !== "-1" && _progress !== "0" && _progress !== "1")
        ) {
            throw new Error(
                "Progress must be set and between -1 and 1 (inclusive)"
            );
        }

        let stmt = {
            ProgressBookLinkAttestation: {
                link: _link,
                progress: parseInt(_progress),
                subject,
            },
        };

        // TODO: Figure out bigint handling
        //@ts-ignore
        await handler(stmt);

        reset();
    };
</script>

<div>
    <FormSlot
        label="Book Link"
        labelFor="src"
        placeholder="http://link-to-your-book.com"
        handler={(s) => {
            link.set(s);
        }}
    />
    <p>
        <input
            checked={_progress === "-1"}
            on:change={(e) => {
                progress.set(e.currentTarget.value);
            }}
            type="radio"
            name="progress"
            value="-1"
        /> Haven't started
    </p>
    <p>
        <input
            checked={_progress === "0"}
            on:change={(e) => {
                progress.set(e.currentTarget.value);
            }}
            type="radio"
            name="progress"
            value="0"
        /> Currently reading
    </p>
    <p>
        <input
            checked={_progress === "1"}
            on:change={(e) => {
                progress.set(e.currentTarget.value);
            }}
            type="radio"
            name="progress"
            value="1"
        /> Completed
    </p>
    <button on:click={f}>Issue</button>
</div>
