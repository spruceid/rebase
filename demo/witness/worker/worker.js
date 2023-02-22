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

function witnessOpts() {  
  let o = {};
  o.dns = {};
  o.reddit = {};
  o.same = {};

  if (GITHUB_USER_AGENT) {
    o.github = {
      user_agent: GITHUB_USER_AGENT,
      delimitor: "\n\n"
    }
  }

  if (SOUNDCLOUD_CLIENT_ID) {
    let limit = parseInt(SOUNDCLOUD_LIMIT);
    let offset = parseInt(SOUNDCLOUD_MAX_OFFSET);
    if (!isNaN(limit) && !isNaN(offset)) {
      o.soundcloud = {
        client_id: SOUNDCLOUD_CLIENT_ID,
        limit,
        max_offset: offset,
      }
    }
  }

  if (TWITTER_BEARER_TOKEN) {
    o.twitter = {
      api_key: TWITTER_BEARER_TOKEN,
      delimitor: "\n\n"
    }
  }

  let useSendGrid = SENDGRID_BEARER_TOKEN 
    && SENDGRID_FROM_ADDRESS 
    && SENDGRID_FROM_NAME
    && SENDGRID_SUBJECT_NAME
    && SENDGRID_MAX_ELAPSED_MINS
    && !isNaN(parseInt(SENDGRID_MAX_ELAPSED_MINS));

  if (useSendGrid) {
    o.email = {
      api_key: SENDGRID_BEARER_TOKEN,
      from_addr: SENDGRID_FROM_ADDRESS,
      from_name: SENDGRID_FROM_NAME,
      max_elapsed_minutes: parseInt(SENDGRID_MAX_ELAPSED_MINS),
      subject_name: SENDGRID_SUBJECT_NAME,
    }
  }

  let useAlchemy = ALCHEMY_API_KEY 
    && ALCHEMY_MAX_ELAPSED_MINS 
    && !isNaN(parseInt(ALCHEMY_MAX_ELAPSED_MINS));
  if (useAlchemy) {
    o.nft_ownership = {
      api_key: ALCHEMY_API_KEY,
      max_elapsed_minutes: parseInt(ALCHEMY_MAX_ELAPSED_MINS)
    }
  }

  let usePoap = POAP_API_KEY 
    && POAP_MAX_ELAPSED_MINS 
    && !isNaN(parseInt(POAP_MAX_ELAPSED_MINS));

  if (usePoap) {
    o.poap_ownership = {
      api_key: POAP_API_KEY,
      max_elapsed_minutes: parseInt(POAP_MAX_ELAPSED_MINS),
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

