<script lang="ts">
    import { AttestationStatement, Subjects } from "@spruceid/rebase-client";
    import { Writable, writable } from "svelte/store";
    import FormSlot from "./FormSlot.svelte";

    export let handler = (statement: AttestationStatement) => Promise<void>;
    export let subject: Subjects;

    let link: Writable<string> = writable("");
    let _link: string = "";
    link.subscribe((x) => (_link = x));

    let title: Writable<string> = writable("");
    let _title: string = "";
    title.subscribe((x) => (_title = x));

    let review: Writable<string> = writable("");
    let _review: string = "";
    review.subscribe((x) => (_review = x));

    let rating: Writable<number> = writable(0);
    let _rating: number = 0;
    rating.subscribe((x) => (_rating = x));

    const reset = () => {
        link.set("");
        title.set("");
        review.set("");
        rating.set(0);
    };

    const f = async () => {
        if (!_link) {
            throw new Error("Book link is required");
        }

        if (!_title) {
            throw new Error("Review title is required");
        }

        if (!_review) {
            throw new Error("Review body is required");
        }

        let stmt = {
            BookReviewAttestation: {
                subject,
                link: _link,
                title: _title,
                review: _review,
                rating: _rating,
            },
        };

        // TODO: Improve BigInt <-> Number issues.
        // @ts-ignore
        await handler(stmt);

        reset();
    };
</script>

<!-- TODO: IMPL -->
<div>
    <FormSlot
        label="Link to Book"
        labelFor="booklink"
        placeholder="http://link-to-google-books.com"
        handler={(s) => {
            link.set(s);
        }}
    />
    <FormSlot
        label="Review Title"
        labelFor="title"
        placeholder="A book review title"
        handler={(s) => {
            title.set(s);
        }}
    />
    <FormSlot
        label="Review Body"
        labelFor="review"
        placeholder="The body of the review"
        handler={(s) => {
            review.set(s);
        }}
    />
    <div class="w-full mx-2 px-4">
        <label for="rating">
            <p class="font-bold">Rating:</p>
            <input
                class="form-text-input"
                bind:value={_rating}
                on:input={(e) => {
                    // NOTE: We are safely handling if target is malformed.
                    // @ts-ignore
                    let x = parseInt(e.target?.value ?? NaN);
                    rating.set(x);
                }}
                name="rating"
                type="number"
            />
        </label>
    </div>
    <button on:click={f}>Issue</button>
</div>
