use rebase::{proof, statement, test_util::util::*, types::types::Statement};

use rebase_witness_sdk::{
    client::{Client, Endpoints},
    types::{Proofs, StatementReq, Statements, WitnessReq},
};
use std::env;
use tokio;
use url::Url;

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

async fn check_statement(client: &Client, opts: Statements, statement: &str) -> Result<(), String> {
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

    let inner = statement::dns::Dns {
        domain: "tzprofiles.dev".to_owned(),
        prefix: "rebase_sig=".to_owned(),
        subject: did,
    };

    let opts = Statements::Dns(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("DNS statement valid...");

    let req = WitnessReq {
        proof: Proofs::Dns(inner.clone()),
    };

    client.jwt(req).await.unwrap();

    println!("DNS credential issued");

    println!("Tesing GitHub...");
    let did = test_eth_did();
    let inner = statement::github::GitHub {
        handle: "krhoda".to_string(),
        subject: did,
    };

    let opts = Statements::GitHub(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("GitHub statement valid...");

    let proof = proof::github::GitHub {
        gist_id: "28fb83438a26e70350ef3195d999882d".to_string(),
        statement: inner,
    };

    let req = WitnessReq {
        proof: Proofs::GitHub(proof),
    };

    client.jwt(req).await.unwrap();

    println!("GitHub credential issued");

    println!("Testing Reddit...");
    let did = test_eth_did();
    let inner = statement::reddit::Reddit {
        handle: "eval-apply-quote".to_string(),
        subject: did,
    };

    let opts = Statements::Reddit(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Reddit statement valid...");

    let req = WitnessReq {
        proof: Proofs::Reddit(inner),
    };

    client.jwt(req).await.unwrap();

    println!("Reddit credential issued");

    println!("Testing SoundCloud...");
    let did = test_eth_did();
    let inner = statement::soundcloud::SoundCloud {
        permalink: "spruce-systems-dev".to_string(),
        subject: did,
    };

    let opts = Statements::SoundCloud(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("SoundCloud statement valid...");

    let req = WitnessReq {
        proof: Proofs::SoundCloud(inner),
    };

    client.jwt(req).await.unwrap();

    println!("SoundCloud credential issued");

    println!("Testing Twitter...");

    let did = test_eth_did();
    let inner = statement::twitter::Twitter {
        handle: "evalapplyquote".to_string(),
        subject: did,
    };

    let opts = Statements::Twitter(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Twitter statement valid...");

    let proof = proof::twitter::Twitter {
        tweet_url: "https://twitter.com/evalapplyquote/status/1542901885815820288".to_string(),
        statement: inner,
    };

    let req = WitnessReq {
        proof: Proofs::Twitter(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Twitter credential issued");

    println!("Testing Self Signed...");

    let did = test_eth_did();
    let did2 = test_eth_did_2();

    let inner = statement::same::Same {
        id1: did,
        id2: did2,
    };

    let opts = Statements::Same(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Self Signed Statement valid...");

    let proof = proof::same::Same {
        statement: inner,
        signature1: TEST_2KEY_ETH_SIG_1.to_owned(),
        signature2: TEST_2KEY_ETH_SIG_2.to_owned(),
    };

    let req = WitnessReq {
        proof: Proofs::Same(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Self Signed Credential issued");
    println!("All Ethereum Live Posts tested!");

    println!("Starting Solana tests:");
    println!("NOTE: Does not test DNS, Reddit, or Soundcloud flows");
    println!("Testing GitHub...");
    let did = test_solana_did();
    let inner = statement::github::GitHub {
        handle: "krhoda".to_string(),
        subject: did,
    };

    let opts = Statements::GitHub(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("GitHub statement valid...");

    let proof = proof::github::GitHub {
        gist_id: "b300fd41272159662bccf9702c0a66fd".to_string(),
        statement: inner,
    };

    let req = WitnessReq {
        proof: Proofs::GitHub(proof),
    };

    client.jwt(req).await.unwrap();

    println!("GitHub credential issued");
    println!("Testing Twitter...");

    let did = test_solana_did();
    let inner = statement::twitter::Twitter {
        handle: "evalapplyquote".to_string(),
        subject: did,
    };

    let opts = Statements::Twitter(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Twitter statement valid...");

    let proof = proof::twitter::Twitter {
        tweet_url: "https://twitter.com/evalapplyquote/status/1561743461287505920".to_string(),
        statement: inner,
    };

    let req = WitnessReq {
        proof: Proofs::Twitter(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Twitter credential issued");

    println!("Testing Self Signed...");

    let did = test_solana_did();
    let did2 = test_solana_did_2();
    let inner = statement::same::Same {
        id1: did,
        id2: did2,
    };

    let opts = Statements::Same(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Self Signed Statement valid...");

    let proof = proof::same::Same {
        statement: inner,
        signature1: TEST_2KEY_SOLANA_SIG_1.to_owned(),
        signature2: TEST_2KEY_SOLANA_SIG_2.to_owned(),
    };

    let req = WitnessReq {
        proof: Proofs::Same(proof),
    };

    client.jwt(req).await.unwrap();

    println!("Self Signed Credential issued");

    println!("All Live Posts tested!");
}
