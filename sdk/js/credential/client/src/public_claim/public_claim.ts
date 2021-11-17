import { v4 as uuidV4 } from 'uuid';
import {
  JWKFromTezos,
  // prepareIssueCredential,
} from 'didkit-wasm';
import type {
  Credential,
} from '../credential';

import { SignerType } from '../signer';

import { exhaustiveCheck } from '../utils/utils';

// Types of public witness supported in default Rebase settings.
export type PublicWitnessType = 'discord' | 'twitter';

export function isPublicWitnessType(s: PublicWitnessType): boolean {
  switch (s) {
    case 'discord':
    case 'twitter':
      return true;
    default:
  }

  exhaustiveCheck(s);
  return false;
}

export interface ClaimInfo {
  type: PublicWitnessType;
  posterId: string;
  signerId: string;
}

export type ClaimLocation = DiscordLocation | TwitterLocation;

export interface PublicClaimInfo extends ClaimInfo {
  location: ClaimLocation;
}

// Generic data structure representing the needed information to make a claim
// assumes a signerId (public key in practice)
// and a signed and unsigned version of the message.
// Other information for specific claim making can be passed in the message type param.
export interface SignedClaim<Info> {
  credentialSubjectId: string;
  info: Info;
  signed: string;
  unsigned: string;
  full: string;
}

// String-creation functions shared with the rebase-issuer.
// Allows for quick recreation and validation of handles etc.
export function signerPrefix(signerType: SignerType): string {
  const t = signerType;
  switch (signerType) {
    case 'eth':
    case 'tz':
      // TODO: Use for signing, i.e. 'Tezos Signed Message', etc.
      return '';
    default:
  }

  exhaustiveCheck(signerType);
  throw new Error(`Unknown handle type: ${t}`);
}

export function posterPrefix(posterType: PublicWitnessType): string {
  const t = posterType;
  switch (posterType) {
    case 'discord':
      return '';
    case 'twitter':
      return '@';
    default:
  }

  exhaustiveCheck(posterType);
  throw new Error(`Unknown handle type: ${t}`);
}

export function seperator(posterType: PublicWitnessType): string {
  const t = posterType;
  switch (posterType) {
    case 'discord':
    case 'twitter':
      return '\n\n';
    default:
  }

  exhaustiveCheck(posterType);
  throw new Error(`Unknown handle type: ${t}`);
}

export function signerDisplay(signerType: SignerType): string {
  const t = signerType;
  switch (signerType) {
    case 'eth':
      return 'the Ethereum Address';
    case 'tz':
      return 'the Tezos Address';
    default:
  }

  exhaustiveCheck(signerType);
  throw new Error(`Unknown handle type: ${t}`);
}

// Now put it all together.
export function toUnsignedClaim(info: ClaimInfo, signerType: SignerType): string {
  const { posterId, signerId } = info;
  return `I attest ${posterPrefix(info.type)}${posterId} is linked to ${signerDisplay(signerType)} ${signerId}${seperator(info.type)}`;
}

export interface PrepareIssueCredentialOpts {
  proofOptions: string;
  keyType: string;
}

export async function issueOpts(
  pk: string,
  pkh: string,
  signerType: SignerType,
): Promise<PrepareIssueCredentialOpts> {
  let keyType = ''; let suffix = ''; let
    pkhPrefix = '';
  switch (signerType) {
    case 'eth':
      keyType = JSON.stringify({
        kty: 'EC',
        crv: 'secp256k1',
        alg: 'ES256K-R',
        key_ops: ['signTypedData'],
      });
      suffix = '#Recovery2020';
      pkhPrefix = 'eth';
      break;
    case 'tz':
      keyType = await JWKFromTezos(pk) as string;
      suffix = '#TezosMethod2021';
      pkhPrefix = 'tz';
      break;
    default:
  }

  const opts = {
    verificationMethod: `did:pkh:${pkhPrefix}:${pkh}${suffix}`,
    proofPurpose: 'assertionMethod',
  };

  if (signerType === 'eth') {
    // TODO: Add message schema opt to add here for Eth?
  }

  return {
    proofOptions: JSON.stringify(opts),
    keyType,
  };
}

// Discord Specific
export interface DiscordLocation {
  type: 'discord';
  messageId: string;
  channelId: string;
}

export function toUnsignedDiscordClaim(
  signedClaim: SignedClaim<PublicClaimInfo>,
  issuer: string,
): Credential {
  if (
    signedClaim.info.location.type !== 'discord'
      || signedClaim.info.type !== 'discord'
  ) {
    throw new Error('Not a discord message');
  }

  return {
    '@context': [
      'https://www.w3.org/2018/credentials/v1',
      {
        sameAs: 'http://schema.org/sameAs',
        DiscordVerification: 'https://w3id.org/rebase/DiscordVerification',
        DiscordVerificationContents: {
          '@id': 'https://w3id.org/rebase/DiscordVerificationContents',
          '@context': {
            '@version': 1.1,
            '@protected': true,
            handle: 'https://schema.org/text',
            timestamp: {
              '@id': 'https://schema.org/datetime',
              '@type': 'http://www.w3.org/2001/XMLSchema#dateTime',
            },
            channelId: 'https://schema.org/text',
            messageId: 'https://schema.org/text',
          },
        },
      },
    ],
    credentialSubject: {
      id: signedClaim.credentialSubjectId,
      sameAs: `urn:discord:${signedClaim.info.posterId}`,
    },
    evidence: {
      channelId: signedClaim.info.location.channelId,
      handle: signedClaim.info.posterId,
      messageId: signedClaim.info.location.messageId,
      timestamp: new Date().toISOString(),
      type: ['DiscordVerificationContents'],
    },
    id: `urn:uuid:${uuidV4()}`,
    issuer,
    type: ['VerifiableCredential', 'DiscordVerification'],
  };
}

// Twitter Specific
export interface TwitterLocation {
  type: 'twitter';
  postId: string;
}
