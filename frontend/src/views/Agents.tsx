/**
 * Agents view - Agent management and monitoring
 * Full agent monitoring with state visualization
 */

import { useState, useEffect, useCallback } from 'react';
import type { AgentStatus } from '../types';
import { getAgentStatus } from '../services/agents';
import { AgentCard, StateMachineDiagram, AgentDetails } from '../components/agents';
import { LoadingSpinner } from '../components/ui';

export function Agents() {
  const [agents, setAgents] = useState<AgentStatus[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedAgent, setSelectedAgent] = useState<AgentStatus | null>(null);
  const [autoRefresh, setAutoRefresh] = useState(true);

  const fetchAgents = useCallback(async () => {
    try {
      setError(null);
      const data = await getAgentStatus();
      setAgents(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load agents');
      console.error('Agent status error:', err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchAgents();

    if (autoRefresh) {
      const interval = setInterval(() => {
        fetchAgents();
      }, 5000); // Refresh every 5 seconds

      return () => clearInterval(interval);
    }
  }, [fetchAgents, autoRefresh]);

  const handleAgentClick = (agent: AgentStatus) => {
    setSelectedAgent(agent);
  };

  const handleCloseDetails = () => {
    setSelectedAgent(null);
  };

  if (isLoading && agents.length === 0) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-display-2 font-display text-rust-orange mb-2">
            Agents
          </h1>
          <p className="text-light-gray text-body-lg">
            Agent monitoring and state visualization
          </p>
        </div>
        <LoadingSpinner text="Loading agents..." />
      </div>
    );
  }

  if (error && agents.length === 0) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-display-2 font-display text-rust-orange mb-2">
            Agents
          </h1>
          <p className="text-light-gray text-body-lg">
            Agent monitoring and state visualization
          </p>
        </div>
        <div className="card border-error-red/40 bg-error-red/10">
          <h3 className="text-h4 font-display text-error-red mb-2">Error</h3>
          <p className="text-body text-light-gray mb-4">{error}</p>
          <button onClick={fetchAgents} className="btn btn-secondary">
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-display-2 font-display text-rust-orange mb-2">
            Agents
          </h1>
          <p className="text-light-gray text-body-lg">
            Agent monitoring and state visualization
          </p>
        </div>
        <div className="flex items-center gap-4">
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
              className="rounded"
            />
            <span className="text-body-sm text-light-gray">Auto-refresh</span>
          </label>
          <button onClick={fetchAgents} className="btn btn-secondary" disabled={isLoading}>
            Refresh
          </button>
        </div>
      </div>

      {error && (
        <div className="card border-warning-amber/40 bg-warning-amber/10">
          <p className="text-body text-warning-amber">{error}</p>
        </div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Agent List */}
        <div className="lg:col-span-2 space-y-4">
          <div className="card">
            <h2 className="text-h3 font-display text-cyan-electric mb-4">
              Agent List ({agents.length})
            </h2>
            {agents.length === 0 ? (
              <div className="text-center py-12 text-medium-gray">
                <p className="text-body-lg">No agents found</p>
                <p className="text-body-sm mt-2">Agents will appear here when they are created</p>
              </div>
            ) : (
              <div className="space-y-3">
                {agents.map((agent) => (
                  <AgentCard
                    key={agent.id}
                    agent={agent}
                    onClick={() => handleAgentClick(agent)}
                  />
                ))}
              </div>
            )}
          </div>
        </div>

        {/* State Machine & Details */}
        <div className="space-y-4">
          <StateMachineDiagram currentState={selectedAgent?.state} />
          {selectedAgent && (
            <AgentDetails agent={selectedAgent} onClose={handleCloseDetails} />
          )}
        </div>
      </div>
    </div>
  );
}
