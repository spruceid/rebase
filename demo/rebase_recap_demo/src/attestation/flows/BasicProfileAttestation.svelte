<script lang="ts">
    import { Types } from "@spruceid/rebase-client";
    import { Writable, writable } from "svelte/store";
    import FormSlot from "./FormSlot.svelte";

    export let handler = (statement: Types.AttestationStatement) =>
        Promise<void>;
    export let subject: Types.Subjects;

    let username: Writable<string> = writable("");
    let _username: string = "";
    username.subscribe((x) => (_username = x));

    let website: Writable<string> = writable("");
    let _website: string = "";
    website.subscribe((x) => (_website = x));

    let description: Writable<string | null> = writable(null);
    let _description: string | null = null;
    description.subscribe((x) => (_description = x));

    let image: Writable<string | null> = writable(null);
    let _image: string | null = null;
    image.subscribe((x) => (_image = x));

    const reset = () => {
        username.set("");
        website.set("");
        description.set(null);
        image.set(null);
    };

    const f = async () => {
        if (!_username) {
            throw new Error("Username cannot be blank");
        }

        if (!_website) {
            throw new Error("Website cannot be blank");
        }

        let stmt = {
            BasicProfileAttestation: {
                subject,
                username: _username,
                website: _website,
                description: _description,
                image: _image,
            },
        };

        await handler(stmt);

        reset();
    };
</script>

<div>
    <FormSlot
        label="Username"
        labelFor="username"
        placeholder="Enter a username (required)"
        handler={(s) => {
            username.set(s);
        }}
    />
    <FormSlot
        label="Website"
        labelFor="website"
        placeholder="Enter a website url (required)"
        handler={(s) => {
            website.set(s);
        }}
    />
    <FormSlot
        label="Description"
        labelFor="description"
        placeholder="Enter a description"
        handler={(s) => {
            description.set(s);
        }}
    />
    <FormSlot
        label="Image Source"
        labelFor="image"
        placeholder="http://url-to-your-image.jpg"
        handler={(s) => {
            image.set(s);
        }}
    />
    <button on:click={f}>Issue</button>
</div>
