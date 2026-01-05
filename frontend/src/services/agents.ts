/**
 * Agents API service - Handles agent status requests
 * Uses centralized API client
 */

import type { AgentStatus } from '../types';
import { api } from './api';

/**
 * Get agent status (requires Read auth)
 */
export async function getAgentStatus(): Promise<AgentStatus[]> {
  if (!api.hasApiKey()) {
    throw new Error('API key required for agent status');
  }

  return api.get<AgentStatus[]>('/v1/agents/status');
}


