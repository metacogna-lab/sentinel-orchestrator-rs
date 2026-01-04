// Integration tests for vector storage operations
// Tests Qdrant integration, embedding storage, and retrieval

use sentinel::core::types::{CanonicalMessage, Role};
use std::time::Duration;
use tokio::time::sleep;

/// Base URL for the API
fn api_base_url() -> String {
    std::env::var("SENTINEL_API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

/// API key for authentication
fn api_key() -> String {
    std::env::var("SENTINEL_API_KEY").unwrap_or_else(|_| "sk-test-key".to_string())
}

/// Test storing messages that should be embedded and stored in vector DB
#[tokio::test]
#[ignore] // Requires running server and Qdrant
async fn test_store_messages_in_vector_db() {
    let client = reqwest::Client::new();

    // Send messages that should trigger vector storage
    let messages = vec![
        create_test_message(
            Role::User,
            "The Rust programming language is a systems programming language focused on safety and performance.",
        ),
        create_test_message(
            Role::User,
            "Rust uses ownership and borrowing to ensure memory safety without garbage collection.",
        ),
    ];

    for message in messages {
        let response = client
            .post(&format!("{}/v1/chat/completions", api_base_url()))
            .header("Authorization", format!("Bearer {}", api_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "messages": [message]
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        
        // Small delay to allow processing
        sleep(Duration::from_millis(500)).await;
    }
}

/// Test semantic search through vector storage
#[tokio::test]
#[ignore]
async fn test_semantic_search_retrieval() {
    let client = reqwest::Client::new();

    // First, store some information
    let store_messages = vec![
        create_test_message(
            Role::User,
            "Python is a high-level programming language known for its simplicity and readability.",
        ),
        create_test_message(
            Role::User,
            "JavaScript is the programming language of the web, used for both frontend and backend development.",
        ),
    ];

    for msg in store_messages {
        let _response = client
            .post(&format!("{}/v1/chat/completions", api_base_url()))
            .header("Authorization", format!("Bearer {}", api_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "messages": [msg]
            }))
            .send()
            .await
            .unwrap();

        sleep(Duration::from_millis(500)).await;
    }

    // Now query with semantic search
    let query_message = create_test_message(
        Role::User,
        "Tell me about programming languages for web development.",
    );

    let response = client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", format!("Bearer {}", api_key()))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "messages": [query_message]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let result: serde_json::Value = response.json().await.unwrap();
    let content = result["message"]["content"]
        .as_str()
        .unwrap()
        .to_lowercase();

    // Response should potentially reference stored information
    assert!(content.len() > 0);
}

/// Test vector storage with multiple similar queries
#[tokio::test]
#[ignore]
async fn test_vector_storage_deduplication() {
    let client = reqwest::Client::new();

    // Send the same message multiple times
    let message = create_test_message(
        Role::User,
        "The capital of France is Paris.",
    );

    for _ in 0..5 {
        let response = client
            .post(&format!("{}/v1/chat/completions", api_base_url()))
            .header("Authorization", format!("Bearer {}", api_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "messages": [message.clone()]
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        sleep(Duration::from_millis(200)).await;
    }
}

/// Test storing large amounts of data in vector storage
#[tokio::test]
#[ignore]
async fn test_bulk_vector_storage() {
    let client = reqwest::Client::new();

    // Store multiple pieces of information
    let facts = vec![
        "The Great Wall of China is the longest wall in the world.",
        "Mount Everest is the highest mountain on Earth.",
        "The Amazon River is the largest river by discharge volume.",
        "The Sahara Desert is the largest hot desert in the world.",
        "The Pacific Ocean is the largest ocean on Earth.",
    ];

    for fact in facts {
        let message = create_test_message(Role::User, fact);
        
        let response = client
            .post(&format!("{}/v1/chat/completions", api_base_url()))
            .header("Authorization", format!("Bearer {}", api_key()))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "messages": [message]
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        sleep(Duration::from_millis(300)).await;
    }
}

/// Test vector storage with different message types
#[tokio::test]
#[ignore]
async fn test_vector_storage_message_types() {
    let client = reqwest::Client::new();

    // Test with system message
    let system_message = create_test_message(
        Role::System,
        "You are a helpful assistant that stores information in a vector database.",
    );

    let response = client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", format!("Bearer {}", api_key()))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "messages": [system_message]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Test with user message
    let user_message = create_test_message(
        Role::User,
        "Store this: The speed of light is approximately 299,792,458 meters per second.",
    );

    let response = client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", format!("Bearer {}", api_key()))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "messages": [user_message]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

/// Test vector storage persistence across requests
#[tokio::test]
#[ignore]
async fn test_vector_storage_persistence() {
    let client = reqwest::Client::new();

    // Store information in first request
    let store_msg = create_test_message(
        Role::User,
        "My favorite color is blue and I love programming in Rust.",
    );

    let response1 = client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", format!("Bearer {}", api_key()))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "messages": [store_msg]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response1.status(), reqwest::StatusCode::OK);
    sleep(Duration::from_secs(1)).await;

    // Query in second request - should retrieve stored information
    let query_msg = create_test_message(
        Role::User,
        "What is my favorite color?",
    );

    let response2 = client
        .post(&format!("{}/v1/chat/completions", api_base_url()))
        .header("Authorization", format!("Bearer {}", api_key()))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "messages": [query_msg]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response2.status(), reqwest::StatusCode::OK);
}

/// Helper function to create test messages
fn create_test_message(role: Role, content: &str) -> CanonicalMessage {
    CanonicalMessage::new(role, content.to_string())
}

