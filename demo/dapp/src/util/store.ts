import { writable, Writable } from "svelte/store";
import { DiscordIcon, EthereumIcon, GlobeIcon, TwitterIcon, GitHubIcon, SolanaIcon } from '../components/icons';
import type {Claim} from "./claim";
import {connectSigner, disconnectSigner, Signer, SignerMap, SignerType} from "./signer";
import type { KeyType, Workflow } from "./witness";

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
    {
        credentials: [], 
        credential_type: "dns",
        icon: GlobeIcon,
        title: "DNS",
        type: "public"
    },
    {
        credentials: [],
        credential_type: "github",
        icon: GitHubIcon,
        title: "Github",
        type: "public"
    },
    {
        credentials: [],
        credential_type: "self_signed",
        icon: GlobeIcon,
        title: "Self Signed",
        type: "public"
    }
]);

export let currentType: Writable<SignerType> = writable("ethereum");
export let _currentType: SignerType = "ethereum";
currentType.subscribe(x => (_currentType = x));

export let signerMap: Writable<SignerMap> = writable({"ethereum": false});
export let _signerMap: SignerMap = {"ethereum": false};
signerMap.subscribe(x => (_signerMap = x));

export let signer: Signer | false = false;
currentType.subscribe(x => (signer = _signerMap[x]));
signerMap.subscribe(x => (signer = x[_currentType]));

export const getKeyType = (): KeyType => {
    if (!signer) {
        throw new Error("No signer set");
    }

    switch (_currentType)  {
        case "ethereum": 
        return {
            pkh: {
                eip155: {
                    address: signer.id(),
                    chain_id: "1",
                },
            },
        }
    };
};

export const connect = async (): Promise<void> => {
    let s = await connectSigner(_currentType);
    let next = Object.assign({}, _signerMap);

    next[_currentType] = s;

    signerMap.set(next);
}

export const disconnect = async (): Promise<void> => {
    if (!_signerMap[_currentType]) {
        return
    }

    let next = Object.assign({}, _signerMap);
    next[_currentType] = false;
    signerMap.set(next);

    await disconnectSigner(_currentType);
};

export const sign = async (statement: string): Promise<string> => {
    let s = _signerMap[_currentType];
    if (!s) {
        throw new Error(`No signer for current type: ${_currentType}`);
    }
    
    return s.sign(statement);
}