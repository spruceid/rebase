/**
 * Helper functions that when passed a request will return a
 * boolean indicating if the request uses that HTTP method,
 * header, host or referrer.
 */

const Method = (method) => (req) => req.method.toLowerCase() === method.toLowerCase();
const Connect = Method("connect");
const Delete = Method("delete");
const Get = Method("get");
const Head = Method("head");
const Options = Method("options");
const Patch = Method("patch");
const Post = Method("post");
const Put = Method("put");
const Trace = Method("trace");

const Header = (header, val) => (req) => req.headers.get(header) === val;
const Host = (host) => Header("host", host.toLowerCase());
const Referrer = (host) => Header("referrer", host.toLowerCase());

const Path = (regExp) => (req) => {
  const url = new URL(req.url);
  const path = url.pathname;
  const match = path.match(regExp) || [];
  return match[0] === path;
};

/**
 * The Router handles determines which handler is matched given the
 * conditions present for each request.
 */
class Router {
  constructor() {
    this.routes = [];
  }

  handle(conditions, handler) {
    this.routes.push({
      conditions,
      handler,
    });
    return this;
  }

  connect(url, handler) {
    return this.handle([Connect, Path(url)], handler);
  }

  delete(url, handler) {
    return this.handle([Delete, Path(url)], handler);
  }

  get(url, handler) {
    return this.handle([Get, Path(url)], handler);
  }

  head(url, handler) {
    return this.handle([Head, Path(url)], handler);
  }

  options(url, handler) {
    return this.handle([Options, Path(url)], handler);
  }

  patch(url, handler) {
    return this.handle([Patch, Path(url)], handler);
  }

  post(url, handler) {
    return this.handle([Post, Path(url)], handler);
  }

  put(url, handler) {
    return this.handle([Put, Path(url)], handler);
  }

  trace(url, handler) {
    return this.handle([Trace, Path(url)], handler);
  }

  all(handler) {
    return this.handle([], handler);
  }

  route(req) {
    const route = this.resolve(req);

    if (route) {
      return route.handler(req);
    }

    return new Response("resource not found", {
      status: 404,
      statusText: "not found",
      headers: {
        "content-type": "text/plain",
      },
    });
  }

  /**
   * resolve returns the matching route for a request that returns
   * true for all conditions (if any).
   */
  resolve(req) {
    return this.routes.find((r) => {
      if (!r.conditions || (Array.isArray(r) && !r.conditions.length)) {
        return true;
      }

      if (typeof r.conditions === "function") {
        return r.conditions(req);
      }

      return r.conditions.every((c) => c(req));
    });
  }
}

const headers = new Headers({
  "Content-Type": "application/json",
  "Access-Control-Allow-Origin": "*",
});

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET,POST,OPTIONS",
  "Access-Control-Max-Age": "86400",
};

addEventListener("fetch", (event) => {
  const request = event.request;
  if (request.method === "OPTIONS") {
    // Handle CORS preflight requests
    event.respondWith(handleOptions(request));
  } else {
    event.respondWith(handleRequest(request));
  }
});

function handleOptions(request) {
  // Make sure the necessary headers are present
  // for this to be a valid pre-flight request
  let headers = request.headers;
  if (headers.get("Origin") !== null && headers.get("Access-Control-Request-Method") !== null && headers.get("Access-Control-Request-Headers") !== null) {
    // Handle CORS pre-flight request.
    // If you want to check or reject the requested method + headers
    // you can do that here.
    let respHeaders = {
      ...corsHeaders,
      // Allow all future content Request headers to go back to browser
      // such as Authorization (Bearer) or X-Client-Name-Version
      "Access-Control-Allow-Headers": request.headers.get("Access-Control-Request-Headers"),
    };

    return new Response(null, {
      headers: respHeaders,
    });
  } else {
    // Handle standard OPTIONS request.
    // If you want to allow other HTTP Methods, you can do that here.
    return new Response(null, {
      headers: {
        Allow: "GET, POST, OPTIONS",
      },
    });
  }
}

// TODO: Make a type for this config obj?
// TODO: Change to use the Flow VC's type for keys in future.
// To disable a flow, change the value of the flow in configDefaults to a falsey value.
const configDefaults = {
  NftOwnershipVerification: {
    max_elapsed_minutes: 15,
    challenge_delimiter: "\n\n"
  },
  GitHubVerification: {
    user_agent: "Spruce Systems",
    delimiter: "\n\n"
  }, 
  PoapOwnershipVerification: {
    max_elapsed_minutes: 15,
    challenge_delimiter: "\n\n"
  },
  EmailVerification: {
    challenge_delimiter: ":::",
    from_addr: "hello@rebaseexample.com",
    from_name: "Spruce",
    subject_name: "Rebase Credentialing",
    max_elapsed_minutes: 15,
  },
  SoundCloudVerification: {
    limit: 100,
    max_offset: 9000,
  },
  TwitterVerification: {
    delimiter: "\n\n"
  }
}

function witnessOpts() {  
  let o = {};
  o.DnsVerification = {};
  o.RedditVerification = {};
  o.SameControllerAssertion = {};

  if (configDefaults.GitHubVerification) {
    o.GitHubVerification = configDefaults.GitHubVerification;
  }

  if (configDefaults.SoundCloudVerification && SOUNDCLOUD_CLIENT_ID) {
    let {limit, max_offset} = configDefaults.SoundCloudVerification;
    o.SoundCloudVerification = {
      client_id: SOUNDCLOUD_CLIENT_ID,
      limit,
      max_offset,
    }
  }

  if (configDefaults.TwitterVerification && TWITTER_BEARER_TOKEN) {
    o.TwitterVerification = {
      api_key: TWITTER_BEARER_TOKEN,
      delimiter: configDefaults.TwitterVerification.delimiter,
    }
  }


  if (configDefaults.EmailVerification && SENDGRID_BEARER_TOKEN) {
    let {
      challenge_delimiter,
      from_addr,
      from_name,
      max_elapsed_minutes,
      subject_name
    } = configDefaults.EmailVerification;

    o.EmailVerification = {
      api_key: SENDGRID_BEARER_TOKEN,
      challenge_delimiter,
      from_addr,
      from_name,
      max_elapsed_minutes,
      subject_name,
    }
  }

  if (configDefaults.NftOwnershipVerification && ALCHEMY_API_KEY) {
    let {challenge_delimiter, max_elapsed_minutes} = configDefaults.NftOwnershipVerification;
    o.NftOwnershipVerification = {
      api_key: ALCHEMY_API_KEY,
      challenge_delimiter,
      max_elapsed_minutes,
    }
  }

  if (configDefaults.PoapOwnershipVerification && POAP_API_KEY) {
    let {challenge_delimiter, max_elapsed_minutes} = configDefaults.PoapOwnershipVerification;
    o.PoapOwnershipVerification = {
      api_key: POAP_API_KEY,
      challenge_delimiter,
      max_elapsed_minutes,
    };
  }
  return o
};

const opts = {
  witness: witnessOpts(),
  did: DID_WEB
}

const {statement, witness, instructions, verify} = wasm_bindgen;
const instance = wasm_bindgen(wasm);

async function stmt(request) {
  try {
    await instance;
    const h = request.headers;
    const contentType = h.get('content-type') || '';
    if (contentType.includes('application/json')) {
      const body_str = JSON.stringify(await request.json());
      const res = await statement(REBASE_SK, body_str, JSON.stringify(opts));
      return new Response(res, {status: 200, headers: headers});
    } else {
      return new Response(JSON.stringify({ error: `Expected content-type application/json, got: ${contentType}`}), {
        status: 400,
        headers: headers
      });
    }
  } catch (e) {
    return new Response(JSON.stringify({ error: e?.message ?? `${e}` }), {status: 400, headers: headers});
  }
}

async function wtns(request) {
  try {
    await instance;
    const h = request.headers;

    const contentType = h.get('content-type') || '';

    if (contentType.includes('application/json')) {
      let body = await request.json();

      const credential = await witness(REBASE_SK, JSON.stringify(body), JSON.stringify(opts));

      return new Response(credential, {status: 200, headers: headers});

    } else {
      throw new Error(`Expected content-type application/json, got: ${contentType}`)
    }
  } catch (e) {
    return new Response(JSON.stringify({error: e?.message ? e.message : e}), {status: 400, headers: headers});
  }
}

async function inst(request) {
  try {
    await instance;
    const h = request.headers;

    const contentType = h.get('content-type') || '';

    if (contentType.includes('application/json')) {
      let body = await request.json();
      const res = await instructions(JSON.stringify(body), JSON.stringify(opts));

      return new Response(res, {status: 200, headers: headers});
    } else {
      throw new Error(`Expected content-type application/json, got: ${contentType}`)
    }
  } catch (e) {
    return new Response(JSON.stringify({error: e?.message ? e.message : e}), {status: 400, headers: headers});
  }
}

async function verif(request) {
  try {
    await instance;
    const h = request.headers;

    const contentType = h.get('content-type') || '';

    if (contentType.includes('application/json')) {
      let body = await request.json();
      const res = await verify(REBASE_SK, JSON.stringify(body), JSON.stringify(opts));

      return new Response(res, {status: 200, headers: headers});
    } else {
      throw new Error(`Expected content-type application/json, got: ${contentType}`)
    }
  } catch (e) {
    return new Response(JSON.stringify({error: e?.message ? e.message : e}), {status: 400, headers: headers});
  }
}

async function handleRequest(request) {
  const r = new Router();
  r.post("/statement", (request) => stmt(request));
  r.post("/witness", (request) => wtns(request));
  r.post("/instructions", (request) => inst(request));
  r.post("/verify", (request) => verif(request))
  const resp = await r.route(request);
  return resp;
}

