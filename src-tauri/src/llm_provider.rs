use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
}

pub trait LLMProvider {
    fn name(&self) -> &str;
    async fn generate(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse, Box<dyn Error>>;
}

pub struct OllamaProvider {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            model,
        }
    }
}

impl LLMProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn generate(&self, messages: Vec<ChatMessage>) -> Result<ChatResponse, Box<dyn Error>> {
        let url = format!("{}/api/chat", self.base_url);
        
        let mut payload = HashMap::new();
        payload.insert("model".to_string(), serde_json::to_value(&self.model)?);
        payload.insert("messages".to_string(), serde_json::to_value(messages)?);
        payload.insert("stream".to_string(), serde_json::to_value(false)?);

        let res = self.client.post(&url)
            .json(&payload)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let content = res["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(ChatResponse { content })
    }
}
