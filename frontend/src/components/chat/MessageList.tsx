/**
 * MessageList component - Displays chat messages
 * Renders messages with markdown support and syntax highlighting
 */

import type { CanonicalMessage } from '../../types';
import { MessageItem } from './MessageItem';

interface MessageListProps {
  messages: CanonicalMessage[];
}

export function MessageList({ messages }: MessageListProps) {
  return (
    <div className="flex flex-col gap-4 pb-4">
      {messages.length === 0 ? (
        <div className="text-center py-12 text-medium-gray">
          <p className="text-body-lg">No messages yet</p>
          <p className="text-body-sm mt-2">Start a conversation to see messages here</p>
        </div>
      ) : (
        messages.map((message) => (
          <MessageItem key={message.id} message={message} />
        ))
      )}
    </div>
  );
}

