import axios from 'axios';
import {
  verifyCredential,
} from 'didkit-wasm';
import { Provider, signClaim as innerSignClaim, getDID } from './signer/index';
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

type VerifyResult = {
  errors: Array<string>;
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
      credentialSubjectId: await getDID(provider),
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
    const info = this.toPublicClaim(signedClaim.info, location);
    return {
      ...signedClaim,
      ...{ info },
    };
  };

  async issuePublicClaimVC(signedClaim: SignedClaim<PublicClaimInfo>): Promise<Credential> {
    let targetUrl = '';
    if (typeof this.issuer === 'string') {
      targetUrl = this.issuer;
    } else {
      const temp = this.issuer[signedClaim.info.type];
      if (!temp) {
        throw new Error(`No issuer for claim of type: ${signedClaim.info.type}`);
      }
      targetUrl = temp;
    }

    const res = await axios.post(
      targetUrl,
      JSON.stringify(signedClaim),
      { headers: 'application/json' },
    );
    if (res.status !== 200) {
      throw new Error(`Failed request, ${res.statusText}`);
    }

    const verifyOptionsString = '{}';
    const verifyResult = JSON.parse(
      await verifyCredential(JSON.stringify(res.data), verifyOptionsString),
    ) as VerifyResult;

    if (!verifyResult?.errors) {
      throw new Error('Invalid result from verify credential');
    }

    if (verifyResult.errors.length > 0) {
      const errorMessage = `Unable to verify credential: ${verifyResult.errors.join(
        ', ',
      )}`;
      throw new Error(errorMessage);
    }

    return res.data as Credential;
  }
}
