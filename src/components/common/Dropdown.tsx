import React, { useState, useRef, useEffect } from 'react';
import { cn } from '@/utils';
import type { DropdownItem } from '@/types';

interface DropdownProps {
  trigger: React.ReactNode;
  items: DropdownItem[];
  align?: 'left' | 'right';
}

export function Dropdown({ trigger, items, align = 'left' }: DropdownProps) {
  const [open, setOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClick(e: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    if (open) {
      document.addEventListener('mousedown', handleClick);
    }
    return () => document.removeEventListener('mousedown', handleClick);
  }, [open]);

  useEffect(() => {
    function handleKey(e: KeyboardEvent) {
      if (e.key === 'Escape') setOpen(false);
    }
    if (open) {
      document.addEventListener('keydown', handleKey);
    }
    return () => document.removeEventListener('keydown', handleKey);
  }, [open]);

  return (
    <div ref={containerRef} className="relative inline-block">
      <div onClick={() => setOpen(!open)}>{trigger}</div>

      {open && (
        <div
          className={cn(
            'absolute top-full mt-1 z-40 min-w-[160px] bg-card border border-border rounded-lg shadow-xl overflow-hidden animate-scale-in',
            align === 'right' ? 'right-0' : 'left-0',
          )}
        >
          {items.map((item, idx) => {
            const Icon = item.icon;
            return (
              <button
                key={`${item.label}-${idx}`}
                onClick={() => {
                  item.onClick();
                  setOpen(false);
                }}
                disabled={item.disabled}
                className={cn(
                  'w-full flex items-center gap-2 px-4 py-2.5 text-sm text-left transition-colors',
                  'hover:bg-border hover:text-text',
                  'disabled:opacity-40 disabled:cursor-not-allowed',
                  'text-text-secondary',
                )}
              >
                {Icon && <Icon size={16} />}
                {item.label}
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}
