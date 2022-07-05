import { ethers } from "ethers";
import Web3Modal from "web3modal";
import WalletConnectProvider from "@walletconnect/web3-provider";

export interface Signer {
    ens: ENSType;
    provider: ethers.providers.Web3Provider;
    sign: (statement: string) => Promise<string>;
    id: () => string;
};

export type SignerType = "ethereum"; // | "tezos" | "solana" | "etc"
export const signerTypes: Array<SignerType> = ["ethereum"];

export type SignerMap = Record<SignerType, Signer | false>;
export type ENSType = {
    name: string | null;
    avatar: string | null;
};


export const connectSigner = async (signerType: SignerType): Promise<Signer> => {
    let ens: ENSType;
    let sign: (statement: string) => Promise<string>;
    let id: () => string;
    switch (signerType) {
        case "ethereum":
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

            ens = { name: null, avatar: null };
            ens.name = await provider.lookupAddress(ids[0]);
            const network =
                provider.network.name === "homestead"
                    ? "mainnet"
                    : provider.network.name;

            ens.avatar = ens.name
                ? `https://metadata.ens.domains/${network}/avatar/${ens.name}`
                : null;


            sign = async (statement: string): Promise<string> => {
                return s.signMessage(statement)
            };

            id = (): string => ids[0];

            return { sign, id, provider, ens };

        default:
            throw new Error(`Unknown signerType: ${signerType}`);
    }
};

export const connectSigner2nd = async (signerType: SignerType): Promise<Signer> => {
    let ens: ENSType;
    let sign: (statement: string) => Promise<string>;
    let id: () => string;
    switch (signerType) {
        case "ethereum":
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

            ens = { name: null, avatar: null };
            ens.name = await provider.lookupAddress(ids[0]);
            const network =
                provider.network.name === "homestead"
                    ? "mainnet"
                    : provider.network.name;

            ens.avatar = ens.name
                ? `https://metadata.ens.domains/${network}/avatar/${ens.name}`
                : null;

            sign = async (statement: string): Promise<string> => {
                return s.signMessage(statement)
            };

            id = (): string => ids[0];

            return { sign, id, provider, ens };

        default:
            throw new Error(`Unknown signerType: ${signerType}`);
    }
};

export const disconnectSigner = async (signerType: SignerType): Promise<void> => {
    switch (signerType) {
        case "ethereum":
            const providerOptions = {
                /* See Provider Options Section */
            };

            const web3Modal = new Web3Modal({
                network: "mainnet",
                cacheProvider: true,
                providerOptions,
            });

            await web3Modal.clearCachedProvider();

            return;
        default:
            throw new Error(`Unknown signerType: ${signerType}`);
    }
}

export const disconnectSigner2nd = async (signerType: SignerType): Promise<void> => {
    switch (signerType) {
        case "ethereum":
            const providerOptions = {
                /* See Provider Options Section */
            };

            const web3Modal = new Web3Modal({
                network: "mainnet",
                cacheProvider: true,
                providerOptions,
            });

            await web3Modal.clearCachedProvider();

            return;
        default:
            throw new Error(`Unknown signerType: ${signerType}`);
    }
}