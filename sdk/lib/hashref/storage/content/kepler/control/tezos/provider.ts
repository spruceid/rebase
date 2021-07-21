import type { BeaconWallet } from '@taquito/beacon-wallet';
import {
  Kepler, authenticator, getOrbitId,
} from 'kepler-sdk';
import { ContentControl } from '../../../../../client/provider/contentStorage';

export type Ref = string;

// TODO: Accept all signer types, not just wallet?
export async function contentControl(
  wallet: BeaconWallet,
  domain: string,
): Promise<ContentControl<any, Ref>> {
  // TODO: Figure out how to get DAppClient working with other signer types.
  const auth = await authenticator(wallet.client, domain);
  const kepler = new Kepler(domain, auth);
  const pkh = await wallet.getPKH();

  const id = await getOrbitId(pkh, { domain, index: 0 });
  return {
    id,

    create: async (contentList: Array<any>): Promise<Array<Ref>> => {
      if (contentList.length <= 0) {
        throw new Error('Empty array passed to Kepler put');
      }

      const first = contentList.pop();

      try {
        const res = await kepler.put(id, first, ...contentList);
        if (!res.ok || res.status !== 200) {
          throw new Error(`Failed to save to Kepler orbit: ${res.statusText}`);
        }

        const addresses = await res.text();

        return addresses.split('\n');
      } catch (err) {
        const res = await kepler.createOrbit(first, ...contentList);
        if (!res.ok || res.status !== 200) {
          throw new Error(`Failed to create or save to orbit: ${res.statusText}`);
        }
        const addresses = await res.text();

        return addresses.split('\n');
      }
    },

    read: async (ref: Ref): Promise<any> => {
      // TODO: Add auth here?
      const res = await kepler.resolve(ref, false);
      return res.text();
    },

    remove: async (refList: Array<Ref>): Promise<void> => {
      await Promise.all(refList.map(async (x) => {
        const ref = x.slice(9);
        const arr = ref.split('/');
        if (arr[1]) {
          await kepler.del(id, arr[1]);
        }
      }));
    },
  };
}
