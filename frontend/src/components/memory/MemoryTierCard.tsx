/**
 * MemoryTierCard component - Displays a memory tier with statistics
 * Shows tier name, message count, token count, and status
 */

interface MemoryTierCardProps {
  title: string;
  description: string;
  messageCount: number;
  tokenCount?: number;
  status: 'active' | 'idle' | 'consolidating';
  icon: string;
  color: 'cyan' | 'green' | 'amber' | 'orange';
}

export function MemoryTierCard({
  title,
  description,
  messageCount,
  tokenCount,
  status,
  icon,
  color,
}: MemoryTierCardProps) {
  const colorClasses = {
    cyan: 'text-cyan-electric border-cyan-electric/40 bg-cyan-electric/10',
    green: 'text-neon-green border-neon-green/40 bg-neon-green/10',
    amber: 'text-warning-amber border-warning-amber/40 bg-warning-amber/10',
    orange: 'text-rust-orange border-rust-orange/40 bg-rust-orange/10',
  };

  const colorClass = colorClasses[color];
  const statusColors = {
    active: 'text-neon-green',
    idle: 'text-medium-gray',
    consolidating: 'text-warning-amber',
  };

  return (
    <div className={`card border ${colorClass.split(' ')[1]}`}>
      <div className="flex items-start justify-between mb-4">
        <div className="flex items-center gap-3">
          <span className="text-3xl">{icon}</span>
          <div>
            <h3 className={`text-h4 font-display ${colorClass.split(' ')[0]}`}>
              {title}
            </h3>
            <p className="text-body-sm text-medium-gray mt-1">{description}</p>
          </div>
        </div>
        <span className={`status-dot status-${status === 'active' ? 'active' : status === 'consolidating' ? 'warning' : 'idle'}`} />
      </div>

      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-body-sm text-medium-gray">Messages</span>
          <span className={`text-body font-mono font-medium ${colorClass.split(' ')[0]}`}>
            {messageCount.toLocaleString()}
          </span>
        </div>
        {tokenCount !== undefined && (
          <div className="flex items-center justify-between">
            <span className="text-body-sm text-medium-gray">Tokens</span>
            <span className={`text-body font-mono font-medium ${colorClass.split(' ')[0]}`}>
              {tokenCount.toLocaleString()}
            </span>
          </div>
        )}
        <div className="flex items-center justify-between pt-2 border-t border-cyan-electric/20">
          <span className="text-body-sm text-medium-gray">Status</span>
          <span className={`text-body-sm font-medium capitalize ${statusColors[status]}`}>
            {status}
          </span>
        </div>
      </div>
    </div>
  );
}

