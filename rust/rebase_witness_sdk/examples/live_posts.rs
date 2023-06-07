use rebase::{proof, statement, test_util::util::*, types::defs::Statement};

use rebase_witness_sdk::{
    client::{Client, Endpoints},
    types::{Proofs, Statements},
};
use std::env;
use url::Url;

fn new_client(base_url: &str) -> Result<Client, String> {
    // TODO: Update to use a worker that supports LD routes.
    let endpoints = Endpoints {
        witness_jwt: Some(Url::parse(&format!("{}/witness", base_url)).unwrap()),
        witness_ld: None,
        statement: Url::parse(&format!("{}/statement", base_url)).unwrap(),
        instructions: Url::parse(&format!("{}/instructions", base_url)).unwrap(),
        verify: None,
    };

    Client::new(endpoints).map_err(|e| e.to_string())
}

async fn check_statement(
    client: &Client,
    stmts: Statements,
    statement: &str,
) -> Result<(), String> {
    let res = client.statement(stmts).await.unwrap();

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
    let url = env::args().nth(1).unwrap();
    let client = new_client(&url).unwrap();

    println!("Starting Ethereum tests:");
    println!("Testing DNS...");
    let did = test_eth_did();

    let inner = statement::dns_verification::DnsVerificationStatement {
        domain: "tzprofiles.dev".to_owned(),
        prefix: "rebase_sig=".to_owned(),
        subject: did,
    };

    let opts = Statements::DnsVerification(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("DNS statement valid...");

    let req = Proofs::DnsVerification(inner.clone());

    client.witness_jwt(req).await.unwrap();

    println!("DNS credential issued");

    println!("Tesing GitHub...");
    let did = test_eth_did();
    let inner = statement::github_verification::GitHubVerificationStatement {
        handle: "krhoda".to_string(),
        subject: did,
    };

    let opts = Statements::GitHubVerification(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("GitHub statement valid...");

    let proof = proof::github_verification::GitHubVerificationProof {
        gist_id: "28fb83438a26e70350ef3195d999882d".to_string(),
        statement: inner,
    };

    let req = Proofs::GitHubVerification(proof);

    client.witness_jwt(req).await.unwrap();

    println!("GitHub credential issued");

    println!("Testing Reddit...");
    let did = test_eth_did();
    let inner = statement::reddit_verification::RedditVerificationStatement {
        handle: "eval-apply-quote".to_string(),
        subject: did,
    };

    let opts = Statements::RedditVerification(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Reddit statement valid...");

    let req = Proofs::RedditVerification(inner);

    client.witness_jwt(req).await.unwrap();

    println!("Reddit credential issued");

    println!("Testing SoundCloud...");
    let did = test_eth_did();
    let inner = statement::soundcloud_verification::SoundCloudVerificationStatement {
        permalink: "spruce-systems-dev".to_string(),
        subject: did,
    };

    let opts = Statements::SoundCloudVerification(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("SoundCloud statement valid...");

    let req = Proofs::SoundCloudVerification(inner);

    client.witness_jwt(req).await.unwrap();

    println!("SoundCloud credential issued");

    println!("Testing Twitter...");

    let did = test_eth_did();
    let inner = statement::twitter_verification::TwitterVerificationStatement {
        handle: "evalapplyquote".to_string(),
        subject: did,
    };

    let opts = Statements::TwitterVerification(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Twitter statement valid...");

    let proof = proof::twitter_verification::TwitterVerificationProof {
        tweet_url: "https://twitter.com/evalapplyquote/status/1542901885815820288".to_string(),
        statement: inner,
    };

    let req = Proofs::TwitterVerification(proof);

    client.witness_jwt(req).await.unwrap();

    println!("Twitter credential issued");

    println!("Testing Self Signed...");

    let did = test_eth_did();
    let did2 = test_eth_did_2();

    let inner = statement::same_controller_assertion::SameControllerAssertionStatement {
        id1: did,
        id2: did2,
    };

    let opts = Statements::SameControllerAssertion(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Self Signed Statement valid...");

    let proof = proof::same_controller_assertion::SameControllerAssertionProof {
        statement: inner,
        signature1: TEST_2KEY_ETH_SIG_1.to_owned(),
        signature2: TEST_2KEY_ETH_SIG_2.to_owned(),
    };

    let req = Proofs::SameControllerAssertion(proof);

    client.witness_jwt(req).await.unwrap();

    println!("Self Signed Credential issued");
    println!("All Ethereum Live Posts tested!");

    println!("Starting Solana tests:");
    println!("NOTE: Does not test DNS, Reddit, or Soundcloud flows");
    println!("Testing GitHub...");
    let did = test_solana_did();
    let inner = statement::github_verification::GitHubVerificationStatement {
        handle: "krhoda".to_string(),
        subject: did,
    };

    let opts = Statements::GitHubVerification(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("GitHub statement valid...");

    let proof = proof::github_verification::GitHubVerificationProof {
        gist_id: "b300fd41272159662bccf9702c0a66fd".to_string(),
        statement: inner,
    };

    let req = Proofs::GitHubVerification(proof);

    client.witness_jwt(req).await.unwrap();

    println!("GitHub credential issued");
    println!("Testing Twitter...");

    let did = test_solana_did();
    let inner = statement::twitter_verification::TwitterVerificationStatement {
        handle: "evalapplyquote".to_string(),
        subject: did,
    };

    let opts = Statements::TwitterVerification(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Twitter statement valid...");

    let proof = proof::twitter_verification::TwitterVerificationProof {
        tweet_url: "https://twitter.com/evalapplyquote/status/1561743461287505920".to_string(),
        statement: inner,
    };

    let req = Proofs::TwitterVerification(proof);

    client.witness_jwt(req).await.unwrap();

    println!("Twitter credential issued");

    println!("Testing Self Signed...");

    let did = test_solana_did();
    let did2 = test_solana_did_2();
    let inner = statement::same_controller_assertion::SameControllerAssertionStatement {
        id1: did,
        id2: did2,
    };

    let opts = Statements::SameControllerAssertion(inner.clone());

    let statement = opts.generate_statement().unwrap();

    check_statement(&client, opts, &statement).await.unwrap();

    println!("Self Signed Statement valid...");

    let proof = proof::same_controller_assertion::SameControllerAssertionProof {
        statement: inner,
        signature1: TEST_2KEY_SOLANA_SIG_1.to_owned(),
        signature2: TEST_2KEY_SOLANA_SIG_2.to_owned(),
    };

    let req = Proofs::SameControllerAssertion(proof);

    client.witness_jwt(req).await.unwrap();

    println!("Self Signed Credential issued");

    println!("All Live Posts tested!");
}
