use serde::{Deserialize, Serialize};
use crate::graphql_client::graphql_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonStatusResult {
    pub version: String,
    pub status: DaemonStatus,
}

#[derive(Debug, Serialize, Deserialize)]
struct DaemonStatus {
    consensusConfiguration: ConsensusConfiguration,
    chainId: String,
    blockchainLength: i32,
    consensusMechanism: String,
    highestBlockLengthReceived: i32,
    highestUnvalidatedBlockLengthReceived: i32,
    globalSlotSinceGenesisBestTip: i32,
    ledgerMerkleRoot: String,
    uptimeSecs: i32,
    syncStatus: String,
    stateHash: String,
    commitId: String,
    consensusTimeBestTip: ConsensusTime,
    consensusTimeNow: ConsensusTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConsensusConfiguration {
    slotsPerEpoch: i32,
    slotDuration: i32,
    k: i32,
    genesisStateTimestamp: String,
    epochDuration: i32,
    delta: i32,
    acceptableNetworkDelay: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConsensusTime {
    startTime: String,
    slot: String,
    globalSlot: String,
    epoch: String,
    endTime: String,
}


impl DaemonStatusResult {
    pub async fn get(graphql_url: &str) -> Self {
        let query = r#"
            query DaemonStatus {
              version
              daemonStatus {
                consensusConfiguration {
                  slotsPerEpoch
                  slotDuration
                  k
                  genesisStateTimestamp
                  epochDuration
                  delta
                  acceptableNetworkDelay
                }
                chainId
                blockchainLength
                consensusMechanism
                highestBlockLengthReceived
                highestUnvalidatedBlockLengthReceived
                globalSlotSinceGenesisBestTip
                ledgerMerkleRoot
                uptimeSecs
                syncStatus
                stateHash
                commitId
                consensusTimeBestTip {
                  startTime
                  slot
                  globalSlot
                  epoch
                  endTime
                }
                consensusTimeNow {
                  startTime
                  globalSlot
                  slot
                  epoch
                  endTime
                }
              }
            }"#;

        let res = graphql_request(graphql_url, query).await;

        let version = res["data"]["version"].as_str().unwrap().to_string();
        let status = serde_json::from_value(res["data"]["daemonStatus"].clone()).unwrap();

        Self {
            version,
            status,
        }
    }
}