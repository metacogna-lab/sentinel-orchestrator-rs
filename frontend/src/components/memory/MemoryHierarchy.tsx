/**
 * MemoryHierarchy component - Visual representation of three-tier memory system
 * Shows the flow from Short-term → Medium-term → Long-term
 */

interface MemoryHierarchyProps {
  shortTermCount: number;
  mediumTermCount: number;
  longTermCount: number;
  consolidating?: boolean;
}

export function MemoryHierarchy({
  shortTermCount,
  mediumTermCount,
  longTermCount,
  consolidating = false,
}: MemoryHierarchyProps) {
  return (
    <div className="card">
      <h3 className="text-h4 font-display text-cyan-electric mb-4">Memory Hierarchy</h3>
      <div className="relative">
        {/* Short-term Memory */}
        <div className="bg-cyan-electric/20 border border-cyan-electric/40 rounded-lg p-4 mb-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <span className="text-2xl">💾</span>
              <div>
                <h4 className="text-h4 font-display text-cyan-electric">Short-Term</h4>
                <p className="text-body-sm text-medium-gray">In-Memory</p>
              </div>
            </div>
            <span className="text-display-2 font-mono text-cyan-electric">
              {shortTermCount}
            </span>
          </div>
        </div>

        {/* Consolidation Arrow */}
        <div className="flex items-center justify-center mb-4">
          <div className={`flex items-center gap-2 ${consolidating ? 'text-warning-amber' : 'text-medium-gray'}`}>
            <span className="text-xl">{consolidating ? '⚙️' : '↓'}</span>
            <span className="text-body-sm font-medium">
              {consolidating ? 'Consolidating...' : 'Consolidation'}
            </span>
            <span className="text-xl">{consolidating ? '⚙️' : '↓'}</span>
          </div>
        </div>

        {/* Medium-term Memory */}
        <div className="bg-neon-green/20 border border-neon-green/40 rounded-lg p-4 mb-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <span className="text-2xl">🗄️</span>
              <div>
                <h4 className="text-h4 font-display text-neon-green">Medium-Term</h4>
                <p className="text-body-sm text-medium-gray">Sled Database</p>
              </div>
            </div>
            <span className="text-display-2 font-mono text-neon-green">
              {mediumTermCount}
            </span>
          </div>
        </div>

        {/* Embedding Arrow */}
        <div className="flex items-center justify-center mb-4">
          <div className="flex items-center gap-2 text-medium-gray">
            <span className="text-xl">↓</span>
            <span className="text-body-sm font-medium">Embedding</span>
            <span className="text-xl">↓</span>
          </div>
        </div>

        {/* Long-term Memory */}
        <div className="bg-warning-amber/20 border border-warning-amber/40 rounded-lg p-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <span className="text-2xl">🔍</span>
              <div>
                <h4 className="text-h4 font-display text-warning-amber">Long-Term</h4>
                <p className="text-body-sm text-medium-gray">Qdrant Vector Store</p>
              </div>
            </div>
            <span className="text-display-2 font-mono text-warning-amber">
              {longTermCount}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}

