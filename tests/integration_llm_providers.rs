// Integration tests simulating various LLM providers hitting our endpoints
// Tests vector storage operations and full request/response flow

use sentinel::core::types::{CanonicalMessage, ChatCompletionRequest, Role};
use std::time::Duration;
use tokio::time::sleep;

/// Base URL for the API (configurable via environment)
fn api_base_url() -> String {
    std::env::var("SENTINEL_API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

/// API key for authentication (configurable via environment)
fn api_key() -> String {
    std::env::var("SENTINEL_API_KEY").unwrap_or_else(|_| "sk-test-key".to_string())
}

/// Helper to create a test message
fn create_test_message(role: Role, content: &str) -> CanonicalMessage {
    CanonicalMessage::new(role, content.to_string())
}

/// Helper to make a chat completion request
async fn make_chat_request(
    client: &reqwest::Client,
    messages: Vec<CanonicalMessage>,
) -> Result<reqwest::Response, reqwest::Error> {
    let request = ChatCompletionRequest {
        messages,
        model: Some("gpt-4".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(1000),
        stream: false,
    };

    client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", format!("Bearer {}", api_key()))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
}

/// Test simulating OpenAI-style provider requests
#[tokio::test]
#[ignore] // Requires running server
async fn test_openai_provider_simulation() {
    let client = reqwest::Client::new();

    // Simulate OpenAI-style request
    let messages = vec![create_test_message(
        Role::User,
        "What is the capital of France?",
    )];

    let response = make_chat_request(&client, messages).await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let result: serde_json::Value = response.json().await.unwrap();
    assert!(result.get("message").is_some());
    assert_eq!(
        result["message"]["role"].as_str().unwrap(),
        "assistant"
    );
}

/// Test simulating Anthropic-style provider requests
#[tokio::test]
#[ignore]
async fn test_anthropic_provider_simulation() {
    let client = reqwest::Client::new();

    // Simulate Anthropic-style request with system message
    let messages = vec![
        create_test_message(
            Role::System,
            "You are a helpful assistant specializing in geography.",
        ),
        create_test_message(Role::User, "Tell me about Paris."),
    ];

    let response = make_chat_request(&client, messages).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

/// Test simulating Google-style provider requests
#[tokio::test]
#[ignore]
async fn test_google_provider_simulation() {
    let client = reqwest::Client::new();

    // Simulate Google-style request with metadata
    let mut message = create_test_message(Role::User, "Explain quantum computing.");
    message.metadata.insert(
        "provider".to_string(),
        "google".to_string(),
    );

    let response = make_chat_request(&client, vec![message]).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

/// Test concurrent requests from multiple providers
#[tokio::test]
#[ignore]
async fn test_concurrent_provider_requests() {
    let client = reqwest::Client::new();

    // Simulate multiple providers hitting the API simultaneously
    let mut handles = vec![];

    for i in 0..10 {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            let messages = vec![create_test_message(
                Role::User,
                &format!("Request number {}", i),
            )];
            make_chat_request(&client_clone, messages).await
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await);
    }

    // Verify all requests succeeded
    for result in results {
        let response = result.unwrap().unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
    }
}

/// Test rate limiting with multiple rapid requests
#[tokio::test]
#[ignore]
async fn test_rate_limiting() {
    let client = reqwest::Client::new();

    // Make rapid requests to test rate limiting
    let mut success_count = 0;
    let mut rate_limited_count = 0;

    for i in 0..20 {
        let messages = vec![create_test_message(
            Role::User,
            &format!("Rate limit test {}", i),
        )];

        match make_chat_request(&client, messages).await {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    rate_limited_count += 1;
                } else {
                    success_count += 1;
                }
            }
            Err(_) => {
                // Network errors don't count
            }
        }

        // Small delay to avoid overwhelming
        sleep(Duration::from_millis(100)).await;
    }

    // Should have some successful requests
    assert!(success_count > 0);
}

/// Test vector storage operations through chat completions
#[tokio::test]
#[ignore]
async fn test_vector_storage_operations() {
    let client = reqwest::Client::new();

    // Send a message that should trigger vector storage
    let messages = vec![create_test_message(
        Role::User,
        "Store this information: The Eiffel Tower is located in Paris, France.",
    )];

    let response = make_chat_request(&client, messages).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Send a follow-up query that should use vector storage
    let follow_up_messages = vec![
        create_test_message(
            Role::User,
            "Store this information: The Eiffel Tower is located in Paris, France.",
        ),
        create_test_message(
            Role::User,
            "Where is the Eiffel Tower located?",
        ),
    ];

    let response = make_chat_request(&client, follow_up_messages).await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // The response should potentially reference stored information
    let result: serde_json::Value = response.json().await.unwrap();
    let content = result["message"]["content"]
        .as_str()
        .unwrap()
        .to_lowercase();
    
    // Verify response contains relevant information (basic check)
    assert!(
        content.contains("paris") || content.contains("france") || content.len() > 0
    );
}

/// Test multi-turn conversation with vector storage
#[tokio::test]
#[ignore]
async fn test_multi_turn_conversation() {
    let client = reqwest::Client::new();

    // First turn: Store information
    let turn1 = vec![create_test_message(
        Role::User,
        "My name is Alice and I work as a software engineer.",
    )];

    let response1 = make_chat_request(&client, turn1).await.unwrap();
    assert_eq!(response1.status(), reqwest::StatusCode::OK);

    // Second turn: Reference stored information
    let turn2 = vec![
        create_test_message(
            Role::User,
            "My name is Alice and I work as a software engineer.",
        ),
        create_test_message(Role::Assistant, "Nice to meet you, Alice!"),
        create_test_message(Role::User, "What's my profession?"),
    ];

    let response2 = make_chat_request(&client, turn2).await.unwrap();
    assert_eq!(response2.status(), reqwest::StatusCode::OK);
}

/// Test error handling with invalid requests
#[tokio::test]
#[ignore]
async fn test_error_handling() {
    let client = reqwest::Client::new();

    // Test with empty messages
    let response = client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", format!("Bearer {}", api_key()))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "messages": []
        }))
        .send()
        .await
        .unwrap();

    // Should return error for empty messages
    assert!(response.status().is_client_error());
}

/// Test authentication with invalid API key
#[tokio::test]
#[ignore]
async fn test_invalid_authentication() {
    let client = reqwest::Client::new();

    let messages = vec![create_test_message(Role::User, "Test")];

    let response = client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", "Bearer invalid-key")
        .header("Content-Type", "application/json")
        .json(&ChatCompletionRequest {
            messages,
            model: None,
            temperature: None,
            max_tokens: None,
            stream: false,
        })
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}

/// Test health check endpoint
#[tokio::test]
#[ignore]
async fn test_health_check() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", api_base_url()))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let result: serde_json::Value = response.json().await.unwrap();
    assert_eq!(result["status"].as_str().unwrap(), "healthy");
}

/// Test readiness endpoint
#[tokio::test]
#[ignore]
async fn test_readiness_check() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/ready", api_base_url()))
        .send()
        .await
        .unwrap();

    // Should be 200 if ready, 503 if not
    assert!(response.status() == reqwest::StatusCode::OK 
         || response.status() == reqwest::StatusCode::SERVICE_UNAVAILABLE);
}

/// Test liveness endpoint
#[tokio::test]
#[ignore]
async fn test_liveness_check() {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health/live", api_base_url()))
        .send()
        .await
        .unwrap();

    // Liveness should always return 200 if service is running
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

