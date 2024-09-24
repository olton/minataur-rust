use serde::{Deserialize, Serialize};
use crate::graphql_client::graphql_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct MinaVersionResult {
    pub version: String,
    pub network: String,
}

impl MinaVersionResult {
    pub async fn get(graphql_url: &str) -> Self {
        let query = r#"
            query MinaVersion {
                version
                networkID
            }"#;

        let res = graphql_request(graphql_url, query).await;

        let version = res["data"]["version"].as_str();
        let network = res["data"]["networkID"].as_str();

        Self {
            version: version.unwrap().to_string(),
            network: network.unwrap().to_string(),
        }
    }
}