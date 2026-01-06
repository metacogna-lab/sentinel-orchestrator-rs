//! Performance benchmarks for API response times
//!
//! Measures performance of API endpoints including:
//! - Health check endpoint
//! - Chat completion endpoint (with mock LLM)
//! - Agent status endpoint
//! - Authentication middleware overhead

use async_trait::async_trait;
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mockall::mock;
use sentinel::api::middleware::ApiKeyStore;
use sentinel::api::routes::{create_router, AppState};
use sentinel::core::auth::{ApiKeyId, AuthLevel};
use sentinel::core::error::SentinelError;
use sentinel::core::traits::LLMProvider;
use sentinel::core::types::{CanonicalMessage, Role};
use std::sync::Arc;
use tokio::runtime::Runtime;
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

/// Create a test router with mock LLM provider
fn create_test_router() -> axum::Router {
    let key_store = Arc::new(ApiKeyStore::new());
    let mut mock_llm = MockTestLLMProvider::new();
    mock_llm.expect_complete().returning(|_| {
        Ok(CanonicalMessage::new(
            Role::Assistant,
            "test response".to_string(),
        ))
    });
    let llm_provider: Arc<dyn LLMProvider> = Arc::new(mock_llm);
    let app_state = AppState::new(key_store, llm_provider, None);
    create_router(app_state)
}

/// Benchmark health check endpoint
fn bench_health_check(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let router = create_test_router();

    c.bench_function("health_check_endpoint", |b| {
        b.to_async(&rt).iter(|| async {
            let request = Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap();

            let response = router.clone().oneshot(request).await.unwrap();

            assert_eq!(response.status(), StatusCode::OK);
            black_box(response)
        });
    });
}

/// Benchmark chat completion endpoint with authentication
fn bench_chat_completion(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let router = create_test_router();

    // Add API key to store
    let key_store = Arc::new(ApiKeyStore::new());
    let api_key = "sk-test123456789012345678901234567890";
    rt.block_on(async {
        key_store
            .add_key(
                api_key.to_string(),
                ApiKeyId::new("test-key".to_string()),
                AuthLevel::Write,
            )
            .await;
    });

    let request_body = serde_json::json!({
        "messages": [
            {
                "role": "user",
                "content": "Hello, how are you?"
            }
        ]
    });

    c.bench_function("chat_completion_endpoint", |b| {
        b.to_async(&rt).iter(|| async {
            let request = Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
                .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                .unwrap();

            let response = router.clone().oneshot(request).await.unwrap();

            assert_eq!(response.status(), StatusCode::OK);
            black_box(response)
        });
    });
}

/// Benchmark authentication middleware overhead
fn bench_auth_middleware(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let router = create_test_router();

    let api_key = "sk-test123456789012345678901234567890";
    rt.block_on(async {
        let key_store = Arc::new(ApiKeyStore::new());
        key_store
            .add_key(
                api_key.to_string(),
                ApiKeyId::new("test-key".to_string()),
                AuthLevel::Write,
            )
            .await;
    });

    c.bench_function("auth_middleware_overhead", |b| {
        b.to_async(&rt).iter(|| async {
            // Request with valid auth
            let request = Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"messages":[]}"#))
                .unwrap();

            let _response = router.clone().oneshot(request).await.unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_health_check,
    bench_chat_completion,
    bench_auth_middleware
);
criterion_main!(benches);
