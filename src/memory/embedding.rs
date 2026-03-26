use reqwest::Client;
use serde_json::json;

#[derive(Clone)]
pub struct EmbeddingService {
    pub api_key: String,
}

impl EmbeddingService {
    pub async fn generate(&self, input: &str) -> anyhow::Result<Vec<f32>> {
        let client = Client::new();

        let res = client
            .post("https://openrouter.ai/api/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "text-embedding-3-small",
                "input": input
            }))
            .send()
            .await?;

        let body: serde_json::Value = res.json().await?;

        let embedding = body["data"][0]["embedding"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap() as f32)
            .collect();

        Ok(embedding)
    }
}