import React from 'react';
import { useTheme } from '@/hooks/useTheme';

interface ProvidersProps {
  children: React.ReactNode;
}

export function Providers({ children }: ProvidersProps) {
  useTheme();

  return <>{children}</>;
}
