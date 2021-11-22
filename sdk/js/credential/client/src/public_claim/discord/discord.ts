import { v4 as uuidV4 } from 'uuid';

import type {
  Credential,
} from '../../credential';

import {
  BaseLocation, PublicClaimData, SignedClaim,
} from '../common/common';

import { RebaseClaimType } from '../rebase/rebase_types';

export interface DiscordLocation extends BaseLocation<RebaseClaimType> {
  type: 'DiscordVerification';
  messageId: string;
  channelId: string;
}

export const RebaseDiscordVersions = [1];

export interface DiscordLocatorOpts {
  apiKey: string;
}

export function toUnsignedDiscordCredentialV1(
  signedClaim: SignedClaim<PublicClaimData<RebaseClaimType, DiscordLocation>>,
  issuer: string,
): Credential {
  if (
    signedClaim.data.location.type !== 'DiscordVerification'
      || signedClaim.data.type !== 'DiscordVerification'
  ) {
    throw new Error('Not a DiscordVerification claim');
  }

  return {
    '@context': [
      'https://www.w3.org/2018/credentials/v1',
      {
        sameAs: 'http://schema.org/sameAs',
        DiscordVerification: 'https://w3id.org/rebase/v1/DiscordVerification',
        DiscordVerificationContents: {
          '@id': 'https://w3id.org/rebase/v1/DiscordVerificationContents',
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
      sameAs: `urn:discord:${signedClaim.data.posterId}`,
    },
    evidence: {
      channelId: signedClaim.data.location.channelId,
      handle: signedClaim.data.posterId,
      messageId: signedClaim.data.location.messageId,
      timestamp: new Date().toISOString(),
      type: ['DiscordVerificationContents'],
    },
    id: `urn:uuid:${uuidV4()}`,
    issuer,
    type: ['VerifiableCredential', 'DiscordVerification'],
  };
}
