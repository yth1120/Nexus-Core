import { Loader2 } from 'lucide-react';
import { cn } from '@/utils';

interface LoadingProps {
  size?: 'sm' | 'md' | 'lg';
  text?: string;
  fullPage?: boolean;
}

const sizeMap = {
  sm: 16,
  md: 24,
  lg: 36,
} as const;

export function Loading({ size = 'md', text, fullPage = false }: LoadingProps) {
  const content = (
    <div className="flex flex-col items-center justify-center gap-3">
      <Loader2 size={sizeMap[size]} className="animate-spin text-primary" />
      {text && <p className="text-sm text-text-secondary">{text}</p>}
    </div>
  );

  if (fullPage) {
    return <div className="fixed inset-0 flex items-center justify-center bg-bg/80">{content}</div>;
  }

  return <div className={cn('flex items-center justify-center py-12')}>{content}</div>;
}
