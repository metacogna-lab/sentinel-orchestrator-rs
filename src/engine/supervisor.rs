// Supervisor for agent lifecycle management
// Monitors agent health, detects zombies, and manages agent lifecycle

use crate::core::types::{AgentId, AgentState};
use crate::engine::actor::spawn_actor;
use crate::engine::channels::ActorMessage;
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::watch;
use tokio::time::interval;
use tracing::{error, info, warn};

/// Default health check interval (10 seconds)
pub const DEFAULT_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(10);

/// Default zombie timeout (60 seconds)
pub const DEFAULT_ZOMBIE_TIMEOUT: Duration = Duration::from_secs(60);

/// Handle for a managed agent
pub struct AgentHandle {
    /// Channel sender for communicating with the agent
    pub tx: mpsc::Sender<ActorMessage>,
    /// Shutdown signal sender
    pub shutdown_tx: watch::Sender<()>,
    /// Task join handle
    pub handle: tokio::task::JoinHandle<Result<()>>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Current agent state (best effort tracking)
    pub state: AgentState,
}

impl AgentHandle {
    /// Create a new agent handle
    pub fn new(
        tx: mpsc::Sender<ActorMessage>,
        shutdown_tx: watch::Sender<()>,
        handle: tokio::task::JoinHandle<Result<()>>,
    ) -> Self {
        Self {
            tx,
            shutdown_tx,
            handle,
            last_activity: Utc::now(),
            state: AgentState::Idle,
        }
    }

    /// Update the last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
    }

    /// Check if the agent task is still running
    pub fn is_alive(&self) -> bool {
        !self.handle.is_finished()
    }
}

/// Supervisor for managing agent lifecycle
pub struct Supervisor {
    /// Map of agent IDs to their handles
    agents: HashMap<AgentId, AgentHandle>,
    /// Interval between health checks
    health_check_interval: Duration,
    /// Timeout for zombie detection
    zombie_timeout: Duration,
}

impl Supervisor {
    /// Create a new supervisor with default settings
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            health_check_interval: DEFAULT_HEALTH_CHECK_INTERVAL,
            zombie_timeout: DEFAULT_ZOMBIE_TIMEOUT,
        }
    }

    /// Create a new supervisor with custom settings
    pub fn with_settings(health_check_interval: Duration, zombie_timeout: Duration) -> Self {
        Self {
            agents: HashMap::new(),
            health_check_interval,
            zombie_timeout,
        }
    }

    /// Spawn a new agent and register it with the supervisor
    ///
    /// # Returns
    /// * `Ok(AgentId)` - The ID of the newly spawned agent
    /// * `Err(anyhow::Error)` - Error if spawning fails
    pub fn spawn_agent(&mut self) -> Result<AgentId> {
        let (tx, shutdown_tx, handle) = spawn_actor(32);
        let agent_id = AgentId::new();

        let agent_handle = AgentHandle::new(tx, shutdown_tx, handle);
        self.agents.insert(agent_id, agent_handle);

        info!("Supervisor spawned agent {}", agent_id);
        Ok(agent_id)
    }

    /// Terminate an agent and remove it from tracking
    ///
    /// # Arguments
    /// * `id` - The ID of the agent to terminate
    ///
    /// # Returns
    /// * `Ok(())` - Agent terminated successfully
    /// * `Err(anyhow::Error)` - Error if termination fails
    pub async fn terminate_agent(&mut self, id: AgentId) -> Result<()> {
        let agent_handle = self
            .agents
            .remove(&id)
            .ok_or_else(|| anyhow::anyhow!("Agent {} not found", id))?;

        info!("Supervisor terminating agent {}", id);

        // Send shutdown signal
        let _ = agent_handle.shutdown_tx.send(());

        // Wait for task to complete (with timeout)
        match tokio::time::timeout(Duration::from_secs(5), agent_handle.handle).await {
            Ok(join_result) => {
                if let Err(e) = join_result {
                    warn!("Agent {} task error: {}", id, e);
                }
            }
            Err(_) => {
                warn!("Agent {} did not terminate within timeout", id);
                // Task will be dropped, which will abort it
            }
        }

        info!("Supervisor terminated agent {}", id);
        Ok(())
    }

    /// Restart an agent (terminate and spawn new one)
    ///
    /// # Arguments
    /// * `id` - The ID of the agent to restart
    ///
    /// # Returns
    /// * `Ok(AgentId)` - The ID of the newly spawned agent
    /// * `Err(anyhow::Error)` - Error if restart fails
    pub async fn restart_agent(&mut self, id: AgentId) -> Result<AgentId> {
        info!("Supervisor restarting agent {}", id);
        self.terminate_agent(id).await?;
        self.spawn_agent()
    }

    /// Check the health of a specific agent
    ///
    /// # Arguments
    /// * `id` - The ID of the agent to check
    ///
    /// # Returns
    /// * `Ok(AgentHealth)` - Health status of the agent
    /// * `Err(anyhow::Error)` - Error if agent not found
    pub fn check_agent_health(&self, id: AgentId) -> Result<AgentHealth> {
        let handle = self
            .agents
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("Agent {} not found", id))?;

        let time_since_activity = Utc::now() - handle.last_activity;
        let is_zombie = time_since_activity.num_seconds() > self.zombie_timeout.as_secs() as i64
            && handle.is_alive();

        Ok(AgentHealth {
            id,
            state: handle.state,
            last_activity: handle.last_activity,
            is_alive: handle.is_alive(),
            is_zombie,
        })
    }

    /// Detect all zombie agents (stuck >60s)
    ///
    /// # Returns
    /// Vector of agent IDs that are zombies
    pub fn detect_zombies(&self) -> Vec<AgentId> {
        let mut zombies = Vec::new();

        for (id, handle) in &self.agents {
            let time_since_activity = Utc::now() - handle.last_activity;
            let is_zombie = time_since_activity.num_seconds()
                > self.zombie_timeout.as_secs() as i64
                && handle.is_alive();

            if is_zombie {
                warn!(
                    "Detected zombie agent {} (stuck for {}s)",
                    id,
                    time_since_activity.num_seconds()
                );
                zombies.push(*id);
            }
        }

        zombies
    }

    /// Update activity for an agent (called when agent processes a message)
    ///
    /// # Arguments
    /// * `id` - The ID of the agent
    pub fn update_agent_activity(&mut self, id: AgentId) {
        if let Some(handle) = self.agents.get_mut(&id) {
            handle.update_activity();
        }
    }

    /// Get all agent IDs currently managed
    pub fn agent_ids(&self) -> Vec<AgentId> {
        self.agents.keys().copied().collect()
    }

    /// Get the number of agents currently managed
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Run the supervisor event loop
    ///
    /// This loop periodically checks for zombies and handles shutdown signals.
    ///
    /// # Arguments
    /// * `shutdown_rx` - Shutdown signal receiver
    ///
    /// # Returns
    /// * `Ok(())` - Graceful shutdown
    /// * `Err(anyhow::Error)` - Error during operation
    pub async fn run(&mut self, mut shutdown_rx: watch::Receiver<()>) -> Result<()> {
        let mut health_check_interval = interval(self.health_check_interval);

        info!("Supervisor started with {} agents", self.agent_count());

        loop {
            tokio::select! {
                // Health check tick
                _ = health_check_interval.tick() => {
                    let zombies = self.detect_zombies();
                    for zombie_id in zombies {
                        if let Err(e) = self.terminate_agent(zombie_id).await {
                            error!("Failed to terminate zombie agent {}: {}", zombie_id, e);
                        }
                    }
                }
                // Shutdown signal
                _ = shutdown_rx.changed() => {
                    info!("Supervisor received shutdown signal");
                    break;
                }
            }
        }

        // Graceful shutdown: terminate all agents
        info!("Supervisor shutting down, terminating all agents");
        let agent_ids: Vec<AgentId> = self.agents.keys().copied().collect();
        for agent_id in agent_ids {
            if let Err(e) = self.terminate_agent(agent_id).await {
                error!(
                    "Failed to terminate agent {} during shutdown: {}",
                    agent_id, e
                );
            }
        }

        info!("Supervisor stopped");
        Ok(())
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}

/// Health status of an agent
#[derive(Debug, Clone)]
pub struct AgentHealth {
    /// Agent identifier
    pub id: AgentId,
    /// Current state
    pub state: AgentState,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Whether the agent task is still running
    pub is_alive: bool,
    /// Whether the agent is a zombie (stuck >60s)
    pub is_zombie: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_supervisor_spawns_agents() {
        let mut supervisor = Supervisor::new();
        let agent_id = supervisor.spawn_agent().unwrap();

        assert_eq!(supervisor.agent_count(), 1);
        assert!(supervisor.agent_ids().contains(&agent_id));
    }

    #[tokio::test]
    async fn test_supervisor_tracks_agent_handles() {
        let mut supervisor = Supervisor::new();
        let agent_id1 = supervisor.spawn_agent().unwrap();
        let agent_id2 = supervisor.spawn_agent().unwrap();

        assert_eq!(supervisor.agent_count(), 2);
        assert!(supervisor.agent_ids().contains(&agent_id1));
        assert!(supervisor.agent_ids().contains(&agent_id2));
    }

    #[tokio::test]
    async fn test_health_check_detects_healthy_agents() {
        let mut supervisor = Supervisor::new();
        let agent_id = supervisor.spawn_agent().unwrap();

        let health = supervisor.check_agent_health(agent_id).unwrap();
        assert!(health.is_alive);
        assert!(!health.is_zombie);
    }

    #[tokio::test]
    async fn test_zombie_detection() {
        let mut supervisor = Supervisor::with_settings(
            Duration::from_secs(1),
            Duration::from_secs(2), // Short timeout for testing
        );

        let agent_id = supervisor.spawn_agent().unwrap();

        // Wait longer than zombie timeout without updating activity
        tokio::time::sleep(Duration::from_secs(3)).await;

        let zombies = supervisor.detect_zombies();
        assert!(
            zombies.contains(&agent_id),
            "Agent should be detected as zombie"
        );
    }

    #[tokio::test]
    async fn test_terminate_agent() {
        let mut supervisor = Supervisor::new();
        let agent_id = supervisor.spawn_agent().unwrap();

        assert_eq!(supervisor.agent_count(), 1);

        supervisor.terminate_agent(agent_id).await.unwrap();

        assert_eq!(supervisor.agent_count(), 0);
        assert!(supervisor.check_agent_health(agent_id).is_err());
    }

    #[tokio::test]
    async fn test_restart_agent() {
        let mut supervisor = Supervisor::new();
        let agent_id1 = supervisor.spawn_agent().unwrap();

        let agent_id2 = supervisor.restart_agent(agent_id1).await.unwrap();

        assert_ne!(agent_id1, agent_id2);
        assert_eq!(supervisor.agent_count(), 1);
        assert!(supervisor.agent_ids().contains(&agent_id2));
        assert!(!supervisor.agent_ids().contains(&agent_id1));
    }

    #[tokio::test]
    async fn test_update_agent_activity() {
        let mut supervisor = Supervisor::new();
        let agent_id = supervisor.spawn_agent().unwrap();

        let health1 = supervisor.check_agent_health(agent_id).unwrap();
        let last_activity1 = health1.last_activity;

        // Wait a bit
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Update activity
        supervisor.update_agent_activity(agent_id);

        let health2 = supervisor.check_agent_health(agent_id).unwrap();
        assert!(health2.last_activity > last_activity1);
    }

    #[tokio::test]
    async fn test_graceful_shutdown_terminates_all_agents() {
        let mut supervisor = Supervisor::new();
        let _agent_id1 = supervisor.spawn_agent().unwrap();
        let _agent_id2 = supervisor.spawn_agent().unwrap();

        assert_eq!(supervisor.agent_count(), 2);

        let (shutdown_tx, shutdown_rx) = watch::channel(());

        // Spawn supervisor in background
        let supervisor_handle = tokio::spawn(async move { supervisor.run(shutdown_rx).await });

        // Give supervisor time to start
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Send shutdown signal
        shutdown_tx.send(()).unwrap();

        // Wait for supervisor to finish
        let result = timeout(Duration::from_secs(2), supervisor_handle).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }

    #[tokio::test]
    async fn test_supervisor_zombie_cleanup() {
        let mut supervisor = Supervisor::with_settings(
            Duration::from_millis(100), // Fast health checks
            Duration::from_millis(200), // Short zombie timeout
        );

        let _agent_id = supervisor.spawn_agent().unwrap();

        let (shutdown_tx, shutdown_rx) = watch::channel(());

        // Spawn supervisor in background
        let supervisor_handle = tokio::spawn(async move { supervisor.run(shutdown_rx).await });

        // Wait for zombie detection and cleanup
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Shutdown supervisor
        shutdown_tx.send(()).unwrap();

        let _ = timeout(Duration::from_secs(1), supervisor_handle).await;
    }
}
