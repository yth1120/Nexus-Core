import React from 'react';
import { cn } from '@/utils';
import type { BadgeVariant } from '@/types';

interface BadgeProps {
  variant?: BadgeVariant;
  size?: 'sm' | 'md';
  children: React.ReactNode;
  dot?: boolean;
}

const variantStyles: Record<BadgeVariant, string> = {
  default: 'bg-border text-text-secondary',
  success: 'bg-success/10 text-success',
  warning: 'bg-warning/10 text-warning',
  error: 'bg-error/10 text-error',
  info: 'bg-primary/10 text-primary',
};

const dotColors: Record<BadgeVariant, string> = {
  default: 'bg-text-secondary',
  success: 'bg-success',
  warning: 'bg-warning',
  error: 'bg-error',
  info: 'bg-primary',
};

export function Badge({ variant = 'default', size = 'md', children, dot = false }: BadgeProps) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 font-medium rounded-md',
        size === 'sm' ? 'px-2 py-0.5 text-[10px]' : 'px-2.5 py-0.5 text-xs',
        variantStyles[variant],
      )}
    >
      {dot && <span className={cn('inline-block w-1.5 h-1.5 rounded-full', dotColors[variant])} />}
      {children}
    </span>
  );
}
