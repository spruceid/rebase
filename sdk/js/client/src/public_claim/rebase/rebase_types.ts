import { exhaustiveCheck } from '../../utils/utils';

// Types of public claims supported by default Rebase settings.
export type RebaseClaimType = 'DiscordVerification' | 'TwitterVerification';

// Useful for runtime contract checking.
export function isRebaseClaimType(s: RebaseClaimType): boolean {
  switch (s) {
    case 'DiscordVerification':
    case 'TwitterVerification':
      return true;
    default:
  }

  exhaustiveCheck(s);
  return false;
}
