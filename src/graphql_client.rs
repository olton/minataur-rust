use serde_json::Value;

pub async fn graphql_request(url: &str, query: &str) -> Value {
    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    res
}