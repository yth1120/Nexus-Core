import type { DashboardStatus } from '@/types';
import { SEED_DASHBOARD_STATUS } from '@/mock/seed';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

export const dashboardService = {
  async getStatus(): Promise<DashboardStatus> {
    await delay(100, 300);
    return { ...SEED_DASHBOARD_STATUS };
  },

  async getTrafficHistory(): Promise<number[]> {
    await delay(200, 500);
    return Array.from({ length: 20 }, () => Math.random() * 15);
  },
};
