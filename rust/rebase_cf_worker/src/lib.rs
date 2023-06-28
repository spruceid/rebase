use rebase_witness_sdk::types::{
    handle_verify, issuer::ed25519::Ed25519Jwk, Alchemy, AttestationFlow, DelegatedAttestationFlow,
    DnsVerificationFlow, EmailVerificationFlow, GitHubVerificationFlow, InstructionsReq,
    NftOwnershipVerificationFlow, PoapOwnershipVerificationFlow, Proofs, RedditVerificationFlow,
    SameControllerAssertionFlow, SoundCloudVerificationFlow, Statements, TwitterVerificationFlow,
    VCWrapper, WitnessFlow,
};
use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

fn new_flow(env: &Env) -> WitnessFlow {
    let mut flow = WitnessFlow {
        dns_verification: Some(DnsVerificationFlow {}),
        email_verification: None,
        github_verification: Some(GitHubVerificationFlow {
            user_agent: "Spruce Systems".to_string(),
            delimiter: "\n\n".to_string(),
        }),
        nft_ownership_verification: None,
        poap_ownership_verification: None,
        reddit_verification: Some(RedditVerificationFlow {
            user_agent: "rebase-server:rebase-witness-sdk:0.0.1 (by eval-apply-quote)".to_string(),
        }),
        same_controller_assertion: Some(SameControllerAssertionFlow {}),
        soundcloud_verification: None,
        twitter_verification: None,
        attestation: Some(AttestationFlow {}),
        delegated_attestation: None,
    };

    match &env.secret("DID_WEB") {
        Err(_) => {}
        Ok(s) => {
            flow.delegated_attestation = Some(DelegatedAttestationFlow {
                service_key: format!("rebase:{}", s.to_string()),
            });
        }
    }

    match env.secret("SENDGRID_BEARER_TOKEN") {
        Err(_) => {}
        Ok(s) => {
            flow.email_verification = Some(EmailVerificationFlow {
                api_key: s.to_string(),
                challenge_delimiter: ":::".to_string(),
                from_addr: "hello@rebaseexample.com".to_string(),
                from_name: "Spruce".to_string(),
                subject_name: "Rebase Credentialing".to_string(),
                max_elapsed_minutes: 15,
            })
        }
    };

    match env.secret("ALCHEMY_API_KEY") {
        Err(_) => {}
        Ok(s) => {
            flow.nft_ownership_verification =
                Some(NftOwnershipVerificationFlow::Alchemy(Alchemy {
                    api_key: s.to_string(),
                    challenge_delimiter: "\n\n".to_string(),
                    max_elapsed_minutes: 15,
                }));
        }
    }

    match env.secret("POAP_API_KEY") {
        Err(_) => {}
        Ok(s) => {
            flow.poap_ownership_verification = Some(PoapOwnershipVerificationFlow {
                api_key: s.to_string(),
                challenge_delimiter: "\n\n".to_string(),
                max_elapsed_minutes: 15,
            })
        }
    }

    match env.secret("SOUNDCLOUD_CLIENT_ID") {
        Err(_) => {}
        Ok(s) => {
            flow.soundcloud_verification = Some(SoundCloudVerificationFlow {
                client_id: s.to_string(),
                limit: 100,
                max_offset: 9000,
            })
        }
    }

    match env.secret("TWITTER_BEARER_TOKEN") {
        Err(_) => {}
        Ok(s) => {
            flow.twitter_verification = Some(TwitterVerificationFlow {
                api_key: s.to_string(),
                delimiter: "\n\n".to_string(),
            })
        }
    }

    flow
}

fn new_issuer(env: &Env) -> Result<Ed25519Jwk> {
    Ed25519Jwk::new(
        &env.secret("DID_WEB")?.to_string(),
        &env.secret("REBASE_SK")?.to_string(),
        "controller",
    )
    .map_err(|e| format!("failed to create issuer: {}", e).into())
}

fn preflight_response() -> Result<Response> {
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Methods", "GET,POST,OPTIONS")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Headers", "content-type")?;
    headers.set("Vary", "Origin")?;
    headers.set("Access-Control-Max-Age", "86400")?;

    Ok(Response::empty()
        .unwrap()
        .with_headers(headers)
        .with_status(204))
}

fn post_resp_headers() -> Result<Headers> {
    let mut headers = worker::Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Headers", "content-type")?;
    Ok(headers)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let flow = new_flow(&env);
    let issuer = new_issuer(&env)?;
    let router = Router::with_data((flow, issuer));

    router
        // TODO: Investigate if there is a wild card pattern instead of repetition
        .options("/instructions", |_req, _ctx| preflight_response())
        .post_async("/instructions", |mut req, ctx| async move {
            let _ = &ctx;
            if let Ok(b) = req.json::<InstructionsReq>().await {
                if let Ok(r) = ctx.data.0.handle_instructions(&b).await {
                    let res = Response::from_json(&r)?;
                    return Ok(res.with_headers(post_resp_headers()?));
                };
            };
            Response::error("Bad Request", 400)
        })
        // TODO: Investigate if there is a wild card pattern instead of repetition
        .options("/statement", |_req, _ctx| preflight_response())
        .post_async("/statement", |mut req, ctx| async move {
            let _ = &ctx;
            if let Ok(b) = req.json::<Statements>().await {
                if let Ok(r) = ctx.data.0.handle_statement(&b, &ctx.data.1).await {
                    let res = Response::from_json(&r)?;
                    return Ok(res.with_headers(post_resp_headers()?));
                };
            };
            Response::error("Bad Request", 400)
        })
        // TODO: Investigate if there is a wild card pattern instead of repetition
        .options("/witness_jwt", |_req, _ctx| preflight_response())
        .post_async("/witness_jwt", |mut req, ctx| async move {
            let _ = &ctx;
            if let Ok(b) = req.json::<Proofs>().await {
                if let Ok(r) = ctx.data.0.handle_jwt(&b, &ctx.data.1).await {
                    let res = Response::from_json(&r)?;
                    return Ok(res.with_headers(post_resp_headers()?));
                };
            };
            Response::error("Bad Request", 400)
        })
        // TODO: Investigate if there is a wild card pattern instead of repetition
        .options("/witness_ld", |_req, _ctx| preflight_response())
        .post_async("/witness_ld", |mut req, ctx| async move {
            let _ = &ctx;
            if let Ok(b) = req.json::<Proofs>().await {
                if let Ok(r) = ctx.data.0.handle_ld(&b, &ctx.data.1).await {
                    let res = Response::from_json(&r)?;
                    return Ok(res.with_headers(post_resp_headers()?));
                };
            };
            Response::error("Bad Request", 400)
        })
        // TODO: Investigate if there is a wild card pattern instead of repetition
        .options("/verify", |_req, _ctx| preflight_response())
        .post_async("/verify", |mut req, _ctx| async move {
            if let Ok(t) = req.text().await {
                if let Ok(b) = serde_json::from_str::<VCWrapper>(&t) {
                    if handle_verify(&b, &None).await.is_ok() {
                        return Ok(Response::from_json(&json!({"success": true}))?
                            .with_headers(post_resp_headers()?));
                    };
                }
            }

            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
