// Integration tests for Sentinel Orchestrator API
// These tests verify the full HTTP stack including authentication, routing, and responses

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use sentinel::api::middleware::ApiKeyStore;
use sentinel::api::routes::{create_router, AppState};
use sentinel::core::auth::{ApiKeyId, AuthLevel};
use sentinel::core::error::SentinelError;
use sentinel::core::traits::LLMProvider;
use sentinel::core::types::{
    CanonicalMessage, ChatCompletionRequest, ChatCompletionResponse, HealthState,
    HealthStatus, Role,
};
use async_trait::async_trait;
use mockall::mock;
use std::sync::Arc;
use tower::ServiceExt;

// Mock LLM provider for testing (test-only, does NOT affect API contract)
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

/// Helper to create a test router with API key store and mock LLM provider
/// NOTE: This is test-only code and does NOT affect the API contract
fn create_test_router() -> (axum::Router, Arc<ApiKeyStore>) {
    let key_store = Arc::new(ApiKeyStore::new());
    let mut mock_llm = MockTestLLMProvider::new();
    mock_llm
        .expect_complete()
        .returning(|messages| {
            if messages.is_empty() {
                Ok(CanonicalMessage::new(
                    Role::Assistant,
                    "No messages provided".to_string(),
                ))
            } else {
                let last_message = messages.last().unwrap();
                Ok(CanonicalMessage::new(
                    Role::Assistant,
                    format!("Echo: {}", last_message.content),
                ))
            }
        });
    let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
    let app_state = AppState::new(key_store.clone(), llm_provider, None);
    let app = create_router(app_state);
    (app, key_store)
}

/// Helper to add a test API key
async fn add_test_key(
    key_store: &Arc<ApiKeyStore>,
    key: &str,
    key_id: &str,
    level: AuthLevel,
) {
    key_store
        .add_key(key.to_string(), ApiKeyId::new(key_id.to_string()), level)
        .await;
}

/// Helper to make a GET request
async fn make_get_request(router: &axum::Router, uri: &str) -> (StatusCode, Vec<u8>) {
    let response = router
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let body = http_body_util::BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    (status, body.to_vec())
}

/// Helper to make a POST request with JSON body
async fn make_post_request(
    router: &axum::Router,
    uri: &str,
    body: &str,
    auth_header: Option<&str>,
) -> (StatusCode, Vec<u8>) {
    let mut request_builder = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");

    if let Some(auth) = auth_header {
        request_builder = request_builder.header(header::AUTHORIZATION, auth);
    }

    let response = router
        .clone()
        .oneshot(
            request_builder
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body_bytes = http_body_util::BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    (status, body_bytes.to_vec())
}

#[tokio::test]
async fn test_health_check_public_access() {
    let (router, _) = create_test_router();

    let (status, body) = make_get_request(&router, "/health").await;

    assert_eq!(status, StatusCode::OK);
    let health: HealthStatus = serde_json::from_slice(&body).unwrap();
    assert_eq!(health.status, HealthState::Healthy);
}

#[tokio::test]
async fn test_health_check_no_auth_required() {
    let (router, _) = create_test_router();

    // Health check should work without any authentication
    let (status, body) = make_get_request(&router, "/health").await;

    assert_eq!(status, StatusCode::OK);
    let health: HealthStatus = serde_json::from_slice(&body).unwrap();
    assert_eq!(health.status, HealthState::Healthy);
}

#[tokio::test]
async fn test_chat_completion_requires_authentication() {
    let (router, _) = create_test_router();

    let request = ChatCompletionRequest {
        messages: vec![],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };

    // Request without authentication should fail
    let body_json = serde_json::to_string(&request).unwrap();
    let (status, body) = make_post_request(&router, "/v1/chat/completions", &body_json, None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    // Middleware returns error in nested format: {"error": {"code": "...", "message": "...", "type": "..."}}
    let error_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let error_code = error_json["error"]["code"].as_str().unwrap();
    assert!(error_code == "missing_authorization" || error_code == "invalid_api_key");
}

#[tokio::test]
async fn test_chat_completion_with_valid_auth() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    add_test_key(&key_store, api_key, "test-key", AuthLevel::Write).await;

    let request = ChatCompletionRequest {
        messages: vec![CanonicalMessage::new(
            Role::User,
            "Hello, how are you?".to_string(),
        )],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };

    let body_json = serde_json::to_string(&request).unwrap();
    let auth_header = format!("Bearer {}", api_key);
    let (status, body) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;

    assert_eq!(status, StatusCode::OK);
    let completion: ChatCompletionResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(completion.message.role, Role::Assistant);
    assert!(!completion.message.content.is_empty());
    assert_eq!(completion.model, "sentinel-orchestrator");
}

#[tokio::test]
async fn test_chat_completion_requires_write_access() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    // Add key with read-only access
    add_test_key(&key_store, api_key, "read-key", AuthLevel::Read).await;

    let request = ChatCompletionRequest {
        messages: vec![],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };

    let body_json = serde_json::to_string(&request).unwrap();
    let auth_header = format!("Bearer {}", api_key);
    let (status, body) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    // Middleware returns error in nested format
    let error_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let error_code = error_json["error"]["code"].as_str().unwrap();
    assert_eq!(error_code, "insufficient_permissions");
}

#[tokio::test]
async fn test_chat_completion_with_admin_access() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    // Admin should have write access
    add_test_key(&key_store, api_key, "admin-key", AuthLevel::Admin).await;

    let request = ChatCompletionRequest {
        messages: vec![CanonicalMessage::new(
            Role::User,
            "Test message".to_string(),
        )],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };

    let body_json = serde_json::to_string(&request).unwrap();
    let auth_header = format!("Bearer {}", api_key);
    let (status, _) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_chat_completion_invalid_api_key() {
    let (router, _) = create_test_router();

    let request = ChatCompletionRequest {
        messages: vec![],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };

    // Use a key that doesn't exist
    let body_json = serde_json::to_string(&request).unwrap();
    let (status, body) = make_post_request(&router, "/v1/chat/completions", &body_json, Some("Bearer sk-invalid-key-1234567890123456")).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    // Middleware returns error in nested format: {"error": {"code": "...", "message": "...", "type": "..."}}
    let error_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let error_code = error_json["error"]["code"].as_str().unwrap();
    assert!(error_code == "missing_authorization" || error_code == "invalid_api_key");
}

#[tokio::test]
async fn test_chat_completion_bearer_token_format() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    add_test_key(&key_store, api_key, "test-key", AuthLevel::Write).await;

    let request = ChatCompletionRequest {
        messages: vec![],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };

    // Test with "Bearer " prefix
    let body_json = serde_json::to_string(&request).unwrap();
    let auth_header = format!("Bearer {}", api_key);
    let (status, _) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_agent_status_requires_authentication() {
    let (router, _) = create_test_router();

    // Request without authentication should fail
    let (status, body) = make_get_request(&router, "/v1/agents/status").await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    // Middleware returns error in nested format: {"error": {"code": "...", "message": "...", "type": "..."}}
    let error_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let error_code = error_json["error"]["code"].as_str().unwrap();
    assert!(error_code == "missing_authorization" || error_code == "invalid_api_key");
}

#[tokio::test]
async fn test_agent_status_with_read_access() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    add_test_key(&key_store, api_key, "read-key", AuthLevel::Read).await;

    // Make request with auth header
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/agents/status")
                .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = http_body_util::BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    let statuses: Vec<sentinel::core::types::AgentStatus> = serde_json::from_slice(&body).unwrap();
    // Currently returns empty list, but should be valid JSON
    assert!(statuses.is_empty());
}

#[tokio::test]
async fn test_agent_status_with_write_access() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    // Write access should also work for read endpoints
    add_test_key(&key_store, api_key, "write-key", AuthLevel::Write).await;

    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/agents/status")
                .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_agent_status_with_admin_access() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    add_test_key(&key_store, api_key, "admin-key", AuthLevel::Admin).await;

    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/agents/status")
                .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_chat_completion_request_validation() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    add_test_key(&key_store, api_key, "test-key", AuthLevel::Write).await;

    // Test with valid request containing messages
    let request = ChatCompletionRequest {
        messages: vec![
            CanonicalMessage::new(Role::System, "You are a helpful assistant.".to_string()),
            CanonicalMessage::new(Role::User, "What is 2+2?".to_string()),
        ],
        model: Some("gpt-4".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(100),
        stream: false,
    };

    let body_json = serde_json::to_string(&request).unwrap();
    let auth_header = format!("Bearer {}", api_key);
    let (status, body) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;

    assert_eq!(status, StatusCode::OK);
    let completion: ChatCompletionResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(completion.message.role, Role::Assistant);
}

#[tokio::test]
async fn test_multiple_api_keys() {
    let (router, key_store) = create_test_router();
    let read_key = "sk-read123456789012345678901234567890";
    let write_key = "sk-write123456789012345678901234567890";

    add_test_key(&key_store, read_key, "read-key", AuthLevel::Read).await;
    add_test_key(&key_store, write_key, "write-key", AuthLevel::Write).await;

    // Read key should work for agent status
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/agents/status")
                .header(header::AUTHORIZATION, format!("Bearer {}", read_key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Read key should NOT work for chat completion
    let request = ChatCompletionRequest {
        messages: vec![],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };
    let body_json = serde_json::to_string(&request).unwrap();
    let auth_header = format!("Bearer {}", read_key);
    let (status, _) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Write key should work for both
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/agents/status")
                .header(header::AUTHORIZATION, format!("Bearer {}", write_key))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let auth_header = format!("Bearer {}", write_key);
    let (status, _) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_error_response_format() {
    let (router, _) = create_test_router();

    // Test unauthorized error format
    let (status, body) = make_get_request(&router, "/v1/agents/status").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Middleware returns error in nested format
    let error_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let error_code = error_json["error"]["code"].as_str().unwrap();
    let error_message = error_json["error"]["message"].as_str().unwrap();
    assert!(!error_code.is_empty());
    assert!(!error_message.is_empty());
}

#[tokio::test]
async fn test_chat_completion_response_structure() {
    let (router, key_store) = create_test_router();
    let api_key = "sk-test123456789012345678901234567890";
    add_test_key(&key_store, api_key, "test-key", AuthLevel::Write).await;

    let request = ChatCompletionRequest {
        messages: vec![CanonicalMessage::new(
            Role::User,
            "Hello".to_string(),
        )],
        model: None,
        temperature: None,
        max_tokens: None,
        stream: false,
    };

    let body_json = serde_json::to_string(&request).unwrap();
    let auth_header = format!("Bearer {}", api_key);
    let (status, body) = make_post_request(&router, "/v1/chat/completions", &body_json, Some(&auth_header)).await;

    assert_eq!(status, StatusCode::OK);
    let completion: ChatCompletionResponse = serde_json::from_slice(&body).unwrap();

    // Verify response structure
    assert_eq!(completion.message.role, Role::Assistant);
    assert!(!completion.message.content.is_empty());
    assert!(!completion.model.is_empty());
    // Usage is optional and may be None
}

