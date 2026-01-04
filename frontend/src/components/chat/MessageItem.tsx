/**
 * MessageItem component - Individual chat message
 * Renders message with markdown, role-based styling, and actions
 */

import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import type { CanonicalMessage } from '../../types';
import { formatDistanceToNow } from 'date-fns';
import clsx from 'clsx';

interface MessageItemProps {
  message: CanonicalMessage;
}

export function MessageItem({ message }: MessageItemProps) {
  const isUser = message.role === 'user';
  const isAssistant = message.role === 'assistant';
  const isSystem = message.role === 'system';

  return (
    <div
      className={clsx('rounded-lg p-4 border', {
        'bg-cyan-electric/10 border-cyan-electric/20 ml-auto max-w-[80%]': isUser,
        'bg-deep-navy border-cyan-electric/20 mr-auto max-w-[80%]': isAssistant,
        'bg-warning-amber/10 border-warning-amber/20 mx-auto max-w-[60%]': isSystem,
      })}
    >
      <div className="flex items-start justify-between gap-4 mb-2">
        <div className="flex items-center gap-2">
          <span
            className={clsx('text-body-sm font-medium', {
              'text-cyan-electric': isUser,
              'text-rust-orange': isAssistant,
              'text-warning-amber': isSystem,
            })}
          >
            {isUser ? 'You' : isAssistant ? 'Assistant' : 'System'}
          </span>
        </div>
        <span className="text-caption text-medium-gray">
          {formatDistanceToNow(new Date(message.timestamp), { addSuffix: true })}
        </span>
      </div>
      <div
        className={clsx('prose prose-invert max-w-none', {
          'prose-headings:text-cyan-electric': isUser,
          'prose-headings:text-rust-orange': isAssistant,
          'prose-headings:text-warning-amber': isSystem,
          'prose-code:text-cyan-electric': isUser,
          'prose-code:text-rust-orange': isAssistant,
          'prose-a:text-cyan-electric': isUser,
          'prose-a:text-rust-orange': isAssistant,
        })}
      >
        <ReactMarkdown
          remarkPlugins={[remarkGfm]}
          rehypePlugins={[rehypeHighlight]}
          components={{
            code({ className, children, ...props }: { className?: string; children?: React.ReactNode; [key: string]: any }) {
              const match = /language-(\w+)/.exec(className || '');
              const isInline = !match;
              return !isInline && match ? (
                <pre className="bg-deep-navy rounded-lg p-4 overflow-x-auto border border-cyan-electric/20">
                  <code className={className} {...props}>
                    {children}
                  </code>
                </pre>
              ) : (
                <code className="bg-deep-navy px-1.5 py-0.5 rounded text-cyan-electric font-mono text-sm" {...props}>
                  {children}
                </code>
              );
            },
          }}
        >
          {message.content}
        </ReactMarkdown>
      </div>
    </div>
  );
}

