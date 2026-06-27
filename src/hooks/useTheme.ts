import { useEffect } from 'react';
import { useSettingsStore } from '@/stores/settingsStore';
import type { ThemeMode } from '@/types';

export function useTheme(): { theme: ThemeMode; setTheme: (theme: ThemeMode) => void } {
  const theme = useSettingsStore((s) => s.theme);
  const setTheme = useSettingsStore((s) => s.setTheme);

  useEffect(() => {
    function applyTheme(resolved: 'dark' | 'light') {
      document.documentElement.classList.toggle('dark', resolved === 'dark');
    }

    if (theme === 'system') {
      const mq = window.matchMedia('(prefers-color-scheme: dark)');
      applyTheme(mq.matches ? 'dark' : 'light');

      const handler = (e: MediaQueryListEvent) => {
        applyTheme(e.matches ? 'dark' : 'light');
      };
      mq.addEventListener('change', handler);
      return () => mq.removeEventListener('change', handler);
    }

    applyTheme(theme);
    return;
  }, [theme]);

  return { theme, setTheme };
}
