<script lang="ts">
	import { Router, Route } from "svelte-navigator";
	import { Account, Explainer, Witness } from "routes";
</script>

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

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
	@font-face {
		font-family: "Satoshi";
		font-style: normal;
		font-weight: 100 700;
		src: url("/fonts/Satoshi.eot"); /* IE9 Compat Modes */
		src: local(""),
			url("/fonts/Satoshi.eot?#iefix")
				format("embedded-opentype"),
			/* IE6-IE8 */ url("/fonts/Satoshi.woff2") format("woff2"),
			/* Super Modern Browsers */ url("/fonts/Satoshi.woff")
				format("woff"),
			/* Modern Browsers */ url("/fonts/Satoshi.ttf")
				format("truetype"),
			/* Safari, Android, iOS */ url("/fonts/Satoshi.svg#Satoshi")
				format("svg"); /* Legacy iOS */
	}
	.tooltip {
		@apply invisible absolute;
	}
	.has-tooltip:hover .tooltip {
		@apply visible z-50;
	}
	html,
	body {
		@apply relative bg-purple-light w-screen min-h-screen m-0 p-0 text-base flex font-satoshi flex-col overflow-x-hidden;
	}
	h1,
	h2,
	h3,
	h4,
	h5 {
		@apply font-satoshi font-semibold text-gray-850;
	}
	body {
		@apply text-gray-850;
	}
	/* a {
		@apply no-underline text-blue-1;
	} */
	td,
	th {
		@apply font-satoshi font-normal;
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
