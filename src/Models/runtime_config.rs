use serde::{Deserialize, Serialize};
use crate::graphql::request;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfigResult {
    pub version: String,
    pub runtimeConfig: RuntimeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct RuntimeConfig {
    daemon: Daemon,
    genesis: Genesis,
    proof: Proof,
    ledger: Ledger,
    epoch_data: EpochData,
}

#[derive(Debug, Serialize, Deserialize)]
struct Daemon {
    txpool_max_size: u32,
    zkapp_proof_update_cost: f32,
    zkapp_signed_single_update_cost: f32,
    zkapp_signed_pair_update_cost: f32,
    zkapp_transaction_cost_limit: f32,
    max_event_elements: u32,
    max_action_elements: u32,
    zkapp_cmd_limit_hardcap: u32,
    network_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Genesis {
    k: u32,
    delta: u32,
    slots_per_epoch: u32,
    slots_per_sub_window: u32,
    grace_period_slots: u32,
    genesis_state_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Proof {
    level: String,
    sub_windows_per_window: u32,
    ledger_depth: u32,
    work_delay: f32,
    block_window_duration_ms: u32,
    transaction_capacity: TransactionCapacity,
    coinbase_amount: String,
    supercharged_coinbase_factor: u8,
    account_creation_fee: String,
    fork: Fork,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ledger {
    hash: String,
    s3_data_hash: String,
    add_genesis_winner: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct EpochData {
    staking: Staking,
    next: Staking
}

#[derive(Debug, Serialize, Deserialize)]
struct Staking {
    seed: String,
    s3_data_hash: String,
    hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionCapacity {
    #[serde(rename = "2_to_the")]
    two_to_the: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Fork {
    state_hash: String,
    blockchain_length: u32,
    global_slot_since_genesis: u32,
}

impl RuntimeConfigResult {
    pub async fn get(graphql_url: &str) -> Self {
        let query = r#"
            query Runtime {
                version
                runtimeConfig
            }"#;

        let res = request(graphql_url, query).await;

        let version = res["data"]["version"].as_str().unwrap().to_string();
        let runtime_config = serde_json::from_value(res["data"]["runtimeConfig"].clone()).unwrap();

        Self {
            version,
            runtimeConfig: runtime_config,
        }
    }
}