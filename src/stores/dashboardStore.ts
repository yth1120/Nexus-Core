import { create } from 'zustand';
import type { DashboardStore } from '@/types/store';
import { SEED_DASHBOARD_STATUS } from '@/mock/seed';

export const useDashboardStore = create<DashboardStore>((set) => ({
  isRunning: true,
  status: { ...SEED_DASHBOARD_STATUS },
  uploadHistory: Array.from({ length: 20 }, () => Math.random() * 5),
  downloadHistory: Array.from({ length: 20 }, () => Math.random() * 15),

  toggleRunning: () =>
    set((state) => ({
      isRunning: !state.isRunning,
      status: {
        ...state.status,
        status: state.isRunning ? ('stopped' as const) : ('running' as const),
      },
    })),

  updateStatus: (partial) =>
    set((state) => ({
      status: { ...state.status, ...partial },
    })),

  pushTrafficData: (up, down) =>
    set((state) => ({
      uploadHistory: [...state.uploadHistory.slice(1), up],
      downloadHistory: [...state.downloadHistory.slice(1), down],
    })),
}));
