// Axum route handlers with authentication

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tracing::info;

use crate::api::middleware::{create_auth_middleware, ApiKeyStore, AuthInfo};
use crate::core::auth::AuthLevel;
use crate::core::types::{
    AgentStatus, ChatCompletionRequest, ChatCompletionResponse, ErrorResponse, HealthState,
    HealthStatus,
};

/// Health check endpoint (no authentication required)
pub async fn health_check() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: HealthState::Healthy,
        timestamp: chrono::Utc::now(),
    })
}

/// Chat completion endpoint (requires write access)
pub async fn chat_completion(
    State(_key_store): State<Arc<ApiKeyStore>>,
    auth_info: Option<axum::extract::Extension<AuthInfo>>,
    Json(_request): Json<ChatCompletionRequest>,
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

    info!("Chat completion request received");

    // TODO: Implement actual chat completion logic
    // For now, return a placeholder response
    Ok(Json(ChatCompletionResponse {
        message: crate::core::types::CanonicalMessage::new(
            crate::core::types::Role::Assistant,
            "This is a placeholder response. Chat completion not yet implemented.".to_string(),
        ),
        model: "sentinel-orchestrator".to_string(),
        usage: None,
    }))
}

/// Agent status endpoint (requires read access)
pub async fn agent_status(
    State(_key_store): State<Arc<ApiKeyStore>>,
    auth_info: Option<axum::extract::Extension<AuthInfo>>,
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

    // TODO: Implement actual agent status logic
    // For now, return empty list
    Ok(Json(vec![]))
}

/// Create the API router with authentication middleware
pub fn create_router(key_store: Arc<ApiKeyStore>) -> Router {
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
        .with_state(key_store)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::auth::{ApiKeyId, AuthLevel};
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check_no_auth() {
        let key_store = Arc::new(ApiKeyStore::new());
        let app = create_router(key_store);

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

        let app = create_router(key_store);

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

        let app = create_router(key_store);

        // Test with valid auth header
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

        let app = create_router(key_store);

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

        let app = create_router(key_store);

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

        let app = create_router(key_store);

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
