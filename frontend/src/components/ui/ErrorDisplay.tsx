/**
 * Error display component
 * Displays error messages with optional retry functionality
 */

interface ErrorDisplayProps {
  error: Error | string;
  onRetry?: () => void;
}

export function ErrorDisplay({ error, onRetry }: ErrorDisplayProps) {
  const errorMessage = typeof error === 'string' ? error : error.message;

  return (
    <div className="card border-error-red/40 bg-error-red/10">
      <div className="flex items-start gap-4">
        <div className="status-dot status-error mt-1" />
        <div className="flex-1">
          <h3 className="text-h4 font-display text-error-red mb-2">Error</h3>
          <p className="text-body text-light-gray mb-4">{errorMessage}</p>
          {onRetry && (
            <button onClick={onRetry} className="btn btn-secondary">
              Retry
            </button>
          )}
        </div>
      </div>
    </div>
  );
}

