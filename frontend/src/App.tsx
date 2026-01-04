/**
 * App component - Main application component
 * Sets up routing and layout with lazy loading
 */

import { lazy, Suspense } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { Layout } from './components/layout/Layout';
import { ErrorBoundary } from './components/ErrorBoundary';
import { LoadingSpinner } from './components/ui';

// Lazy load views for code splitting
const Dashboard = lazy(() => import('./views/Dashboard').then((m) => ({ default: m.Dashboard })));
const Chat = lazy(() => import('./views/Chat').then((m) => ({ default: m.Chat })));
const Agents = lazy(() => import('./views/Agents').then((m) => ({ default: m.Agents })));
const Metrics = lazy(() => import('./views/Metrics').then((m) => ({ default: m.Metrics })));
const Memory = lazy(() => import('./views/Memory').then((m) => ({ default: m.Memory })));
const Config = lazy(() => import('./views/Config').then((m) => ({ default: m.Config })));
const Docs = lazy(() => import('./views/Docs').then((m) => ({ default: m.Docs })));
const CLI = lazy(() => import('./views/CLI').then((m) => ({ default: m.CLI })));
const Chaos = lazy(() => import('./views/Chaos').then((m) => ({ default: m.Chaos })));

function App() {
  return (
    <ErrorBoundary>
      <BrowserRouter>
        <Layout>
          <Suspense
            fallback={
              <div className="flex items-center justify-center min-h-[400px]">
                <LoadingSpinner />
              </div>
            }
          >
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/chat" element={<Chat />} />
              <Route path="/agents" element={<Agents />} />
              <Route path="/metrics" element={<Metrics />} />
              <Route path="/memory" element={<Memory />} />
              <Route path="/config" element={<Config />} />
              <Route path="/docs" element={<Docs />} />
              <Route path="/cli" element={<CLI />} />
              <Route path="/chaos" element={<Chaos />} />
            </Routes>
          </Suspense>
        </Layout>
      </BrowserRouter>
    </ErrorBoundary>
  );
}

export default App;
