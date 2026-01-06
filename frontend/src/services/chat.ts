/**
 * Chat API service - Handles chat completion requests
 * Uses centralized API client
 */

import type { ChatCompletionRequest, ChatCompletionResponse } from '../types';
import { api } from './api';

/**
 * Create chat completion (non-streaming)
 */
export async function createChatCompletion(
  request: ChatCompletionRequest
): Promise<ChatCompletionResponse> {
  return api.post<ChatCompletionResponse>('/v1/chat/completions', request);
}

/**
 * Create chat completion with streaming
 * Returns an async generator that yields message chunks
 * Note: Streaming uses fetch directly as it requires special handling
 */
export async function* createChatCompletionStream(
  request: ChatCompletionRequest
): AsyncGenerator<string, void, unknown> {
  const baseURL = api.getBaseUrl();
  const apiKey = localStorage.getItem('sentinel_api_key');
  
  const response = await fetch(`${baseURL}/v1/chat/completions`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(apiKey && { Authorization: `Bearer ${apiKey}` }),
    },
    body: JSON.stringify({ ...request, stream: true }),
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({
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

