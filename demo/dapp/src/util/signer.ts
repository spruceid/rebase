import { ethers } from "ethers";
import { EthereumIcon, SolanaIcon } from "src/components";
import Web3Modal from "web3modal";
import WalletConnectProvider from "@walletconnect/web3-provider";

export type SignerIcon = typeof EthereumIcon | typeof SolanaIcon;

export type SignerType = "ethereum" | "solana"; // | "tezos" | "bitcoin" | "etc"
export const signerTypes: Array<SignerType> = ["ethereum", "solana"];

export function signerTypeToIcon(signerType: SignerType): SignerIcon {
    switch(signerType) {
        case "ethereum": 
            return EthereumIcon;
        case "solana":
            return SolanaIcon;
    }
}

export function displaySignerType(signerType: SignerType): string {
    switch (signerType) {
        case "ethereum":
            return "Ethereum"
        case "solana": 
            return "Solana"
        default: 
            throw new Error(`Unknown signerType ${signerType}`);
    }
}

export type EthereumProvider = "metamask" | "wallet_connect";
export type SolanaProvider = "phantom";
export type ProviderType = EthereumProvider | SolanaProvider;

export const providerTypes: Record<SignerType, Array<ProviderType>> =  {
    "ethereum": ["metamask", "wallet_connect"],
    "solana": ["phantom"]
};

export function displayProviderType(providerType: ProviderType): string {
    switch (providerType) {
        case "metamask":
            return "MetaMask";
        case "wallet_connect": 
            return "WalletConnect";
        case "phantom": 
            return "Phantom";
        default: 
            throw new Error(`Unknown providerType ${providerType}`) 
    }
}

export interface SignerDisplay {
    signerType: SignerType,
    id: string
}

export function displaySignerId(signerOrQuery: SignerDisplay): string {
    switch (signerOrQuery.signerType) {
        case "ethereum":
            return `${signerOrQuery.id.slice(0, 14)}...${signerOrQuery.id.slice(-12)}`
        case "solana":
            return `${signerOrQuery.id.slice(0, 12)}...${signerOrQuery.id.slice(-12)}`
        default: 
            throw new Error(`Unknown signerType ${signerOrQuery.signerType}`);
    }
}

export function displaySigner(signerOrQuery: SignerDisplay): string {
    return `${displaySignerType(signerOrQuery.signerType)}: ${displaySignerId(signerOrQuery)}`
}

export interface SignerQuery extends SignerDisplay {
    providerType: ProviderType,
}

export function compareQueries(q1: SignerQuery, q2: SignerQuery): boolean {
    return (
        q1.id === q2.id &&
        q1.signerType === q2.signerType &&
        q1.providerType === q2.providerType
    );
}
export interface BaseSigner extends SignerQuery {
    icon: SignerIcon,
    disconnect: () => Promise<void>;
    sign: (statement: string) => Promise<string>;
};

export interface EthereumSigner extends BaseSigner {
    ens: ENSType,
    providerType: EthereumProvider,
    signerType: "ethereum",
    web3Provider: ethers.providers.Web3Provider
}

export interface SolanaSigner extends BaseSigner {
    providerType: SolanaProvider,
    signerType: "solana"
}

export type Signer = EthereumSigner | SolanaSigner;

export function toQuery(signer: Signer): SignerQuery {
    const {id, signerType, providerType} = signer;

    return {
        id,
        signerType,
        providerType
    }
}
export interface SignerEntry {
    signer: Signer,
    active: boolean
}

export type SignerMap = Record<SignerType, Record<ProviderType, Array<SignerEntry>>>;

export function newSignerMap(): SignerMap {
    let s = {};
    const outerTypes = Object.keys(providerTypes) as Array<SignerType>;
    outerTypes.forEach((st) => {
        s[st] = {};
        const innerTypes = providerTypes[st];
        innerTypes.forEach((it) => {
            s[st][it] = [];
        })
    })

    return s as SignerMap;
}

export type ConnectedSignersByType = Record<SignerType, Array<Signer>>;

export function connectedSignersByType(signerMap: SignerMap): ConnectedSignersByType {
    let result = {};
    Object.keys(signerMap).forEach((key) => {
        if (!result[key]) {
            result[key] = [];
        }

        Object.keys(signerMap[key]).forEach((innerKey) => {
            let arr = signerMap[key][innerKey];
            result[key].concat(arr);
        });
    });

    return result as ConnectedSignersByType;
}

export function connectedCount(signerMap: SignerMap): number {
    const r = connectedSignersByType(signerMap);
    const keys = Object.keys(r);
    let acc = 0;
    keys.forEach((k) => {
        acc += r[k];
    });

    return acc;
}

export function getAllConnected(signerMap: SignerMap): Array<Signer> {
    return getAllEntries(signerMap).filter((x) => x.active).map((x) => x.signer);
}

export function getAllEntries(signerMap: SignerMap): Array<SignerEntry> {
    let acc: Array<SignerEntry> = [];
    Object.keys(signerMap).forEach((key) => {
        Object.keys(signerMap[key]).forEach((innerKey) => {
            signerMap[key][innerKey].forEach((entry) => {
                acc.push(entry);
            })
        })
    });

    return acc;
}

export type ENSType = {
    name: string | null;
    avatar: string | null;
};

declare global {
    interface Window {
        phantom: {solana: any, ethereum: any};
    }
}

const isPhantom = window?.phantom?.solana?.isPhantom;

async function connectEthereum(): Promise<Signer> {
    const providerOptions = {
        walletconnect: {
            package: WalletConnectProvider, // required
            options: {
                infuraId: process.env.INFURA_ID // required
            }
        }
    };

    const web3Modal = new Web3Modal({
        cacheProvider: false,
        providerOptions,
    });
    web3Modal.clearCachedProvider()

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
            throw new Error(`Signer has changed on Provider's side, expected: ${ids[0]}, got ${inner_ids[0]}`)
        }
        return s.signMessage(statement)
    };


    const disconnect = async (): Promise<void> => {
        const providerOptions = {
            walletconnect: {
                package: WalletConnectProvider, // required
                options: {
                    infuraId: process.env.INFURA_ID // required
                }
            }
        };

        const web3Modal = new Web3Modal({
            network: "mainnet",
            cacheProvider: true,
            providerOptions,
        });

        await web3Modal.clearCachedProvider();

        return;
    }

    return { 
        disconnect,
        sign, 
        id: ids[0], 
        signerType: "ethereum",
        icon: EthereumIcon,
        // TODO: Break into provider detection fn?
        providerType:  provider?.connection?.url === 'metamask' ? 'metamask' : 'wallet_connect',
        web3Provider: provider,
        ens 
    } as EthereumSigner;
}

async function connectSolana(): Promise<Signer> {
    // TODO: Change when using other provider types.
    const providerType: SolanaProvider | false = isPhantom ? "phantom" : false;
    if (isPhantom) {
        const solSigner = window.phantom.solana;
        await solSigner.connect();
        const id = solSigner.publicKey.toString();
        const sign = async (statement: string): Promise<string> => {
            // Note: Future proofing
            switch (providerType) {
                case "phantom": 
                    if (!window?.phantom?.solana?.publicKey?.toString) {
                        throw new Error("Could not find solana provider instanace");
                    }
                    let got = window.phantom.solana.publicKey.toString();
                    if (id !== got) {
                        throw new Error(`Signer has changed on Provider's side, expected ${id}, got: ${got}`)                
                    }
                    break;
                default:
                    throw new Error(`Unknown Provider: ${providerType}`)
            }
            

            let o = await solSigner.signMessage(
                new TextEncoder().encode(statement),
                "utf-8"
            )

            return [...new Uint8Array(o.signature.buffer)]
                .map(x => x.toString(16).padStart(2, '0'))
                .join('');
        }

        const disconnect = async () => {
            await solSigner.disconnect();
        }

        return {
            disconnect,
            sign,
            id,
            icon: SolanaIcon,
            signerType: "solana",
            providerType,
        } as SolanaSigner;
    }

    throw new Error(`Failed to find Phantom wallet, other wallets currently unsupported`)
}

export async function connectSignerType(signerType: SignerType, _providerType: ProviderType): Promise<Signer> {
    // NOTE: Provider type is currently unused, but could be important.
    switch (signerType) {
        case "ethereum":
            return connectEthereum();
        case "solana":
            return connectSolana();
        default:
            throw new Error(`Unknown signerType: ${signerType}`);
    }
};

export function signerMapAppend(nextSigner: Signer, signerMap: SignerMap): SignerMap {
    if (!signerMap[nextSigner.signerType]) {
        throw new Error(`Improperly formatted signerMap, no top-level entry for Signer Type: ${nextSigner.signerType}`);
    }

    if (!signerMap[nextSigner.signerType][nextSigner.providerType]) {
        throw new Error(`Improperly formatted signerMap, no entry in ${nextSigner.signerType} for provider ${nextSigner.providerType}`);
    }

    if (!Array.isArray(signerMap[nextSigner.signerType][nextSigner.providerType])) {
        throw new Error(`Improperly formatted signerMap, entry at ${nextSigner.signerType} for provider ${nextSigner.providerType} is not an Array`);
    }

    const arr = signerMap[nextSigner.signerType][nextSigner.providerType];
    const nextId = nextSigner.id;

    let next = [];
    let found = false;
    arr.forEach((entry) => {
        let active = false;
        if (entry.signer.id === nextId) {
            found = true;
            active = true;
        } else {
            active = entry.active;
        }
        next.push(Object.assign({}, entry, {active}))
    })

    if (!found) {
        const newEntry: SignerEntry = {
            signer: nextSigner,
            active: true
        };

        next.push(newEntry);
    }

    signerMap[nextSigner.signerType][nextSigner.providerType] = next;

    return signerMap
}

// Note: Mutates the given signerMap, then returns the mutated var.
export async function connectSigner(signerType: SignerType, providerType: ProviderType, signerMap: SignerMap): Promise<[SignerMap, Signer]> {
    const nextSigner = await connectSignerType(signerType, providerType);
    return [signerMapAppend(nextSigner, signerMap), nextSigner];
}

// Note: Mutates the given signerMap, then returns the mutated var.
export async function disconnectSigner(signer: Signer, signerMap: SignerMap): Promise<SignerMap> {
    let {id} = signer;
    await signer.disconnect();

    let arr = signerMap[signer.signerType][signer.providerType];
    let next = [];
    arr.forEach((entry) => {
        let cont = {};

        if (entry.signer.id === id) {
            cont["active"] = false;
        }

        next.push(Object.assign({}, entry, cont));
    })
    
    signerMap[signer.signerType][signer.providerType] = next;

    return signerMap;
}



export function retrieveSignerEntry(signerMap: SignerMap, query: SignerQuery): SignerEntry | false {
    const arr = signerMap[query.signerType][query.providerType];

    for (let i = 0, x = arr.length; i < x; i++) {
        let entry = arr[i];
        if (entry.signer.id === query.id) {
            return entry;
        }
    }

    return false;
}

export function retrieveSigner(signerMap: SignerMap, query: SignerQuery): Signer | false {
    let entry = retrieveSignerEntry(signerMap, query);
    return entry && entry.signer
}

export async function signWith(statement: string, signerMap: SignerMap, query: SignerQuery): Promise<string> {
    let s = retrieveSigner(signerMap, query);
    if (!s) {
        throw new Error(`Failed to find sender of type ${query.signerType}, provider ${query.providerType}, and id ${query.id}`);
    }

    return s.sign(statement);
}