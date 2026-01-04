/**
 * Header component - Top navigation bar
 * Features the Sentinel Orchestrator branding and navigation
 */

export function Header() {
  return (
    <header className="border-b border-cyan-electric/20 bg-dark-slate/80 backdrop-blur-sm sticky top-0 z-50">
      <div className="container mx-auto px-4 py-4 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <h1 className="text-2xl font-display font-bold text-rust-orange">
            Sentinel
          </h1>
          <span className="text-cyan-electric font-mono text-sm">Orchestrator</span>
        </div>
        <nav className="hidden md:flex items-center gap-6">
          <a
            href="/"
            className="text-light-gray hover:text-cyan-electric transition-colors"
          >
            Dashboard
          </a>
          <a
            href="/chat"
            className="text-light-gray hover:text-cyan-electric transition-colors"
          >
            Chat
          </a>
          <a
            href="/agents"
            className="text-light-gray hover:text-cyan-electric transition-colors"
          >
            Agents
          </a>
          <a
            href="/metrics"
            className="text-light-gray hover:text-cyan-electric transition-colors"
          >
            Metrics
          </a>
        </nav>
      </div>
    </header>
  );
}

