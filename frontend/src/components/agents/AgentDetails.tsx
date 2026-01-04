/**
 * AgentDetails component - Detailed agent information panel
 * Shows full agent status information
 */

import type { AgentStatus } from '../../types';
import { formatDistanceToNow, format } from 'date-fns';

interface AgentDetailsProps {
  agent: AgentStatus | null;
  onClose?: () => void;
}

export function AgentDetails({ agent, onClose }: AgentDetailsProps) {
  if (!agent) {
    return null;
  }

  const stateColors: Record<string, string> = {
    idle: 'text-medium-gray',
    thinking: 'text-cyan-electric',
    toolcall: 'text-warning-amber',
    reflecting: 'text-neon-green',
  };

  const stateColor = stateColors[agent.state] || 'text-medium-gray';

  return (
    <div className="card border-cyan-electric/40">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-h3 font-display text-rust-orange">Agent Details</h3>
        {onClose && (
          <button
            onClick={onClose}
            className="btn btn-ghost text-body-sm"
          >
            Close
          </button>
        )}
      </div>

      <div className="space-y-4">
        <div>
          <label className="text-body-sm font-medium text-medium-gray">Agent ID</label>
          <p className="text-body font-mono text-cyan-electric mt-1 break-all">{agent.id}</p>
        </div>

        <div>
          <label className="text-body-sm font-medium text-medium-gray">Current State</label>
          <div className="flex items-center gap-2 mt-1">
            <span className={`text-body font-medium capitalize ${stateColor}`}>
              {agent.state}
            </span>
          </div>
        </div>

        <div>
          <label className="text-body-sm font-medium text-medium-gray">Messages Processed</label>
          <p className="text-display-1 font-mono text-cyan-electric mt-1">
            {agent.messages_processed}
          </p>
        </div>

        <div>
          <label className="text-body-sm font-medium text-medium-gray">Last Activity</label>
          <p className="text-body text-light-gray mt-1">
            {formatDistanceToNow(new Date(agent.last_activity), { addSuffix: true })}
          </p>
          <p className="text-body-sm text-medium-gray mt-1">
            {format(new Date(agent.last_activity), 'PPpp')}
          </p>
        </div>
      </div>
    </div>
  );
}

