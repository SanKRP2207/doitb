use reqwest::Client;
use serde_json::json;

#[derive(Clone)]
pub struct OpenAI {
    pub api_key: String,
}

impl OpenAI {
    pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let client = Client::new();

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": "gpt-4o-mini",
                "messages": [
                    {"role": "system", "content": "You are a senior software engineer."},
                    {"role": "user", "content": prompt}
                ]
            }))
            .send()
            .await?;

        let body: serde_json::Value = res.json().await?;

        Ok(body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }
}