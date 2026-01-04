/**
 * MessageInput component - Chat input field
 * Handles message input with submit functionality
 */

import { useState, type FormEvent } from 'react';

interface MessageInputProps {
  onSend: (message: string) => void;
  disabled?: boolean;
  placeholder?: string;
}

export function MessageInput({ onSend, disabled = false, placeholder = 'Type your message...' }: MessageInputProps) {
  const [input, setInput] = useState('');

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    if (input.trim() && !disabled) {
      onSend(input.trim());
      setInput('');
    }
  };

  return (
    <form onSubmit={handleSubmit} className="flex gap-4" aria-label="Message input form">
      <label htmlFor="message-input" className="sr-only">
        Message input
      </label>
      <input
        id="message-input"
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
        className="input flex-1"
        autoFocus
        aria-label="Type your message"
        aria-required="true"
      />
      <button
        type="submit"
        disabled={disabled || !input.trim()}
        className="btn btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
        aria-label="Send message"
      >
        Send
      </button>
    </form>
  );
}

