// TODO: Add NFT / POAP icons.
import type { DiscordIcon, EmailIcon, EthereumIcon, ImageIcon, TwitterIcon, GitHubIcon, GlobeIcon, SolanaIcon, RedditIcon, RibbonIcon, SoundCloudIcon } from 'src/components/icons';
import { parseJWT } from './jwt';
import { FlowType } from '@spruceid/rebase-client';

export type ClaimType = "self_attested" | "blockchain" | "public";

export type ClaimIcon = typeof TwitterIcon 
    | typeof EthereumIcon 
    | typeof EmailIcon
    | typeof DiscordIcon 
    | typeof GitHubIcon
    | typeof SolanaIcon
    | typeof GlobeIcon
    | typeof ImageIcon
    | typeof RibbonIcon
    | typeof RedditIcon
    | typeof SoundCloudIcon;

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
        case "EmailVerification": {
            let email = vc?.credentialSubject?.sameAs;
            let did = vc?.credentialSubject?.id;
            let address = did.split(":")[did.split(":").length - 1]
            return {
                type: "basic_public",
                handle: email,
                address
            }
        }
        case "SameControllerAssertion": {
            let key1 = vc?.credentialSubject?.id1;
            let key2 = vc?.credentialSubject?.id2;
            let handle = key1.split(":")[key1.split(":").length - 1]
            let address = key2.split(":")[key2.split(":").length - 1]

            return {
                type: "basic_public",
                handle,
                address,
            }
        }
        case "NftOwnershipVerification": {
            let did = vc?.credentialSubject?.id;
            if (!did) {
                throw new Error("No credentialSubject.id found");
            }

            let address = did.split(":")[did.split(":").length - 1]
            let contract = vc?.credentialSubject?.owns_asset_from;
            if (!contract) {
                throw new Error("No credentialSubject.owns_asset_from found");
            }
            let timeOf = vc?.evidence?.timestamp;
            if (!timeOf) {
                throw new Error("No evidence.timestamp found");
            }

            let handle = `NFT from contract ${contract} owned at ${timeOf}`;

            return {
                type: "basic_public",
                address,
                handle,
            };
    }
    case "PoapOwnershipVerification": {
        let did = vc?.credentialSubject?.id;
        if (!did) {
            throw new Error("No credentialSubject.id found");
        }

        let address = did.split(":")[did.split(":").length - 1]
        let event_id = vc?.evidence?.event_id;
        if (!event_id) {
            throw new Error("No credentialSubject.event_id found");
        }

        let timeOf = vc?.evidence?.timestamp;
        if (!timeOf) {
            throw new Error("No evidence.timestamp found");
        }

        let handle = `POAP from event ${event_id} owned at ${timeOf}`;

        return {
          type: "basic_public",
          address,
          handle,
        };

    }
    case "BasicProfileAttestation": {
        let did = vc?.credentialSubject?.id;
        if (!did) {
            throw new Error("No credentialSubject.id found");
        }
        let address = did.split(":")[did.split(":").length - 1]
        let username = vc?.credentialSubject?.username;
        if (!username) {
            throw new Error("No credentialSubject.username found")
        }
        return {
          type: "basic_public",
          address,
          handle: username,
        };
    }
        default:
            throw new Error(`Unsupported credential type: ${t[1]}`)
    }
}

export type Claim = {
    // NOTE: we could use object instead of string for credential, but for now, assume a JWT
    credentials: Array<string>,
    credential_type: FlowType,
    icon: ClaimIcon,
    title: string,
    type: ClaimType,
    available: boolean,
}