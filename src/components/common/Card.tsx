import React from 'react';
import { cn } from '@/utils';

interface CardProps {
  children: React.ReactNode;
  className?: string;
  hover?: boolean;
  padding?: 'none' | 'sm' | 'md' | 'lg';
  onClick?: () => void;
}

const paddingStyles = {
  none: '',
  sm: 'p-3',
  md: 'p-5',
  lg: 'p-8',
};

export function Card({ children, className, hover = false, padding = 'md', onClick }: CardProps) {
  return (
    <div
      onClick={onClick}
      className={cn(
        'rounded-lg border border-border/80 bg-card shadow-sm',
        hover &&
          'transition-all duration-150 hover:-translate-y-px hover:border-primary/45 hover:shadow-md',
        onClick && 'cursor-pointer',
        paddingStyles[padding],
        className,
      )}
    >
      {children}
    </div>
  );
}
