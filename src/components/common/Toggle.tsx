import { cn } from '@/utils';

interface ToggleProps {
  enabled: boolean;
  onChange: () => void;
  disabled?: boolean;
  size?: 'sm' | 'md';
}

export function Toggle({ enabled, onChange, disabled = false, size = 'md' }: ToggleProps) {
  const isSm = size === 'sm';

  return (
    <button
      type="button"
      role="switch"
      aria-checked={enabled}
      onClick={onChange}
      disabled={disabled}
      className={cn(
        'relative inline-flex items-center rounded-full transition-colors duration-300',
        'focus:outline-none focus-visible:ring-2 focus-visible:ring-primary/50 focus-visible:ring-offset-2 focus-visible:ring-offset-bg',
        'disabled:opacity-50 disabled:cursor-not-allowed',
        isSm ? 'h-5 w-9' : 'h-7 w-12',
        enabled ? 'bg-primary' : 'bg-border',
      )}
    >
      <span
        className={cn(
          'inline-block rounded-full bg-white transition-transform duration-300',
          isSm ? 'h-3.5 w-3.5' : 'h-5 w-5',
          enabled ? (isSm ? 'translate-x-[18px]' : 'translate-x-[26px]') : 'translate-x-1',
        )}
      />
    </button>
  );
}
