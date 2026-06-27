import { cn } from '@/utils';
import type { SelectOption } from '@/types';

interface SelectProps {
  options: SelectOption[];
  value: string;
  onChange: (value: string) => void;
  label?: string;
  placeholder?: string;
  className?: string;
}

export function Select({ options, value, onChange, label, placeholder, className }: SelectProps) {
  return (
    <div className="flex flex-col gap-1.5">
      {label && <label className="text-sm font-medium text-text">{label}</label>}
      <select
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className={cn(
          'bg-card border border-border text-text text-sm rounded-lg px-3 py-2',
          'focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary',
          'appearance-none cursor-pointer transition-colors duration-200',
          className,
        )}
      >
        {placeholder && (
          <option value="" disabled>
            {placeholder}
          </option>
        )}
        {options.map((opt) => (
          <option key={opt.value} value={opt.value}>
            {opt.label}
          </option>
        ))}
      </select>
    </div>
  );
}
