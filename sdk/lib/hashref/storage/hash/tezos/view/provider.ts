import * as taquito from '@taquito/taquito';
import * as tzip16 from '@taquito/tzip16';
import { NetworkType } from '@airgap/beacon-sdk';

import { HashViewer } from '../../../../client/provider/hashStorage';
import { contractAddressesFromOwner, TQContract } from '../tezos';
import { Storage } from '../../../../client/abstraction/storage';

// TODO: Use indexer if on mainnet in these examples.
// OR break into tzprofiles specific impl.

// The location type to find all contracts of the owner.
// Must be paired with a screening function.
export type ByOwner = {
  network: NetworkType;
  tzktBase: string;
  nodeUrl: string;
  ownerAddress: string;
};

// The location type required to find a contract at a given address
export type ByAddress = {
  network: NetworkType;
  nodeUrl: string;
  contractAddress: string;
};

export type Screener = (
  contractAddress: string,
  tzKit: taquito.TezosToolkit,
  useWallet: boolean
) => Promise<TQContract | false>;

// Takes an address and a target metadata interface string and checks against a contract
export function metadataInterfaceScreener(target: string): Screener {
  return async (
    contractAddress: string,
    tzKit: taquito.TezosToolkit,
    useWallet: boolean,
  ): Promise<TQContract | false> => {
    try {
      const contract = useWallet
        ? await tzKit.wallet.at(contractAddress, tzip16.tzip16)
        : await tzKit.contract.at(contractAddress, tzip16.tzip16);

      const metadata = await contract.tzip16().getMetadata();
      if (metadata.metadata.interfaces) {
        for (let i = 0, n = metadata.metadata.interfaces.length; i < n; i += 1) {
          const interf = metadata.metadata.interfaces[i];
          if (interf && interf.includes(target)) {
            return contract;
          }
        }
      }

      return false;
    } catch (_) {
      return false;
    }
  };
}

export function addressProvider<Hash, RawStorage, Ref>(
  contractOperation: string,
  fmt: (r: RawStorage) => Storage<Hash, Ref>,
): HashViewer<Hash, ByAddress, Ref> {
  return async (location: ByAddress): Promise<Array<Storage<Hash, Ref>>> => {
    const tzKit = new taquito.TezosToolkit(location.nodeUrl);
    tzKit.addExtension(new tzip16.Tzip16Module());

    const contract = await tzKit.contract.at(location.contractAddress, tzip16.tzip16);

    // TODO: annotate this:
    const views = await contract.tzip16().metadataViews();
    const op = views[contractOperation];
    if (!op) {
      // be more descriptive?
      throw new Error('Could not read storage');
    }

    return [fmt(await op().executeView() as RawStorage)];
  };
}

// This configures a HashViewer by owner address which
// screens for a given owners contracts for a specific one filtered
// by the screener.
export function ownerViewer<Hash, RawStorage, Ref>(
  contractOperation: string,
  fmt: (r: RawStorage) => Storage<Hash, Ref>,
  screener: Screener,
): HashViewer<Hash, ByOwner, Ref> {
  return async (location: ByOwner): Promise<Array<Storage<Hash, Ref>>> => {
    const tzKit = new taquito.TezosToolkit(location.nodeUrl);
    tzKit.addExtension(new tzip16.Tzip16Module());

    const addressList = await contractAddressesFromOwner(
      0,
      location.ownerAddress,
      location.tzktBase,
    );

    const storageList: Array<Storage<Hash, Ref>> = [];
    for (let i = 0, n = addressList.length; i < n; i += 1) {
      const address = addressList[i];

      // for now, agreeing to disagree with no await in a loop here.
      // TODO: something else?
      /* eslint-disable no-await-in-loop */
      if (address && await screener(address, tzKit, false)) {
        const contract = await tzKit.contract.at(address, tzip16.tzip16);
        const views = await contract.tzip16().metadataViews();
        const op = views[contractOperation];
        if (!op) {
          // be more descriptive?
          throw new Error(`Could not read storage for contract ${contract.address}`);
        }

        storageList.push(fmt(await op().executeView() as RawStorage));
      }
    }

    return storageList;
  };
}
