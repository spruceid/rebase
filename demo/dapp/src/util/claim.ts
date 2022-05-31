import type { DiscordIcon, EthereumIcon, TwitterIcon, GitHubIcon, SolanaIcon } from '../components/icons';

export type ClaimType = "self_attested" | "blockchain" | "public";
export type Issuer = "witness" | "local";
export type CredentialType = "twitter" | "discord" | "github" | "dns"
export type ClaimIcon = typeof TwitterIcon 
    | typeof EthereumIcon 
    | typeof DiscordIcon 
    | typeof GitHubIcon
    | typeof SolanaIcon;

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

export const credentialToDisplay = (credential: string): CredentialDisplay => {
    // TODO: Gracefully handle errs!
    let j = JSON.parse(credential);
    let t = j?.type;
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
            let handle = j?.evidence?.handle;
            let did = j?.credentialSubject?.id;
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
    issuer: Issuer,
    title: string,
    type: ClaimType,
}