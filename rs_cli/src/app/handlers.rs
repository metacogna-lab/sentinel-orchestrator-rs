// Event handlers for different modes

use crate::app::AppState;
use crate::types::*;
use anyhow::Result;
use futures::StreamExt;

/// Handle sending a chat message and streaming the response
pub async fn handle_chat_message(state: &mut AppState, message: String) -> Result<()> {
    if message.trim().is_empty() {
        return Ok(());
    }

    // Create user message
    let user_msg = CanonicalMessage::new(Role::User, message.clone());
    state.add_message(user_msg);

    // Create request
    let request = ChatCompletionRequest {
        messages: state.messages.clone(),
        model: None,
        temperature: None,
        max_tokens: None,
        stream: true, // Use streaming
    };

    // Stream the response
    let mut stream = state
        .api_client
        .stream_chat_completion(request)
        .await
        .map_err(|e| {
            state.set_error(format!("Failed to start streaming: {}", e));
            e
        })?;

    // Create assistant message that we'll build up
    let mut assistant_content = String::new();
    let assistant_id = MessageId::new();

    // Collect stream chunks
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                assistant_content.push_str(&chunk);
            }
            Err(e) => {
                state.set_error(format!("Stream error: {}", e));
                break;
            }
        }
    }

    // Create final assistant message
    if !assistant_content.trim().is_empty() {
        let assistant_msg = CanonicalMessage {
            id: assistant_id,
            role: Role::Assistant,
            content: assistant_content,
            timestamp: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };
        state.add_message(assistant_msg);
    }

    Ok(())
}

/// Handle investigation query
pub async fn handle_investigation(state: &mut AppState, query: String) -> Result<()> {
    if query.trim().is_empty() {
        return Ok(());
    }

    // For now, just add a placeholder result
    // In the future, this could query memory, search logs, etc.
    state.investigation_results.push(format!(
        "Investigation query: '{}' - Results would appear here",
        query
    ));

    Ok(())
}

/// Add a debug log entry
pub fn add_debug_log(state: &mut AppState, level: &str, message: String) {
    let timestamp = chrono::Utc::now().format("%H:%M:%S").to_string();
    let log_entry = format!("[{}] {}: {}", timestamp, level, message);
    state.debug_logs.push(log_entry);

    // Keep only last 100 logs
    if state.debug_logs.len() > 100 {
        state.debug_logs.remove(0);
    }
}

