import { create } from 'zustand';
import type { StatisticsStore } from '@/types/store';
import { statisticsService } from '@/services/statistics.service';

export const useStatisticsStore = create<StatisticsStore>((set) => ({
  data: null,
  timeRange: '30d',
  isLoading: false,

  fetchStatistics: async () => {
    set({ isLoading: true });
    try {
      const data = await statisticsService.getStatistics(get().timeRange);
      set({ data, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  setTimeRange: (range) => set({ timeRange: range }),
}));

const get = useStatisticsStore.getState;
