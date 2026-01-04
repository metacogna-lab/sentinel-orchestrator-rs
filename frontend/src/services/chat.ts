/**
 * Chat API service - Handles chat completion requests
 * Simple API client for chat functionality
 */

import type { ChatCompletionRequest, ChatCompletionResponse, ErrorResponse } from '../types';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

/**
 * Get API key from localStorage
 */
function getApiKey(): string | null {
  return localStorage.getItem('sentinel_api_key');
}

/**
 * Create chat completion (non-streaming)
 */
export async function createChatCompletion(
  request: ChatCompletionRequest
): Promise<ChatCompletionResponse> {
  const apiKey = getApiKey();
  
  const response = await fetch(`${API_BASE_URL}/v1/chat/completions`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(apiKey && { Authorization: `Bearer ${apiKey}` }),
    },
    body: JSON.stringify(request),
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

/**
 * Create chat completion with streaming
 * Returns an async generator that yields message chunks
 */
export async function* createChatCompletionStream(
  request: ChatCompletionRequest
): AsyncGenerator<string, void, unknown> {
  const apiKey = getApiKey();
  
  const response = await fetch(`${API_BASE_URL}/v1/chat/completions`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(apiKey && { Authorization: `Bearer ${apiKey}` }),
    },
    body: JSON.stringify({ ...request, stream: true }),
  });

  if (!response.ok) {
    const error: ErrorResponse = await response.json().catch(() => ({
      code: 'unknown_error',
      message: `HTTP ${response.status}: ${response.statusText}`,
    }));
    throw new Error(error.message || `HTTP ${response.status}`);
  }

  const reader = response.body?.getReader();
  const decoder = new TextDecoder();

  if (!reader) {
    throw new Error('No response body');
  }

  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      const chunk = decoder.decode(value, { stream: true });
      const lines = chunk.split('\n').filter((line) => line.trim());

      for (const line of lines) {
        if (line.startsWith('data: ')) {
          const data = line.slice(6);
          if (data === '[DONE]') {
            return;
          }
          try {
            const parsed = JSON.parse(data);
            if (parsed.choices?.[0]?.delta?.content) {
              yield parsed.choices[0].delta.content;
            }
          } catch {
            // Skip invalid JSON
          }
        }
      }
    }
  } finally {
    reader.releaseLock();
  }
}

