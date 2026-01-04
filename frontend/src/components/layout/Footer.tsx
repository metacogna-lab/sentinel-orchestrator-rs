/**
 * Footer component - Application footer
 * Displays copyright and links to documentation
 */

export function Footer() {
  return (
    <footer className="border-t border-cyan-electric/20 bg-dark-slate/80 backdrop-blur-sm mt-auto">
      <div className="container mx-auto px-4 py-6">
        <div className="flex items-center justify-between">
          <div className="text-medium-gray text-sm">
            <p>© 2025 Sentinel Orchestrator</p>
            <p className="text-xs mt-1">Built with Rust & React</p>
          </div>
          <div className="flex gap-6">
            <a
              href="/docs"
              className="text-medium-gray hover:text-cyan-electric text-sm transition-colors"
            >
              Documentation
            </a>
            <a
              href="https://github.com"
              target="_blank"
              rel="noopener noreferrer"
              className="text-medium-gray hover:text-cyan-electric text-sm transition-colors"
            >
              GitHub
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}

