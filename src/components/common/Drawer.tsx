import React, { useEffect } from 'react';
import { X } from 'lucide-react';
import { cn } from '@/utils';

interface DrawerProps {
  open: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  side?: 'left' | 'right';
  width?: string;
}

export function Drawer({
  open,
  onClose,
  title,
  children,
  side = 'right',
  width = '400px',
}: DrawerProps) {
  useEffect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => {
      document.body.style.overflow = '';
    };
  }, [open]);

  useEffect(() => {
    function handleKey(e: KeyboardEvent) {
      if (e.key === 'Escape') onClose();
    }
    if (open) document.addEventListener('keydown', handleKey);
    return () => document.removeEventListener('keydown', handleKey);
  }, [open, onClose]);

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 flex">
      {/* Overlay */}
      <div
        className="absolute inset-0 bg-black/60 backdrop-blur-sm animate-fade-in"
        onClick={onClose}
      />

      {/* Panel */}
      <div
        className={cn(
          'relative h-full bg-card border-border shadow-2xl flex flex-col animate-slide-in-right',
          side === 'left' ? 'border-r order-first' : 'border-l order-last ml-auto',
        )}
        style={{ width, maxWidth: '90vw' }}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-border shrink-0">
          <h2 className="text-lg font-semibold text-text">{title}</h2>
          <button
            onClick={onClose}
            className="p-1.5 rounded-lg text-text-secondary hover:text-text hover:bg-border transition-colors"
            aria-label="Close"
          >
            <X size={18} />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">{children}</div>
      </div>
    </div>
  );
}
