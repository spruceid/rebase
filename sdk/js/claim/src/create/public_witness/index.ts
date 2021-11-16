import type {Claim, ClaimMaker, SignedMessage, SignedToClaim} from '../../claim/index';
import {SignerType} from '../../claim/index';

export type ToPost<Info, Message> = (info: Info) => Promise<Message>
export type PublicWitnessType = 'discord' | 'twitter';

export const exhaustiveCheck = (arg: never) => {
  // Forces the type checker to complain if you've missed a sum type.
  // See https://dev.to/babak/exhaustive-type-checking-with-typescript-4l3f
};

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

export function handlePrefix(handleType: PublicWitnessType): string {
    let t = handleType;
    switch (handleType) {
        case 'discord':
            return '';
        case 'twitter':
            return '@';
    }

    exhaustiveCheck(handleType);
    throw new Error(`Unknown handle type: ${t}`);
}

export function seperator(handleType: PublicWitnessType): string {
    let t = handleType;
    switch (handleType) {
        case 'discord':
        case 'twitter':
            return '\n\n';
    }

    exhaustiveCheck(handleType);
    throw new Error(`Unknown handle type: ${t}`);
}

export function signerDisplay(signerType: SignerType): string {
    let t = signerType;
    switch (signerType) {
        case 'eth':
            return 'Ethereum Address';
        case 'tz':
            return 'Tezos Address';
    }

    exhaustiveCheck(signerType);
    throw new Error(`Unknown handle type: ${t}`);
}

export interface PublicWitnessInfo {
    type: PublicWitnessType,
    handle: string,
    signerId: string,
}

export const toUnsignedMessage = (info: PublicWitnessInfo, signerType: SignerType): string => {
    let {handle, signerId} = info;
    return `I attest the account ${handlePrefix(info.type)}${handle} is linked to the ${signerDisplay(signerType)} ${signerId}${seperator(info.type)}`;
}
export interface DiscordMessage extends PublicWitnessInfo {
    type: 'discord',
    unsignedMessage: string,
    message: string,
    postId: string,
}



// Twitter Specific





