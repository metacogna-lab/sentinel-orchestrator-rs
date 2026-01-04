/**
 * Sidebar component - Navigation sidebar
 * Provides navigation links to all main views
 */

import { Link, useLocation } from 'react-router-dom';
import clsx from 'clsx';

const navigation = [
  { name: 'Dashboard', href: '/', icon: '📊' },
  { name: 'Chat', href: '/chat', icon: '💬' },
  { name: 'Agents', href: '/agents', icon: '🤖' },
  { name: 'Metrics', href: '/metrics', icon: '📈' },
  { name: 'Memory', href: '/memory', icon: '🧠' },
  { name: 'Config', href: '/config', icon: '⚙️' },
  { name: 'Docs', href: '/docs', icon: '📚' },
  { name: 'CLI', href: '/cli', icon: '⌨️' },
];

export function Sidebar() {
  const location = useLocation();

  return (
    <aside className="w-64 border-r border-cyan-electric/20 bg-deep-navy/50 min-h-screen">
      <div className="p-4 border-b border-cyan-electric/20">
        <h2 className="text-xl font-display text-rust-orange">Sentinel</h2>
        <p className="text-sm text-cyan-electric font-mono">Orchestrator</p>
      </div>
      <nav className="p-4 space-y-2" aria-label="Main navigation">
        {navigation.map((item) => {
          const isActive = location.pathname === item.href;
          return (
            <Link
              key={item.name}
              to={item.href}
              className={clsx(
                'flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200',
                isActive
                  ? 'bg-cyan-electric/20 text-cyan-electric border border-cyan-electric/40 shadow-glow-cyan'
                  : 'text-light-gray hover:text-cyan-electric hover:bg-cyan-electric/10'
              )}
              aria-current={isActive ? 'page' : undefined}
            >
              <span className="text-xl" aria-hidden="true">{item.icon}</span>
              <span className="font-medium">{item.name}</span>
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}

