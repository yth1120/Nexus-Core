import type { LogEntry, LogLevel } from '@/types';
import { generateLogs } from '@/mock/seed';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

const initialLogs = generateLogs(50);

export const logService = {
  async getAll(levelFilter?: LogLevel | 'ALL'): Promise<LogEntry[]> {
    await delay(100, 300);
    if (!levelFilter || levelFilter === 'ALL') {
      return [...initialLogs];
    }
    return initialLogs.filter((l) => l.level === levelFilter);
  },

  async getRecent(limit = 100): Promise<LogEntry[]> {
    await delay(50, 200);
    return initialLogs.slice(-limit);
  },
};
