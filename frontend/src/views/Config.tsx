/**
 * Config view - System configuration
 * API key management and backend URL configuration
 */

import { useState, useEffect } from 'react';
import { useAuth } from '../store/auth';
import { api } from '../services/api';
import { LoadingSpinner } from '../components/ui';

type ConnectionStatus = 'idle' | 'testing' | 'connected' | 'error';

export function Config() {
  const { apiKey, setApiKey, clearApiKey, hasApiKey } = useAuth();
  const [backendUrl, setBackendUrl] = useState(api.getBaseUrl());
  const [apiKeyInput, setApiKeyInput] = useState('');
  const [showApiKey, setShowApiKey] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<ConnectionStatus>('idle');
  const [connectionError, setConnectionError] = useState<string | null>(null);
  const [saveMessage, setSaveMessage] = useState<string | null>(null);

  // Load current backend URL on mount
  useEffect(() => {
    setBackendUrl(api.getBaseUrl());
  }, []);

  // Pre-fill API key input if API key exists (masked)
  useEffect(() => {
    if (apiKey) {
      setApiKeyInput('•'.repeat(20)); // Mask existing API key
    }
  }, [apiKey]);

  /**
   * Validate API key format (basic validation)
   */
  const validateApiKey = (key: string): boolean => {
    const trimmed = key.trim();
    return trimmed.length >= 8; // Minimum length check
  };

  /**
   * Handle API key save
   */
  const handleSaveApiKey = () => {
    const trimmed = apiKeyInput.trim();
    
    if (!trimmed) {
      setSaveMessage('API key cannot be empty');
      setTimeout(() => setSaveMessage(null), 3000);
      return;
    }

    // If input is masked, don't update
    if (trimmed.startsWith('•')) {
      setSaveMessage('API key already set. Clear it first to set a new one.');
      setTimeout(() => setSaveMessage(null), 3000);
      return;
    }

    if (!validateApiKey(trimmed)) {
      setSaveMessage('API key must be at least 8 characters long');
      setTimeout(() => setSaveMessage(null), 3000);
      return;
    }

    setApiKey(trimmed);
    setApiKeyInput('•'.repeat(20)); // Mask after saving
    setSaveMessage('API key saved successfully');
    setTimeout(() => setSaveMessage(null), 3000);
  };

  /**
   * Handle API key clear
   */
  const handleClearApiKey = () => {
    clearApiKey();
    setApiKeyInput('');
    setSaveMessage('API key cleared');
    setTimeout(() => setSaveMessage(null), 3000);
  };

  /**
   * Handle backend URL save
   */
  const handleSaveBackendUrl = () => {
    const trimmed = backendUrl.trim();
    
    if (!trimmed) {
      setSaveMessage('Backend URL cannot be empty');
      setTimeout(() => setSaveMessage(null), 3000);
      return;
    }

    // Basic URL validation
    try {
      new URL(trimmed);
      api.setBaseUrl(trimmed);
      setSaveMessage('Backend URL saved successfully');
      setTimeout(() => setSaveMessage(null), 3000);
    } catch {
      setSaveMessage('Invalid URL format');
      setTimeout(() => setSaveMessage(null), 3000);
    }
  };

  /**
   * Test connection to backend
   */
  const handleTestConnection = async () => {
    setConnectionStatus('testing');
    setConnectionError(null);

    try {
      // Test with health endpoint (public, no auth required)
      const response = await fetch(`${backendUrl}/health`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        setConnectionStatus('connected');
        setConnectionError(null);
      } else {
        setConnectionStatus('error');
        setConnectionError(`HTTP ${response.status}: ${response.statusText}`);
      }
    } catch (error) {
      setConnectionStatus('error');
      setConnectionError(
        error instanceof Error ? error.message : 'Failed to connect to backend'
      );
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          Configuration
        </h1>
        <p className="text-light-gray text-body-lg">
          System configuration and API key management
        </p>
      </div>

      {/* API Key Management */}
      <div className="card">
        <h2 className="text-h3 font-display text-rust-orange mb-4">API Key Management</h2>
        
        <div className="space-y-4">
          <div>
            <label htmlFor="api-key-input" className="block text-body font-medium text-light-gray mb-2">
              API Key
            </label>
            <div className="flex gap-2">
              <div className="flex-1 relative">
                <input
                  id="api-key-input"
                  type={showApiKey ? 'text' : 'password'}
                  value={apiKeyInput}
                  onChange={(e) => {
                    // Don't allow editing if masked
                    if (!apiKeyInput.startsWith('•')) {
                      setApiKeyInput(e.target.value);
                    }
                  }}
                  placeholder="Enter your API key"
                  className="input w-full pr-10"
                  aria-label="API key input"
                />
                <button
                  type="button"
                  onClick={() => setShowApiKey(!showApiKey)}
                  className="absolute right-2 top-1/2 -translate-y-1/2 text-cyan-electric hover:text-rust-orange transition-colors"
                  aria-label={showApiKey ? 'Hide API key' : 'Show API key'}
                >
                  {showApiKey ? '👁️' : '👁️‍🗨️'}
                </button>
              </div>
              <button
                onClick={handleSaveApiKey}
                className="btn btn-primary"
                disabled={apiKeyInput.startsWith('•')}
              >
                Save
              </button>
              {hasApiKey && (
                <button
                  onClick={handleClearApiKey}
                  className="btn btn-danger"
                >
                  Clear
                </button>
              )}
            </div>
            {hasApiKey && (
              <p className="text-caption text-medium-gray mt-1">
                API key is set {apiKeyInput.startsWith('•') && '(masked)'}
              </p>
            )}
          </div>

          {saveMessage && (
            <div
              className={`p-3 rounded-lg ${
                saveMessage.includes('successfully')
                  ? 'bg-neon-green/20 border border-neon-green/40 text-neon-green'
                  : 'bg-error-red/20 border border-error-red/40 text-error-red'
              }`}
            >
              {saveMessage}
            </div>
          )}
        </div>
      </div>

      {/* Backend URL Configuration */}
      <div className="card">
        <h2 className="text-h3 font-display text-rust-orange mb-4">Backend Configuration</h2>
        
        <div className="space-y-4">
          <div>
            <label htmlFor="backend-url-input" className="block text-body font-medium text-light-gray mb-2">
              Backend URL
            </label>
            <div className="flex gap-2">
              <input
                id="backend-url-input"
                type="url"
                value={backendUrl}
                onChange={(e) => setBackendUrl(e.target.value)}
                placeholder="http://localhost:3000"
                className="input flex-1"
                aria-label="Backend URL input"
              />
              <button
                onClick={handleSaveBackendUrl}
                className="btn btn-primary"
              >
                Save
              </button>
            </div>
            <p className="text-caption text-medium-gray mt-1">
              Current backend URL: {api.getBaseUrl()}
            </p>
          </div>

          {/* Connection Test */}
          <div>
            <button
              onClick={handleTestConnection}
              disabled={connectionStatus === 'testing'}
              className="btn btn-secondary"
            >
              {connectionStatus === 'testing' ? (
                <>
                  <LoadingSpinner />
                  <span className="ml-2">Testing...</span>
                </>
              ) : (
                'Test Connection'
              )}
            </button>

            {connectionStatus === 'connected' && (
              <div className="mt-3 p-3 rounded-lg bg-neon-green/20 border border-neon-green/40">
                <div className="flex items-center gap-2 text-neon-green">
                  <span className="status-dot status-active"></span>
                  <span>Connected successfully</span>
                </div>
              </div>
            )}

            {connectionStatus === 'error' && connectionError && (
              <div className="mt-3 p-3 rounded-lg bg-error-red/20 border border-error-red/40">
                <div className="flex items-center gap-2 text-error-red">
                  <span className="status-dot status-error"></span>
                  <span>Connection failed: {connectionError}</span>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Configuration Info */}
      <div className="card">
        <h2 className="text-h3 font-display text-cyan-electric mb-4">Configuration Info</h2>
        <div className="space-y-2 text-body text-light-gray">
          <p>
            <strong className="text-cyan-electric">API Key:</strong> Used for authenticating requests to protected endpoints.
          </p>
          <p>
            <strong className="text-cyan-electric">Backend URL:</strong> The base URL of the Sentinel Orchestrator API.
          </p>
          <p className="text-caption text-medium-gray mt-4">
            All settings are stored locally in your browser and persist across sessions.
          </p>
        </div>
      </div>
    </div>
  );
}
