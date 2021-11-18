import { SigningType } from '@airgap/beacon-sdk';
import { TzSigner, getMichelineStringBytes } from './tz/tz';
import { EthSigner } from './eth/eth';

// Should this not be a circular dep? Or embrace the lispiness of JS?

export type Provider = EthSigner | TzSigner;

export async function getDID(provider: Provider): Promise<string> {
  const t = provider.type;
  switch (provider.type) {
    case 'eth':
      return `did:pkh:eth:${await provider.provider.getAddress()}`;
    case 'tz':
      return `did:pkh:tz:${await provider.provider.getPKH()}`;
    default:
      throw new Error(`Unknowner signer type, ${t}`);
  }
}

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
