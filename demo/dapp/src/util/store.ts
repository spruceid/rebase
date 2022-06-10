import { writable, Writable } from "svelte/store";
import { DiscordIcon, EthereumIcon, GlobeIcon, TwitterIcon, GitHubIcon, SolanaIcon } from '../components/icons';
import type {Claim} from "./claim";
import {connectSigner, Signer, SignerMap, SignerType} from "./signer";
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
        title: "Twitter",
        type: "public"
    },
    // {
    //     credentials: [], 
    //     credential_type: "discord",
    //     icon: DiscordIcon,
    //     title: "Discord",
    //     type: "public"
    // },
    // {
    //     credentials: [], 
    //     credential_type: "dns",
    //     icon: GlobeIcon,
    //     title: "Discord",
    //     type: "public"
    // },
    {
        credentials: [],
        credential_type: "github",
        icon: GitHubIcon,
        title: "Github",
        type: "public"
    }
]);

export let signerMap: Writable<SignerMap> = writable({"ethereum": {}});
export let _signerMap: SignerMap = {"ethereum": {}};
signerMap.subscribe(x => (_signerMap = x));

export let currentSigner: Writable<[SignerType, Signer]> = writable(null);

export const connectNewSigner = async (signerType: SignerType): Promise<void> => {
    let signer = await connectSigner(signerType);
    let next = Object.assign({}, _signerMap);

    next[signerType][signer.id()] = signer;

    signerMap.set(next);
    currentSigner.set([signerType, signer]);
}

export const signWith = async (signerType: SignerType, id: string, statement: string): Promise<string> => {
    let t = _signerMap[signerType];
    let s = t[id];
    if (!s) {
        throw new Error(`Failed to find a ${signerType} signer at id: ${id}`)
    }
    return s.sign(statement);
}