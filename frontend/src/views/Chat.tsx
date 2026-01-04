/**
 * Chat view - Interactive chat interface
 * Full chat interface with streaming support
 */

import { useState, useCallback, useRef, useEffect } from 'react';
import type { CanonicalMessage } from '../types';
import { MessageList, MessageInput } from '../components/chat';
import { createChatCompletion, createChatCompletionStream } from '../services/chat';
import { v4 as uuidv4 } from 'uuid';

export function Chat() {
  const [messages, setMessages] = useState<CanonicalMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [streaming, setStreaming] = useState(false);
  const [currentStreamingMessage, setCurrentStreamingMessage] = useState<string>('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, currentStreamingMessage]);

  const handleSend = useCallback(async (content: string) => {
    if (!content.trim() || isLoading) return;

    setError(null);
    
    // Add user message
    const userMessage: CanonicalMessage = {
      id: uuidv4(),
      role: 'user',
      content,
      timestamp: new Date().toISOString(),
    };

    const newMessages = [...messages, userMessage];
    setMessages(newMessages);
    setIsLoading(true);
    setStreaming(true);
    setCurrentStreamingMessage('');

    try {
      // Create assistant message for streaming
      const assistantMessageId = uuidv4();
      let streamedContent = '';

      // Try streaming first
      try {
        const stream = createChatCompletionStream({
          messages: newMessages,
          stream: true,
        });

        for await (const chunk of stream) {
          streamedContent += chunk;
          setCurrentStreamingMessage(streamedContent);
        }

        // Add completed streaming message
        const assistantMessage: CanonicalMessage = {
          id: assistantMessageId,
          role: 'assistant',
          content: streamedContent,
          timestamp: new Date().toISOString(),
        };
        setMessages([...newMessages, assistantMessage]);
        } catch {
        // Fallback to non-streaming
        const response = await createChatCompletion({
          messages: newMessages,
          stream: false,
        });
        setMessages([...newMessages, response.message]);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to send message');
      console.error('Chat error:', err);
    } finally {
      setIsLoading(false);
      setStreaming(false);
      setCurrentStreamingMessage('');
    }
  }, [messages, isLoading]);

  return (
    <div className="flex flex-col h-[calc(100vh-200px)]">
      <div className="mb-6">
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          Chat
        </h1>
        <p className="text-light-gray text-body-lg">
          Interactive chat interface with streaming responses
        </p>
      </div>

      {error && (
        <div className="card border-error-red/40 bg-error-red/10 mb-4">
          <p className="text-body text-error-red">{error}</p>
        </div>
      )}

      <div className="card flex-1 flex flex-col overflow-hidden">
        <div className="flex-1 overflow-y-auto pr-2">
          <MessageList messages={messages} />
          {streaming && currentStreamingMessage && (
            <div className="bg-deep-navy border-cyan-electric/20 rounded-lg p-4 mr-auto max-w-[80%]">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-body-sm font-medium text-rust-orange">Assistant</span>
                <span className="status-dot status-thinking" />
              </div>
              <div className="prose prose-invert max-w-none">
                <p className="text-body text-light-gray whitespace-pre-wrap">
                  {currentStreamingMessage}
                </p>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        <div className="mt-4 pt-4 border-t border-cyan-electric/20">
          <MessageInput onSend={handleSend} disabled={isLoading} />
        </div>
      </div>
    </div>
  );
}
