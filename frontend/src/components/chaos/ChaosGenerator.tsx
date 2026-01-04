/**
 * ChaosGenerator component - High-frequency chat completion request generator
 * Generates valid ChatCompletionRequest objects and sends them to the backend
 * Strictly adheres to backend types and interfaces
 */

import { useState, useEffect, useRef, useCallback } from 'react';
import type { CanonicalMessage, ChatCompletionRequest } from '../../types';
import { createChatCompletion } from '../../services/chat';
import { v4 as uuidv4 } from 'uuid';

interface ChaosStats {
  requestsSent: number;
  requestsSucceeded: number;
  requestsFailed: number;
  totalLatency: number;
  lastError: string | null;
  startTime: number | null;
}

const CHAOS_MESSAGES = [
  'Test message 1',
  'What is Rust?',
  'Explain async programming',
  'How does ownership work?',
  'What are traits in Rust?',
  'Explain the borrow checker',
  'What is pattern matching?',
  'How does error handling work?',
  'What are lifetimes?',
  'Explain memory safety',
];

/**
 * Generate a valid CanonicalMessage following backend contract exactly
 */
function generateCanonicalMessage(content: string, role: 'user' | 'assistant' | 'system' = 'user'): CanonicalMessage {
  return {
    id: uuidv4(),
    role,
    content: content.trim(),
    timestamp: new Date().toISOString(),
    metadata: {
      source: 'chaos-generator',
      generated_at: Date.now().toString(),
    },
  };
}

/**
 * Generate a valid ChatCompletionRequest following backend contract exactly
 */
function generateChatRequest(messageIndex: number): ChatCompletionRequest {
  const messageContent = CHAOS_MESSAGES[messageIndex % CHAOS_MESSAGES.length];
  const message = generateCanonicalMessage(messageContent, 'user');
  
  return {
    messages: [message],
    stream: false,
  };
}

export function ChaosGenerator() {
  const [isRunning, setIsRunning] = useState(false);
  const [frequency, setFrequency] = useState(1000); // milliseconds between requests
  const [stats, setStats] = useState<ChaosStats>({
    requestsSent: 0,
    requestsSucceeded: 0,
    requestsFailed: 0,
    totalLatency: 0,
    lastError: null,
    startTime: null,
  });
  
  const intervalRef = useRef<number | null>(null);
  const requestCounterRef = useRef(0);
  const abortControllerRef = useRef<AbortController | null>(null);

  const sendRequest = useCallback(async () => {
    const requestIndex = requestCounterRef.current++;
    const request = generateChatRequest(requestIndex);
    const startTime = performance.now();

    try {
      await createChatCompletion(request);
      const latency = performance.now() - startTime;
      
      setStats((prev) => ({
        ...prev,
        requestsSent: prev.requestsSent + 1,
        requestsSucceeded: prev.requestsSucceeded + 1,
        totalLatency: prev.totalLatency + latency,
        lastError: null,
      }));
    } catch (error) {
      const latency = performance.now() - startTime;
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      
      setStats((prev) => ({
        ...prev,
        requestsSent: prev.requestsSent + 1,
        requestsFailed: prev.requestsFailed + 1,
        totalLatency: prev.totalLatency + latency,
        lastError: errorMessage,
      }));
    }
  }, []);

  const startChaos = useCallback(() => {
    if (isRunning) return;
    
    setIsRunning(true);
    setStats((prev) => ({
      ...prev,
      startTime: Date.now(),
    }));
    requestCounterRef.current = 0;

    // Send requests at specified frequency
    intervalRef.current = window.setInterval(() => {
      sendRequest();
    }, frequency);
  }, [isRunning, frequency, sendRequest]);

  const stopChaos = useCallback(() => {
    if (!isRunning) return;
    
    setIsRunning(false);
    
    if (intervalRef.current !== null) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }

    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      abortControllerRef.current = null;
    }
  }, [isRunning]);

  useEffect(() => {
    return () => {
      if (intervalRef.current !== null) {
        clearInterval(intervalRef.current);
      }
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }
    };
  }, []);

  const resetStats = () => {
    setStats({
      requestsSent: 0,
      requestsSucceeded: 0,
      requestsFailed: 0,
      totalLatency: 0,
      lastError: null,
      startTime: null,
    });
    requestCounterRef.current = 0;
  };

  const averageLatency = stats.requestsSucceeded > 0 
    ? (stats.totalLatency / stats.requestsSucceeded).toFixed(2)
    : '0.00';

  const successRate = stats.requestsSent > 0
    ? ((stats.requestsSucceeded / stats.requestsSent) * 100).toFixed(2)
    : '0.00';

  const requestsPerSecond = stats.startTime && isRunning
    ? ((stats.requestsSent / ((Date.now() - stats.startTime) / 1000)) * 1000 / frequency).toFixed(2)
    : '0.00';

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-h2 font-display text-rust-orange mb-2">Chaos Generator</h2>
        <p className="text-light-gray text-body-lg">
          High-frequency chat completion request generator for backend stress testing
        </p>
      </div>

      {/* Controls */}
      <div className="card">
        <div className="flex items-center gap-4 mb-4">
          <div className="flex-1">
            <label htmlFor="frequency" className="block text-body-sm font-medium text-light-gray mb-2">
              Request Frequency (ms)
            </label>
            <input
              id="frequency"
              type="number"
              min="100"
              max="10000"
              step="100"
              value={frequency}
              onChange={(e) => setFrequency(Math.max(100, parseInt(e.target.value) || 1000))}
              disabled={isRunning}
              className="input w-32"
            />
          </div>
          <div className="flex gap-4">
            {!isRunning ? (
              <button onClick={startChaos} className="btn btn-primary">
                Start Chaos
              </button>
            ) : (
              <button onClick={stopChaos} className="btn btn-danger">
                Stop Chaos
              </button>
            )}
            <button onClick={resetStats} disabled={isRunning} className="btn btn-secondary">
              Reset Stats
            </button>
          </div>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">Requests Sent</h3>
          <p className="text-display-2 font-mono text-rust-orange">{stats.requestsSent}</p>
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-neon-green mb-2">Succeeded</h3>
          <p className="text-display-2 font-mono text-neon-green">{stats.requestsSucceeded}</p>
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-error-red mb-2">Failed</h3>
          <p className="text-display-2 font-mono text-error-red">{stats.requestsFailed}</p>
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">Success Rate</h3>
          <p className="text-display-2 font-mono text-cyan-electric">{successRate}%</p>
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-warning-amber mb-2">Avg Latency</h3>
          <p className="text-display-2 font-mono text-warning-amber">{averageLatency}ms</p>
        </div>
        <div className="card">
          <h3 className="text-h4 font-display text-cyan-electric mb-2">Req/sec</h3>
          <p className="text-display-2 font-mono text-cyan-electric">{requestsPerSecond}</p>
        </div>
      </div>

      {/* Last Error */}
      {stats.lastError && (
        <div className="card border-error-red/40 bg-error-red/10">
          <h3 className="text-h4 font-display text-error-red mb-2">Last Error</h3>
          <p className="text-body text-light-gray font-mono text-sm">{stats.lastError}</p>
        </div>
      )}

      {/* Status Indicator */}
      <div className="card">
        <div className="flex items-center gap-4">
          <div className={`status-dot ${isRunning ? 'status-active' : 'status-idle'}`} />
          <div>
            <h3 className="text-h4 font-display text-cyan-electric mb-1">
              Status: {isRunning ? 'Running' : 'Stopped'}
            </h3>
            <p className="text-body-sm text-medium-gray">
              {isRunning 
                ? `Sending requests every ${frequency}ms to /v1/chat/completions`
                : 'Chaos generator is stopped'}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

