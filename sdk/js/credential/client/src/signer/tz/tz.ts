import { BeaconWallet } from '@taquito/beacon-wallet';
import { char2Bytes } from '@taquito/utils';
import { ProviderBase } from '../common/common';

export interface TzSigner extends ProviderBase {
  type: 'tz';
  provider: BeaconWallet;
}

export const getMichelineStringBytes = (str: string): string => {
  const convertedBytes = char2Bytes(str);
  const strLength = (convertedBytes.length / 2).toString(16).padStart(8, '0');
  const bytes = `0501${strLength}${convertedBytes}`;
  return bytes;
};

// TODO: Implement discovery for did:pkh:tz<N>
