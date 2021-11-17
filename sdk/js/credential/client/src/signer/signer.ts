import { RequestSignPayloadInput, SigningType } from '@airgap/beacon-sdk';
import { TzSigner, getMichelineStringBytes } from './tz/tz';
import { EthSigner } from './eth/eth';

export type SignerType = 'eth' | 'tz';
export type Provider = EthSigner | TzSigner;

export async function signClaim(claim: string, provider: Provider): Promise<string> {
  const t = provider.type;
  switch (provider.type) {
    case 'eth':
      // TODO: TODO: TODO: REMOVE AND REPLACE WITH IMPL.
      throw new Error('Need to implement with personal sign');
      return '';
    case 'tz': {
      const { signature } = await provider.provider.client.requestSignPayload({
        signingType: SigningType.MICHELINE,
        payload: getMichelineStringBytes(`Tezos Signed Message: ${claim}`),
        sourceAddress: await provider.provider.getPKH(),
      });
      return signature;
    }
    default:
      throw new Error(`Unknown Provider type: ${t}`);
  }
}
