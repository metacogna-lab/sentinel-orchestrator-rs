/**
 * Docs view - Documentation viewer and links
 * Links to Rust backend documentation and guides
 */

import { DocLink } from '../components/docs';

export function Docs() {
  const docLinks = [
    {
      title: 'Architecture Documentation',
      description: 'Complete system architecture overview, design principles, and module structure',
      href: '../docs/architecture.md',
      icon: '🏗️',
    },
    {
      title: 'Product Requirements Document',
      description: 'Full PRD with all phases, requirements, and acceptance criteria',
      href: '../docs/prd.md',
      icon: '📋',
    },
    {
      title: 'Backend PRD',
      description: 'Backend-specific PRD with implementation phases and technical details',
      href: '../tasks/prd.md',
      icon: '📝',
    },
    {
      title: 'API Documentation',
      description: 'Complete API reference with endpoints, request/response formats, and examples',
      href: '../docs/api.md',
      icon: '🔌',
    },
    {
      title: 'Rust Orchestrators Comparison',
      description: 'Research comparison of existing Rust orchestrator frameworks',
      href: '../docs/research/rust_orchestrators_comparison.md',
      icon: '🔬',
    },
    {
      title: 'Main Branch Sync Process',
      description: 'Guidelines for syncing feature branches with main branch',
      href: '../docs/MAIN_BRANCH_SYNC.md',
      icon: '🔄',
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          Documentation
        </h1>
        <p className="text-light-gray text-body-lg">
          Links to Rust backend documentation and guides
        </p>
      </div>

      {/* Documentation Links */}
      <div className="space-y-4">
        {docLinks.map((doc) => (
          <DocLink
            key={doc.title}
            title={doc.title}
            description={doc.description}
            href={doc.href}
            icon={doc.icon}
          />
        ))}
      </div>

      {/* Quick Reference */}
      <div className="card">
        <h2 className="text-h3 font-display text-cyan-electric mb-4">Quick Reference</h2>
        <div className="space-y-4 text-body text-light-gray">
          <div>
            <h3 className="text-h4 font-display text-rust-orange mb-2">Key Concepts</h3>
            <ul className="list-disc list-inside space-y-1 ml-4">
              <li>
                <strong className="text-cyan-electric">Hexagonal Architecture:</strong> Strict separation
                between domain logic and infrastructure
              </li>
              <li>
                <strong className="text-cyan-electric">Canonical Message Model:</strong> Pure domain types
                with no external dependencies
              </li>
              <li>
                <strong className="text-cyan-electric">Actor Model:</strong> Message-passing communication
                using channels
              </li>
              <li>
                <strong className="text-cyan-electric">Three-Tier Memory:</strong> Short-term → Medium-term
                → Long-term consolidation
              </li>
            </ul>
          </div>
          <div>
            <h3 className="text-h4 font-display text-rust-orange mb-2">Getting Started</h3>
            <ol className="list-decimal list-inside space-y-1 ml-4">
              <li>Read the Architecture Documentation for design principles</li>
              <li>Review the PRD for system requirements and phases</li>
              <li>Check the API Documentation for endpoint details</li>
              <li>Follow the Main Branch Sync Process for development workflow</li>
            </ol>
          </div>
        </div>
      </div>
    </div>
  );
}
