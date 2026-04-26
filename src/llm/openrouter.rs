// use reqwest::Client;
// use serde_json::json;

// #[derive(Clone)]
// pub struct OpenRouter {
//     pub api_key: String,
// }

// impl OpenRouter {
//     pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
//         let client = Client::new();

//         let res = client
//             .post("https://openrouter.ai/api/v1/chat/completions")
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .header("Content-Type", "application/json")
//             .header("HTTP-Referer", "http://localhost")
//             .header("X-Title", "AI Coding Agent")
//             .json(&json!({
//                 "model": "qwen/qwen3-coder:free",   // ✅ YOUR MODEL
//                 "messages": [
//                     {
//                         "role": "system",
//                         "content": "You are an expert senior software engineer."
//                     },
//                     {
//                         "role": "user",
//                         "content": prompt
//                     }
//                 ],
//                 "max_tokens": 800
//             }))
//             .send()
//             .await?;

//         let body: serde_json::Value = res.json().await?;

//         println!("QWEN DEBUG: {:?}", body);

//         if body.get("error").is_some() {
//             return Err(anyhow::anyhow!("API Error: {:?}", body));
//         }

//         let content = body["choices"][0]["message"]["content"]
//             .as_str()
//             .unwrap_or("No response");

//         Ok(content.to_string())
//     }
// }

// use reqwest::Client;
// use serde_json::json;

// #[derive(Clone)]
// pub struct OpenRouter {
//     pub api_key: String,
// }

// impl OpenRouter {
//     pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
//         let client = Client::new();

//         let res = client
//             .post("https://api.aimlapi.com/v1/responses")
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .header("Content-Type", "application/json")
//             .json(&json!({
//                 "model": "openai/gpt-5-4-pro",
//                 "messages": [
//                     {"role": "user", "content": prompt}
//                 ],
//                 "max_tokens": 500
//             }))
//             .send()
//             .await?;

//         let body: serde_json::Value = res.json().await?;

//         println!("DEBUG AIML: {:?}", body);

//         Ok(body["choices"][0]["message"]["content"]
//             .as_str()
//             .unwrap_or("No response")
//             .to_string())
//     }
// }

// use reqwest::Client;
// use serde_json::json;

// #[derive(Clone)]
// pub struct OpenRouter {
//     pub api_key: String,
// }

// impl OpenRouter {
//     pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
//         let client = Client::new();

//         let res = client
//             .post("https://api.aimlapi.com/v1/chat/completions") // ✅ correct endpoint
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .header("Content-Type", "application/json")
//             .json(&json!({
//                 "model": "openai/gpt-5-4-pro",
//                 "messages": [
//                     {
//                         "role": "user",
//                         "content": prompt
//                     }
//                 ],
//                 "max_tokens": 500
//             }))
//             .send()
//             .await?;

//         let body: serde_json::Value = res.json().await?;

//         println!("AIML DEBUG: {:?}", body);

//         // ✅ Safe extraction
//         let content = body["choices"][0]["message"]["content"]
//             .as_str()
//             .unwrap_or("No response");

//         Ok(content.to_string())
//     }
// }

use rand::Rng;
use reqwest::Client;
use serde_json::json;
use serde_json::{Value};
use tokio::time::{Duration, sleep};

#[derive(Clone)]
pub struct OpenRouter {
    pub api_key: String,
}

impl OpenRouter {
    pub async fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let client = Client::new();

let user_content = prompt.to_string();

        let models = [
            "qwen/qwen3-coder:free",
            "meta-llama/llama-3.2-3b-instruct:free",
            "deepseek/deepseek-r1:free",
            "openrouter/free",
        ];

        let mut last_err: Option<anyhow::Error> = None;

        for model in models {
            for attempt in 0..4 {
                let res = client
                    .post("https://openrouter.ai/api/v1/chat/completions")
                    .header("Authorization", format!("Bearer {}", self.api_key))
                    .header("Content-Type", "application/json")
                    .header("HTTP-Referer", "http://localhost")
                    .header("X-Title", "AI Coding Agent")
                    .json(&json!({
                        "model": model,
                        "messages": [
                            {
                                "role": "system",
                                "content": "You are an expert senior software engineer. Optimize the given code and return only the improved code unless explanation is necessary."
                            },
                            {
                                "role": "user",
                                "content": user_content
                            }
                        ],
                        "max_tokens": 800
                    }))
                    .send()
                    .await;

                let res = match res {
                    Ok(r) => r,
                    Err(e) => {
                        last_err = Some(anyhow::anyhow!("network error on model {}: {}", model, e));
                        break;
                    }
                };

                let status = res.status();
                let body: Value = match res.json().await {
                    Ok(v) => v,
                    Err(e) => {
                        last_err = Some(anyhow::anyhow!("invalid json on model {}: {}", model, e));
                        break;
                    }
                };

                println!("MODEL {} DEBUG: {:?}", model, body);

                if status.is_success() {
                    if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
                        return Ok(content.to_string());
                    } else {
                        last_err = Some(anyhow::anyhow!("missing content for model {}", model));
                        break;
                    }
                }

                let code_num = body["error"]["code"]
                    .as_i64()
                    .unwrap_or(status.as_u16() as i64);
                let msg = body["error"]["message"].as_str().unwrap_or("unknown error");

                if code_num == 429 {
                    let base = 500u64 * (1u64 << attempt);
                    let jitter = rand::thread_rng().gen_range(0..250u64);
                    sleep(Duration::from_millis(base + jitter)).await;
                    last_err = Some(anyhow::anyhow!("rate limited on {}: {}", model, msg));
                    continue;
                }

                last_err = Some(anyhow::anyhow!("API error on {}: {}", model, body));
                break;
            }
        }

        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("all models failed")))
    }
}
