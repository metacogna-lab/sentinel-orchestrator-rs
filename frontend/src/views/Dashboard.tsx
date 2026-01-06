/**
 * Dashboard view - Main landing page
 * Displays system health overview, quick metrics, and recent activity
 */

import { useState, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import type { HealthStatus, AgentStatus } from '../types';
import { getHealthStatus, checkBackendConnection } from '../services/health';
import { getAgentStatus } from '../services/agents';
import { useAuth } from '../store/auth';
import { LoadingSpinner } from '../components/ui';

type ConnectionStatus = 'checking' | 'connected' | 'disconnected';

export function Dashboard() {
  const { hasApiKey } = useAuth();
  const [healthStatus, setHealthStatus] = useState<HealthStatus | null>(null);
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>('checking');
  const [agents, setAgents] = useState<AgentStatus[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchDashboardData = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      // Check backend connection
      const isConnected = await checkBackendConnection();
      setConnectionStatus(isConnected ? 'connected' : 'disconnected');

      if (isConnected) {
        // Get health status
        const health = await getHealthStatus();
        setHealthStatus(health);

        // Get agent status if authenticated
        if (hasApiKey) {
          try {
            const agentData = await getAgentStatus();
            setAgents(agentData);
          } catch (agentError) {
            // Agent status might fail if no agents or auth issues
            console.warn('Failed to fetch agent status:', agentError);
            setAgents([]);
          }
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load dashboard data');
      setConnectionStatus('disconnected');
    } finally {
      setIsLoading(false);
    }
  }, [hasApiKey]);

  useEffect(() => {
    fetchDashboardData();

    // Auto-refresh every 30 seconds
    const interval = setInterval(fetchDashboardData, 30000);
    return () => clearInterval(interval);
  }, [fetchDashboardData]);

  const getHealthColor = (status: string) => {
    switch (status) {
      case 'healthy':
      case 'ready':
      case 'alive':
        return 'text-neon-green';
      case 'unhealthy':
        return 'text-error-red';
      default:
        return 'text-warning-amber';
    }
  };

  const getConnectionColor = (status: ConnectionStatus) => {
    switch (status) {
      case 'connected':
        return 'text-neon-green';
      case 'disconnected':
        return 'text-error-red';
      default:
        return 'text-warning-amber';
    }
  };

  if (isLoading) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-display-2 font-display text-rust-orange mb-2">
            Dashboard
          </h1>
          <p className="text-light-gray text-body-lg">
            System overview and quick metrics
          </p>
        </div>
        <LoadingSpinner text="Loading dashboard..." />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          Dashboard
        </h1>
        <p className="text-light-gray text-body-lg">
          System overview and quick metrics
        </p>
      </div>

      {/* Connection Status */}
      <div className="card">
        <h2 className="text-h3 font-display text-cyan-electric mb-4">Connection Status</h2>
        <div className="flex items-center gap-2">
          <span className={`status-dot ${connectionStatus === 'connected' ? 'status-active' : 'status-error'}`}></span>
          <span className={`text-body ${getConnectionColor(connectionStatus)}`}>
            {connectionStatus === 'checking' && 'Checking connection...'}
            {connectionStatus === 'connected' && 'Connected to backend'}
            {connectionStatus === 'disconnected' && 'Disconnected from backend'}
          </span>
        </div>
        {connectionStatus === 'disconnected' && (
          <p className="text-caption text-medium-gray mt-2">
            Check your backend URL in <Link to="/config" className="text-cyan-electric hover:text-rust-orange">Configuration</Link>
          </p>
        )}
      </div>

      {/* Health Status Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">System Health</h3>
          {healthStatus ? (
            <div className="flex items-center gap-2">
              <span className={`status-dot ${healthStatus.status === 'healthy' ? 'status-active' : 'status-error'}`}></span>
              <span className={`text-body capitalize ${getHealthColor(healthStatus.status)}`}>
                {healthStatus.status}
              </span>
            </div>
          ) : (
            <div className="flex items-center gap-2">
              <span className="status-dot status-error"></span>
              <span className="text-body text-error-red">Unknown</span>
            </div>
          )}
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">Active Agents</h3>
          <p className="text-display-1 font-mono text-neon-green">{agents.length}</p>
          {hasApiKey ? (
            <p className="text-caption text-medium-gray mt-1">
              {agents.length === 0 ? 'No agents active' : `${agents.length} agent${agents.length === 1 ? '' : 's'} running`}
            </p>
          ) : (
            <p className="text-caption text-medium-gray mt-1">
              <Link to="/config" className="text-cyan-electric hover:text-rust-orange">Configure API key</Link> to view agents
            </p>
          )}
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">Messages Processed</h3>
          <p className="text-display-1 font-mono text-cyan-electric">
            {agents.reduce((total, agent) => total + agent.messages_processed, 0)}
          </p>
          <p className="text-caption text-medium-gray mt-1">
            Total across all agents
          </p>
        </div>
      </div>

      {/* Error Display */}
      {error && (
        <div className="card border-error-red/40 bg-error-red/10">
          <h3 className="text-h4 font-display text-error-red mb-2">Error</h3>
          <p className="text-body text-light-gray mb-4">{error}</p>
          <button onClick={fetchDashboardData} className="btn btn-secondary">
            Retry
          </button>
        </div>
      )}

      {/* Quick Actions */}
      <div className="card">
        <h2 className="text-h3 font-display text-rust-orange mb-4">Quick Actions</h2>
        <div className="flex flex-wrap gap-4">
          <Link to="/chat" className="btn btn-primary">
            Start Chat
          </Link>
          <Link to="/agents" className="btn btn-secondary">
            View Agents
          </Link>
          <Link to="/metrics" className="btn btn-ghost">
            View Metrics
          </Link>
          <Link to="/config" className="btn btn-ghost">
            Configure
          </Link>
        </div>
      </div>
    </div>
  );
}

