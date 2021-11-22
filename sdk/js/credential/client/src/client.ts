import axios from 'axios';
import {
  verifyCredential,
} from 'didkit-wasm';
import { authenticator, Kepler, getOrbitId } from 'kepler-sdk';
import {
  Provider, signClaim as innerSignClaim, getClaimAddress, getDID, SignerType,
} from './signer/index';
import {
  ClaimData,
  RebaseClaimLocation,
  PublicClaimData,
  RebaseClaimType,
  SignedClaim,
  isRebaseClaimType,
  toUnsignedClaim,
  BaseLocation,
} from './public_claim';
import { TzSigner } from './signer/tz/tz';

// Used to marshal DIDKit's result to.
type VerifyResult = {
  errors: Array<string>;
};

// Used for default options to create client of type <RebaseClaimType, RebaseClaimLocation>
// compatable with the companion server lib.
function castToPublicClaimClaimData(
  data: ClaimData<unknown>,
): ClaimData<RebaseClaimType> {
  const t = data?.type;
  if (typeof t === 'string' && isRebaseClaimType(t as RebaseClaimType)) {
    return {
      ...data,
      type: t as RebaseClaimType,
    };
  }

  const errDetail = typeof t === 'string' ? `had value of ${t}` : `had type of ${typeof t}`;
  throw new Error(`ClaimData.type was unexpected, ${errDetail}, unsupported by the default handler`);
}
export interface Opts<ClaimType, ClaimLocation> {
  // If a string is given, this issuer is used for all public witness credentials
  // Assumed to follow the Rebase Issuer route pattern.
  // TODO: Elaborate / codify route pattern.
  // If an object is given, it must have a full url for each of the supported
  // RebaseClaimTypes, and will be used accordingly.
  // If not supplied, SpruceID's hosted witness is used.
  issuer?: string | { [index: string]: string };
  keplerHost?: string;
  locateClaim?: (claim: ClaimLocation) => Promise<string>;
  toUnsignedClaim?: (claim: ClaimData<ClaimType>, signer: SignerType) => string;
}

// TODO: Change to hosted public verifier once implemented.
const defaultIssuer = 'http://localhost:8787/rebase';
// TODO: Change to hosted public verifier once implemented.
const defaultKeplerHost = 'https://localhost:9999';

// If no opts are passed, provides glue code for typing, assumes
const defaultToUnsignedClaim = (data: ClaimData<unknown>, signer: SignerType): string => {
  const next = castToPublicClaimClaimData(data);
  return toUnsignedClaim(next, signer);
};

const defaultOpts = {
  issuer: defaultIssuer,
  keplerHost: defaultKeplerHost,
  toUnsignedClaim: defaultToUnsignedClaim,
};

export default class Client<
  ClaimType extends string,
  ClaimLocation extends BaseLocation<ClaimType>,
> {
  issuer: string | { [index: string]: string };

  keplerHost: string;

  keplerMap: Record<string, Kepler> = {};

  toUnsignedClaim: (data: ClaimData<ClaimType>, signer: SignerType) => string;

  constructor(opts?: Opts<ClaimType, ClaimLocation>) {
    let next: Opts<ClaimType, ClaimLocation>;
    if (opts) {
      next = opts;
    } else {
      next = defaultOpts;
    }

    if (!next?.issuer) {
      throw new Error('Invalid Configuration: No issuer');
    }

    if (!next?.toUnsignedClaim) {
      throw new Error('Invalid Configuration: No toUnsignedClaim function');
    }

    if (!next?.keplerHost) {
      throw new Error('Invalid Configuration: No kepler host');
    }

    this.issuer = next.issuer;
    this.keplerHost = next.keplerHost;
    this.toUnsignedClaim = next.toUnsignedClaim;
  }

  // Sign claim from ClaimData in such a way the remote server
  // is able to reconstruct everthing for validation purposes.
  signClaim = async (
    claimData: ClaimData<ClaimType>,
    provider: Provider,
  ): Promise<SignedClaim<ClaimData<ClaimType>>> => {
    const unsigned = this.toUnsignedClaim(claimData, provider.type);
    const signed = await innerSignClaim(unsigned, provider);
    const full = `${unsigned}${signed}`;
    return {
      credentialSubjectId: await getDID(provider),
      signerType: provider.type,
      data: claimData,
      signed,
      unsigned,
      full,
    };
  };

  toPublicClaim = (
    data: ClaimData<ClaimType>,
    location: ClaimLocation,
  ): PublicClaimData<ClaimType, ClaimLocation> => {
    if (data.type !== location.type) {
      throw new Error(`Types ${data.type} and ${location.type} are mismatched`);
    }
    return {
      ...data,
      location,
    };
  };

  toSignedPublicClaim = (
    signedClaim: SignedClaim<ClaimData<ClaimType>>,
    location: ClaimLocation,
  ): SignedClaim<PublicClaimData<ClaimType, ClaimLocation>> => {
    const data = this.toPublicClaim(signedClaim.data, location);
    return {
      ...signedClaim,
      ...{ data },
    };
  };

  async claimToCredential(
    signedClaim: SignedClaim<PublicClaimData<ClaimType, ClaimLocation>>,
  ): Promise<Credential> {
    let targetUrl = '';
    if (typeof this.issuer === 'string') {
      targetUrl = `${
        this.issuer.endsWith('/')
          ? this.issuer.slice(0, -1)
          : this.issuer
      }/v${signedClaim.data.version}/${signedClaim.data.type}`;
    } else {
      const temp = this.issuer[signedClaim.data.type];
      if (!temp) {
        throw new Error(`No issuer for claim of type: ${signedClaim.data.type}`);
      }
      targetUrl = `${
        temp.endsWith('/')
          ? temp.slice(0, -1)
          : temp
      }/v${signedClaim.data.version}/${signedClaim.data.type}`;
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

  async saveSignedClaim(
    signedClaim: SignedClaim<ClaimData<ClaimType>>,
    location: ClaimLocation,
    provider: Provider,
  ): Promise<{
      credential: Credential;
      cid: string;
    }> {
    const addr = await getClaimAddress(provider);
    if (signedClaim.data.signerId !== addr) {
      throw new Error('signedClaim.data.signerId must match the provider claim address');
    }

    const next = this.toSignedPublicClaim(signedClaim, location);
    const cred = await this.claimToCredential(next);

    if (!this.keplerMap[addr]) {
      const t = next.signerType;
      switch (next.signerType) {
        case 'eth':
          // TODO: IMPLEMENT!
          throw new Error('IMPLEMENT');
        case 'tz': {
          const p = provider as TzSigner;
          this.keplerMap[next.credentialSubjectId] = new Kepler(
            this.keplerHost,
            await authenticator(p.provider.client, this.keplerHost),
          );
          break;
        }
        default:
          throw new Error(`Unrecognized signer type ${t}`);
      }
    }

    const k = this.keplerMap[addr];
    if (!k) {
      throw new Error(`No Kepler instance found for signer: ${addr}`);
    }

    // TODO: Make sure that signerId can universally be a pkh.
    const id = await getOrbitId(
      addr, {
        domain: this.keplerHost,
        index: 0,
      },
    );

    let res = await k.put(id, cred);
    if (!(res.status === 200 || res.status === 404)) {
      throw new Error(`Failed to save to orbit: ${res.statusText}`);
    }

    if (res.status === 404) {
      res = await k.createOrbit(cred);
      if (!res.ok || res.status !== 200) {
        throw new Error(`Failed to create orbit: ${res.statusText}`);
      }
    }

    const cid = await res.text();
    return { credential: cred, cid };
  }
}

/**
 * defaultClient creates a Client configured to interact the server library
 * provided and defaults to the Spruce ID Public Issuer
 * @returns An initialized client ready to work with the Spruce ID Public Issuer.
 */
export const defaultClient = (): Client<RebaseClaimType, RebaseClaimLocation> => new Client({
  issuer: defaultIssuer,
  toUnsignedClaim,
});
