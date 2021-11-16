import type {
    Claim, 
    SignedMessage, 
} from '../../claim/index';

import {SignerType} from '../../claim/index';

import {v4 as uuidV4} from 'uuid';

// Used to typecheck up to date with imported Claim types.
// See https://dev.to/babak/exhaustive-type-checking-with-typescript-4l3f
const exhaustiveCheck = (arg: never) => arg;

// Types of public witness supported in default Rebase settings.
export type PublicWitnessType = 'discord' | 'twitter';

// A generic interface for public witness postings, includs
// a posterId (twitter / discord handle) 
// a signerId (a public key)
// a type to case switch on.
export interface PublicWitnessInfo {
    posterId: string,
    posterType: PublicWitnessType,
    signerId: string,
    signerType: SignerType,
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
export function toUnsignedMessage(info: PublicWitnessInfo, signerType: SignerType): string {
    let {posterId, signerId} = info;
    return `I attest ${posterPrefix(info.posterType)}${posterId} is linked to ${signerDisplay(signerType)} ${signerId}${seperator(info.posterType)}`;
}

// Discord Specific
export interface DiscordInfo extends PublicWitnessInfo {
    type: 'discord',
    messageId: string,
    channelId: string
}

export async function toDiscordClaim(signedMessage: SignedMessage<DiscordInfo>, issuer: string): Promise<Claim> {
    return {
        "@context": [
          "https://www.w3.org/2018/credentials/v1",
          {
              "sameAs": "http://schema.org/sameAs",
              "DiscordVerification": "https://w3id.org/rebase/DiscordVerification",
              "DiscordVerificationContents": {
                  "@id": "https://w3id.org/rebase/DiscordVerificationContents",
                  "@context": {
                      "@version": 1.1,
                      "@protected": true,
                      "handle": "https://schema.org/text",
                      "timestamp": {
                          "@id": "https://schema.org/datetime",
                          "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                      },
                      "channelId": "https://schema.org/text",
                      "messageId": "https://schema.org/text"
                  }
              }
          }
        ],
        credentialSubject: {
            id: signedMessage.credentialSubjectId,
            sameAs: `urn:discord:${signedMessage.message.posterId}`
        },
        evidence: {
            channelId: signedMessage.message.channelId,
            handle: signedMessage.message.posterId,
            messageId: signedMessage.message.messageId,
            timestamp: new Date().toISOString(),
            type: ['DiscordVerificationContents']
        },
        id: `urn:uuid:${uuidV4()}`,
        issuer,
        // TODO: MAKE PROOF BASED ON SIGNER TYPE:
        type: ['VerifiableCredential', 'DiscordVerification']
    };
}

// Twitter Specific





