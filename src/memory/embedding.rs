use reqwest::Client;
use serde_json::json;

// #[derive(Clone)]
// pub struct EmbeddingService {
//     pub api_key: String,
// }

// impl EmbeddingService {
//     pub fn new(api_key: String) -> Self {
//         Self { api_key }
//     }

//     pub async fn generate(&self, input: &str) -> anyhow::Result<Vec<f32>> {
//         let client = reqwest::Client::new();

//         let res = client
//             .post("https://openrouter.ai/api/v1/embeddings")
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .header("Content-Type", "application/json")
//             .json(&serde_json::json!({
//                 "model": "text-embedding-3-small",
//                 "input": input
//             }))
//             .send()
//             .await?;

//         let body: serde_json::Value = res.json().await?;
//         println!("Embedding response: {:?}", body);

//         let embedding = body["data"][0]["embedding"]
//             .as_array()
//             .unwrap()
//             .iter()
//             .map(|v| v.as_f64().unwrap() as f32)
//             .collect();

//         Ok(embedding)
//     }
// }

#[derive(Clone)]
pub struct EmbeddingService {
    pub api_key: String,
}

impl EmbeddingService {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn generate(&self, input: &str) -> anyhow::Result<Vec<f32>> {
        let client = Client::new();

        let res = client
            .post("https://openrouter.ai/api/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "http://localhost")
            .header("X-Title", "AI Coding Agent")
            .json(&json!({
                "model": "nvidia/llama-nemotron-embed-vl-1b-v2:free",
                "input": input
            }))
            .send()
            .await?;

        let status = res.status();
        let body: serde_json::Value = res.json().await?;

        println!("Embedding response: {:?}", body);

        if !status.is_success() {
            return Err(anyhow::anyhow!("Embedding API Error: {}", body));
        }

        let embedding = body["data"][0]["embedding"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing embedding data"))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        Ok(embedding)
    }
}

