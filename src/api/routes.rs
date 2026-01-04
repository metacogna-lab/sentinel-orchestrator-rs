// Axum route handlers with authentication

use axum::extract::Extension;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::api::middleware::{create_auth_middleware, ApiKeyStore, AuthInfo};
use crate::core::auth::AuthLevel;
use crate::core::error::SentinelError;
use crate::core::traits::LLMProvider;
use crate::core::types::{
    AgentStatus, CanonicalMessage, ChatCompletionRequest, ChatCompletionResponse, ErrorResponse,
    HealthState, HealthStatus,
};
use crate::engine::supervisor::Supervisor;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    /// API key store for authentication
    pub key_store: Arc<ApiKeyStore>,
    /// LLM provider for chat completions
    pub llm_provider: Arc<dyn LLMProvider>,
    /// Supervisor for agent management (optional, wrapped in Arc<RwLock> for thread safety)
    pub supervisor: Option<Arc<RwLock<Supervisor>>>,
}

impl AppState {
    /// Create a new application state
    pub fn new(
        key_store: Arc<ApiKeyStore>,
        llm_provider: Arc<dyn LLMProvider>,
        supervisor: Option<Arc<RwLock<Supervisor>>>,
    ) -> Self {
        Self {
            key_store,
            llm_provider,
            supervisor,
        }
    }
}

/// Health check endpoint (no authentication required)
pub async fn health_check() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: HealthState::Healthy,
        timestamp: chrono::Utc::now(),
    })
}

/// Validate chat completion request
fn validate_chat_request(
    request: &ChatCompletionRequest,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    if request.messages.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                code: "invalid_request".to_string(),
                message: "Messages cannot be empty".to_string(),
                details: Some(std::collections::HashMap::from([(
                    "field".to_string(),
                    "messages".to_string(),
                )])),
            }),
        ));
    }

    // Validate each message has non-empty content
    for (idx, msg) in request.messages.iter().enumerate() {
        if msg.content.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    code: "invalid_request".to_string(),
                    message: format!("Message at index {} has empty content", idx),
                    details: Some(std::collections::HashMap::from([(
                        "field".to_string(),
                        format!("messages[{}].content", idx),
                    )])),
                }),
            ));
        }
    }

    Ok(())
}

/// Convert SentinelError to HTTP error response
fn error_to_response(err: SentinelError) -> (StatusCode, Json<ErrorResponse>) {
    match err {
        SentinelError::InvalidMessage { reason } => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                code: "invalid_request".to_string(),
                message: reason,
                details: None,
            }),
        ),
        SentinelError::DomainViolation { rule } => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                code: "internal_error".to_string(),
                message: rule,
                details: None,
            }),
        ),
        SentinelError::AuthenticationFailed { reason } => (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                code: "authentication_failed".to_string(),
                message: reason,
                details: None,
            }),
        ),
        SentinelError::AuthorizationFailed { reason } => (
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                code: "authorization_failed".to_string(),
                message: reason,
                details: None,
            }),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                code: "internal_error".to_string(),
                message: err.to_string(),
                details: None,
            }),
        ),
    }
}

/// Chat completion endpoint (requires write access)
pub async fn chat_completion(
    State(app_state): State<AppState>,
    auth_info: Option<Extension<AuthInfo>>,
    Json(request): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Auth info should be present due to middleware, but check for safety
    let _auth = auth_info.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                code: "not_authenticated".to_string(),
                message: "Request is not authenticated".to_string(),
                details: None,
            }),
        )
    })?;

    info!(
        "Chat completion request received with {} messages",
        request.messages.len()
    );

    // Validate request
    validate_chat_request(&request)?;

    // Convert request messages to CanonicalMessage (they should already be CanonicalMessage)
    let messages: Vec<CanonicalMessage> = request.messages;

    // Call LLM provider
    let response = app_state
        .llm_provider
        .complete(messages)
        .await
        .map_err(error_to_response)?;

    info!("Chat completion successful");

    // Determine model name (use from request or default)
    let model = request
        .model
        .unwrap_or_else(|| "sentinel-orchestrator".to_string());

    Ok(Json(ChatCompletionResponse {
        message: response,
        model,
        // Token usage tracking deferred - requires LLMProvider trait changes
        usage: None,
    }))
}

/// Agent status endpoint (requires read access)
pub async fn agent_status(
    State(app_state): State<AppState>,
    auth_info: Option<Extension<AuthInfo>>,
) -> Result<Json<Vec<AgentStatus>>, (StatusCode, Json<ErrorResponse>)> {
    // Auth info should be present due to middleware, but check for safety
    let _auth = auth_info.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                code: "not_authenticated".to_string(),
                message: "Request is not authenticated".to_string(),
                details: None,
            }),
        )
    })?;

    info!("Agent status request received");

    // Get supervisor if available
    let supervisor = app_state.supervisor.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                code: "service_unavailable".to_string(),
                message: "Supervisor not available".to_string(),
                details: None,
            }),
        )
    })?;

    // Query supervisor for agent statuses
    let supervisor_guard = supervisor.read().await;
    let agent_ids = supervisor_guard.agent_ids();

    let mut agent_statuses = Vec::new();
    for agent_id in agent_ids {
        match supervisor_guard.check_agent_health(agent_id) {
            Ok(health) => {
                // Count messages processed (simplified - would need actual tracking)
                // For now, use 0 as placeholder until we add message counting to AgentHandle
                let messages_processed = 0;

                agent_statuses.push(AgentStatus {
                    id: health.id,
                    state: health.state,
                    last_activity: health.last_activity,
                    messages_processed,
                });
            }
            Err(e) => {
                warn!("Failed to get health for agent {}: {}", agent_id, e);
            }
        }
    }

    drop(supervisor_guard);

    info!("Returning status for {} agents", agent_statuses.len());
    Ok(Json(agent_statuses))
}

/// Create the API router with authentication middleware
pub fn create_router(app_state: AppState) -> Router {
    let key_store = app_state.key_store.clone();
    Router::new()
        .route("/health", get(health_check))
        .route(
            "/v1/chat/completions",
            post(chat_completion).layer(axum::middleware::from_fn(create_auth_middleware(
                key_store.clone(),
                AuthLevel::Write,
            ))),
        )
        .route(
            "/v1/agents/status",
            get(agent_status).layer(axum::middleware::from_fn(create_auth_middleware(
                key_store.clone(),
                AuthLevel::Read,
            ))),
        )
        .with_state(app_state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::auth::{ApiKeyId, AuthLevel};
    use crate::core::traits::LLMProvider;
    use crate::core::types::Role;
    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
    };
    use mockall::mock;
    use tower::ServiceExt;

    // Create mock LLM provider for testing
    mock! {
        TestLLMProvider {}

        #[async_trait]
        impl LLMProvider for TestLLMProvider {
            async fn complete(
                &self,
                messages: Vec<CanonicalMessage>,
            ) -> Result<CanonicalMessage, SentinelError>;

            async fn stream(
                &self,
                messages: Vec<CanonicalMessage>,
            ) -> Result<Box<dyn futures::Stream<Item = Result<String, SentinelError>> + Send + Unpin>, SentinelError>;
        }
    }

    #[tokio::test]
    async fn test_health_check_no_auth() {
        let key_store = Arc::new(ApiKeyStore::new());
        let mut mock_llm = MockTestLLMProvider::new();
        mock_llm
            .expect_complete()
            .returning(|_| Ok(CanonicalMessage::new(Role::Assistant, "test".to_string())));
        let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
        let app_state = AppState::new(key_store, llm_provider, None);
        let app = create_router(app_state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let health: HealthStatus = serde_json::from_slice(&body).unwrap();
        assert_eq!(health.status, HealthState::Healthy);
    }

    #[tokio::test]
    async fn test_chat_completion_requires_auth() {
        let key_store = Arc::new(ApiKeyStore::new());
        let key = "sk-1234567890123456".to_string();
        let key_id = ApiKeyId::new("test-key".to_string());

        key_store
            .add_key(key.clone(), key_id, AuthLevel::Write)
            .await;

        let mut mock_llm = MockTestLLMProvider::new();
        mock_llm
            .expect_complete()
            .returning(|_| Ok(CanonicalMessage::new(Role::Assistant, "test".to_string())));
        let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
        let app_state = AppState::new(key_store, llm_provider, None);
        let app = create_router(app_state);

        // Test without auth header
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/chat/completions")
                    .method("POST")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"messages":[]}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_chat_completion_with_valid_auth() {
        let key_store = Arc::new(ApiKeyStore::new());
        let key = "sk-1234567890123456".to_string();
        let key_id = ApiKeyId::new("test-key".to_string());

        key_store
            .add_key(key.clone(), key_id, AuthLevel::Write)
            .await;

        let mut mock_llm = MockTestLLMProvider::new();
        mock_llm.expect_complete().returning(|_| {
            Ok(CanonicalMessage::new(
                Role::Assistant,
                "test response".to_string(),
            ))
        });
        let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
        let app_state = AppState::new(key_store, llm_provider, None);
        let app = create_router(app_state);

        // Test with valid auth header and valid messages
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/chat/completions")
                    .method("POST")
                    .header(header::AUTHORIZATION, format!("Bearer {}", key))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"messages":[{"id":"550e8400-e29b-41d4-a716-446655440000","role":"user","content":"Hello","timestamp":"2024-01-01T00:00:00Z"}]}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_chat_completion_requires_write_access() {
        let key_store = Arc::new(ApiKeyStore::new());
        let key = "sk-1234567890123456".to_string();
        let key_id = ApiKeyId::new("test-key".to_string());

        // Add key with read-only access
        key_store
            .add_key(key.clone(), key_id, AuthLevel::Read)
            .await;

        let mut mock_llm = MockTestLLMProvider::new();
        let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
        let app_state = AppState::new(key_store, llm_provider, None);
        let app = create_router(app_state);

        // Test with read-only key
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/chat/completions")
                    .method("POST")
                    .header(header::AUTHORIZATION, format!("Bearer {}", key))
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"messages":[]}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_agent_status_requires_auth() {
        let key_store = Arc::new(ApiKeyStore::new());
        let key = "sk-1234567890123456".to_string();
        let key_id = ApiKeyId::new("test-key".to_string());

        key_store
            .add_key(key.clone(), key_id, AuthLevel::Read)
            .await;

        let mut mock_llm = MockTestLLMProvider::new();
        let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
        let app_state = AppState::new(key_store, llm_provider, None);
        let app = create_router(app_state);

        // Test without auth header
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/agents/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_agent_status_with_valid_auth() {
        let key_store = Arc::new(ApiKeyStore::new());
        let key = "sk-1234567890123456".to_string();
        let key_id = ApiKeyId::new("test-key".to_string());

        key_store
            .add_key(key.clone(), key_id, AuthLevel::Read)
            .await;

        let mut mock_llm = MockTestLLMProvider::new();
        let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
        let supervisor = Arc::new(RwLock::new(Supervisor::new()));
        let app_state = AppState::new(key_store, llm_provider, Some(supervisor));
        let app = create_router(app_state);

        // Test with valid auth header
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/agents/status")
                    .header(header::AUTHORIZATION, format!("Bearer {}", key))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
