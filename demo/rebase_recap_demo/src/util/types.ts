import { ethers } from "ethers";
import { AttestationStatement, AttestationTypes, Subjects } from "@spruceid/rebase-client";

export interface Signer {
    id: string;
    disconnect: () => Promise<void>;
    sign: (statement: string) => Promise<string>;
    subject: () => Subjects;
    web3Provider: ethers.providers.Web3Provider;
}
export interface JWTFMT {
    type: AttestationTypes;
    details: AttestationStatement;
    uuid: string;
    json: string;
};
