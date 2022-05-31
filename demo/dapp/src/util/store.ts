import { writable, Writable } from "svelte/store";
import { DiscordIcon, EthereumIcon, TwitterIcon, GitHubIcon, SolanaIcon } from '../components/icons';
import type {Claim} from "./claim";
import type {Signer} from "./signer";
import type { Workflow } from "./witness";

// TODO: Break into UI file?
export type AccountState = "available" | "obtained";

// TODO: Break into UI file?
export const iconColor = "#625ff5";

export let accountState: Writable<AccountState> = writable("obtained");

export let witnessState: Writable<Workflow> = writable("statement");

export let claims: Writable<Array<Claim>> = writable([
    {
        credentials: [],
        credential_type: "twitter",
        icon: TwitterIcon,
        issuer: "witness",
        title: "Twitter",
        type: "public"
    },
    {
        credentials: [], 
        credential_type: "discord",
        icon: DiscordIcon,
        issuer: "witness",
        title: "Discord",
        type: "public"
    },
    {
        credentials: [],
        credential_type: "github",
        icon: GitHubIcon,
        issuer: "witness",
        title: "Github",
        type: "public"
    }
]);

// TODO: IMPL
export let signer: Writable<Signer> = writable(false);

// TODO: On signer change, populate claims if possible.