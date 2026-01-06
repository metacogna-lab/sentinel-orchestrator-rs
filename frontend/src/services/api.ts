/**
 * API Client - Centralized API client with authentication and error handling
 * Provides a unified interface for all API calls to the backend
 */

import type { ErrorResponse } from '../types';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

/**
 * Get API key from localStorage
 */
function getApiKey(): string | null {
  return localStorage.getItem('sentinel_api_key');
}

/**
 * Get backend URL from localStorage or environment
 */
function getBackendUrl(): string {
  return localStorage.getItem('sentinel_backend_url') || API_BASE_URL;
}

/**
 * Custom API error class
 */
export class ApiError extends Error {
  status: number;
  code?: string;
  response?: unknown;

  constructor(message: string, status: number, code?: string, response?: unknown) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.code = code;
    this.response = response;
  }
}

/**
 * API client configuration
 */
interface ApiConfig {
  baseURL?: string;
  timeout?: number;
  retries?: number;
  retryDelay?: number;
}

/**
 * Default API configuration
 */
const defaultConfig: Required<ApiConfig> = {
  baseURL: API_BASE_URL,
  timeout: 30000, // 30 seconds
  retries: 3,
  retryDelay: 1000, // 1 second
};

/**
 * Check if error is retryable
 */
function isRetryableError(error: unknown): boolean {
  if (error instanceof ApiError) {
    // Retry on 5xx errors and network errors
    return error.status >= 500 || error.status === 0;
  }
  return false;
}

/**
 * Sleep utility for retry delays
 */
function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Make an API request with retry logic
 */
async function request<T>(
  url: string,
  options: RequestInit,
  config: ApiConfig = {}
): Promise<T> {
  const finalConfig = { ...defaultConfig, ...config };
  const baseURL = config.baseURL || getBackendUrl();
  const fullUrl = url.startsWith('http') ? url : `${baseURL}${url}`;
  
  const apiKey = getApiKey();
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...(apiKey && { Authorization: `Bearer ${apiKey}` }),
    ...options.headers,
  };

  let lastError: unknown;
  
  for (let attempt = 0; attempt <= finalConfig.retries; attempt++) {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), finalConfig.timeout);

      const response = await fetch(fullUrl, {
        ...options,
        headers,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        let errorData: ErrorResponse;
        try {
          errorData = await response.json();
        } catch {
          errorData = {
            code: 'unknown_error',
            message: `HTTP ${response.status}: ${response.statusText}`,
          };
        }

        const error = new ApiError(
          errorData.message || `HTTP ${response.status}`,
          response.status,
          errorData.code,
          errorData
        );

        // Don't retry on 4xx errors (client errors)
        if (response.status < 500) {
          throw error;
        }

        lastError = error;
      } else {
        // Handle empty responses
        const contentType = response.headers.get('content-type');
        if (contentType?.includes('application/json')) {
          return await response.json();
        }
        return {} as T;
      }
    } catch (error) {
      if (error instanceof ApiError) {
        lastError = error;
      } else if (error instanceof Error && error.name === 'AbortError') {
        throw new ApiError('Request timeout', 0, 'timeout_error');
      } else {
        lastError = new ApiError(
          error instanceof Error ? error.message : 'Network error',
          0,
          'network_error'
        );
      }

      // Don't retry on last attempt
      if (attempt < finalConfig.retries && isRetryableError(lastError)) {
        await sleep(finalConfig.retryDelay * (attempt + 1)); // Exponential backoff
        continue;
      }

      throw lastError;
    }
  }

  throw lastError;
}

/**
 * API client methods
 */
export const api = {
  /**
   * GET request
   */
  async get<T>(url: string, config?: ApiConfig): Promise<T> {
    return request<T>(url, { method: 'GET' }, config);
  },

  /**
   * POST request
   */
  async post<T>(url: string, data?: unknown, config?: ApiConfig): Promise<T> {
    return request<T>(
      url,
      {
        method: 'POST',
        body: data ? JSON.stringify(data) : undefined,
      },
      config
    );
  },

  /**
   * PUT request
   */
  async put<T>(url: string, data?: unknown, config?: ApiConfig): Promise<T> {
    return request<T>(
      url,
      {
        method: 'PUT',
        body: data ? JSON.stringify(data) : undefined,
      },
      config
    );
  },

  /**
   * DELETE request
   */
  async delete<T>(url: string, config?: ApiConfig): Promise<T> {
    return request<T>(url, { method: 'DELETE' }, config);
  },

  /**
   * Get base URL
   */
  getBaseUrl(): string {
    return getBackendUrl();
  },

  /**
   * Set base URL
   */
  setBaseUrl(url: string): void {
    localStorage.setItem('sentinel_backend_url', url);
  },

  /**
   * Check if API key is set
   */
  hasApiKey(): boolean {
    return getApiKey() !== null;
  },
};

