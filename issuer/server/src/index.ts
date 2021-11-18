import express, { Request, Response } from 'express';
import { config } from 'dotenv';
import { verifyCredential } from '@spruceid/didkit-wasm-node';
import asyncHandler from 'express-async-handler';

// TODO: Import from NPM instead of locally.
import {
  PublicClaimInfo,
  DiscordLocation,
  TwitterLocation,
  RebaseDiscordVersions,
  RebaseTwitterVersions,
  SignedClaim,
} from '../../../sdk/js/credential/client/src/public_claim/public_claim';

type DIDKitVerifyResult = {
  errors: Array<string>;
};

config();

const port = process.env['PORT'] || 9999;

/**
 * credentialMap is used to determine what credential is requested by the client.
 * Its indexes are namespaces and its values are Array<string> of credential types.
 */
const credentialMap: { [index: string]: { [index: string]: Array<number> } } = {
  rebase: {
    DiscordVerification: RebaseDiscordVersions,
    TwitterVerification: RebaseTwitterVersions,
  },
};

/**
 * areValidParams takes a namespace, a credential, a version and returns whether it is valid.
 */
function areValidParams(
  namespace: string,
  version: string,
  credential: string,
) {
  const credentialEntry = credentialMap[namespace];
  if (!credentialEntry) {
    return false;
  }

  const versionEntry = credentialEntry[credential];
  if (!versionEntry) {
    return false;
  }

  const versionStr = version.substr(1);
  try {
    const v = parseInt(versionStr, 10);
    return versionEntry.includes(v);
  } catch (err) {
    return false;
  }
}

const app = express();

app.use(express.json());
app.listen(port, () => {
  console.log(`Rebase Issuer running on port ${port}`);
});

const handleVerifyCredential = async (
  request: Request,
  response: Response,
) => {
  let resultStr = '';
  const verifyOptionsString = '{}';
  try {
    resultStr = await verifyCredential(JSON.stringify(request.body), verifyOptionsString) as string;
    const verifyResult = JSON.parse(resultStr) as DIDKitVerifyResult;

    if (!verifyResult?.errors) {
      response.status(500).json({ verified: false, error: 'Invalid result from verify credential, malformed result' });
    } else if (verifyResult.errors.length > 0) {
      const errorMessage = `Unable to verify credential: ${verifyResult.errors.join(
        ', ',
      )}`;
      response.status(400).json({ verified: false, error: errorMessage });
    } else {
      response.status(200).json({ verified: true });
    }
  } catch (err) {
    const error = err as Error;
    response.status(500).json({ verified: false, error: `Invalid result from verify credential: ${error.message}` });
  }
};

app.post('/verify', asyncHandler(handleVerifyCredential));

const handleRebaseWitness = async (
  request: Request,
  response: Response,
) => {
};

app.post('/:namespace/:version/:verification', asyncHandler(handleRebaseWitness));
