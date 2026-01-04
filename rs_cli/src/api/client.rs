// HTTP client for communicating with the Sentinel backend API

use crate::types::*;
use anyhow::{Context, Result};
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;
use std::time::Duration;

/// API client for Sentinel Orchestrator backend
pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url,
            api_key: None,
        })
    }

    /// Create a new API client with authentication
    pub fn with_api_key(base_url: String, api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url,
            api_key: Some(api_key),
        })
    }

    /// Add authentication header to request builder
    fn add_auth_header(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(api_key) = &self.api_key {
            request.header("Authorization", format!("Bearer {}", api_key))
        } else {
            request
        }
    }

    /// Get health status
    pub async fn health(&self) -> Result<HealthStatus> {
        let url = format!("{}/health", self.base_url);
        let request = self.client.get(&url);
        let response = self
            .add_auth_header(request)
            .send()
            .await
            .context("Failed to send health check request")?;

        let status = response
            .json::<HealthStatus>()
            .await
            .context("Failed to parse health status")?;

        Ok(status)
    }

    /// Get readiness status
    pub async fn ready(&self) -> Result<HealthStatus> {
        let url = format!("{}/health/ready", self.base_url);
        let request = self.client.get(&url);
        let response = self
            .add_auth_header(request)
            .send()
            .await
            .context("Failed to send readiness check request")?;

        let status = response
            .json::<HealthStatus>()
            .await
            .context("Failed to parse readiness status")?;

        Ok(status)
    }

    /// Get liveness status
    pub async fn live(&self) -> Result<HealthStatus> {
        let url = format!("{}/health/live", self.base_url);
        let request = self.client.get(&url);
        let response = self
            .add_auth_header(request)
            .send()
            .await
            .context("Failed to send liveness check request")?;

        let status = response
            .json::<HealthStatus>()
            .await
            .context("Failed to parse liveness status")?;

        Ok(status)
    }

    /// Create a chat completion (non-streaming)
    pub async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let request_builder = self.client.post(&url).json(&request);
        let response = self
            .add_auth_header(request_builder)
            .send()
            .await
            .context("Failed to send chat completion request")?;

        let status = response.status();
        if !status.is_success() {
            let status_code = status.as_u16();
            let error: ErrorResponse = response
                .json()
                .await
                .unwrap_or_else(|_| ErrorResponse {
                    code: "unknown".to_string(),
                    message: format!("HTTP {}", status_code),
                    details: None,
                });
            anyhow::bail!("API error: {} - {}", error.code, error.message);
        }

        let completion = response
            .json::<ChatCompletionResponse>()
            .await
            .context("Failed to parse chat completion response")?;

        Ok(completion)
    }

    /// Stream a chat completion
    /// Returns a stream of text chunks from the LLM response
    pub async fn stream_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, anyhow::Error>> + Send>>> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let mut stream_request = request;
        stream_request.stream = true;

        let request_builder = self.client.post(&url).json(&stream_request);
        let response = self
            .add_auth_header(request_builder)
            .send()
            .await
            .context("Failed to send streaming chat completion request")?;

        let status = response.status();
        if !status.is_success() {
            // For error responses, try to parse error message
            // We need to clone status before consuming response
            let status_code = status.as_u16();
            // Consume response for error parsing - we can't use it after this
            let error_msg = match response.json::<ErrorResponse>().await {
                Ok(error) => format!("API error: {} - {}", error.code, error.message),
                Err(_) => format!("HTTP error: {}", status_code),
            };
            anyhow::bail!("{}", error_msg);
        }

        // For Server-Sent Events (SSE) or chunked responses
        // Parse the stream line by line
        let stream = response
            .bytes_stream()
            .map(|result| {
                result
                    .map(|bytes| {
                        // Try to parse as UTF-8, handling partial chunks
                        String::from_utf8_lossy(bytes.as_ref()).to_string()
                    })
                    .map_err(|e| anyhow::anyhow!("Stream error: {}", e))
            });

        Ok(Box::pin(stream))
    }
}

