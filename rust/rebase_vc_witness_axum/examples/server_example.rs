extern crate rebase_vc_witness_axum;
use serde_json::from_str;

#[tokio::main]
async fn main() {
    let s = include_str!("../rebase.json");
    let config: rebase_vc_witness_axum::Config = from_str(s).unwrap();

    let app = rebase_vc_witness_axum::service(config).await;
    axum::Server::bind(&"0.0.0.0:8787".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
