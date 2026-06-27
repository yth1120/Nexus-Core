import React from 'react';
import { cn } from '@/utils';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  icon?: React.ComponentType<{ size?: number; className?: string }>;
}

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, icon: Icon, className, ...props }, ref) => {
    return (
      <div className="flex flex-col gap-1.5">
        {label && <label className="text-sm font-medium text-text">{label}</label>}
        <div className="relative">
          {Icon && (
            <Icon
              size={16}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary pointer-events-none"
            />
          )}
          <input
            ref={ref}
            className={cn(
              'w-full bg-card border rounded-lg text-sm transition-colors duration-200',
              'placeholder:text-text-secondary/60',
              'focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary',
              error ? 'border-error focus:ring-error/30 focus:border-error' : 'border-border',
              Icon ? 'pl-10 pr-4 py-2' : 'px-4 py-2',
              className,
            )}
            {...props}
          />
        </div>
        {error && <p className="text-xs text-error">{error}</p>}
      </div>
    );
  },
);

Input.displayName = 'Input';
