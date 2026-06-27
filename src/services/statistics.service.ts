import type { StatisticsData, TimeRange } from '@/types';
import { generateSeedStatistics } from '@/mock/seed';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

export const statisticsService = {
  async getStatistics(_timeRange: TimeRange): Promise<StatisticsData> {
    await delay(400, 1000);
    return generateSeedStatistics();
  },
};
