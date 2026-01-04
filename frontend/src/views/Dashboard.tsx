/**
 * Dashboard view - Main landing page
 * Displays system health overview, quick metrics, and recent activity
 */

export function Dashboard() {
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

      {/* Health Status Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">System Health</h3>
          <div className="flex items-center gap-2">
            <span className="status-dot status-active"></span>
            <span className="text-body">Healthy</span>
          </div>
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">Active Agents</h3>
          <p className="text-display-1 font-mono text-neon-green">0</p>
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">Messages Processed</h3>
          <p className="text-display-1 font-mono text-cyan-electric">0</p>
        </div>
      </div>

      {/* Quick Actions */}
      <div className="card">
        <h2 className="text-h3 font-display text-rust-orange mb-4">Quick Actions</h2>
        <div className="flex gap-4">
          <button className="btn btn-primary">Start Chat</button>
          <button className="btn btn-secondary">View Agents</button>
          <button className="btn btn-ghost">View Metrics</button>
        </div>
      </div>
    </div>
  );
}

