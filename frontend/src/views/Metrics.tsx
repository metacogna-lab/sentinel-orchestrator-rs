/**
 * Metrics view - Performance metrics and analytics
 * Real-time metrics dashboard with charts
 */

import { useState, useEffect, useCallback } from 'react';
import { MetricCard, TimeSeriesChart, BarChart } from '../components/metrics';
import { getAgentStatus } from '../services/agents';
import type { AgentStatus } from '../types';
import { LoadingSpinner } from '../components/ui';

// Mock data generator for demonstration
function generateMockTimeSeriesData(count: number, baseValue: number, variance: number = 0.2) {
  const data = [];
  const now = new Date();
  
  for (let i = count - 1; i >= 0; i--) {
    const time = new Date(now.getTime() - i * 60000); // 1 minute intervals
    const value = baseValue * (1 + (Math.random() - 0.5) * variance);
    data.push({
      time: time.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' }),
      value: Math.max(0, Math.round(value)),
    });
  }
  
  return data;
}

export function Metrics() {
  const [agents, setAgents] = useState<AgentStatus[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [autoRefresh, setAutoRefresh] = useState(true);

  const fetchAgents = useCallback(async () => {
    try {
      setError(null);
      const data = await getAgentStatus();
      setAgents(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load metrics');
      console.error('Metrics error:', err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchAgents();

    if (autoRefresh) {
      const interval = setInterval(() => {
        fetchAgents();
      }, 10000); // Refresh every 10 seconds

      return () => clearInterval(interval);
    }
  }, [fetchAgents, autoRefresh]);

  // Calculate metrics from agents
  const totalAgents = agents.length;
  const activeAgents = agents.filter(a => a.state !== 'idle').length;
  const totalMessages = agents.reduce((sum, a) => sum + a.messages_processed, 0);
  const avgMessagesPerAgent = totalAgents > 0 ? Math.round(totalMessages / totalAgents) : 0;

  // State distribution
  const stateDistribution = agents.reduce((acc, agent) => {
    acc[agent.state] = (acc[agent.state] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  // Mock time series data (in production, this would come from metrics API)
  const requestRateData = generateMockTimeSeriesData(30, 50, 0.3);
  const latencyData = generateMockTimeSeriesData(30, 200, 0.2);

  if (isLoading && agents.length === 0) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-display-2 font-display text-rust-orange mb-2">
            Metrics
          </h1>
          <p className="text-light-gray text-body-lg">
            Real-time performance metrics and analytics
          </p>
        </div>
        <LoadingSpinner text="Loading metrics..." />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-display-2 font-display text-rust-orange mb-2">
            Metrics
          </h1>
          <p className="text-light-gray text-body-lg">
            Real-time performance metrics and analytics
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
          <p className="text-body text-warning-amber">
            Note: Metrics API endpoint not yet implemented. Showing agent-based metrics only.
          </p>
        </div>
      )}

      {/* Key Metrics Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <MetricCard
          title="Total Agents"
          value={totalAgents}
          color="cyan"
          icon="🤖"
        />
        <MetricCard
          title="Active Agents"
          value={activeAgents}
          color="green"
          icon="⚡"
        />
        <MetricCard
          title="Messages Processed"
          value={totalMessages}
          color="orange"
          icon="💬"
        />
        <MetricCard
          title="Avg Messages/Agent"
          value={avgMessagesPerAgent}
          color="amber"
          icon="📊"
        />
      </div>

      {/* Time Series Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <TimeSeriesChart
          title="Request Rate"
          data={requestRateData.map(d => ({ time: d.time, requests: d.value }))}
          lines={[
            { key: 'requests', name: 'Requests/sec', color: '#00D9FF' },
          ]}
        />
        <TimeSeriesChart
          title="Average Latency"
          data={latencyData.map(d => ({ time: d.time, latency: d.value }))}
          lines={[
            { key: 'latency', name: 'Latency (ms)', color: '#FF6B35' },
          ]}
        />
      </div>

      {/* State Distribution */}
      <BarChart
        title="Agent State Distribution"
        data={[
          { name: 'Idle', count: stateDistribution.idle || 0 },
          { name: 'Thinking', count: stateDistribution.thinking || 0 },
          { name: 'Tool Call', count: stateDistribution.toolcall || 0 },
          { name: 'Reflecting', count: stateDistribution.reflecting || 0 },
        ]}
        bars={[
          { key: 'count', name: 'Agents', color: '#00D9FF' },
        ]}
      />
    </div>
  );
}
