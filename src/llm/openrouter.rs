use reqwest::Client;
use serde_json::json;

#[derive(Clone)]
pub struct OpenRouter {
    pub api_key: String,
}

impl OpenRouter {
    pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let client = Client::new();

        let res = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "http://localhost")
            .header("X-Title", "AI Agent")
            .json(&json!({
                "model": "nvidia/nemotron-3-super-120b-a12b:free",
                "messages": [
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 500
            }))
            .send()
            .await?;

        let body: serde_json::Value = res.json().await?;

        println!("DEBUG: {:?}", body);

        Ok(body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response")
            .to_string())
    }
}