export type Credential = {
  '@context': string
  | Record<string, unknown>
  | Array<string | Record<string, unknown>>;
  id?: string;
  type: string | Array<string>;
  credentialSubject: {
    id?: string;
    [index: string]: unknown;
  };
  issuer?: string | {
    id: string;
    [index: string]: unknown;
  };
  issuanceDate?: string;
  proof?: ClaimProof | Array<ClaimProof>;
  expirationDate?: string;
  credentialStatus?: {
    id: string;
    type: string;
    [index: string]: unknown;
  };
  termsOfUse?: Array<{
    id?: string;
    type: string;
    [index: string]: unknown;
  }>;
  evidence?: {
    id?: string;
    type: Array<string>;
    [index: string]: unknown;
  } | Array<{
    id?: string;
    type: Array<string>;
    [index: string]: unknown;
  }>;
  credentialSchema?: {
    id: string;
    type: string;
    [index: string]: unknown;
  } | Array<{
    id: string;
    type: string;
    [index: string]: unknown;
  }>;
  refreshService?: {
    id: string;
    type: string;
    [index: string]: unknown;
  } | Array<{
    id: string;
    type: string;
    [index: string]: unknown;
  }>;

  [index: string]: unknown;
};

export type ClaimProof = {
  '@context': string | Record<string, unknown> | Array<string | Record<string, unknown>>;
  type: string;
  proofPurpose?: 'assertionMethod'
  | 'authentication'
  | 'keyAgreement'
  | 'contractAgreement'
  | 'capabilityInvocation'
  | 'capabilityDelegation';
  proofValue?: string;
  challenge?: string;
  creator?: string;
  verificationMethod?: string;
  created?: string;
  domain?: string;
  nonce?: string;
  jws?: string;
  [index: string]: unknown;
};
