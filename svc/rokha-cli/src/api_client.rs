use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: &str = "3.2.0";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct SchemaVersion {
    pub version: String,
}

pub struct RokhaClient {
    http: reqwest::Client,
    base_url: String,
}

impl RokhaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn health(&self) -> Result<serde_json::Value, reqwest::Error> {
        self.http
            .get(format!("{}/health", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn schema_version(&self) -> Result<SchemaVersion, reqwest::Error> {
        self.http
            .get(format!("{}/api/schema/version", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list_tools(&self) -> Result<Vec<Tool>, reqwest::Error> {
        self.http
            .get(format!("{}/api/marketplace/listings", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_tool_info(&self, name: &str) -> Result<Tool, reqwest::Error> {
        self.http
            .get(format!("{}/api/marketplace/listings/{}", self.base_url, name))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn chat(&self, message: &str) -> Result<String, reqwest::Error> {
        let body = serde_json::json!({ "message": message });
        let resp: serde_json::Value = self
            .http
            .post(format!("{}/api/agents/rokha-agent/chat/public", self.base_url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp["response"]
            .as_str()
            .or_else(|| resp["message"].as_str())
            .unwrap_or("")
            .to_string())
    }
}
