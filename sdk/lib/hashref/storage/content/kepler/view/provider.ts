import {
  Kepler, Authenticator, Action,
} from 'kepler-sdk';
import desync from '../../../../../util/util';
import { ContentViewer } from '../../../../client/provider/contentStorage';

export type Ref = string;
export type Content = string;

// TODO: Verify this is the right way to do this:
const dummyAuthenticator: Authenticator = {
  /* eslint-disable @typescript-eslint/no-unused-vars */
  content: desync((_orbit: string, _cids: string[], _action: Action) => ''),
  createOrbit: desync((_cids: string[]) => ''),
};

export function contentViewer(keplerHost: string): ContentViewer<Content, Ref> {
  const k = new Kepler(keplerHost, dummyAuthenticator);

  return async (ref: Ref): Promise<Content> => {
    const res = await k.resolve(ref, false);
    return res.text();
  };
}
