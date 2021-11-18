export type SignerType = 'eth' | 'tz';

export interface ProviderBase {
  type: SignerType;
}
