/**
 * App component - Main application component
 * Sets up routing and layout
 */

import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { Layout } from './components/layout/Layout';
import {
  Dashboard,
  Chat,
  Agents,
  Metrics,
  Memory,
  Config,
  Docs,
  CLI,
} from './views';

function App() {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/chat" element={<Chat />} />
          <Route path="/agents" element={<Agents />} />
          <Route path="/metrics" element={<Metrics />} />
          <Route path="/memory" element={<Memory />} />
          <Route path="/config" element={<Config />} />
          <Route path="/docs" element={<Docs />} />
          <Route path="/cli" element={<CLI />} />
        </Routes>
      </Layout>
    </BrowserRouter>
  );
}

export default App;
