import type { BeaconWallet } from '@taquito/beacon-wallet';
import * as taquito from '@taquito/taquito';
import { ContractAbstraction, ContractProvider } from '@taquito/taquito';
import { InMemorySigner, importKey } from '@taquito/signer';
import axios from 'axios';

// Magic Number controlling how long to wait before confirming success.
// Seems to be an art more than a science, 3 was suggested by a help thread.
export const CONFIRMATION_CHECKS = 3;

export type TQContract = ContractAbstraction<ContractProvider | taquito.Wallet>;

export async function contractAddressesFromOwner(
  offset: number,
  ownerAddress: string,
  tzktBase: string,
): Promise<Array<string>> {
  const pageLimit = 100;
  const searchRes = await axios.get(
    `${tzktBase}contracts?creator=${ownerAddress}&offset=${offset}&limit=${pageLimit}&sort=firstActivity&select=address`,
  );

  if (searchRes.status !== 200) {
    throw new Error(`Failed in explorer request: ${searchRes.statusText}`);
  }

  if (
    !searchRes.data
    || !Array.isArray(searchRes.data)
    || searchRes.data?.length === 0
  ) {
    return [];
  }

  const data = searchRes.data as Array<string>;
  const pageCount = data.length;
  if (pageCount === pageLimit) {
    return data.concat(
      await contractAddressesFromOwner(
        offset + pageCount,
        ownerAddress,
        tzktBase,
      ),
    );
  }

  return data;
}

// Create signer from beacon wallet
interface WalletSigner {
  type: 'wallet';
  wallet: BeaconWallet;
}

// Create signer from secret key
interface SecretSigner {
  type: 'secret';
  secret: string;
}

// Create signer from key file
interface KeyFileSigner {
  type: 'key_file';
  file: {
    email: string;
    password: string;
    mnemonic: Array<string>;
    secret: string;
  };
}

export type Signer = WalletSigner | SecretSigner | KeyFileSigner;

export async function loadSigner(
  signer: Signer,
  tzKit: taquito.TezosToolkit,
): Promise<void> {
  // Get around the type checker for err reporting.
  const t = signer.type;

  switch (signer.type) {
    case 'key_file':
      await importKey(
        tzKit,
        signer.file.email,
        signer.file.password,
        signer.file.mnemonic.join(' '),
        signer.file.secret,
      );
      return;
    case 'secret':
      tzKit.setProvider({
        signer: new InMemorySigner(signer.secret),
      });
      return;
    case 'wallet':
      tzKit.setWalletProvider(signer.wallet);
      return;
    default:
      throw new Error(`Unknown signer type ${t}`);
  }
}
