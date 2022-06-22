import { ethers } from "ethers";
import Web3Modal from "web3modal";

export interface Signer {
    sign: (statement: string) => Promise<string>;
    id: () => string;
};

export type SignerType = "ethereum"; // | "tezos" | "solana" | "etc"
export const signerTypes: Array<SignerType> = ["ethereum"];

export type SignerMap = Record<SignerType, Signer | false>;


export const connectSigner = async (signerType: SignerType): Promise<Signer> => {
    let sign;
    let id;
    switch (signerType) {
        case "ethereum":
            const providerOptions = {
                /* See Provider Options Section */
            };

            const web3Modal = new Web3Modal({
                network: "mainnet",
                cacheProvider: false,
                providerOptions,
            });

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

            sign = async (statement: string): Promise<string> => {
                return s.signMessage(statement)
            };

            id = (): string => ids[0];

            return { sign, id };

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