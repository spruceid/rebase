import { ethers } from "ethers";
import { Types } from "@spruceid/rebase-client";

export interface Signer {
    id: string;
    disconnect: () => Promise<void>;
    sign: (statement: string) => Promise<string>;
    subject: () => Types.Subjects;
    web3Provider: ethers.providers.Web3Provider;
}
export interface JWTFMT {
    type: Types.AttestationTypes;
    details: Types.AttestationStatement;
    uuid: string;
    json: string;
};
