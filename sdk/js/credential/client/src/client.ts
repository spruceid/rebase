import { SignerType } from './signer/index';
import { ClaimInfo, isPublicWitnessType, toUnsignedClaim } from './public_claim';

export interface Opts {
    // If a string is given, this issuer is used for all public witness credentials
    // Assumed to follow the Rebase Issuer route pattern.
    // TODO: Elaborate / codify route pattern.
    // If an object is given, it must have a full url for each of the supported
    // PublicWitnessTypes, and will be used accordingly.  
    // If not supplied, SpruceID's hosted witness is used.
   issuer?: string | { [index: string]: string } 
};

const defaultIssuer = 'http://localhost:8787/'

const defaultOpts = {
    issuer: defaultIssuer,
};

export default class Client {
    issuer: string | { [index: string]: string };
    constructor(opts?: Opts) {
        let next: Opts = defaultOpts; 
        if (!opts) {
            next = Object.assign({}, defaultOpts, opts);;
        }

        if (!next?.issuer) {
            throw new Error('Invalid Configuration: No issuer')
        }

        this.issuer = next.issuer;
    }

    unsignedClaim(claimInfo: ClaimInfo, signerType: SignerType): string {
        if (!isPublicWitnessType(claimInfo.type)) {
            throw new Error(`Unknown claim type: ${claimInfo.type}`)
        }

        return toUnsignedClaim(claimInfo, signerType);
    }
};