use anyhow::{Context, Result};
use serde_json::{json, Value};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL:   &str = "claude-sonnet-4-6";

#[derive(Debug, Clone)]
pub struct Message {
    pub role:    String,
    pub content: String,
}

pub struct ClaudeClient {
    api_key: String,
    system:  String,
    history: Vec<Message>,
    client:  reqwest::Client,
}

impl ClaudeClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            system:  "You are Cortex, an intelligent personal AI assistant. \
                      You are precise, helpful, and have a calm British wit. \
                      You enhance the user — you do not replace them.".into(),
            history: vec![],
            client:  reqwest::Client::new(),
        }
    }

    pub fn set_system(&mut self, system: impl Into<String>) {
        self.system = system.into();
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn history(&self) -> &[Message] {
        &self.history
    }

    /// Send a user message and get a response. Maintains conversation history.
    pub async fn chat(&mut self, user_msg: &str) -> Result<String> {
        self.history.push(Message { role: "user".into(), content: user_msg.into() });

        let messages: Vec<Value> = self.history.iter().map(|m| json!({
            "role":    m.role,
            "content": m.content,
        })).collect();

        let body = json!({
            "model":      MODEL,
            "max_tokens": 1024,
            "system":     self.system,
            "messages":   messages,
        });

        let resp = self.client
            .post(API_URL)
            .header("x-api-key",           &self.api_key)
            .header("anthropic-version",   "2023-06-01")
            .header("content-type",        "application/json")
            .json(&body)
            .send().await
            .context("Claude API request failed")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text   = resp.text().await.unwrap_or_else(|_| "<no body>".into());
            anyhow::bail!("Claude API error {status}: {text}");
        }

        let json: Value = resp.json().await.context("invalid JSON from Claude API")?;
        let reply = json["content"][0]["text"]
            .as_str()
            .context("missing text in Claude response")?
            .to_string();

        self.history.push(Message { role: "assistant".into(), content: reply.clone() });
        Ok(reply)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_starts_with_empty_history() {
        let client = ClaudeClient::new("test-key");
        assert_eq!(client.history().len(), 0);
    }

    #[test]
    fn set_system_updates_prompt() {
        let mut client = ClaudeClient::new("test-key");
        client.set_system("Custom system prompt.");
        assert!(client.system.contains("Custom system prompt"));
    }

    #[test]
    fn clear_history_empties_vec() {
        let mut client = ClaudeClient::new("test-key");
        client.history.push(Message { role: "user".into(), content: "hi".into() });
        client.clear_history();
        assert_eq!(client.history().len(), 0);
    }

    #[test]
    fn model_constant_is_correct() {
        assert_eq!(MODEL, "claude-sonnet-4-6");
    }
}
