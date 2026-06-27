import { useEffect, useState } from 'react';
import { X, CheckCircle2, AlertTriangle, AlertCircle, Info } from 'lucide-react';
import { useAppStore } from '@/stores/appStore';
import { cn } from '@/utils';
import type { ToastType, Toast as ToastItem } from '@/types';

const toastConfig: Record<ToastType, { icon: typeof CheckCircle2; bg: string; border: string }> = {
  success: {
    icon: CheckCircle2,
    bg: 'bg-success/10',
    border: 'border-success/30',
  },
  warning: {
    icon: AlertTriangle,
    bg: 'bg-warning/10',
    border: 'border-warning/30',
  },
  error: {
    icon: AlertCircle,
    bg: 'bg-error/10',
    border: 'border-error/30',
  },
  info: {
    icon: Info,
    bg: 'bg-primary/10',
    border: 'border-primary/30',
  },
};

function ToastCard({ toast }: { toast: ToastItem }) {
  const removeToast = useAppStore((s) => s.removeToast);
  const [isExiting, setIsExiting] = useState(false);
  const config = toastConfig[toast.type];
  const Icon = config.icon;

  useEffect(() => {
    if (toast.duration > 0) {
      const timer = setTimeout(() => {
        setIsExiting(true);
        setTimeout(() => removeToast(toast.id), 300);
      }, toast.duration);
      return () => clearTimeout(timer);
    }
    return;
  }, [toast, removeToast]);

  function handleDismiss() {
    setIsExiting(true);
    setTimeout(() => removeToast(toast.id), 300);
  }

  return (
    <div
      className={cn(
        'flex items-start gap-3 p-4 rounded-lg border shadow-lg min-w-[320px] max-w-[420px] backdrop-blur-md',
        config.bg,
        config.border,
        isExiting ? 'animate-slide-out-right' : 'animate-slide-in-right',
      )}
    >
      <Icon size={18} className="mt-0.5 shrink-0 text-text" />
      <div className="flex-1 min-w-0">
        {toast.title && <p className="text-sm font-semibold text-text">{toast.title}</p>}
        <p className="text-sm text-text-secondary">{toast.message}</p>
      </div>
      <button
        onClick={handleDismiss}
        className="p-0.5 rounded text-text-secondary hover:text-text transition-colors shrink-0"
      >
        <X size={14} />
      </button>
    </div>
  );
}

export function ToastContainer() {
  const toasts = useAppStore((s) => s.toasts);

  if (toasts.length === 0) return null;

  return (
    <div className="fixed top-4 right-4 z-[100] flex flex-col gap-2 pointer-events-none">
      {toasts.map((toast) => (
        <div key={toast.id} className="pointer-events-auto">
          <ToastCard toast={toast} />
        </div>
      ))}
    </div>
  );
}
