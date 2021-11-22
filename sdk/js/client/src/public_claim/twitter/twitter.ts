import { v4 as uuidV4 } from 'uuid';
import axios from 'axios';

import type {
  Credential,
} from '../../credential';

import {
  BaseLocation, PublicClaimData, SignedClaim,
} from '../common/common';

import { RebaseClaimType } from '../rebase/rebase_types';

export interface TwitterLocation extends BaseLocation<RebaseClaimType> {
  type: 'TwitterVerification';
  postId: string;
}

export const RebaseTwitterVersions = [1];

export interface TwitterLocatorOpts {
  apiKey: string;
}

interface TweetData {
  handle: string;
  text: string;
}

function dataFromApi(x: unknown): TweetData {
  const t: TweetData = {
    handle: '',
    text: '',
  };
  if (x && typeof x === 'object' && !Array.isArray(x)) {
    const body = x as Record<string, unknown>;
    if (
      Array.isArray(body['data'])
      && body['data'][0]
      && typeof body['data'][0] === 'object'
      && !Array.isArray(body['data'][0])
    ) {
      const textWrapper = body['data'][0] as Record<string, unknown>;
      if (typeof textWrapper['text'] === 'string') {
        t.text = textWrapper['text'];
        if (
          body['includes']
          && typeof body['includes'] === 'object'
          && !Array.isArray(body['includes'])
        ) {
          const usersWrapper = body['includes'] as Record<string, unknown>;
          if (
            Array.isArray(usersWrapper['users'])
            && usersWrapper['users'][0]
            && typeof usersWrapper['users'][0] === 'object'
            && !Array.isArray(usersWrapper['users'][0])
          ) {
            const usernameWrapper = usersWrapper['users'][0] as Record<string, unknown>;
            const { username } = usernameWrapper;
            if (!username || typeof username !== 'string') {
              throw new Error('Could not find username from Twitter API response');
            }

            t.handle = username;
            return t;
          }
          throw new Error('Could not find user in "users" in "includes" in response from Twitter APi');
        } else {
          throw new Error('Could not find "includes" in response from Twitter API');
        }
      } else {
        throw new Error('Could not find tweet text in response from Twitter API');
      }
    } else {
      throw new Error('Unexpected response format from Twitter API');
    }
  }

  throw new Error('Unexpected response format from Twitter API');
}

export async function locateTwitterClaim(
  publicClaim: PublicClaimData<RebaseClaimType, TwitterLocation>,
  apiKey: string,
): Promise<string> {
  if (publicClaim.type !== 'TwitterVerification' || publicClaim.location.type !== 'TwitterVerification') {
    throw new Error('');
  }

  const headers = {
    authorization: `Bearer: ${apiKey}`,
  };

  const idQuery = `ids=${publicClaim.location.postId}`;

  const target = `https://api.twitter.com/2/tweets?${idQuery}&expansions=author_id&user.fields=username`;
  const res = await axios.get(target, { headers });

  const tweetData = dataFromApi(res.data);

  if (tweetData.handle !== publicClaim.posterId) {
    throw new Error('Poster handle does not match claim handle');
  }

  return tweetData.text;
}

export function toUnsignedTwitterCredentialV1(
  signedClaim: SignedClaim<PublicClaimData<RebaseClaimType, TwitterLocation>>,
  issuer: string,
): Credential {
  if (
    signedClaim.data.location.type !== 'TwitterVerification'
      || signedClaim.data.type !== 'TwitterVerification'
  ) {
    throw new Error('Not a twitter claim');
  }

  return {
    '@context': [
      'https://www.w3.org/2018/credentials/v1',
      {
        sameAs: 'http://schema.org/sameAs',
        TwitterVerification: 'https://w3id.org/rebase/v1/TwitterVerification',
        TwitterVerificationPublicTweet: {
          '@id': 'https://w3id.org/rebase/v1/TwitterVerificationPublicTweet',
          '@context': {
            '@version': 1.1,
            '@protected': true,
            handle: 'https://schema.org/text',
            timestamp: {
              '@id': 'https://schema.org/datetime',
              '@type': 'http://www.w3.org/2001/XMLSchema#dateTime',
            },
            tweetId: 'https://schema.org/text',
          },
        },
      },
    ],
    credentialSubject: {
      id: signedClaim.credentialSubjectId,
      sameAs: `https://twitter.com/${signedClaim.data.posterId}`,
    },
    evidence: {
      handle: signedClaim.data.posterId,
      timestamp: new Date().toISOString(),
      tweetId: signedClaim.data.signerId,
      type: ['TwitterVerifciationPublicTweet'],
    },
    id: `urn:uuid:${uuidV4()}`,
    issuer,
    type: ['VerifiableCredential', 'TwitterVerification'],
  };
}
