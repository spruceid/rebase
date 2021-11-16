export type Claim = {
    '@context': string | object | Array<string | object>,
    id?: string,
    type: string | Array<string>,
    credentialSubject: {
        id?: string,
        [index: string]: any,
    },
    issuer?: string | {
        id: string,
        [index: string]: any,
    },
    issuanceDate?: string,
    proof?: ClaimProof | Array<ClaimProof>
    expirationDate?: string,
    credentialStatus?: {
        id: string,
        type: string,
        [index: string]: any,
    },
    termsOfUse?: Array<{
        id?: string,
        type: string,
        [index: string]: any,
    }>,
    evidence?: {
        id?: string,
        type: Array<string>,
        [index: string]: any,
    } | Array<{
        id?: string,
        type: Array<string>,
        [index: string]: any,
    }>,
    credentialSchema?: {
        id: string,
        type: string,
        [index: string]: any,
    } | Array<{
        id: string,
        type: string,
        [index: string]: any,
    }>
    refreshService?: {
        id: string,
        type: string,
        [index: string]: any,
    } | Array<{
        id: string,
        type: string,
        [index: string]: any,
    }>

    [index: string]: any
}


export type ClaimProof = {
    '@context': string | object | Array<string | object>,
    type: string,
    proofPurpose?: 'assertionMethod' 
    | 'authentication'
    | 'keyAgreement'
    | 'contractAgreement'
    | 'capabilityInvocation'
    | 'capabilityDelegation',
    proofValue?: string,
    challenge?: string,
    creator?: string,
    verificationMethod?: string,
    created?: string,
    domain?: string,
    nonce?: string,
    jws?: string,
    [index: string]: any
};

// Generic data structure representing the needed information to make a claim 
// assumes a signerId (public key in practice)
// and a signed and unsigned version of the message.
// Other information for specific claim making can be passed in the message type param.
export interface SignedMessage<Message> {
    credentialSubjectId: string,
    message: Message,
    signed: string,
    unsigned: string
}

// The over-arching abstraction, composed of...
export type ClaimMaker<Message> = (message: Message) => Promise<Claim>;
// ...this function follewed by ...
export type MessageSigner<Message> = (message: Message) => Promise<SignedMessage<Message>>;
// ...this function.
export type SignedToClaim<Message> = (signedMessage: SignedMessage<Message>) => Promise<Claim>

// These are explicitly defined to allow mix 'n match.
export interface ClaimMakerOpts<Message> {
    signMessage: MessageSigner<Message>,
    toClaim: SignedToClaim<Message>
}

export function newClaimMaker<Message>(opts: ClaimMakerOpts<Message>): ClaimMaker<Message> {
    return async (message: Message) => {
        let signed = await opts.signMessage(message);
        return opts.toClaim(signed);
    };
}

// Specific claim implementations as sum types.
// Should track did:pkh:n.
// Break into own file?
export type SignerType = 'eth' | 'tz';


