/**
 * Health API service - Handles health check requests
 * Provides real-time system status monitoring
 */

import type { HealthStatus } from '../types';
import { api } from './api';

/**
 * Get system health status
 */
export async function getHealthStatus(): Promise<HealthStatus> {
  return api.get<HealthStatus>('/health');
}

/**
 * Get readiness status
 */
export async function getReadinessStatus(): Promise<HealthStatus> {
  return api.get<HealthStatus>('/health/ready');
}

/**
 * Get liveness status
 */
export async function getLivenessStatus(): Promise<HealthStatus> {
  return api.get<HealthStatus>('/health/live');
}

/**
 * Check if backend is reachable (no auth required)
 */
export async function checkBackendConnection(baseUrl?: string): Promise<boolean> {
  try {
    const url = baseUrl || api.getBaseUrl();
    const response = await fetch(`${url}/health`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    return response.ok;
  } catch {
    return false;
  }
}