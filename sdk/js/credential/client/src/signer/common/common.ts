import { exhaustiveCheck } from '../../utils/utils';

export type SignerType = 'eth' | 'tz';

export function isSignerType(s: SignerType): boolean {
  switch (s) {
    case 'eth':
    case 'tz':
      return true;
    default:
  }

  exhaustiveCheck(s);
  return false;
}

export interface ProviderBase {
  type: SignerType;
}
