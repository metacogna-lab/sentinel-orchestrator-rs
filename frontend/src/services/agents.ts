/**
 * Agents API service - Handles agent status requests
 * Simple API client for agent functionality
 */

import type { AgentStatus, ErrorResponse } from '../types';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

/**
 * Get API key from localStorage
 */
function getApiKey(): string | null {
  return localStorage.getItem('sentinel_api_key');
}

/**
 * Get agent status (requires Read auth)
 */
export async function getAgentStatus(): Promise<AgentStatus[]> {
  const apiKey = getApiKey();
  
  if (!apiKey) {
    throw new Error('API key required for agent status');
  }

  const response = await fetch(`${API_BASE_URL}/v1/agents/status`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${apiKey}`,
    },
  });

  if (!response.ok) {
    const error: ErrorResponse = await response.json().catch(() => ({
      code: 'unknown_error',
      message: `HTTP ${response.status}: ${response.statusText}`,
    }));
    throw new Error(error.message || `HTTP ${response.status}`);
  }

  return response.json();
}

