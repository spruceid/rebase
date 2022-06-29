import { writable, Writable } from "svelte/store";
import { GlobeIcon, TwitterIcon, GitHubIcon } from 'components/icons';
import type { Claim } from "./claim";
import { connectSigner, connectSigner2nd, disconnectSigner, disconnectSigner2nd, Signer, SignerMap, SignerType } from "./signer";
import type { KeyType, Workflow } from "./witness";
import { copyObjArray } from "./util";

// TODO: Break into UI file?
export type AccountState = "available" | "obtained";

// TODO: Break into UI file?
export const iconColor = "#625ff5";

// The UI element for poping toast-like alerts
export const alert: Writable<{
    message: string;
    variant: 'error' | 'warning' | 'success' | 'info';
}> = writable<{
    message: string;
    variant: 'error' | 'warning' | 'success' | 'info';
}>(null);

export let witnessState: Writable<Workflow> = writable("statement");

const defaultClaims: Claim[] = [
    {
        credentials: [],
        credential_type: "twitter",
        icon: TwitterIcon,
        title: "Twitter",
        type: "public",
        available: true,
    },
    {
        credentials: [],
        credential_type: "github",
        icon: GitHubIcon,
        title: "Github",
        type: "public",
        available: true,
    },
    {
        credentials: [],
        credential_type: "dns",
        icon: GlobeIcon,
        title: "DNS",
        type: "public",
        available: true,
    },
    {
        credentials: [],
        credential_type: "self_signed",
        icon: GlobeIcon,
        title: "Ethereum Account",
        type: "public",
        available: true,
    },
    // {
    //     credentials: [],
    //     credential_type: "discord",
    //     icon: DiscordIcon,
    //     title: "Discord",
    //     type: "public",
    //     available: false,
    // },

    // {
    //     credentials: [],
    //     credential_type: "ethereum",
    //     icon: EthereumIcon,
    //     title: "Ethereum Account",
    //     type: "blockchain",
    //     available: false,
    // },

    // {
    //     credentials: [],
    //     credential_type: "solana",
    //     icon: SolanaIcon,
    //     title: "Solana Account",
    //     type: "blockchain",
    //     available: false,
    // },
]
export let claims: Writable<Array<Claim>> = writable(copyObjArray(defaultClaims));

export let currentType: Writable<SignerType> = writable("ethereum");
export let currentType2nd: Writable<SignerType> = writable("ethereum");
export let _currentType: SignerType = "ethereum";
currentType.subscribe(x => (_currentType = x));
export let _currentType2nd: SignerType = "ethereum";
currentType.subscribe(x => (_currentType = x));

export let signerMap: Writable<SignerMap> = writable({
    "ethereum": false,
});
export let signerMap2nd: Writable<SignerMap> = writable({
    "ethereum": false,
});
export let _signerMap: SignerMap = {
    "ethereum": false,
};
export let _signerMap2nd: SignerMap = {
    "ethereum": false,
};
signerMap.subscribe(x => (_signerMap = x));
signerMap2nd.subscribe(x => (_signerMap2nd = x));

export let signer: Signer | false = false;
currentType.subscribe(x => (signer = _signerMap[x]));
signerMap.subscribe(x => (signer = x[_currentType]));

export let signer2nd: Signer | false = false;
currentType2nd.subscribe(x => (signer2nd = _signerMap2nd[x]));
signerMap2nd.subscribe(x => (signer2nd = x[_currentType2nd]));

export const getKeyType = (): KeyType => {
    if (!signer) {
        throw new Error("No signer set");
    }

    switch (_currentType) {
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

export const getKeyType2nd = (): KeyType => {
    if (!signer2nd) {
        throw new Error("No 2nd signer set");
    }

    switch (_currentType) {
        case "ethereum":
            return {
                pkh: {
                    eip155: {
                        address: signer2nd.id(),
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

export const connect2nd = async (): Promise<void> => {
    let s = await connectSigner2nd(_currentType2nd);
    let next = Object.assign({}, _signerMap2nd);

    next[_currentType2nd] = s;

    signerMap2nd.set(next);
}


export const disconnect = async (): Promise<void> => {
    if (!_signerMap[_currentType]) {
        return
    }

    let next = Object.assign({}, _signerMap);
    next[_currentType] = false;
    signerMap.set(next);
    claims.set(copyObjArray(defaultClaims));
    await disconnectSigner(_currentType);
};

export const disconnect2nd = async (): Promise<void> => {
    if (!_signerMap2nd[_currentType2nd]) {
        return
    }

    let next = Object.assign({}, _signerMap2nd);
    next[_currentType2nd] = false;
    signerMap2nd.set(next);

    await disconnectSigner2nd(_currentType2nd);
};

export const sign = async (statement: string): Promise<string> => {
    let s = _signerMap[_currentType];
    if (!s) {
        throw new Error(`No signer for current type: ${_currentType}`);
    }

    return s.sign(statement);
}

export const sign2nd = async (statement: string): Promise<string> => {
    let s = _signerMap2nd[_currentType2nd];
    if (!s) {
        throw new Error(`No signer for current type: ${_currentType2nd}`);
    }

    return s.sign(statement);
}