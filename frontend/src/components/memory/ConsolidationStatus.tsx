/**
 * ConsolidationStatus component - Shows memory consolidation status
 * Displays token count, threshold, and consolidation progress
 */

interface ConsolidationStatusProps {
  tokenCount: number;
  threshold: number;
  isConsolidating: boolean;
  lastConsolidation?: string;
}

export function ConsolidationStatus({
  tokenCount,
  threshold,
  isConsolidating,
  lastConsolidation,
}: ConsolidationStatusProps) {
  const percentage = Math.min((tokenCount / threshold) * 100, 100);
  const isNearThreshold = percentage >= 80;
  const isOverThreshold = percentage >= 100;

  return (
    <div className="card">
      <h3 className="text-h4 font-display text-cyan-electric mb-4">Consolidation Status</h3>
      
      <div className="space-y-4">
        {/* Token Count Progress */}
        <div>
          <div className="flex items-center justify-between mb-2">
            <span className="text-body-sm text-medium-gray">Token Count</span>
            <span className={`text-body font-mono font-medium ${isOverThreshold ? 'text-error-red' : isNearThreshold ? 'text-warning-amber' : 'text-cyan-electric'}`}>
              {tokenCount.toLocaleString()} / {threshold.toLocaleString()}
            </span>
          </div>
          <div className="w-full bg-deep-navy rounded-full h-4 overflow-hidden border border-cyan-electric/20">
            <div
              className={`h-full transition-all duration-300 ${
                isOverThreshold
                  ? 'bg-error-red'
                  : isNearThreshold
                  ? 'bg-warning-amber'
                  : 'bg-cyan-electric'
              }`}
              style={{ width: `${percentage}%` }}
            />
          </div>
          <div className="flex items-center justify-between mt-1">
            <span className="text-caption text-medium-gray">0%</span>
            <span className={`text-caption font-medium ${isOverThreshold ? 'text-error-red' : isNearThreshold ? 'text-warning-amber' : 'text-cyan-electric'}`}>
              {percentage.toFixed(1)}%
            </span>
            <span className="text-caption text-medium-gray">100%</span>
          </div>
        </div>

        {/* Status */}
        <div className="flex items-center justify-between pt-4 border-t border-cyan-electric/20">
          <span className="text-body-sm text-medium-gray">Status</span>
          <div className="flex items-center gap-2">
            {isConsolidating ? (
              <>
                <span className="status-dot status-warning" />
                <span className="text-body-sm font-medium text-warning-amber">
                  Consolidating...
                </span>
              </>
            ) : isOverThreshold ? (
              <>
                <span className="status-dot status-error" />
                <span className="text-body-sm font-medium text-error-red">
                  Over Threshold
                </span>
              </>
            ) : isNearThreshold ? (
              <>
                <span className="status-dot status-warning" />
                <span className="text-body-sm font-medium text-warning-amber">
                  Near Threshold
                </span>
              </>
            ) : (
              <>
                <span className="status-dot status-active" />
                <span className="text-body-sm font-medium text-neon-green">
                  Normal
                </span>
              </>
            )}
          </div>
        </div>

        {/* Last Consolidation */}
        {lastConsolidation && (
          <div className="flex items-center justify-between pt-2 border-t border-cyan-electric/20">
            <span className="text-body-sm text-medium-gray">Last Consolidation</span>
            <span className="text-body-sm text-light-gray">{lastConsolidation}</span>
          </div>
        )}
      </div>
    </div>
  );
}

