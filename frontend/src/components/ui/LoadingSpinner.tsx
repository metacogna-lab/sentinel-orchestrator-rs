/**
 * Loading spinner component
 * Displays a loading spinner with optional text
 */

interface LoadingSpinnerProps {
  text?: string;
  size?: 'sm' | 'md' | 'lg';
}

export function LoadingSpinner({ text, size = 'md' }: LoadingSpinnerProps) {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
  };

  return (
    <div className="flex flex-col items-center justify-center gap-4 p-8">
      <div
        className={`${sizeClasses[size]} border-4 border-cyan-electric/20 border-t-cyan-electric rounded-full animate-spin`}
      />
      {text && <p className="text-medium-gray text-body">{text}</p>}
    </div>
  );
}

