<script lang="ts">
	import { writable, type Writable } from "svelte/store";
	import { Signer, JWTFMT } from "./util/types";
	import AttestationForm from "./attestation/AttestationForm.svelte";
	import CredentialDisplay from "./credential/CredentialDisplay.svelte";
	import {
		Types,
		Client,
		// defaultClientConfig,
	} from "@spruceid/rebase-client";
	import { WasmClient } from "@spruceid/rebase-client/wasm";
	import { ethers } from "ethers";
	import Web3Modal from "web3modal";
	import WalletConnectProvider from "@walletconnect/web3-provider";

	let debugMode = writable(false);
	let _debugMode = false;
	debugMode.subscribe((x) => (_debugMode = x));

	// TODO: Restore once updated witness is deployed
	// const clientConfig = defaultClientConfig();
	const DEMO_WITNESS_URL = "http://localhost:8787";
	const clientConfig: Types.ClientConfig = {
		endpoints: {
			instructions: `${DEMO_WITNESS_URL}/instructions`,
			statement: `${DEMO_WITNESS_URL}/statement`,
			witness_jwt: `${DEMO_WITNESS_URL}/witness_jwt`,
			witness_ld: `${DEMO_WITNESS_URL}/witness_ld`,
			verify: `${DEMO_WITNESS_URL}/verify`,
		},
	};

	export const client = new Client(
		new WasmClient(JSON.stringify(clientConfig))
	);

	const rebaseServiceKey = "rebase:did:web:rebasedemokey.pages.dev";

	// Re-constituted from dapp's stuff.

	const signer: Writable<Signer> = writable(null);
	let _signer: Signer = null;
	signer.subscribe((x) => (_signer = x));

	let nextPermissions: Array<Types.AttestationTypes> = [];

	// Consider exporting this properly.
	let allAttestationTypes: Array<Types.AttestationTypes> = [
		"BasicImageAttestation",
		"BasicPostAttestation",
		"BasicProfileAttestation",
		"BasicTagAttestation",
		"BookReviewAttestation",
		"DappPreferencesAttestation",
		"FollowAttestation",
		"LikeAttestation",
		"ProgressBookLinkAttestation",
	];

	type Route = Types.AttestationTypes | "Home" | "Credentials";
	let route: Writable<Route> = writable("Home");
	let _route: Route = "Home";
	route.subscribe((x) => (_route = x));

	// TODO: Make better typed? For now just JWTs.
	let credentials: Writable<Array<string>> = writable([]);
	let _credentials: Array<string> = [];
	credentials.subscribe((x) => (_credentials = x));

	async function issue(
		statement: Types.AttestationStatement,
		currentSigner: Signer
	) {
		let vc = await client.delegated_attestation_jwt(
			statement,
			currentSigner.subject()
		);

		if (!vc?.jwt) {
			throw new Error(`Bad format of VC: ${JSON.stringify(vc)}`);
		}

		let next = _credentials.map((x) => x);
		next.push(vc.jwt);
		credentials.set(next);
		route.set("Home");
	}

	async function connect(): Promise<void> {
		const providerOptions = {
			walletconnect: {
				package: WalletConnectProvider, // required
				options: {
					infuraId: process.env.INFURA_ID, // required
				},
			},
		};

		const web3Modal = new Web3Modal({
			cacheProvider: false,
			providerOptions,
		});
		web3Modal.clearCachedProvider();

		const instance = await web3Modal.connect();
		const provider = new ethers.providers.Web3Provider(instance);
		const s = provider.getSigner();

		if (!s) {
			throw new Error("User cancelled connection");
		}

		const ids = await provider.listAccounts();
		if (ids.length <= 0) {
			throw new Error("No ids found in ethereum connection");
		}
		const nextId = ids[0];

		let ens = { name: null, avatar: null };
		ens.name = await provider.lookupAddress(ids[0]);
		const network =
			provider.network.name === "homestead"
				? "mainnet"
				: provider.network.name;

		ens.avatar = ens.name
			? `https://metadata.ens.domains/${network}/avatar/${ens.name}`
			: null;

		const sign = async (statement: string): Promise<string> => {
			const inner_ids = await provider.listAccounts();
			if (ids[0] !== inner_ids[0]) {
				throw new Error(
					`Signer has changed on Provider's side, expected: ${ids[0]}, got ${inner_ids[0]}`
				);
			}
			return s.signMessage(statement);
		};

		const disconnect = async (): Promise<void> => {
			const providerOptions = {
				walletconnect: {
					package: WalletConnectProvider, // required
					options: {
						infuraId: process.env.INFURA_ID, // required
					},
				},
			};

			const web3Modal = new Web3Modal({
				network: "mainnet",
				cacheProvider: true,
				providerOptions,
			});

			await web3Modal.clearCachedProvider();

			return;
		};

		const subject = (): Types.Subjects => {
			return {
				pkh: {
					eip155: {
						address: _signer.id,
						chain_id: "1",
					},
				},
			};
		};

		let nextSigner: Signer = {
			disconnect,
			sign,
			id: nextId,
			subject,
			web3Provider: provider,
		};

		let now = new Date();

		let lastYear = new Date();
		lastYear.setFullYear(now.getFullYear() - 1);

		let nextCentury = new Date();
		nextCentury.setFullYear(now.getFullYear() + 100);

		let sessionConfig: Types.SessionConfig = {
			actions: {},
			address: nextId,
			chainId: 1,
			domain: window.location.host,
			issuedAt: now.toISOString(),
			notBefore: lastYear.toISOString(),
			expirationTime: nextCentury.toISOString(),
		};

		let delegatedAttestationPreConfig = await client.siwe_message(
			sessionConfig,
			rebaseServiceKey,
			nextPermissions
		);

		let sig = await nextSigner.sign(
			delegatedAttestationPreConfig.siwe_recap_message
		);

		client.set_delegated_attestation_config(
			delegatedAttestationPreConfig,
			sig
		);

		signer.set(nextSigner);
	}

	const encode = (c): string => {
		return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
	};

	const parseJWT = (jwt_str: string): any => {
		const v = jwt_str.split(".");

		if (v.length !== 3) {
			throw new Error("Invalid JWT format");
		}

		const u = v[1];
		const b64 = u.replace(/-/g, "+").replace(/_/g, "/");
		const encoded = atob(b64).split("").map(encode).join("");
		const json_str = decodeURIComponent(encoded);

		return JSON.parse(json_str);
	};

	// NOTE: Returns false so we can map, then filter against !!x
	const fmtJWT = (jwt_str: string): JWTFMT | false => {
		let jwt = parseJWT(jwt_str);
		let json = JSON.stringify(jwt, null, 2);
		let vc = jwt?.vc;
		if (!vc || typeof vc !== "object") {
			return false;
		}

		let credentialSubject = vc?.credentialSubject;
		if (!credentialSubject || typeof credentialSubject !== "object") {
			return false;
		}

		let uuid = vc?.id;
		if (!uuid) {
			return false;
		}

		let t = credentialSubject.type;
		if (
			!t ||
			!Array.isArray(t) ||
			t.length <= 0 ||
			typeof t[0] !== "string"
		) {
			return false;
		}

		let prefixedAttestationType: string = t[0];
		if (!prefixedAttestationType.startsWith("Delegated")) {
			return false;
		}

		let attestationType = prefixedAttestationType.slice(
			"Delegated".length
		) as Types.AttestationTypes;

		if (!allAttestationTypes.includes(attestationType)) {
			return false;
		}

		if (
			!credentialSubject?.id ||
			typeof credentialSubject.id !== "string"
		) {
			return false;
		}

		let v = credentialSubject.id.split(":");
		if (v.length < 2) {
			return false;
		}

		let address = v[v.length - 1];
		let chain_id = v[v.length - 2];

		let subject: Types.Subjects = {
			pkh: {
				eip155: {
					address,
					chain_id,
				},
			},
		};

		let result: JWTFMT = {
			type: attestationType,
			json,
			uuid,
			details: null,
		};

		switch (attestationType) {
			case "BasicImageAttestation":
				if (!credentialSubject?.src) {
					return false;
				}

				result.details = {
					BasicImageAttestation: {
						src: credentialSubject.src,
						subject,
					},
				};
				break;
			case "BasicPostAttestation":
				if (!credentialSubject?.body) {
					return false;
				}

				if (!credentialSubject?.title) {
					return false;
				}

				result.details = {
					BasicPostAttestation: {
						body: credentialSubject.body,
						title: credentialSubject.title,
						reply_to: credentialSubject?.reply_to ?? null,
						subject,
					},
				};
				break;
			case "BasicProfileAttestation":
				if (!credentialSubject?.username) {
					return false;
				}

				if (!credentialSubject?.website) {
					return false;
				}

				result.details = {
					BasicProfileAttestation: {
						username: credentialSubject.username,
						website: credentialSubject.website,
						image: credentialSubject?.image ?? null,
						description: credentialSubject?.description ?? null,
						subject,
					},
				};
				break;
			case "BasicTagAttestation":
				if (!credentialSubject?.post) {
					return false;
				}

				if (!credentialSubject?.users) {
					return false;
				}

				result.details = {
					BasicTagAttestation: {
						users: credentialSubject.users,
						post: credentialSubject.post,
						subject,
					},
				};
				break;
			case "BookReviewAttestation":
				if (!credentialSubject?.link) {
					return false;
				}

				if (!credentialSubject?.review) {
					return false;
				}

				if (!credentialSubject?.title) {
					return false;
				}

				result.details = {
					BookReviewAttestation: {
						link: credentialSubject.link,
						rating: credentialSubject?.rating ?? 0,
						review: credentialSubject.review,
						title: credentialSubject.title,
						subject,
					},
				};
				break;
			case "DappPreferencesAttestation":
				result.details = {
					DappPreferencesAttestation: {
						dark_mode: credentialSubject?.dark_mode ?? false,
						subject,
					},
				};
				break;
			case "FollowAttestation":
				if (!credentialSubject?.target) {
					return false;
				}

				result.details = {
					FollowAttestation: {
						target: credentialSubject.target,
						subject,
					},
				};
				break;
			case "LikeAttestation":
				if (!credentialSubject?.target) {
					return false;
				}

				result.details = {
					LikeAttestation: {
						target: credentialSubject.target,
						subject,
					},
				};
				break;
			case "ProgressBookLinkAttestation":
				if (!credentialSubject?.link) {
					return false;
				}

				result.details = {
					ProgressBookLinkAttestation: {
						link: credentialSubject.link,
						progress: credentialSubject?.rating ?? 0,
						subject,
					},
				};
				break;
			default:
				return false;
		}

		return result;
	};
</script>

<main>
	<h1>Rebase ReCap Example</h1>
	<p>Debug Mode is {_debugMode ? "on" : "off"}</p>
	<p>
		<button on:click={() => debugMode.set(!_debugMode)}
			>Turn {_debugMode ? "off" : "on"} debug mode</button
		>
	</p>
	{#if !_signer}
		<p>
			Before signing in, please select what permissions you wish to
			delegate.
		</p>
		{#each allAttestationTypes as t}
			<label
				><input
					type="checkbox"
					bind:group={nextPermissions}
					value={t}
				/>
				{t}</label
			>
		{/each}

		<button disabled={nextPermissions.length === 0} on:click={connect}
			>Sign In</button
		>
	{:else}
		<p>Signed in as: {_signer.id}</p>
		<p>
			You have {_credentials.length ?? "no"} credential{_credentials.length !==
			1
				? "s"
				: ""}
		</p>
		{#if _route === "Home"}
			{#if _debugMode}
				{#each allAttestationTypes as perm}
					<p>
						<button on:click={() => route.set(perm)}
							>Issue new {perm}</button
						>
					</p>
				{/each}
			{:else}
				{#each nextPermissions as perm}
					<p>
						<button on:click={() => route.set(perm)}
							>Issue new {perm}</button
						>
					</p>
				{/each}
			{/if}
		{:else if _route === "Credentials"}
			{#if _credentials.length === 0}
				<p>No credentials, go to the home screen and generate some!</p>
			{/if}
			{#each _credentials.map(fmtJWT).filter((x) => !!x) as c}
				{#if c}
					<CredentialDisplay
						credential={c}
						allCredentials={_credentials
							.map(fmtJWT)
							.filter((x) => !!x)}
					/>
				{/if}
			{/each}
		{:else}
			<AttestationForm
				signer={_signer}
				attestationType={_route}
				{issue}
			/>
		{/if}

		{#if _credentials.length > 0}
			<p>
				<button on:click={() => route.set("Credentials")}
					>View Credentials</button
				>
			</p>
		{/if}
		<p>
			<button on:click={() => route.set("Home")}>Back to Home</button>
		</p>
	{/if}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
