use axum::{
    routing::{get, post},
    extract::Json,
    response::Json as ResponseJson,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct ClaimRequest {
    user_account: String,
    amount: u128,
    destination_parachain: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaimResponse {
    success: bool,
    message: String,
    xcm_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VestingInfo {
    amount: u128,
    unlock_timestamp: u64,
    is_claimed: bool,
}

// In-memory storage for demo purposes
static mut VESTING_DATA: Option<HashMap<String, VestingInfo>> = None;

async fn hello_world() -> &'static str {
    "Cross-Chain Vesting Vault API - Ready for XCM!"
}

async fn initiate_xcm_claim(
    Json(request): Json<ClaimRequest>,
) -> ResponseJson<ClaimResponse> {
    info!("Initiating XCM claim for user: {}", request.user_account);
    
    // Simulate XCM cross-chain transfer
    let xcm_hash = format!("xcm_{}", hex::encode(&request.user_account.as_bytes()[..8]));
    
    // In a real implementation, this would:
    // 1. Verify user has unlocked tokens in the ink! contract
    // 2. Call the contract's claim function
    // 3. Execute XCM to transfer tokens to destination parachain
    
    info!(
        "XCM transfer initiated: {} tokens to {} (Hash: {})",
        request.amount, request.destination_parachain, xcm_hash
    );
    
    ResponseJson(ClaimResponse {
        success: true,
        message: format!(
            "XCM claim initiated for {} tokens to {}", 
            request.amount, request.destination_parachain
        ),
        xcm_hash: Some(xcm_hash),
    })
}

async fn get_vesting_info(
    Json(account): Json<String>,
) -> ResponseJson<Option<VestingInfo>> {
    unsafe {
        if let Some(ref data) = VESTING_DATA {
            ResponseJson(data.get(&account).cloned())
        } else {
            ResponseJson(None)
        }
    }
}

async fn simulate_deposit(
    Json(request): Json<HashMap<String, serde_json::Value>>,
) -> ResponseJson<ClaimResponse> {
    let account = request.get("account").unwrap().as_str().unwrap().to_string();
    let amount = request.get("amount").unwrap().as_u64().unwrap() as u128;
    let lock_seconds = request.get("lock_seconds").unwrap().as_u64().unwrap();
    
    let unlock_timestamp = chrono::Utc::now().timestamp() as u64 + lock_seconds;
    
    let vesting_info = VestingInfo {
        amount,
        unlock_timestamp,
        is_claimed: false,
    };
    
    unsafe {
        if VESTING_DATA.is_none() {
            VESTING_DATA = Some(HashMap::new());
        }
        VESTING_DATA.as_mut().unwrap().insert(account.clone(), vesting_info);
    }
    
    ResponseJson(ClaimResponse {
        success: true,
        message: format!("Deposit simulated for account: {}", account),
        xcm_hash: None,
    })
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/xcm/claim", post(initiate_xcm_claim))
        .route("/vesting/info", post(get_vesting_info))
        .route("/simulate/deposit", post(simulate_deposit));

    Ok(router.into())
}
