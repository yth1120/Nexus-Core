import React from 'react';
import { Loader2 } from 'lucide-react';
import type { ButtonVariant, ButtonSize } from '@/types';
import { cn } from '@/utils';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  icon?: React.ComponentType<{ size?: number | string; className?: string }>;
  loading?: boolean;
}

const variantStyles: Record<ButtonVariant, string> = {
  primary:
    'bg-primary hover:bg-blue-600 text-white shadow-lg shadow-primary/20 active:scale-[0.98]',
  secondary: 'bg-border hover:bg-gray-600 text-white active:scale-[0.98]',
  danger:
    'bg-transparent hover:bg-error/10 text-error border border-transparent hover:border-error/30',
  ghost: 'bg-transparent hover:bg-border text-text-secondary hover:text-text',
};

const sizeStyles: Record<ButtonSize, string> = {
  sm: 'gap-1.5 px-3 py-1.5 text-xs rounded-lg',
  md: 'gap-2 px-4 py-2 text-sm rounded-lg',
  lg: 'gap-2.5 px-6 py-3 text-base rounded-lg',
};

export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      variant = 'primary',
      size = 'md',
      icon: Icon,
      loading = false,
      disabled,
      className,
      children,
      ...props
    },
    ref,
  ) => {
    return (
      <button
        ref={ref}
        disabled={disabled || loading}
        className={cn(
          'inline-flex items-center justify-center font-medium transition-all duration-200',
          'focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/50 focus-visible:ring-offset-2 focus-visible:ring-offset-bg',
          'disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100',
          variantStyles[variant],
          sizeStyles[size],
          className,
        )}
        {...props}
      >
        {loading ? (
          <Loader2 size={size === 'sm' ? 14 : size === 'lg' ? 20 : 16} className="animate-spin" />
        ) : Icon ? (
          <Icon size={size === 'sm' ? 14 : size === 'lg' ? 20 : 16} />
        ) : null}
        {children}
      </button>
    );
  },
);

Button.displayName = 'Button';
