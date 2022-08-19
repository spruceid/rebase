use rebase::{util::util::*, witness::witness::Statement, witness::*};
use rebase_witness_sdk::{
    witness::{StatementReq, WitnessReq},
    client::{Client, Endpoints}
};
use std::env;
use url::Url;
use tokio;

fn new_client(base_url: &str) -> Result<Client, String> {
    // TODO: Update to use a worker that supports LD routes.
    let endpoints = Endpoints {
        jwt: Some(Url::parse(&format!("{}/witness", base_url)).unwrap()),
        ld: None,
        statement: Url::parse(&format!("{}/statement", base_url)).unwrap(),
        instructions: Url::parse(&format!("{}/instructions", base_url)).unwrap(),
    };

    Client::new(endpoints).map_err(|e| e.to_string())
}

async fn check_statement(
    client: &Client,
    opts: statement_type::StatementTypes,
    statement: &str,
) -> Result<(), String> {
    let req = StatementReq { opts };

    let res = client.statement(req).await.unwrap();

    if res.statement != statement {
        return Err(format!(
            "Expected matching statements, got: {} AND {}",
            res.statement, statement
        ));
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Building client...");
    let url = env::args().skip(1).next().unwrap();
    let client = new_client(&url).unwrap();

    println!("Starting Ethereum tests:");
    println!("Testing DNS...");
    let did = test_eth_did();

    let opts = dns::Claim {
        domain: "tzprofiles.dev".to_owned(),
        prefix: "rebase_sig=".to_owned(),
        key_type: did,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::Dns(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("DNS statement valid...");

    let req = WitnessReq {
        proof: proof_type::ProofTypes::Dns(opts),
    };

    client.jwt(req).await.unwrap();

    println!("DNS credential issued");

    println!("Tesing GitHub...");
    let did = test_eth_did();
    let opts = github::Opts {
        handle: "krhoda".to_string(),
        key_type: did,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::GitHub(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("GitHub statement valid...");

    let proof = github::Claim {
        gist_id: "28fb83438a26e70350ef3195d999882d".to_string(),
        statement_opts: opts,
    };

    let req = WitnessReq {
        proof: proof_type::ProofTypes::GitHub(proof),
    };

    client.jwt(req).await.unwrap();

    println!("GitHub credential issued");

    println!("Tesing Reddit...");
    let did = test_eth_did();
    let opts = reddit::Claim {
        handle: "eval-apply-quote".to_string(),
        key_type: did,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::Reddit(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("Reddit statement valid...");

    let req = WitnessReq {
        proof: proof_type::ProofTypes::Reddit(opts),
    };

    client.jwt(req).await.unwrap();

    println!("Reddit credential issued");

    println!("Tesing SoundCloud...");
    let did = test_eth_did();
    let opts = soundcloud::Claim {
        permalink: "spruce-systems-dev".to_string(),
        key_type: did,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::SoundCloud(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("SoundCloud statement valid...");

    let req = WitnessReq {
        proof: proof_type::ProofTypes::SoundCloud(opts),
    };

    client.jwt(req).await.unwrap();

    println!("SoundCloud credential issued");

    println!("Testing Twitter...");

    let did = test_eth_did();
    let opts = twitter::Opts {
        handle: "evalapplyquote".to_string(),
        key_type: did,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::Twitter(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("Twitter statement valid...");

    let proof = twitter::Claim {
        tweet_url: "https://twitter.com/evalapplyquote/status/1542901885815820288".to_string(),
        statement_opts: opts,
    };

    let req = WitnessReq {
        proof: proof_type::ProofTypes::Twitter(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Twitter credential issued");
    println!("Testing Self Signed...");

    let did = test_eth_did();
    let did2 = test_eth_did_2();

    let opts = self_signed::Opts {
        key_1: did,
        key_2: did2,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::SelfSigned(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("Self Signed Statement valid...");

    let proof = self_signed::Claim::new(
        opts,
        TEST_2KEY_ETH_SIG_1.to_owned(),
        TEST_2KEY_ETH_SIG_2.to_owned(),
    )
    .await
    .unwrap();

    let req = WitnessReq {
        proof: proof_type::ProofTypes::SelfSigned(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Self Signed Credential issued");
    println!("All Ethereum Live Posts tested!");

    println!("Starting Solana tests:");
    println!("NOTE: Does not test DNS, Reddit, or Soundcloud flows");
    println!("Tesing GitHub...");
    let did = test_solana_did();
    let opts = github::Opts {
        handle: "krhoda".to_string(),
        key_type: did,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::GitHub(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("GitHub statement valid...");

    let proof = github::Claim {
        gist_id: "b300fd41272159662bccf9702c0a66fd".to_string(),
        statement_opts: opts,
    };

    let req = WitnessReq {
        proof: proof_type::ProofTypes::GitHub(proof),
    };

    client.jwt(req).await.unwrap();

    println!("GitHub credential issued");

    println!("Testing Twitter...");

    let did = test_solana_did();
    let opts = twitter::Opts {
        handle: "evalapplyquote".to_string(),
        key_type: did,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::Twitter(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("Twitter statement valid...");

    let proof = twitter::Claim {
        tweet_url: "https://twitter.com/evalapplyquote/status/1561743461287505920".to_string(),
        statement_opts: opts,
    };

    let req = WitnessReq {
        proof: proof_type::ProofTypes::Twitter(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Twitter credential issued");
    println!("Testing Self Signed...");

    let did = test_solana_did();
    let did2 = test_solana_did_2();

    let opts = self_signed::Opts {
        key_1: did,
        key_2: did2,
    };

    let statement = opts.generate_statement().unwrap();

    check_statement(
        &client,
        statement_type::StatementTypes::SelfSigned(opts.clone()),
        &statement,
    )
    .await
    .unwrap();

    println!("Self Signed Statement valid...");

    let proof = self_signed::Claim::new(
        opts,
        TEST_2KEY_SOLANA_SIG_1.to_owned(),
        TEST_2KEY_SOLANA_SIG_2.to_owned(),
    )
    .await
    .unwrap();

    let req = WitnessReq {
        proof: proof_type::ProofTypes::SelfSigned(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Self Signed Credential issued");

    println!("All Live Posts tested!");
}
