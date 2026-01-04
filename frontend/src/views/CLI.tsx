/**
 * CLI view - rs_cli integration and command reference
 * Information about the Rust CLI tool and integration
 */

import { DocLink } from '../components/docs';

export function CLI() {
  const cliCommands = [
    {
      command: 'cd rs_cli && cargo run --release',
      description: 'Run the CLI tool (default backend URL: http://localhost:3000)',
    },
    {
      command: 'cargo run --release -- --url http://localhost:8080',
      description: 'Run with custom backend URL',
    },
    {
      command: 'cargo run --release -- --api-key sk-your-key-here',
      description: 'Run with API key authentication',
    },
    {
      command: 'export SENTINEL_API_KEY=sk-your-key-here && cargo run --release',
      description: 'Run with API key from environment variable',
    },
  ];

  const cliFeatures = [
    {
      title: 'Chat Mode',
      description: 'Interactive chat interface with real-time streaming responses',
      icon: '💬',
    },
    {
      title: 'Investigation Mode',
      description: 'Query and investigate system state, search through memory and logs',
      icon: '🔍',
    },
    {
      title: 'Debugging Mode',
      description: 'View system debug logs with color-coded log levels',
      icon: '🐛',
    },
    {
      title: 'System Status Mode',
      description: 'Health check monitoring, readiness and liveness status',
      icon: '📊',
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          CLI Integration
        </h1>
        <p className="text-light-gray text-body-lg">
          rs_cli integration and command reference
        </p>
      </div>

      {/* CLI Overview */}
      <div className="card border-cyan-electric/40">
        <div className="flex items-start gap-4">
          <span className="text-4xl">⌨️</span>
          <div className="flex-1">
            <h2 className="text-h3 font-display text-cyan-electric mb-2">
              Sentinel Orchestrator CLI
            </h2>
            <p className="text-body text-light-gray mb-4">
              The rs_cli is a beautiful, interactive command-line interface for managing and
              interacting with the Sentinel Orchestrator backend. Built with Rust using
              best-in-class TUI libraries, featuring full keyboard navigation, real-time
              streaming, and multiple operational modes.
            </p>
            <DocLink
              title="rs_cli README"
              description="Complete CLI documentation and usage guide"
              href="../rs_cli/README.md"
              icon="📖"
            />
          </div>
        </div>
      </div>

      {/* CLI Features */}
      <div className="card">
        <h2 className="text-h3 font-display text-cyan-electric mb-4">CLI Features</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {cliFeatures.map((feature) => (
            <div
              key={feature.title}
              className="bg-deep-navy rounded-lg p-4 border border-cyan-electric/20"
            >
              <div className="flex items-start gap-3">
                <span className="text-2xl">{feature.icon}</span>
                <div>
                  <h3 className="text-h4 font-display text-cyan-electric mb-1">
                    {feature.title}
                  </h3>
                  <p className="text-body-sm text-light-gray">{feature.description}</p>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Command Reference */}
      <div className="card">
        <h2 className="text-h3 font-display text-cyan-electric mb-4">Command Reference</h2>
        <div className="space-y-4">
          {cliCommands.map((cmd, idx) => (
            <div
              key={idx}
              className="bg-deep-navy rounded-lg p-4 border border-cyan-electric/20"
            >
              <div className="flex items-start gap-4">
                <code className="flex-1 font-mono text-body-sm text-cyan-electric bg-dark-slate px-4 py-2 rounded border border-cyan-electric/20 break-all">
                  {cmd.command}
                </code>
              </div>
              <p className="text-body-sm text-light-gray mt-2">{cmd.description}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Keyboard Navigation */}
      <div className="card">
        <h2 className="text-h3 font-display text-cyan-electric mb-4">Keyboard Navigation</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="space-y-2 text-body-sm text-light-gray">
            <div className="flex items-center justify-between">
              <span>Tab</span>
              <span className="text-cyan-electric font-mono">Cycle through modes</span>
            </div>
            <div className="flex items-center justify-between">
              <span>↑/↓</span>
              <span className="text-cyan-electric font-mono">Navigate menu items</span>
            </div>
            <div className="flex items-center justify-between">
              <span>Enter</span>
              <span className="text-cyan-electric font-mono">Select or send</span>
            </div>
            <div className="flex items-center justify-between">
              <span>Esc</span>
              <span className="text-cyan-electric font-mono">Go back or exit</span>
            </div>
            <div className="flex items-center justify-between">
              <span>q</span>
              <span className="text-cyan-electric font-mono">Quit application</span>
            </div>
          </div>
          <div className="bg-deep-navy rounded-lg p-4 border border-cyan-electric/20">
            <h3 className="text-h4 font-display text-cyan-electric mb-2">Usage Tips</h3>
            <ul className="list-disc list-inside space-y-1 text-body-sm text-light-gray">
              <li>All modes support keyboard navigation</li>
              <li>Real-time streaming in Chat mode</li>
              <li>Color-coded messages and logs</li>
              <li>Auto-refresh in System Status mode</li>
              <li>Multiple operational modes available</li>
            </ul>
          </div>
        </div>
      </div>

      {/* Integration Guide */}
      <div className="card">
        <h2 className="text-h3 font-display text-cyan-electric mb-4">Integration Guide</h2>
        <div className="space-y-4 text-body text-light-gray">
          <div>
            <h3 className="text-h4 font-display text-rust-orange mb-2">Prerequisites</h3>
            <ul className="list-disc list-inside space-y-1 ml-4">
              <li>Rust toolchain installed (1.70+)</li>
              <li>Backend API running (default: http://localhost:3000)</li>
              <li>API key for authenticated endpoints (optional)</li>
            </ul>
          </div>
          <div>
            <h3 className="text-h4 font-display text-rust-orange mb-2">Quick Start</h3>
            <ol className="list-decimal list-inside space-y-1 ml-4">
              <li>Navigate to the rs_cli directory: <code className="text-cyan-electric">cd rs_cli</code></li>
              <li>Build the CLI: <code className="text-cyan-electric">cargo build --release</code></li>
              <li>Run the CLI: <code className="text-cyan-electric">cargo run --release</code></li>
              <li>Use Tab to navigate between modes</li>
              <li>Press 'q' to quit</li>
            </ol>
          </div>
          <div>
            <h3 className="text-h4 font-display text-rust-orange mb-2">Authentication</h3>
            <p className="mb-2">
              The CLI supports API key authentication via command-line flag or environment variable:
            </p>
            <ul className="list-disc list-inside space-y-1 ml-4">
              <li>
                Command-line: <code className="text-cyan-electric">--api-key sk-your-key-here</code>
              </li>
              <li>
                Environment: <code className="text-cyan-electric">SENTINEL_API_KEY=sk-your-key-here</code>
              </li>
              <li>Uses <code className="text-cyan-electric">Authorization: Bearer</code> header format</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
