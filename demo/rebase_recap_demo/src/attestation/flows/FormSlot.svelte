<script lang="ts">
    export let label: string;
    export let labelFor: string;
    export let placeholder: string;
    export let handler: (value: string) => void;
    let val = "";
</script>

<div class="w-full mx-2 px-4">
    <label for={labelFor}>
        <p class="font-bold">{label}</p>
        <input
            class="form-text-input"
            {placeholder}
            bind:value={val}
            on:input={(e) => handler(e.target?.value)}
            name={labelFor}
            type="text"
        />
    </label>
</div>
