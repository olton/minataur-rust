use serde_json::{Value, json};

pub async fn request(url: &str, query: &str) -> Value {
    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&json!({ "query": query }))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    res
}