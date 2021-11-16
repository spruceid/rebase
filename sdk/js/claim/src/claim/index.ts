// TODO: FLESH OUT
export type Claim = {
    [index: string]: object | string,
}

// Generic data structure representing the needed information to make a claim 
// assumes a signerId (public key in practice)
// and a signed and unsigned version of the message.
// Other information for specific claim making can be passed in the message type param.
export interface SignedMessage<Message> {
    message: Message,
    signerId: string,
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

