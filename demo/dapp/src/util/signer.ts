import { ethers } from "ethers";
import Web3Modal from "web3modal";

export interface Signer {
    sign: (statement: string) => Promise<string>;
    id: () => string;
};

export type SignerType = "ethereum"; // | "tezos" | "solana" | "etc"

export type SignerMap = Record<SignerType, Record<string, Signer>>;

export const connectSigner = async (signerType: SignerType): Promise<Signer> => {
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

            const sign = async (statement: string): Promise<string> => {
                return s.signMessage(statement)
            };

            const id = (): string => ids[0];

            return {sign, id};

        default:
            throw new Error(`Unknown signerType: ${signerType}`);
    }
};