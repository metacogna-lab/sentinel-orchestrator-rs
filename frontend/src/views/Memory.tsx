/**
 * Memory view - Three-tier memory system visualization
 * Displays Short-term, Medium-term, and Long-term memory with consolidation status
 */

import { MemoryTierCard, MemoryHierarchy, ConsolidationStatus } from '../components/memory';

export function Memory() {
  // Mock data - in production, this would come from the backend API
  // Note: Memory endpoints are not yet implemented in the backend
  const shortTermCount = 0;
  const mediumTermCount = 0;
  const longTermCount = 0;
  const shortTermTokens = 0;
  const mediumTermTokens = 0;
  const longTermTokens = 0;
  const totalTokens = shortTermTokens + mediumTermTokens + longTermTokens;
  const consolidationThreshold = 10000;
  const isConsolidating = false;
  const lastConsolidation = undefined;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          Memory System
        </h1>
        <p className="text-light-gray text-body-lg">
          Three-tier memory system visualization and management
        </p>
      </div>

      <div className="card border-warning-amber/40 bg-warning-amber/10">
        <div className="flex items-center gap-3">
          <span className="status-dot status-warning" />
          <div>
            <h3 className="text-h4 font-display text-warning-amber mb-1">
              Memory API Not Yet Implemented
            </h3>
            <p className="text-body-sm text-light-gray">
              The memory system endpoints are not yet available in the backend.
              This view will be fully functional once the backend memory system is implemented.
            </p>
          </div>
        </div>
      </div>

      {/* Memory Hierarchy */}
      <MemoryHierarchy
        shortTermCount={shortTermCount}
        mediumTermCount={mediumTermCount}
        longTermCount={longTermCount}
        consolidating={isConsolidating}
      />

      {/* Memory Tiers */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <MemoryTierCard
          title="Short-Term"
          description="In-memory conversation history"
          messageCount={shortTermCount}
          tokenCount={shortTermTokens}
          status={shortTermCount > 0 ? 'active' : 'idle'}
          icon="💾"
          color="cyan"
        />
        <MemoryTierCard
          title="Medium-Term"
          description="Sled database (summarized)"
          messageCount={mediumTermCount}
          tokenCount={mediumTermTokens}
          status={mediumTermCount > 0 ? 'active' : 'idle'}
          icon="🗄️"
          color="green"
        />
        <MemoryTierCard
          title="Long-Term"
          description="Qdrant vector store (embeddings)"
          messageCount={longTermCount}
          tokenCount={longTermTokens}
          status={longTermCount > 0 ? 'active' : 'idle'}
          icon="🔍"
          color="amber"
        />
      </div>

      {/* Consolidation Status */}
      <ConsolidationStatus
        tokenCount={totalTokens}
        threshold={consolidationThreshold}
        isConsolidating={isConsolidating}
        lastConsolidation={lastConsolidation}
      />

      {/* Memory System Information */}
      <div className="card">
        <h3 className="text-h4 font-display text-cyan-electric mb-4">
          Memory System Information
        </h3>
        <div className="space-y-4 text-body text-light-gray">
          <div>
            <h4 className="text-h4 font-display text-rust-orange mb-2">Three-Tier Architecture</h4>
            <p className="mb-2">
              The Sentinel Orchestrator uses a three-tier memory system for efficient context management:
            </p>
            <ul className="list-disc list-inside space-y-1 ml-4">
              <li>
                <strong className="text-cyan-electric">Short-Term:</strong> In-memory conversation history
                for fast access to recent messages
              </li>
              <li>
                <strong className="text-neon-green">Medium-Term:</strong> Sled database storing summarized
                conversations for persistence across restarts
              </li>
              <li>
                <strong className="text-warning-amber">Long-Term:</strong> Qdrant vector store with embeddings
                for semantic search and long-term context
              </li>
            </ul>
          </div>
          <div>
            <h4 className="text-h4 font-display text-rust-orange mb-2">Consolidation Process</h4>
            <p>
              When token count exceeds the threshold ({consolidationThreshold.toLocaleString()} tokens),
              the Dreamer background task automatically consolidates memory:
            </p>
            <ol className="list-decimal list-inside space-y-1 ml-4 mt-2">
              <li>Summarizes short-term messages via LLM</li>
              <li>Stores summary in medium-term memory (Sled)</li>
              <li>Generates embeddings for semantic search</li>
              <li>Stores in long-term memory (Qdrant)</li>
              <li>Clears short-term memory for new messages</li>
            </ol>
          </div>
        </div>
      </div>
    </div>
  );
}
