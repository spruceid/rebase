import {
  JWKFromTezos,
  // prepareIssueCredential,
} from 'didkit-wasm';

import {
  Kepler, authenticator, Action, getOrbitId,
} from 'kepler-sdk';

import { SignerType } from '../signer';
import { Credential } from '../credential';

import { exhaustiveCheck } from '../utils/utils';

import { DiscordLocation, DiscordLocatorOpts, toUnsignedDiscordCredentialV1 } from './discord/discord';
import {
  locateTwitterClaim, TwitterLocation, TwitterLocatorOpts, toUnsignedTwitterCredentialV1,
} from './twitter/twitter';
import {
  ClaimData, SignedClaim, PublicClaimData,
} from './common/common';

import { RebaseClaimType } from './rebase/rebase_types';

export type RebaseClaim = ClaimData<RebaseClaimType>;
export type RebasePublicClaim = PublicClaimData<RebaseClaimType, RebaseClaimLocation>;

// The default implementation of public claim to verifiable credential workflow used by Client.
// This implementation is meant to interact with the companion server library.

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

export function posterPrefix(claim: RebaseClaim): string {
  const t = claim.type;
  switch (claim.type) {
    case 'DiscordVerification':
      return '';
    case 'TwitterVerification':
      return '@';
    default:
  }

  exhaustiveCheck(claim.type);
  throw new Error(`Unknown handle type: ${t}`);
}

export function seperator(claim: RebaseClaim): string {
  const t = claim.type;
  switch (claim.type) {
    case 'DiscordVerification':
    case 'TwitterVerification':
      return '\n\n';
    default:
  }

  exhaustiveCheck(claim.type);
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

export function toUnsignedClaim(
  data: ClaimData<RebaseClaimType>,
  signerType: SignerType,
): string {
  const { posterId, signerId } = data;
  return `I attest ${posterPrefix(data)}${posterId} is linked to ${signerDisplay(signerType)} ${signerId}${seperator(data)}`;
}
// Local signing functions:

export interface PrepareIssueCredentialOpts {
  proofOptions: string;
  keyType: string;
}

export async function issueOpts(
  pk: string,
  pkh: string,
  signerType: SignerType,
): Promise<PrepareIssueCredentialOpts> {
  let keyType = '';
  let suffix = '';
  let pkhPrefix = '';
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

// default locateClaim:

// Location types which match RebaseClaimType
export type RebaseClaimLocation = DiscordLocation | TwitterLocation;

export interface RebaseClaimIssuerOpts {
  issuer: string;
  discord: DiscordLocatorOpts;
  twitter: TwitterLocatorOpts;
}

export type RebaseClaimLocator = (signedClaim: SignedClaim<RebasePublicClaim>) => Promise<string>;

function makeRebaseClaimLocator(
  opts: RebaseClaimIssuerOpts,
): RebaseClaimLocator {
  return (signedClaim: SignedClaim<RebasePublicClaim>) => {
    const t = signedClaim.data.type;
    switch (signedClaim.data.type) {
      case 'DiscordVerification':
      // TODO: Implement!
        throw new Error('IMPLEMENT');
      case 'TwitterVerification':
        if (signedClaim.data.location.type === 'TwitterVerification') {
          return locateTwitterClaim(
            signedClaim.data as PublicClaimData<'TwitterVerification', TwitterLocation>,
            opts.twitter.apiKey,
          );
        }
        throw new Error('Mismatched location and claim type');
      default:
    }

    exhaustiveCheck(signedClaim.data.type);
    throw new Error(`Unknown claim type: ${t}`);
  };
}

export type RebaseClaimWitness = (signedClaim: SignedClaim<RebasePublicClaim>)
=> Promise<Credential>;

function makeRebaseClaimWitness(locator: RebaseClaimLocator, issuer: string): RebaseClaimWitness {
  return async (signedClaim: SignedClaim<RebasePublicClaim>): Promise<Credential> => {
    if (signedClaim.full !== `${signedClaim.unsigned}${signedClaim.signed}`) {
      throw new Error('SignedMessage.full must be the concatination of .unsigned and .signed');
    }

    if (toUnsignedClaim(signedClaim.data, signedClaim.signerType) !== signedClaim.unsigned) {
      throw new Error('SignedMessage.unsigned should match toUnsignedClaim(signedClaim.data, signedClaim.signerType)');
    }

    const retrievedClaim = await locator(signedClaim);
    if (signedClaim.full !== retrievedClaim) {
      throw new Error('Signed message does not match located public claim text');
    }

    // TODO: Impl switch on signerType here to validate!

    const t = signedClaim.data.type;
    switch (signedClaim.data.type) {
      case 'DiscordVerification':
        // TODO: Switch on version (?)
        // TODO: Sign it here (?)
        return toUnsignedDiscordCredentialV1(
          signedClaim as SignedClaim<PublicClaimData<RebaseClaimType, DiscordLocation>>,
          issuer,
        );
      case 'TwitterVerification':
        // TODO: Switch on version (?)
        // TODO: Sign it here (?)
        return toUnsignedTwitterCredentialV1(
          signedClaim as SignedClaim<PublicClaimData<RebaseClaimType, TwitterLocation>>,
          issuer,
        );
      default:
    }

    exhaustiveCheck(signedClaim.data.type);
    throw new Error(`Unknown claim type: ${t}`);
  };
}

export function makeWitness(opts: RebaseClaimIssuerOpts): RebaseClaimWitness {
  const locator = makeRebaseClaimLocator(opts);
  return makeRebaseClaimWitness(locator, opts.issuer);
}
