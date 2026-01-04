/**
 * MetricCard component - Displays a single metric with value and optional trend
 */

interface MetricCardProps {
  title: string;
  value: string | number;
  unit?: string;
  trend?: number; // Positive = increase, negative = decrease
  color?: 'cyan' | 'green' | 'amber' | 'red' | 'orange';
  icon?: string;
}

export function MetricCard({ title, value, unit, trend, color = 'cyan', icon }: MetricCardProps) {
  const colorClasses = {
    cyan: 'text-cyan-electric border-cyan-electric/40',
    green: 'text-neon-green border-neon-green/40',
    amber: 'text-warning-amber border-warning-amber/40',
    red: 'text-error-red border-error-red/40',
    orange: 'text-rust-orange border-rust-orange/40',
  };

  const colorClass = colorClasses[color];

  return (
    <div className={`card border ${colorClass.split(' ')[1]}`}>
      <div className="flex items-start justify-between mb-2">
        <h3 className={`text-h4 font-display ${colorClass.split(' ')[0]} mb-2`}>
          {title}
        </h3>
        {icon && <span className="text-2xl">{icon}</span>}
      </div>
      <div className="flex items-baseline gap-2">
        <p className={`text-display-1 font-mono ${colorClass.split(' ')[0]}`}>
          {typeof value === 'number' ? value.toLocaleString() : value}
        </p>
        {unit && (
          <span className="text-body-sm text-medium-gray">{unit}</span>
        )}
      </div>
      {trend !== undefined && (
        <div className="mt-2 flex items-center gap-1">
          <span className={`text-body-sm ${trend >= 0 ? 'text-neon-green' : 'text-error-red'}`}>
            {trend >= 0 ? '↑' : '↓'} {Math.abs(trend).toFixed(1)}%
          </span>
          <span className="text-body-sm text-medium-gray">vs last period</span>
        </div>
      )}
    </div>
  );
}

