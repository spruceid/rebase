import * as taquito from '@taquito/taquito';
import * as tzip16 from '@taquito/tzip16';
import { NetworkType } from '@airgap/beacon-sdk';
import {
  contractAddressesFromOwner,
  loadSigner,
  CONFIRMATION_CHECKS,
  Signer,
  TQContract,
} from '../tezos';
import { ByOwner, Screener } from '../view/provider';
import { Storage } from '../../../../client/abstraction/storage';
import { HashControl, HashViewer } from '../../../../client/provider/hashStorage';

export type OperationKey = 'create' | 'remove' | 'update';

export type LocationOpts = {
  network: NetworkType;
  tzktBase: string;
  nodeUrl: string;
};

export function locationOptsFromNetwork(network: NetworkType): LocationOpts {
  switch (network) {
    case 'custom':
      return {
        network,
        tzktBase: 'http://localhost:5000/',
        nodeUrl: 'http://localhost:8732',
      };
    default:
      return {
        network,
        tzktBase: `https://api.${network}.tzkt.io/v1/`,
        nodeUrl: `https://${network}.smartpy.io`,
      };
  }
}

export type ToEntryPointArgs<Hash, Ref> = (
  storage: Storage<Hash, Ref>
) => Array<any>;
export type FmtOpts<Hash, RawStorage, Ref, StorageInput> = {
  // For look up
  // Assumed to be unneeded if not supplied
  fromRawStorage?(rawStorage: RawStorage): Promise<Storage<Hash, Ref>>;
  // For origination
  // Assumed to be unneeded if not supplied
  toStorage?(storage: Storage<Hash, Ref>): Promise<StorageInput>;

  operations: {
    create?: ToEntryPointArgs<Hash, Ref>;
    remove?: ToEntryPointArgs<Hash, Ref>;
    update?: ToEntryPointArgs<Hash, Ref>;
  } | false;
};

export type Config<Hash, Ref> = {
  // Literal source code of contract.
  code: string;
  hashViewer: HashViewer<Hash, ByOwner, Ref>;
  // Metadata document.
  // TODO: Link explainer doc.
  metadataUrl: string;
  network: NetworkType;
  // Entry point names for the smart contract for these commands.
  operations: {
    // Allows for multiple options for backwards compat
    create: string;
    remove: string;
    update?: string;
  };
  screener: Screener;
  signer: Signer;
};

export interface ByOwnerHashControl<Hash, Ref> extends HashControl<Hash, ByOwner, Ref> {
  contractAddr(): string;
}

export async function hashControlFromSigner<Hash, RawStorage, Ref, StorageInput>(
  config: Config<Hash, Ref>,
  fmtOpts?: FmtOpts<Hash, RawStorage, Ref, StorageInput>,
): Promise<ByOwnerHashControl<Hash, Ref>> {
  const locationOpts = locationOptsFromNetwork(config.network);

  const tzKit = new taquito.TezosToolkit(locationOpts.nodeUrl);
  tzKit.addExtension(new tzip16.Tzip16Module());

  await loadSigner(config.signer, tzKit);

  let pkh: string;

  const t = config.signer.type;
  switch (config.signer.type) {
    case 'key_file':
      pkh = await tzKit.signer.publicKeyHash();
      break;
    case 'secret':
      pkh = await tzKit.signer.publicKeyHash();
      break;
    case 'wallet':
      pkh = await config.signer.wallet.getPKH();
      break;
    default:
      throw new Error(`Unknown signer type ${t}`);
  }

  const id = `${pkh}:${config.network}`;

  let instance: TQContract | false = false;
  let addr = '';

  const getLocation = (): ByOwner => {
    const { network, nodeUrl, tzktBase } = locationOpts;
    return {
      network,
      nodeUrl,
      tzktBase,
      ownerAddress: pkh,
    };
  };

  const getInstance = async (): Promise<TQContract | false> => {
    const addressList = await contractAddressesFromOwner(
      0,
      pkh,
      locationOpts.tzktBase,
    );

    for (let i = 0, n = addressList.length; i < n; i += 1) {
      const address = addressList[i];
      if (address) {
        // for now, agreeing to disagree with no await in a loop here.
        // TODO: something else?
        /* eslint-disable no-await-in-loop */
        const contract = await config.screener(address, tzKit, config.signer.type === 'wallet');
        if (contract) {
          addr = address;
          return contract;
        }
      }
    }

    return false;
  };

  const originate = async (
    initialStorage: Storage<Hash, Ref>,
  ): Promise<void> => {
    const inst = await getInstance();
    if (inst) {
      throw new Error(`Found existing contract at ${inst.address}`);
    }

    let nextStorage: Storage<Hash, Ref> | StorageInput = initialStorage;
    if (fmtOpts?.toStorage) {
      nextStorage = await fmtOpts.toStorage(initialStorage);
    }

    const metadataBigMap = new taquito.MichelsonMap();
    metadataBigMap.set('', tzip16.char2Bytes(config.metadataUrl));
    const args = {
      code: config.code,
      storage: {
        claims: nextStorage,
        contract_type: 'tzprofiles',
        owner: pkh,
        metadata: metadataBigMap,
      },
    };

    let originationOp;
    if (config.signer.type === 'wallet') {
      const opSender = tzKit.wallet.originate(args);
      originationOp = await opSender.send();

      const c = await originationOp.contract();

      // NOTE: This is a capture of the surrounding scope to propagate the change to
      // other lambdas defined here.
      addr = c.address;
      instance = await tzKit.wallet.at(c.address);
    } else {
      originationOp = await tzKit.contract.originate(args);

      await originationOp.confirmation(CONFIRMATION_CHECKS);
      if (!originationOp.contractAddress) {
        throw new Error('No contract address found after origination');
      }

      // NOTE: This is a capture of the surrounding scope to propagate the change to
      // other lambdas defined here.
      addr = originationOp.contractAddress || '';
      instance = await tzKit.contract.at(originationOp.contractAddress);
    }

    if (!instance) {
      throw new Error('Could not set instance in origination');
    }
  };

  type OperatorFn = (nextStorage: Storage<Hash, Ref>) => Promise<void>;
  const makeOperatorFn = (operatorKey: 'create' | 'update' | 'remove'): OperatorFn => async (nextStorage: Storage<Hash, Ref>): Promise<void> => {
    if (!instance) {
      instance = await getInstance();
      if (!instance) {
        throw new Error(`No instance found for signer ${id}`);
      }
    }

    const operation = config.operations[operatorKey];

    const entrypoints = Object.keys(instance.methods);
    if (operation && entrypoints.includes(operation)) {
      const f = instance?.methods[operation];
      if (!f) {
        throw new Error(
          `No entrypoint at ${operation}, though is listed in methods`,
        );
      }

      let args: Storage<Hash, Ref> | Array<any> = nextStorage;
      if (fmtOpts) {
        const { operations } = fmtOpts;
        if (operations) {
          const g = operations[operatorKey];
          if (g) {
            args = g(nextStorage);

            const mystery = f(...args);
            const op = await mystery.send();

            await op.confirmation(CONFIRMATION_CHECKS);
            return;
          }
        }
      }

      const op = await f(args).send();
      await op.confirmation(CONFIRMATION_CHECKS);
      return;
    }

    throw new Error('No entrypoint to add claim.');
  };

  return {
    id,
    getLocation,
    originate,
    contractAddr: () => addr,
    locate: config.hashViewer,
    create: makeOperatorFn('create'),
    remove: makeOperatorFn('remove'),
  };
}
