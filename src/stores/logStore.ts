import { create } from 'zustand';
import type { LogStore } from '@/types/store';
import type { LogEntry } from '@/types';
import { generateLogs } from '@/mock/seed';

const initialLogs: LogEntry[] = [
  {
    id: 'log-init-1',
    timestamp: Date.now() - 600000,
    level: 'INFO',
    message: 'Configuration loaded from config.yaml',
  },
  {
    id: 'log-init-2',
    timestamp: Date.now() - 599000,
    level: 'INFO',
    message: 'Mixed(http+socks) proxy listening at: 127.0.0.1:7890',
  },
  {
    id: 'log-init-3',
    timestamp: Date.now() - 595000,
    level: 'INFO',
    message: 'DNS resolver initialized with upstream: 1.1.1.1',
  },
  {
    id: 'log-init-4',
    timestamp: Date.now() - 590000,
    level: 'INFO',
    message: 'Rule engine loaded: 8 rule sets active',
  },
  {
    id: 'log-init-5',
    timestamp: Date.now() - 585000,
    level: 'WARN',
    message:
      '[TCP] dial PROXY (match DomainSuffix/google.com) to mtalk.google.com:5228 error: timeout',
  },
  {
    id: 'log-init-6',
    timestamp: Date.now() - 580000,
    level: 'INFO',
    message:
      '[TCP] 127.0.0.1:54321 --> api.telegram.org:443 match DomainKeyword(telegram) using Proxy[HK-01]',
  },
  {
    id: 'log-init-7',
    timestamp: Date.now() - 570000,
    level: 'ERROR',
    message:
      'Update subscription failed: Get "https://sub.example.com/...": net/http: TLS handshake timeout',
  },
  {
    id: 'log-init-8',
    timestamp: Date.now() - 560000,
    level: 'INFO',
    message:
      '[UDP] 192.168.1.100:5353 --> 224.0.0.251:5353 match IP-CIDR(224.0.0.0/4) using DIRECT',
  },
  ...generateLogs(42),
];

export const useLogStore = create<LogStore>((set) => ({
  logs: initialLogs,
  levelFilter: 'ALL',
  autoScroll: true,
  maxLogs: 500,

  addLog: (entry) =>
    set((state) => {
      const logs = [...state.logs, entry];
      if (logs.length > state.maxLogs) {
        return { logs: logs.slice(logs.length - state.maxLogs) };
      }
      return { logs };
    }),

  setLevelFilter: (level) => set({ levelFilter: level }),

  toggleAutoScroll: () => set((state) => ({ autoScroll: !state.autoScroll })),

  clearLogs: () => set({ logs: [] }),

  setMaxLogs: (n) => set({ maxLogs: n }),
}));
