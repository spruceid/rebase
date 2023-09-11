<script lang="ts">
    import { AttestationStatement, Subjects } from "@spruceid/rebase-client";
    import { Writable, writable } from "svelte/store";
    import FormSlot from "./FormSlot.svelte";

    export let handler = (statement: AttestationStatement) => Promise<void>;
    export let subject: Subjects;

    let body: Writable<string> = writable("");
    let _body: string = "";
    body.subscribe((x) => (_body = x));

    let title: Writable<string> = writable("");
    let _title: string = "";
    title.subscribe((x) => (_title = x));

    // NOTE: This would be needed in an application context.
    // let reply_to: Writable<string | null> = writable(null);
    // let _reply_to: string | null = null;
    // reply_to.subscribe((x) => (_reply_to = x));

    const reset = () => {
        body.set("");
        title.set("");
        // NOTE: This would be needed in an application context.
        // reply_to.set(null);
    };

    const f = async () => {
        if (!_body) {
            throw new Error("Body cannot be blank");
        }

        if (!_title) {
            throw new Error("Title cannot be blank");
        }

        let stmt = {
            BasicPostAttestation: {
                subject,
                body: _body,
                title: _title,
                // NOTE: This would be needed in an application context.
                // reply_to: _reply_to,
                reply_to: null,
            },
        };

        await handler(stmt);

        reset();
    };
</script>

<div>
    <FormSlot
        label="Post Title"
        labelFor="title"
        placeholder="A title for your post"
        handler={(s) => {
            title.set(s);
        }}
    />
    <FormSlot
        label="Body"
        labelFor="body"
        placeholder="The body of your post"
        handler={(s) => {
            body.set(s);
        }}
    />
    <button on:click={f}>Issue</button>
</div>
