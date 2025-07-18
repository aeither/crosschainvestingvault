use axum::{Router, routing::post};
use subxt::OnlineClient;
use std::sync::Arc;

async fn claim_crosschain() -> String {
    // optionally call polkadot-js proxy or subxt call to your contract
    "XCM Claim invoked!".to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/claim", post(claim_crosschain));
    Ok(router.into())
}
