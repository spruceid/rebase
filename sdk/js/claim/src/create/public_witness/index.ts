import type {
    Claim, 
    // ClaimMaker, 
    // SignedMessage, 
    // SignedToClaim
} from '../../claim/index';
import {SignerType} from '../../claim/index';

// Used to typecheck up to date with imported Claim types.
// Webpack should remove it, or at least could be configured to.
const exhaustiveCheck = (arg: never) => {
  // Forces the type checker to complain if you've missed a sum type.
  // See https://dev.to/babak/exhaustive-type-checking-with-typescript-4l3f
  console.error(`Impossible value found: ${arg}`);
};

// Types of public witness supported in default Rebase settings.
export type PublicWitnessType = 'discord' | 'twitter';

// A generic interface for public witness postings, includs
// a posterId (twitter / discord handle) 
// a signerId (a public key)
// a type to case switch on.
export interface PublicWitnessInfo {
    type: PublicWitnessType,
    posterId: string,
    signerId: string,
}

// String-creation functions shared with the rebase-issuer.
// Allows for quick recreation and validation of handles etc.
export function signerPrefix(signerType: SignerType): string {
    let t = signerType;
    switch (signerType) {
        case 'eth':
        case 'tz':
            // TODO: Use for signing, i.e. 'Tezos Signed Message', etc.
            return '';
    }

    exhaustiveCheck(signerType);
    throw new Error(`Unknown handle type: ${t}`);
}

export function posterPrefix(posterType: PublicWitnessType): string {
    let t = posterType;
    switch (posterType) {
        case 'discord':
            return '';
        case 'twitter':
            return '@';
    }

    exhaustiveCheck(posterType);
    throw new Error(`Unknown handle type: ${t}`);
}

export function seperator(posterType: PublicWitnessType): string {
    let t = posterType;
    switch (posterType) {
        case 'discord':
        case 'twitter':
            return '\n\n';
    }

    exhaustiveCheck(posterType);
    throw new Error(`Unknown handle type: ${t}`);
}

export function signerDisplay(signerType: SignerType): string {
    let t = signerType;
    switch (signerType) {
        case 'eth':
            return 'the Ethereum Address';
        case 'tz':
            return 'the Tezos Address';
    }

    exhaustiveCheck(signerType);
    throw new Error(`Unknown handle type: ${t}`);
}

// Now put it all togethere.
export const toUnsignedMessage = (info: PublicWitnessInfo, signerType: SignerType): string => {
    let {posterId, signerId} = info;
    return `I attest ${posterPrefix(info.type)}${posterId} is linked to ${signerDisplay(signerType)} ${signerId}${seperator(info.type)}`;
}

// Discord Specific
export interface DiscordInfo extends PublicWitnessInfo {
    type: 'discord',
    signerType: SignerType,
    postId: string,
}

export async function makeDiscordClaim(info: DiscordInfo): Promise<Claim> {
    // TODO: Implement.
    console.log(info);
    return {}
}

// Twitter Specific





