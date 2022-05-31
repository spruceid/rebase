<script lang="ts">
    import { Available, Obtained } from "../components/claims";
    import { accountState, AccountState } from "../util";

    let state: AccountState;
    accountState.subscribe((x) => {
        state = x;
    });

    const toggle = () => {
        accountState.set(state == "available" ? "obtained" : "available");
    };
</script>

<div class="viewer">
    <div class="inner-center">
        <button disabled={state == "available"} on:click={toggle}
            >Available</button
        >
        <button disabled={state == "obtained"} on:click={toggle}
            >Obtained</button
        >
    </div>
    <div class="inner-center">
        {#if state == "available"}
            <Available />
        {:else}
            <Obtained />
        {/if}
    </div>
</div>

<style>
    .viewer {
		height: 70vh;
        width: 75vh;
		background-color: white;
	}
    .inner-center {
		display: flex;
		justify-content: center;
		align-items: center;
        margin-left: 5vh;
        margin-right: 5vh;
    }
</style>
