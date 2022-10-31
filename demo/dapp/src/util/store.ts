import { writable, Writable } from "svelte/store";
import { GlobeIcon, KeyIcon, TwitterIcon, GitHubIcon, RedditIcon, SoundCloudIcon } from 'src/components/icons';
import type { Claim } from "./claim";
import { connectedCount, connectSigner, disconnectSigner, retrieveSignerEntry, getAllConnected, Signer,  SignerType, SignerMap, newSignerMap, SignerQuery, signWith, retrieveSigner, ProviderType, toQuery } from "./signer";
import type { KeyType, Workflow } from "./witness";

// TODO: Break into UI file?
export type AccountState = "available" | "obtained";

// TODO: Break into UI file?
export const iconColor = "#625ff5";

export let signerMap: Writable<SignerMap> = writable(newSignerMap());
export let _signerMap: SignerMap = newSignerMap();
signerMap.subscribe((x) => (_signerMap = x));

// NOTE: This may get removed?
export let lookUp: Writable<SignerQuery> = writable(null);
export let _lookUp: SignerQuery = null;
lookUp.subscribe((x) => (_lookUp = x));

signerMap.subscribe((x) => {
    if (connectedCount(x) === 1) {
        lookUp.set(toQuery(getAllConnected(x)[0]));
    }

    if (_lookUp) {
        let e = retrieveSignerEntry(x, _lookUp);
        if (!e || !e.active) {
            lookUp.set(null);
        }
    }
});

// The UI element for poping toast-like alerts
export const alert: Writable<{
    message: string;
    variant: 'error' | 'warning' | 'success' | 'info';
}> = writable<{
    message: string;
    variant: 'error' | 'warning' | 'success' | 'info';
}>(null);

export let witnessState: Writable<Workflow> = writable("signer");

function defaultClaims(): Claim[] { 
    return [
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
            title: "GitHub",
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
            icon: KeyIcon,
            title: "Two Key Self Signed",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "reddit",
            icon: RedditIcon,
            title: "Reddit",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "soundcloud",
            icon: SoundCloudIcon,
            title: "SoundCloud",
            type: "public",
            available: true,
        }
        // {
        //     credentials: [],
        //     credential_type: "discord",
        //     icon: DiscordIcon,
        //     title: "Discord",
        //     type: "public",
        //     available: false,
        // },
    ]
}

export let claims: Writable<Array<Claim>> = writable(defaultClaims());

export const getKeyType = (signer: Signer): KeyType => {
    if (!signer) {
        throw new Error("Please connect your wallet");
    }

    switch (signer.signerType) {
        case "ethereum":
            return {
                pkh: {
                    eip155: {
                        address: signer.id,
                        chain_id: "1",
                    },
                },
            }
        case "solana": {
            return {
                pkh: {
                    solana: {
                        address: signer.id
                    }
                }
            }
        }
    };
};

// TODO: Make _providerType a store global?
export const connect = async (signerType: SignerType, providerType: ProviderType): Promise<void> => {
    let [nextMap, nextSigner] = await connectSigner(signerType, providerType,  _signerMap);
    signerMap.set(nextMap);
    lookUp.set(toQuery(nextSigner));
}

export const disconnect = async (query: SignerQuery): Promise<void> => {
    let signer = retrieveSigner(_signerMap, query);
    if (signer) {
        let nextMap = await disconnectSigner(signer, _signerMap);
        signerMap.set(nextMap)
    }
};

export const disconnectAll = async (): Promise<void> => {
    let all = getAllConnected(_signerMap);
    all.forEach((s) => {
        let q = toQuery(s);
        disconnect(q);
    });
    lookUp.set(null);
}

export const sign = async (statement: string): Promise<string> => {
    if (!_lookUp) {
        throw new Error(`No signer currently set to active`);
    }

    return signWith(statement, _signerMap, _lookUp);
}