import { useCallback } from 'react';
import { useAppStore } from '@/stores/appStore';
import type { ToastType } from '@/types';

export function useToast() {
  const addToast = useAppStore((s) => s.addToast);

  const success = useCallback(
    (message: string, title?: string) => addToast({ type: 'success', message, title }),
    [addToast],
  );

  const warning = useCallback(
    (message: string, title?: string) => addToast({ type: 'warning', message, title }),
    [addToast],
  );

  const error = useCallback(
    (message: string, title?: string) => addToast({ type: 'error', message, title }),
    [addToast],
  );

  const info = useCallback(
    (message: string, title?: string) => addToast({ type: 'info', message, title }),
    [addToast],
  );

  return { success, warning, error, info };
}
