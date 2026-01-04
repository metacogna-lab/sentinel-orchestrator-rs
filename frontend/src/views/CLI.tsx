/**
 * CLI view - rs_cli integration
 * Placeholder for Phase 7 implementation
 */

export function CLI() {
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
      <div className="card">
        <div className="space-y-4">
          <div>
            <h3 className="text-h4 font-display text-cyan-electric mb-2">
              rs_cli Command Reference
            </h3>
            <p className="text-body text-light-gray mb-4">
              The rs_cli is a beautiful, interactive command-line interface for managing and interacting with the Sentinel backend.
            </p>
            <div className="bg-deep-navy rounded-lg p-4 font-mono text-body-sm">
              <code className="text-cyan-electric">cd rs_cli</code>
              <br />
              <code className="text-cyan-electric">bun run --release</code>
            </div>
          </div>
          <div>
            <h3 className="text-h4 font-display text-cyan-electric mb-2">
              Features
            </h3>
            <ul className="list-disc list-inside space-y-2 text-body text-light-gray">
              <li>Interactive chat interface</li>
              <li>Real-time streaming responses</li>
              <li>Agent status monitoring</li>
              <li>System health checks</li>
              <li>Multiple operational modes</li>
            </ul>
          </div>
          <div>
            <a
              href="../rs_cli/README.md"
              target="_blank"
              rel="noopener noreferrer"
              className="btn btn-secondary"
            >
              View rs_cli README
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}

