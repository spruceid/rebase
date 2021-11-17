import { SignerType, Provider, signClaim as innerSignClaim } from './signer/index';
import {
  ClaimInfo,
  ClaimLocation,
  PublicClaimInfo,
  SignedClaim,
  isPublicWitnessType,
  toUnsignedClaim,
} from './public_claim';

export interface Opts {
  // If a string is given, this issuer is used for all public witness credentials
  // Assumed to follow the Rebase Issuer route pattern.
  // TODO: Elaborate / codify route pattern.
  // If an object is given, it must have a full url for each of the supported
  // PublicWitnessTypes, and will be used accordingly.
  // If not supplied, SpruceID's hosted witness is used.
  issuer?: string | { [index: string]: string };
}

const defaultIssuer = 'http://localhost:8787/';

const defaultOpts = {
  issuer: defaultIssuer,
};

export default class Client {
  issuer: string | { [index: string]: string };

  constructor(opts?: Opts) {
    let next: Opts = defaultOpts;
    if (opts) {
      next = { ...defaultOpts, ...opts };
    }

    if (!next?.issuer) {
      throw new Error('Invalid Configuration: No issuer');
    }

    this.issuer = next.issuer;
  }

  // Sign claim from Claim Info in such a way the remote server
  // is able to reconstruct everthing for validation purposes.
  signClaim = async (
    claimInfo: ClaimInfo,
    provider: Provider,
  ): Promise<SignedClaim<ClaimInfo>> => {
    if (!isPublicWitnessType(claimInfo.type)) {
      throw new Error(`Unknown claim type: ${claimInfo.type}`);
    }

    const unsigned = toUnsignedClaim(claimInfo, provider.type);
    const signed = await innerSignClaim(unsigned, provider);
    const full = `${unsigned}${signed}`;
    return {
      // TODO: Impl
      credentialSubjectId: '',
      info: claimInfo,
      signed,
      unsigned,
      full,
    };
  };

  toPublicClaim = (info: ClaimInfo, location: ClaimLocation): PublicClaimInfo => {
    if (info.type !== location.type) {
      throw new Error(`Types ${info.type} and ${location.type} are mismatched`);
    }
    return {
      ...info,
      location,
    };
  };

  toSignedPublicClaim = (
    signedClaim: SignedClaim<ClaimInfo>,
    location: ClaimLocation,
  ): SignedClaim<PublicClaimInfo> => {
    const {
      credentialSubjectId, full, signed, unsigned, info,
    } = signedClaim;
    const next = this.toPublicClaim(info, location);
    return {
      credentialSubjectId,
      full,
      signed,
      unsigned,
      info: next,
    };
  };

  async issuePublicClaimVC(message: SignedMessage<PublicClaimInfo>): Promise<Credential> {

  }
}
