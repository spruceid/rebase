/**
 * Interface for ethereum provider request args.
*/
export interface EthRequest {
  method: string;
  params?: Record<string, unknown> | Array<unknown>;
}
/**
 * Interface for ethereum providers of various sorts.
 * See documentation here: https://docs.metamask.io/guide/ethereum-provider.html#ethereum-request-args
 * or elsewhere for valid arguments.
*/
export interface EthProvider {
  request(ethRequest: EthRequest): Promise<unknown>;
}

export interface EthSigner {
  type: 'eth';
  provider: EthProvider;
}

// TODO: Implement discovery for did:pkh:eth
