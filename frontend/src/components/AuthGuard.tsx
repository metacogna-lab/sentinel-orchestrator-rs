/**
 * AuthGuard component - Protects routes that require authentication
 * Redirects to config page if API key is not set
 */

import type { ReactNode } from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import { useAuth } from '../store/auth';

interface AuthGuardProps {
  children: ReactNode;
  requireAuth?: boolean;
}

export function AuthGuard({ children, requireAuth = true }: AuthGuardProps) {
  const { hasApiKey } = useAuth();
  const location = useLocation();

  if (requireAuth && !hasApiKey) {
    // Redirect to config page with return URL
    return <Navigate to="/config" state={{ from: location }} replace />;
  }

  return <>{children}</>;
}