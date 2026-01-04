/**
 * TypeScript interfaces matching src/core/types.rs contracts exactly.
 * These are immutable contracts that define the API types.
 * All interfaces must match the Rust types exactly.
 */

/**
 * Unique identifier for a message (UUID string)
 */
export type MessageId = string;

/**
 * Unique identifier for an agent/actor (UUID string)
 */
export type AgentId = string;

/**
 * Role of a message participant
 */
export type Role = "user" | "assistant" | "system";

/**
 * Agent state in the state machine
 */
export type AgentState = "idle" | "thinking" | "toolcall" | "reflecting";

/**
 * Canonical message format - matches Rust CanonicalMessage exactly
 */
export interface CanonicalMessage {
  /** Unique identifier for this message */
  id: MessageId;
  /** Role of the message sender */
  role: Role;
  /** Message content */
  content: string;
  /** Timestamp when message was created (ISO 8601 string) */
  timestamp: string;
  /** Optional metadata (key-value pairs) */
  metadata?: Record<string, string>;
}

/**
 * Health state enum
 */
export type HealthState = "healthy" | "ready" | "alive" | "unhealthy";

/**
 * Health status response
 */
export interface HealthStatus {
  /** Health status */
  status: HealthState;
  /** Timestamp of the health check (ISO 8601 string) */
  timestamp: string;
}

/**
 * Chat completion request (API contract)
 */
export interface ChatCompletionRequest {
  /** List of messages in the conversation */
  messages: readonly CanonicalMessage[];
  /** Model to use (optional, defaults to configured model) */
  model?: string;
  /** Temperature for sampling (0.0 to 2.0) */
  temperature?: number;
  /** Maximum tokens to generate */
  max_tokens?: number;
  /** Stream responses */
  stream?: boolean;
}

/**
 * Token usage information
 */
export interface TokenUsage {
  /** Number of tokens in the prompt */
  prompt_tokens: number;
  /** Number of tokens in the completion */
  completion_tokens: number;
  /** Total tokens used */
  total_tokens: number;
}

/**
 * Chat completion response (API contract)
 */
export interface ChatCompletionResponse {
  /** Generated message */
  message: CanonicalMessage;
  /** Model used for generation */
  model: string;
  /** Number of tokens used */
  usage?: TokenUsage;
}

/**
 * Agent status information
 */
export interface AgentStatus {
  /** Agent identifier */
  id: AgentId;
  /** Current state */
  state: AgentState;
  /** Last activity timestamp (ISO 8601 string) */
  last_activity: string;
  /** Number of messages processed */
  messages_processed: number;
}

/**
 * Error response format (API contract)
 */
export interface ErrorResponse {
  /** Error code */
  code: string;
  /** Error message */
  message: string;
  /** Optional details */
  details?: Record<string, string>;
}

