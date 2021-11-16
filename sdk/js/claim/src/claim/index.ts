// TODO: FLESH OUT
export type Claim = {
    [index: string]: object | string,
}

export interface SignedMessage<Message> {
    message: Message,
    signerId: string,
    signed: string,
    unsigned: string
}


export type ClaimMaker<Message> = (message: Message) => Promise<Claim>;
export type MessageSigner<Message> = (message: Message) => Promise<SignedMessage<Message>>;
export type SignedToClaim<Message> = (signedMessage: SignedMessage<Message>) => Promise<Claim>

// These are explicitly defined to allow mix 'n match.
export interface ClaimMakerOpts<Message> {
    signMessage: MessageSigner<Message>,
    toClaim: SignedToClaim<Message>
}

export type SignerType = 'eth' | 'tz';

export function newClaimMaker<Message>(opts: ClaimMakerOpts<Message>): ClaimMaker<Message> {
    return async (message: Message) => {
        let signed = await opts.signMessage(message);
        return opts.toClaim(signed);
    };
}

