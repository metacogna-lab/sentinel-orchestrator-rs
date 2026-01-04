// Actor event loop implementation for The Sentinel (orchestrator)
// Manages state transitions, message processing, and coordination

use crate::core::types::{AgentId, AgentState, CanonicalMessage, Role};
use crate::engine::channels::{create_actor_channel, ActorMessage, DEFAULT_CHANNEL_SIZE};
use anyhow::{Context, Result};
use tokio::sync::mpsc;
use tokio::sync::watch;
use tracing::{debug, error, info};

/// Actor structure for The Sentinel orchestrator
pub struct Actor {
    /// Unique identifier for this actor
    pub id: AgentId,
    /// Current state of the actor
    pub state: AgentState,
    /// Receiver channel for incoming messages
    rx: mpsc::Receiver<ActorMessage>,
    /// Shutdown signal receiver
    shutdown_rx: watch::Receiver<()>,
}

impl Actor {
    /// Create a new actor with the given receiver and shutdown signal
    ///
    /// # Arguments
    /// * `id` - Unique agent identifier
    /// * `rx` - Message receiver channel
    /// * `shutdown_rx` - Shutdown signal receiver
    pub fn new(
        id: AgentId,
        rx: mpsc::Receiver<ActorMessage>,
        shutdown_rx: watch::Receiver<()>,
    ) -> Self {
        Self {
            id,
            state: AgentState::Idle,
            rx,
            shutdown_rx,
        }
    }

    /// Run the actor event loop
    ///
    /// This is the main event loop that processes messages and manages state transitions.
    /// The loop continues until the channel is closed or a shutdown signal is received.
    ///
    /// # Returns
    /// * `Ok(())` - Graceful shutdown
    /// * `Err(anyhow::Error)` - Error during processing
    pub async fn run(&mut self) -> Result<()> {
        info!("Actor {} started in state {:?}", self.id, self.state);

        loop {
            tokio::select! {
                // Handle incoming messages
                msg = self.rx.recv() => {
                    match msg {
                        Some(actor_msg) => {
                            debug!("Actor {} received message", self.id);
                            match self.process_message(actor_msg).await {
                                Ok(new_state) => {
                                    self.state = new_state;
                                    debug!("Actor {} transitioned to state {:?}", self.id, self.state);
                                }
                                Err(e) => {
                                    error!("Actor {} error processing message: {}", self.id, e);
                                    // Continue processing despite errors
                                }
                            }
                        }
                        None => {
                            info!("Actor {} channel closed, shutting down", self.id);
                            break;
                        }
                    }
                }
                // Handle shutdown signal
                _ = self.shutdown_rx.changed() => {
                    info!("Actor {} received shutdown signal", self.id);
                    break;
                }
            }
        }

        info!("Actor {} stopped", self.id);
        Ok(())
    }

    /// Process a single message and determine the next state
    ///
    /// # Arguments
    /// * `_msg` - The actor message to process
    ///
    /// # Returns
    /// * `Ok(AgentState)` - The new state after processing
    /// * `Err(anyhow::Error)` - Error during processing
    async fn process_message(&self, _msg: ActorMessage) -> Result<AgentState> {
        let current_state = self.state;
        let next_state = match current_state {
            AgentState::Idle => {
                // When idle and receiving a message, transition to Thinking
                debug!("Actor {} processing message in Idle state", self.id);
                AgentState::Thinking
            }
            AgentState::Thinking => {
                // After thinking, transition to Reflecting
                // Future: Could transition to ToolCall if tool needed
                debug!(
                    "Actor {} finished thinking, transitioning to Reflecting",
                    self.id
                );
                AgentState::Reflecting
            }
            AgentState::ToolCall => {
                // After tool call, transition to Reflecting
                debug!(
                    "Actor {} finished tool call, transitioning to Reflecting",
                    self.id
                );
                AgentState::Reflecting
            }
            AgentState::Reflecting => {
                // After reflecting, transition back to Idle
                debug!(
                    "Actor {} finished reflecting, transitioning to Idle",
                    self.id
                );
                AgentState::Idle
            }
        };

        // Validate the state transition
        current_state
            .transition_to(next_state)
            .map_err(|e| anyhow::anyhow!("State transition error: {}", e))
            .context("Failed to transition state")?;

        Ok(next_state)
    }

    /// Get the current state of the actor
    pub fn current_state(&self) -> AgentState {
        self.state
    }

    /// Get the actor ID
    pub fn id(&self) -> AgentId {
        self.id
    }
}

/// Spawn a new actor with a bounded channel
///
/// # Arguments
/// * `buffer_size` - Size of the message channel buffer
///
/// # Returns
/// Tuple of (sender, shutdown_tx, join_handle)
/// * `sender` - Channel sender for sending messages to the actor
/// * `shutdown_tx` - Shutdown signal sender
/// * `join_handle` - Task join handle for awaiting completion
pub fn spawn_actor(
    buffer_size: usize,
) -> (
    mpsc::Sender<ActorMessage>,
    watch::Sender<()>,
    tokio::task::JoinHandle<Result<()>>,
) {
    let agent_id = AgentId::new();
    let (tx, rx) = create_actor_channel(buffer_size);
    let (shutdown_tx, shutdown_rx) = watch::channel(());

    let mut actor = Actor::new(agent_id, rx, shutdown_rx);

    let handle = tokio::spawn(async move { actor.run().await });

    (tx, shutdown_tx, handle)
}

/// Spawn a new actor with default channel size
pub fn spawn_default_actor() -> (
    mpsc::Sender<ActorMessage>,
    watch::Sender<()>,
    tokio::task::JoinHandle<Result<()>>,
) {
    spawn_actor(DEFAULT_CHANNEL_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::channels::ActorMessage;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_actor_spawns_and_receives_messages() {
        let (tx, _shutdown_tx, handle) = spawn_actor(10);

        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));
        tx.send(msg).await.unwrap();

        // Give actor time to process
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Close channel to trigger shutdown
        drop(tx);
        drop(_shutdown_tx);

        // Wait for actor to finish
        let result = timeout(Duration::from_secs(1), handle).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_actor_state_transitions() {
        let (tx, _shutdown_tx, handle) = spawn_actor(10);

        // Send a message to trigger state transition from Idle to Thinking
        let msg1 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg1".to_string()));
        tx.send(msg1).await.unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Send another message to trigger Thinking to Reflecting
        let msg2 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg2".to_string()));
        tx.send(msg2).await.unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Send another message to trigger Reflecting to Idle
        let msg3 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg3".to_string()));
        tx.send(msg3).await.unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Close channel
        drop(tx);
        drop(_shutdown_tx);

        let result = timeout(Duration::from_secs(1), handle).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_actor_channel_closure_graceful_shutdown() {
        let (tx, _shutdown_tx, handle) = spawn_actor(10);

        // Send a message
        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));
        tx.send(msg).await.unwrap();

        // Close channel (drop sender)
        drop(tx);
        drop(_shutdown_tx);

        // Actor should shutdown gracefully
        let result = timeout(Duration::from_secs(1), handle).await;
        assert!(result.is_ok());
        let join_result = result.unwrap();
        assert!(join_result.is_ok());
    }

    #[tokio::test]
    async fn test_actor_shutdown_signal() {
        let (tx, shutdown_tx, handle) = spawn_actor(10);

        // Send shutdown signal
        shutdown_tx.send(()).unwrap();

        // Wait for actor to finish
        let result = timeout(Duration::from_secs(1), handle).await;
        assert!(result.is_ok());

        // Try to send a message (should fail or be ignored)
        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));
        // Channel might still accept, but actor is shutting down
        let _ = tx.send(msg).await;
    }

    #[tokio::test]
    async fn test_actor_multiple_messages_processed() {
        let (tx, _shutdown_tx, handle) = spawn_actor(10);

        // Send multiple messages
        for i in 0..5 {
            let msg =
                ActorMessage::new(CanonicalMessage::new(Role::User, format!("message-{}", i)));
            tx.send(msg).await.unwrap();
        }

        // Give time for processing
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Close channel
        drop(tx);
        drop(_shutdown_tx);

        let result = timeout(Duration::from_secs(1), handle).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_actor_backpressure_handling() {
        let (tx, _shutdown_tx, handle) = spawn_actor(2);

        // Fill channel to capacity
        let msg1 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg1".to_string()));
        let msg2 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg2".to_string()));
        let msg3 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg3".to_string()));

        tx.send(msg1).await.unwrap();
        tx.send(msg2).await.unwrap();

        // Next send should block until space is available
        let send_future = tx.send(msg3);
        tokio::select! {
            _ = send_future => {
                // Should eventually complete as actor processes messages
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                // If this times out, backpressure is working (actor processing slowly)
            }
        }

        drop(tx);
        drop(_shutdown_tx);

        let _ = timeout(Duration::from_secs(1), handle).await;
    }

    #[tokio::test]
    async fn test_actor_with_sender_info() {
        let (tx, _shutdown_tx, handle) = spawn_actor(10);

        let sender_id = AgentId::new();
        let msg = ActorMessage::with_sender(
            CanonicalMessage::new(Role::User, "test".to_string()),
            sender_id,
        );
        tx.send(msg).await.unwrap();

        tokio::time::sleep(Duration::from_millis(10)).await;

        drop(tx);
        drop(_shutdown_tx);

        let _ = timeout(Duration::from_secs(1), handle).await;
    }

    #[tokio::test]
    async fn test_spawn_default_actor() {
        let (tx, _shutdown_tx, handle) = spawn_default_actor();

        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));
        tx.send(msg).await.unwrap();

        drop(tx);
        drop(_shutdown_tx);

        let result = timeout(Duration::from_secs(1), handle).await;
        assert!(result.is_ok());
    }
}
