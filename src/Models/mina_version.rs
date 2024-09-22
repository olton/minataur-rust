use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinaVersion {
    pub version: Option<String>,
    pub network: Option<String>,
}

impl MinaVersion {
    pub async fn get(graphql_url: &str) -> Self {
        let query = r#"
            query MinaVersion {
                version
                networkID
            }"#;

        let client = reqwest::Client::new();
        let res = client.post(graphql_url)
            .json(&serde_json::json!({ "query": query }))
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let version = res["data"]["version"].as_str().map(|s| s.to_string());
        let network = res["data"]["networkID"].as_str().map(|s| s.to_string());

        Self {
            version,
            network,
        }
    }
}