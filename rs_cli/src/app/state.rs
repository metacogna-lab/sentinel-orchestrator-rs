// Application state management

use crate::api::ApiClient;
use crate::modes::Mode;
use crate::types::*;
use anyhow::Result;
use std::sync::Arc;

/// Application state
pub struct AppState {
    /// API client for backend communication
    pub api_client: Arc<ApiClient>,
    /// Current mode
    pub mode: Mode,
    /// Menu selection index (for MainMenu mode)
    pub menu_selection: usize,
    /// Conversation history
    pub messages: Vec<CanonicalMessage>,
    /// Current input buffer (for chat/investigation)
    pub input: String,
    /// Investigation results
    pub investigation_results: Vec<String>,
    /// Debug logs
    pub debug_logs: Vec<String>,
    /// System health status
    pub health: Option<HealthStatus>,
    /// Error message to display
    pub error: Option<String>,
    /// Whether the app should exit
    pub should_exit: bool,
}

impl AppState {
    /// Create new application state
    pub fn new(api_client: Arc<ApiClient>) -> Self {
        Self {
            api_client,
            mode: Mode::MainMenu,
            menu_selection: 0,
            messages: Vec::new(),
            input: String::new(),
            investigation_results: Vec::new(),
            debug_logs: Vec::new(),
            health: None,
            error: None,
            should_exit: false,
        }
    }

    /// Add a message to the conversation
    pub fn add_message(&mut self, message: CanonicalMessage) {
        self.messages.push(message);
    }

    /// Clear error
    pub fn clear_error(&mut self) {
        self.error = None;
    }

    /// Set error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    /// Update health status
    pub async fn update_health(&mut self) -> Result<()> {
        match self.api_client.health().await {
            Ok(status) => {
                self.health = Some(status);
                Ok(())
            }
            Err(e) => {
                self.set_error(format!("Failed to fetch health: {}", e));
                Err(e)
            }
        }
    }
}

