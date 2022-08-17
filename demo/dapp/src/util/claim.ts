import type { DiscordIcon, EthereumIcon, TwitterIcon, GitHubIcon, GlobeIcon, SolanaIcon, RedditIcon, SoundCloudIcon } from 'components/icons';
import { parseJWT } from './jwt';

export type ClaimType = "self_attested" | "blockchain" | "public";
export type CredentialType = "twitter" | "discord" | "github" | "dns" | "self_signed" | "reddit" | "soundcloud";
export type ClaimIcon = typeof TwitterIcon 
    | typeof EthereumIcon 
    | typeof DiscordIcon 
    | typeof GitHubIcon
    | typeof SolanaIcon
    | typeof GlobeIcon
    | typeof RedditIcon
    | typeof SoundCloudIcon;

export type BasicPublic = {
    type: "basic_public"
    handle: string
    address: string
}

// TODO: Use this for cross key?
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
        case "RedditVerification":
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
        case "SoundCloudVerification":
        {
            let handle = vc?.evidence?.permalink;
            let did = vc?.credentialSubject?.id;
            let address = did.split(":")[did.split(":").length - 1]
            return {
                type: "basic_public",
                handle,
                address
            }
        }
        case "DnsVerification": {
            let domain = vc?.credentialSubject?.sameAs;
            let did = vc?.credentialSubject?.id;
            let address = did.split(":")[did.split(":").length - 1]
            return {
                type: "basic_public",
                handle: domain.replace("dns:", ""),
                address
            }
        }
        case "SelfSignedControl": {
            let key1 = vc?.credentialSubject?.id;
            let key2 = vc?.credentialSubject?.sameAs;
            let handle = key1.split(":")[key1.split(":").length - 1]
            let address = key2.split(":")[key2.split(":").length - 1]

            return {
                type: "basic_public",
                handle,
                address,
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
    available: boolean,
}