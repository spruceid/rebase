import type { DiscordIcon, EthereumIcon, TwitterIcon, GitHubIcon, GlobeIcon, SolanaIcon } from '../components/icons';
import { parseJWT } from './jwt';

export type ClaimType = "self_attested" | "blockchain" | "public";
export type CredentialType = "twitter" | "discord" | "github" | "dns"
export type ClaimIcon = typeof TwitterIcon 
    | typeof EthereumIcon 
    | typeof DiscordIcon 
    | typeof GitHubIcon
    | typeof SolanaIcon
    | typeof GlobeIcon;

export type BasicPublic = {
    type: "basic_public"
    handle: string
    address: string
}

export type BasicBlockchain = {
    type: "basic_blockchain"
    address: string
}

export type CredentialDisplay = BasicPublic | BasicBlockchain;

export const credentialToDisplay = (jwt: string): CredentialDisplay => {
    // TODO: Gracefully handle errs!
    let j = parseJWT(jwt);

    let vc = j?.vc;
    if (!vc) {
        throw new Error("Malformed jwt, no vc property")
    }

    let t = vc?.type;
    if (!t) {
        throw new Error("Malformed credential, no type property")
    }

    if (t.length !== 2)  {
        throw new Error("Malformed credential, type property did not have length of 2")
    }

    switch(t[1]) {
        case "TwitterVerification": 
        case "GitHubVerification":
        {
            let handle = vc?.evidence?.handle;
            let did = vc?.credentialSubject?.id;
            let address = did.split(":")[did.split(":").length - 1]
            return {
                type: "basic_public",
                handle,
                address
            }
        }
        default:
            throw new Error(`Unsupported credential type: ${t[1]}`)
    }
}

export type Claim = {
    // NOTE: we could use object instead of string for credential, but for now, assume a JWT
    credentials: Array<string>,
    credential_type: CredentialType,
    icon: ClaimIcon,
    title: string,
    type: ClaimType,
}