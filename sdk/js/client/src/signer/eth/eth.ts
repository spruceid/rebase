import { ProviderBase } from '../common/common';

/**
 * Interface for ethereum provider request args to the Ethereum RPC node.
 * See documentation here: https://docs.metamask.io/guide/ethereum-provider.html#ethereum-request-args
 * or elsewhere for valid arguments.
*/
export interface EthRequest {
  method: string;
  params?: Record<string, unknown> | Array<unknown>;
}
/**
 * Interface for ethereum providers of various sorts.
*/
export interface EthProvider {
  // Make RPC requests.
  request(ethRequest: EthRequest): Promise<unknown>;
  // Get the PKH.
  getAddress(): Promise<string>;
}

export interface EthSigner extends ProviderBase {
  type: 'eth';
  provider: EthProvider;
}

// TODO: Implement discovery for did:pkh:eth
