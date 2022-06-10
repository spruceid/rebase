<script lang="ts">
	import { Router, Route } from "svelte-navigator";
	import { Header } from "./components";
	import { Account, Explainer, Witness } from "./routes";
</script>

<main class="bg">
	<div class="buffer centered" />
	<div class="centered">
		<Header />
	</div>
	<div class="centered">
		<Router>
			<Route path="/">
				<Explainer />
			</Route>
			<Route path="account">
				<Account />
			</Route>
			<Route path="create/witness/:type" let:params>
				<Witness type={params.type} />
			</Route>
		</Router>
	</div>
</main>

<style>
	.bg {
		height: 200vh;
		background-color: lightgray;
	}
	.centered {
		display: flex;
		justify-content: center;
		align-items: center;
	}
	.buffer {
		height: 10vh;
		background-color: lightgray;
	}
</style>
