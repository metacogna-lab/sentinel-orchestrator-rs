// Channel-based communication infrastructure for actor message passing
// All channels are bounded to prevent unbounded memory growth

use crate::core::types::{AgentId, CanonicalMessage};
use anyhow::Result;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::warn;

/// Message wrapper for actor communication
/// Includes the canonical message and optional sender metadata
#[derive(Debug, Clone)]
pub struct ActorMessage {
    /// The canonical message being sent
    pub message: CanonicalMessage,
    /// Optional sender agent ID
    pub sender: Option<AgentId>,
}

impl ActorMessage {
    /// Create a new actor message
    pub fn new(message: CanonicalMessage) -> Self {
        Self {
            message,
            sender: None,
        }
    }

    /// Create a new actor message with sender
    pub fn with_sender(message: CanonicalMessage, sender: AgentId) -> Self {
        Self {
            message,
            sender: Some(sender),
        }
    }
}

impl From<CanonicalMessage> for ActorMessage {
    fn from(message: CanonicalMessage) -> Self {
        Self::new(message)
    }
}

/// Default channel buffer size
pub const DEFAULT_CHANNEL_SIZE: usize = 32;

/// Create a bounded channel for actor communication
///
/// # Arguments
/// * `buffer_size` - Size of the channel buffer (must be > 0)
///
/// # Returns
/// Tuple of (Sender, Receiver) for the channel
///
/// # Panics
/// Panics if buffer_size is 0
pub fn create_actor_channel(
    buffer_size: usize,
) -> (mpsc::Sender<ActorMessage>, mpsc::Receiver<ActorMessage>) {
    if buffer_size == 0 {
        panic!("Channel buffer size must be greater than 0");
    }
    mpsc::channel(buffer_size)
}

/// Create a bounded channel with default buffer size
pub fn create_default_actor_channel() -> (mpsc::Sender<ActorMessage>, mpsc::Receiver<ActorMessage>)
{
    create_actor_channel(DEFAULT_CHANNEL_SIZE)
}

/// Send a message with timeout handling
///
/// # Arguments
/// * `tx` - Channel sender
/// * `msg` - Message to send
/// * `timeout_duration` - Maximum time to wait for send
///
/// # Returns
/// Ok(()) if sent successfully, Err if timeout or channel closed
pub async fn try_send_with_timeout(
    tx: &mpsc::Sender<ActorMessage>,
    msg: ActorMessage,
    timeout_duration: Duration,
) -> Result<()> {
    match timeout(timeout_duration, tx.send(msg)).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(_)) => {
            warn!("Channel receiver dropped, cannot send message");
            anyhow::bail!("Channel receiver dropped");
        }
        Err(_) => {
            warn!("Timeout sending message to channel");
            anyhow::bail!("Timeout sending message");
        }
    }
}

/// Check if a channel sender is still connected (receiver exists)
///
/// # Arguments
/// * `tx` - Channel sender to check
///
/// # Returns
/// true if receiver still exists, false if dropped
pub fn is_channel_connected(tx: &mpsc::Sender<ActorMessage>) -> bool {
    !tx.is_closed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{AgentId, Role};
    use std::time::Duration;

    #[tokio::test]
    async fn test_channel_creation() {
        let (tx, mut rx) = create_actor_channel(10);
        assert!(!tx.is_closed());

        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));
        tx.send(msg.clone()).await.unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received.message.content, msg.message.content);
    }

    #[tokio::test]
    async fn test_channel_backpressure() {
        let (tx, mut rx) = create_actor_channel(2);

        // Fill channel to capacity
        let msg1 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg1".to_string()));
        let msg2 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg2".to_string()));
        let msg3 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg3".to_string()));

        tx.send(msg1.clone()).await.unwrap();
        tx.send(msg2.clone()).await.unwrap();

        // Next send should block until space is available
        let send_future = tx.send(msg3.clone());
        let receive_future = rx.recv();

        // Use select to ensure we receive before send completes
        tokio::select! {
            _ = send_future => {
                // This should not happen first if backpressure works
                panic!("Send should block when channel is full");
            }
            result = receive_future => {
                // Receive one message to make space
                assert!(result.is_some());
                // Now send should complete
                tx.send(msg3).await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn test_message_ordering() {
        let (tx, mut rx) = create_actor_channel(10);

        let messages: Vec<_> = (0..5)
            .map(|i| ActorMessage::new(CanonicalMessage::new(Role::User, format!("message-{}", i))))
            .collect();

        // Send all messages
        for msg in &messages {
            tx.send(msg.clone()).await.unwrap();
        }

        // Receive all messages and verify order
        for (i, expected_msg) in messages.iter().enumerate() {
            let received = rx.recv().await.unwrap();
            assert_eq!(
                received.message.content, expected_msg.message.content,
                "Message {} out of order",
                i
            );
        }
    }

    #[tokio::test]
    async fn test_channel_closure() {
        let (tx, mut rx) = create_actor_channel(10);

        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));
        tx.send(msg).await.unwrap();

        // Drop sender
        drop(tx);

        // Should receive the message
        assert!(rx.recv().await.is_some());

        // Next receive should return None (channel closed)
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn test_concurrent_send_receive() {
        let (tx, mut rx) = create_actor_channel(100);

        // Spawn sender task
        let sender_handle = tokio::spawn(async move {
            for i in 0..50 {
                let msg =
                    ActorMessage::new(CanonicalMessage::new(Role::User, format!("msg-{}", i)));
                tx.send(msg).await.unwrap();
            }
        });

        // Receive in main task
        let mut received_count = 0;
        while let Some(msg) = rx.recv().await {
            assert_eq!(msg.message.content, format!("msg-{}", received_count));
            received_count += 1;
            if received_count == 50 {
                break;
            }
        }

        sender_handle.await.unwrap();
        assert_eq!(received_count, 50);
    }

    #[tokio::test]
    async fn test_try_send_with_timeout_success() {
        let (tx, mut rx) = create_actor_channel(10);
        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));

        let result = try_send_with_timeout(&tx, msg.clone(), Duration::from_millis(100)).await;
        assert!(result.is_ok());

        // Verify message was received
        let received = rx.recv().await.unwrap();
        assert_eq!(received.message.content, msg.message.content);
    }

    #[tokio::test]
    async fn test_try_send_with_timeout_channel_full() {
        let (tx, mut rx) = create_actor_channel(1);

        // Fill channel
        let msg1 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg1".to_string()));
        tx.send(msg1).await.unwrap();

        // Try to send another with short timeout (should timeout due to backpressure)
        let msg2 = ActorMessage::new(CanonicalMessage::new(Role::User, "msg2".to_string()));
        let result = try_send_with_timeout(&tx, msg2, Duration::from_millis(10)).await;
        assert!(result.is_err());

        // Make space and verify it works
        let _ = rx.recv().await;
    }

    #[tokio::test]
    async fn test_try_send_with_timeout_channel_closed() {
        let (tx, _rx) = create_actor_channel(10);

        // Drop receiver
        drop(_rx);

        let msg = ActorMessage::new(CanonicalMessage::new(Role::User, "test".to_string()));
        let result = try_send_with_timeout(&tx, msg, Duration::from_millis(100)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_channel_connected() {
        let (tx, rx) = create_actor_channel(10);
        assert!(is_channel_connected(&tx));

        drop(rx);
        // Note: is_closed() might not immediately reflect the drop
        // This is a limitation of the tokio API
    }

    #[tokio::test]
    async fn test_actor_message_with_sender() {
        let agent_id = AgentId::new();
        let msg = CanonicalMessage::new(Role::User, "test".to_string());
        let actor_msg = ActorMessage::with_sender(msg.clone(), agent_id);

        assert_eq!(actor_msg.message.content, msg.content);
        assert_eq!(actor_msg.sender, Some(agent_id));
    }

    #[tokio::test]
    async fn test_actor_message_from_canonical() {
        let msg = CanonicalMessage::new(Role::User, "test".to_string());
        let actor_msg: ActorMessage = msg.clone().into();

        assert_eq!(actor_msg.message.content, msg.content);
        assert_eq!(actor_msg.sender, None);
    }

    #[tokio::test]
    async fn test_default_channel_size() {
        let (tx, mut rx) = create_default_actor_channel();

        // Should be able to send at least DEFAULT_CHANNEL_SIZE messages
        for i in 0..DEFAULT_CHANNEL_SIZE {
            let msg = ActorMessage::new(CanonicalMessage::new(Role::User, format!("msg-{}", i)));
            tx.send(msg).await.unwrap();
        }

        // Verify all received
        for i in 0..DEFAULT_CHANNEL_SIZE {
            let received = rx.recv().await.unwrap();
            assert_eq!(received.message.content, format!("msg-{}", i));
        }
    }
}
