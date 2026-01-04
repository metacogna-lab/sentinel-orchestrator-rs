/**
 * AgentCard component - Displays agent status card
 * Shows agent ID, state, activity, and message count
 */

import type { AgentStatus } from '../../types';
import { formatDistanceToNow } from 'date-fns';
import clsx from 'clsx';

interface AgentCardProps {
  agent: AgentStatus;
  onClick?: () => void;
}

const stateColors: Record<string, string> = {
  idle: 'text-medium-gray',
  thinking: 'text-cyan-electric',
  toolcall: 'text-warning-amber',
  reflecting: 'text-neon-green',
};

const stateBgColors: Record<string, string> = {
  idle: 'bg-medium-gray/20 border-medium-gray/40',
  thinking: 'bg-cyan-electric/20 border-cyan-electric/40',
  toolcall: 'bg-warning-amber/20 border-warning-amber/40',
  reflecting: 'bg-neon-green/20 border-neon-green/40',
};

export function AgentCard({ agent, onClick }: AgentCardProps) {
  const stateColor = stateColors[agent.state] || 'text-medium-gray';
  const stateBgColor = stateBgColors[agent.state] || 'bg-medium-gray/20 border-medium-gray/40';

  return (
    <div
      className={clsx('card border transition-all duration-200', stateBgColor, {
        'cursor-pointer hover:shadow-glow-cyan hover:scale-[1.02]': onClick,
      })}
      onClick={onClick}
    >
      <div className="flex items-start justify-between gap-4">
        <div className="flex-1">
          <div className="flex items-center gap-3 mb-2">
            <h3 className="text-h4 font-display text-cyan-electric font-mono text-sm">
              {agent.id.substring(0, 8)}...
            </h3>
            <span className={clsx('text-body-sm font-medium capitalize', stateColor)}>
              {agent.state}
            </span>
          </div>
          <div className="space-y-1 text-body-sm text-light-gray">
            <p>
              <span className="text-medium-gray">Messages:</span>{' '}
              <span className="text-cyan-electric font-mono">{agent.messages_processed}</span>
            </p>
            <p>
              <span className="text-medium-gray">Last activity:</span>{' '}
              {formatDistanceToNow(new Date(agent.last_activity), { addSuffix: true })}
            </p>
          </div>
        </div>
        <div className={clsx('status-dot', {
          'status-active': agent.state === 'reflecting',
          'status-thinking': agent.state === 'thinking',
          'status-warning': agent.state === 'toolcall',
          'status-idle': agent.state === 'idle',
        })} />
      </div>
    </div>
  );
}

