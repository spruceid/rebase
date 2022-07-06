<script lang="ts">
	import { Router, Route } from "svelte-navigator";
	import { Account, Explainer, Witness } from "routes";
	
</script>

<Router>
	<Route path="/">
		<Explainer />
	</Route>
	<Route path="account" let:location>
		<Account {location} />
	</Route>
	<Route path="create/witness/:type" let:params>
		<Witness type={params.type} />
	</Route>
</Router>

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	.tooltip {
		@apply invisible absolute;
	}
	.has-tooltip:hover .tooltip {
		@apply visible z-50;
	}
	html,
	body {
		@apply relative bg-purple-light w-screen min-h-screen m-0 p-0 text-base flex font-basier-square font-normal flex-col overflow-x-hidden;
	}
	h1,
	h2,
	h3,
	h4,
	h5 {
		@apply font-basier-square font-semibold text-gray-850;
	}
	body {
		@apply text-gray-850;
	}
	td,
	th {
		@apply font-basier-square font-normal;
	}
	pre {
		white-space: pre-wrap; /* Since CSS 2.1 */
		white-space: -moz-pre-wrap; /* Mozilla, since 1999 */
		white-space: -pre-wrap; /* Opera 4-6 */
		white-space: -o-pre-wrap; /* Opera 7 */
		word-wrap: break-word; /* Internet Explorer 5.5+ */
	}
	@layer components {
		.card {
			background-color: theme("colors.white");
			border-radius: theme("borderRadius.lg");
			padding: theme("spacing.6");
			box-shadow: theme("boxShadow.xl");
		}
	}
	/**
	Custom scrollbar settings
	*/
	::-webkit-scrollbar-track {
		@apply rounded-lg bg-beige-1;
	}
	::-webkit-scrollbar-thumb {
		border-radius: 8px;
		background-color: #625ff5;
	}
	::-webkit-scrollbar {
		@apply bg-beige-1 rounded-lg h-1.5 w-1.5;
	}
</style>
