import express, { Request, Response } from 'express';
import cors from 'cors';
import type { Express as ExpressServer } from 'express';

import { config } from 'dotenv';
import { verifyCredential } from '@spruceid/didkit-wasm-node';
import asyncHandler from 'express-async-handler';

// // TODO: Import from NPM instead of locally.
// import { Credential } from '../../../client/src/credential';

// TODO: Import from NPM instead of locally.
import {
  Credential,
  makeWitness,
  validateRebaseSignedClaim,
  RebaseClaimType,
  isRebaseClaimType,
} from 'rebase-claim-client';

config();

const defaultHandler = makeWitness({
  issuer: process.env['REBASE_ISSUER'] || '',
  twitter: {
    apiKey: process.env['REBASE_TWITTER_API'] || '',
  },
  discord: {
    apiKey: process.env['REBASE_DISCORD_API'] || '',
  },
});

/**
 * DIDKitVerifyResult is used to properly type the result of verifyCredential from DIDKit.
 */
type DIDKitVerifyResult = {
  errors: Array<string>;
};

/**
 * SuccessResult is what a RebaseHandler should return in case of a successful issuance.
 * Configures the response to be served by Express
 */
export interface SuccessResult {
  success: true;
  status: 200;
  credential: Credential;
}

/**
 * FailureResult is what a RebaseHandler should return in case of an unsuccessful issuance.
 * Configures the response to be served by Express
 */
export interface FailureResult {
  success: false;
  status: number;
  error: Error;
}

/**
 * The result of a call of a RebaseHandler.
 */
export type RebaseHandlerResult = SuccessResult | FailureResult;

/**
 * RebaseHandler is the function invoked when a request for issuance is made
 * @param credentialType is the type of claim to be witnessed and credential to be issued
 * example: TwitterVerification. In the default configuration corresponds to a @context schema
 * in the resulting credential.
 * @param version is the version of that credential's specification or schema to be issued.
 * @param body is the request body of client.
 * By default, will be of type SignedClaim<RebasePublicClaim>, but can be customized.
 */
export type RebaseHandler = (
  body: unknown,
  credentialType: string,
  version?: number,
) => Promise<RebaseHandlerResult>;

const defaultRebaseHandler = async (
  body: unknown,
  credentialType: string,
  _version?: number,
): Promise<RebaseHandlerResult> => {
  if (!isRebaseClaimType(credentialType as RebaseClaimType)) {
    return {
      success: false,
      status: 400,
      error: new Error(`Unknown credentialType: ${credentialType}`),
    };
  }
  try {
    const signedClaim = validateRebaseSignedClaim(body);
    try {
      const cred = await defaultHandler(signedClaim);
      return {
        success: true,
        status: 200,
        credential: cred,
      };
    } catch (err) {
      const e = err as Error;
      return {
        success: false,
        status: 500,
        error: new Error(`Failed to issue credential: ${e.message}`),
      };
    }
  } catch (err) {
    const e = err as Error;
    return {
      success: false,
      status: 400,
      error: new Error(`Invalid body: ${e.message}`),
    };
  }
};

/**
 * HandlerMap relates the handlers to namespaces. By default, only 'rebase' is supplied.
 * Routes are expected to be in the form /:namespace/:version/:credentialType
 * The namespace is the key used in the handler map.
 * The version and credential type, combined with the incoming req.body are then passed
 * to the handler found at HandlerMap[namespace]
 */
export type HandlerMap = Record<string, RebaseHandler>;

/**
 * Opts are the point of configuration for the issuer.
 * Defaults are provided, but everything can be over-written, including the handler
 * at handlerMap['rebase']
 * handlerMap is used to find the handler when receiving a request to a route matching
 * /:namespace/:version/:credentialType
 * onStart is invoked when using startIssuer as the callback passed to app.listen
 * corsWildcard determines whether CORS is enforced, true means all requests are respected,
 * false leaves the express defaults.
 */
export interface Opts {
  corsWildcard?: boolean;
  handlerMap?: HandlerMap;
  onStart?: () => unknown;
}

// FullOpts is the result of combining defualtOpts with user supplied opts, but now
// all properties can be counted on.
interface FullOpts {
  corsWildcard: boolean;
  handlerMap: HandlerMap;
  onStart: () => unknown;
}

const port = process.env['PORT'] || 9999;

/**
 * defaultOpts acts as the base for user provided opts to override.
 * Contains a handler for the 'rebase' namespace, wildcard CORS for POST, and a simple log onStart
 */
const defaultOpts = {
  corsWildcard: true,
  onStart: () => {
    console.log(`Issuer listening on port: ${port}`);
  },
  // TODO: Impl!
  handlerMap: {
    rebase: defaultRebaseHandler,
  },
};

function setOpts(opts: undefined | Opts): FullOpts {
  if (!opts) {
    return defaultOpts;
  }

  return {
    ...defaultOpts,
    ...opts,
  };
}

// Exposes DIDKit's verifyCredential over HTTPs at the route /verify
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

// The type of handler that express-async-handler expects, used internally.
type ExpressHandler = (request: Request, response: Response) => Promise<void>;

// Used to generalize over the routing structure.
interface ParamsObj {
  credentialType: string;
  namespace: string;
  version?: number;
}

// Get params and parse version from Request.params
function fromParams(params: Record<string, string>): ParamsObj {
  if (
    params['namespace']
    && params['credentialType']
    && params['version']
  ) {
    const result: ParamsObj = {
      credentialType: params['credentialType'],
      namespace: params['namespace'],
    };
    const versionStr = params['version'];
    if (versionStr) {
      result.version = parseInt(versionStr, 10);
    }
    return result;
  }

  if (!params['credentialType']) {
    throw new Error('No credentialType path param found');
  } else if (!params['namespace']) {
    throw new Error('No namespace path param found');
  } else {
    throw new Error('No version path param found');
  }
}

// Let Over Lambda trick to get the opts accessable to the handler on the fly.
const makeHandleIssue = (opts: FullOpts): ExpressHandler => async (
  request: Request,
  response: Response,
) => {
  const paramsObj = fromParams(request.params);
  const { namespace, credentialType } = paramsObj;
  const h = opts.handlerMap[namespace];
  if (!h) {
    response.status(400).json({ error: `No handler found for namespace '${namespace}'` });
    return;
  }
  let result;
  if (paramsObj.version) {
    result = await h(request.body, credentialType, paramsObj.version);
  } else {
    result = await h(request.body, credentialType);
  }

  if (result.success) {
    response.status(200).json(result.credential);
  } else {
    response.status(result.status).json({ error: result.error });
  }
};

/**
 * newIssuer creates an Express app, but doesn't start listening, allowing the
 * calling application to further configure it.
 * @param opts optional overrides to the defaults.
 * @returns Express application ready for further configuration
 */
export function newIssuer(opts?: Opts): ExpressServer {
  const app = express();
  const fullOpts = setOpts(opts);
  if (fullOpts.corsWildcard) {
    app.use(cors({
      origin: '*',
    }));
  }
  app.use(express.json());
  app.post('/verify', asyncHandler(handleVerifyCredential));
  app.post('/:namespace/:credentialType', asyncHandler(makeHandleIssue(fullOpts)));
  app.post('/:namespace/:credentialType/:version', asyncHandler(makeHandleIssue(fullOpts)));
  return app;
}

/**
 * newIssuer creates and starts a Issuer Express app, blocks and does not return.
 * @param opts optional overrides to the defaults.
 */
export function startIssuer(opts?: Opts): void {
  const app = newIssuer(opts);
  let onStart = opts?.onStart;
  if (!onStart) {
    onStart = defaultOpts.onStart;
  }
  app.listen(port, onStart);
}
