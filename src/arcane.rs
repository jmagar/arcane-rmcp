use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::Method;
use serde_json::Value;

use crate::config::ArcaneConfig;

#[cfg(test)]
#[path = "arcane_tests.rs"]
mod tests;

#[derive(Clone)]
pub struct ArcaneClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl ArcaneClient {
    pub fn new(cfg: &ArcaneConfig) -> Result<Self> {
        if cfg.api_url.trim().is_empty() {
            anyhow::bail!("RARCANE_API_URL is not set");
        }
        if cfg.api_key.trim().is_empty() {
            anyhow::bail!("RARCANE_API_KEY is not set");
        }
        let http = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .build()
            .context("failed to build Arcane HTTP client")?;
        Ok(Self {
            http,
            base_url: normalize_base_url(&cfg.api_url),
            api_key: cfg.api_key.clone(),
        })
    }

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        query: Option<&Value>,
        body: Option<&Value>,
        timeout: Option<Duration>,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self
            .http
            .request(method, url)
            .header("X-API-Key", &self.api_key);
        if let Some(timeout) = timeout {
            request = request.timeout(timeout);
        }
        if let Some(query) = query.and_then(Value::as_object) {
            request = request.query(query);
        }
        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await.map_err(|err| {
            anyhow::anyhow!("Arcane API request failed: {}", redact(&err.to_string()))
        })?;
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|err| anyhow::anyhow!("Arcane API response read failed: {err}"))?;
        if !status.is_success() {
            let message = serde_json::from_str::<Value>(&text)
                .ok()
                .and_then(|value| {
                    value
                        .get("message")
                        .and_then(Value::as_str)
                        .or_else(|| value.get("error").and_then(Value::as_str))
                        .map(str::to_owned)
                })
                .unwrap_or_else(|| text.clone());
            anyhow::bail!("Arcane API error {}: {}", status.as_u16(), redact(&message));
        }
        if text.trim().is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_str(&text).with_context(|| "Arcane API returned invalid JSON")
    }
}

fn normalize_base_url(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.ends_with("/api") {
        trimmed.to_owned()
    } else {
        format!("{trimmed}/api")
    }
}

pub fn encode_path_segment(value: &str) -> String {
    url::form_urlencoded::byte_serialize(value.as_bytes()).collect()
}

fn redact(message: &str) -> String {
    message.replace("X-API-Key", "[redacted-header]")
}
