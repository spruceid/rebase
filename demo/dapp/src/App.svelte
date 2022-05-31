<script lang="ts">
	import { Router, Route } from "svelte-navigator";
	import { Account, Explainer, Local, Witness } from "./routes";
</script>

<main class="centered">
	<Router>
		<Route path="/">
			<Explainer />
		</Route>
		<Route path="account">
			<Account />
		</Route>
		<Route path="create/local/:type" let:params>
			<Local type={params.type} />
		</Route>
		<Route path="create/witness/:type" let:params>
			<Witness type={params.type} />
		</Route>
	</Router>
</main>

<style>
	.centered {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100vh;
		background-color: lightgray;
	}
</style>
