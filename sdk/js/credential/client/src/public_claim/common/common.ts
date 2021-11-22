// TODO: Make signer type a passable param
import { SignerType } from '../../signer';

export interface ClaimData<T> {
  type: T;
  posterId: string;
  signerId: string;
  version?: number;
}

// BaseLocation type must match ClaimData type
export interface BaseLocation<T extends string> {
  type: T;
}

export interface PublicClaimData<Type, Location> extends ClaimData<Type> {
  location: Location;
}

// Generic data structure representing the needed information to make a claim
// assumes a signerId (public key in practice)
// and a signed and unsigned version of the message.
// Other information for specific claim making can be passed in the message type param.
export interface SignedClaim<Data> {
  credentialSubjectId: string;
  data: Data;
  full: string;
  signed: string;
  // TODO: Generalize over!
  signerType: SignerType;
  unsigned: string;
}
