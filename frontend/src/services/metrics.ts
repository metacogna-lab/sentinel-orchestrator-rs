/**
 * Metrics API service - Handles metrics requests
 * Simple API client for metrics functionality
 */

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

/**
 * Get metrics (Prometheus format)
 * Note: This endpoint may not be implemented yet
 */
export async function getMetrics(): Promise<string> {
  const response = await fetch(`${API_BASE_URL}/metrics`, {
    method: 'GET',
    headers: {
      'Content-Type': 'text/plain',
    },
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
  }

  return response.text();
}

/**
 * Parse Prometheus metrics format
 * Returns a map of metric names to values
 */
export function parsePrometheusMetrics(metricsText: string): Map<string, number> {
  const metrics = new Map<string, number>();
  const lines = metricsText.split('\n').filter(line => line.trim() && !line.startsWith('#'));

  for (const line of lines) {
    const parts = line.split(/\s+/);
    if (parts.length >= 2) {
      const name = parts[0];
      const value = parseFloat(parts[1]);
      if (!isNaN(value)) {
        metrics.set(name, value);
      }
    }
  }

  return metrics;
}

