import { writable, Writable } from "svelte/store";
import { GlobeIcon, KeyIcon, TwitterIcon, EmailIcon, GitHubIcon, RedditIcon, SoundCloudIcon, ImageIcon, RibbonIcon } from 'src/components/icons';
import type { Claim } from "./claim";
import { connectedCount, connectSigner, disconnectSigner, retrieveSignerEntry, getAllConnected, Signer,  SignerType, SignerMap, newSignerMap, SignerQuery, signWith, retrieveSigner, ProviderType, toQuery } from "./signer";
import type { Workflow } from "./witness";
import { Subjects } from "@spruceid/rebase-client";

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
            credential_type: "TwitterVerification",
            icon: TwitterIcon,
            title: "Twitter",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "GitHubVerification",
            icon: GitHubIcon,
            title: "GitHub",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "DnsVerification",
            icon: GlobeIcon,
            title: "DNS",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "EmailVerification",
            icon: EmailIcon,
            title: "Email",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "SameControllerAssertion",
            icon: KeyIcon,
            title: "Same Controller",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "RedditVerification",
            icon: RedditIcon,
            title: "Reddit",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "SoundCloudVerification",
            icon: SoundCloudIcon,
            title: "SoundCloud",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "NftOwnershipVerification",
            icon: ImageIcon,
            title: "NFT Ownership",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "PoapOwnershipVerification",
            icon: RibbonIcon,
            title: "POAP Ownership",
            type: "public",
            available: true,
        },
        {
            credentials: [],
            credential_type: "Attestation",
            icon: GlobeIcon,
            title: "Basic Profile",
            type: "public",
            available: true,
        }
    ]
}

export let claims: Writable<Array<Claim>> = writable(defaultClaims());

export const getSubject = (signer: Signer): Subjects => {
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