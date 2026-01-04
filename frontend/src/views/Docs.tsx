/**
 * Docs view - Documentation viewer
 * Placeholder for Phase 7 implementation
 */

export function Docs() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          Documentation
        </h1>
        <p className="text-light-gray text-body-lg">
          Links to Rust documentation and guides
        </p>
      </div>
      <div className="card">
        <div className="space-y-4">
          <div>
            <h3 className="text-h4 font-display text-cyan-electric mb-2">
              Backend Documentation
            </h3>
            <ul className="space-y-2 text-body">
              <li>
                <a
                  href="../docs/architecture.md"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-cyan-electric hover:text-rust-orange transition-colors"
                >
                  Architecture Documentation
                </a>
              </li>
              <li>
                <a
                  href="../docs/prd.md"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-cyan-electric hover:text-rust-orange transition-colors"
                >
                  Product Requirements Document
                </a>
              </li>
              <li>
                <a
                  href="../tasks/prd.md"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-cyan-electric hover:text-rust-orange transition-colors"
                >
                  Backend PRD
                </a>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}

