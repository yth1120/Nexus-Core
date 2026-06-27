import { create } from 'zustand';
import type { ConnectionStore } from '@/types/store';
import { connectionService } from '@/services/connection.service';

export const useConnectionStore = create<ConnectionStore>((set, get) => ({
  connections: [],
  searchQuery: '',
  sortField: null,
  sortDirection: 'asc',
  page: 1,
  pageSize: 25,
  totalCount: 0,
  isLoading: false,
  autoRefresh: true,

  fetchConnections: async () => {
    set({ isLoading: true });
    try {
      const connections = await connectionService.getAll();
      set({ connections, totalCount: connections.length, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  closeConnection: async (id) => {
    try {
      await connectionService.closeById(id);
      const connections = await connectionService.getAll();
      set({ connections, totalCount: connections.length });
    } catch {
      // silently fail
    }
  },

  closeAll: async () => {
    try {
      await connectionService.closeAll();
      set({ connections: [], totalCount: 0 });
    } catch {
      // silently fail
    }
  },

  setSearchQuery: (q) => set({ searchQuery: q, page: 1 }),

  setSortField: (field) =>
    set((state) => {
      const newDir = state.sortField === field && state.sortDirection === 'asc' ? 'desc' : 'asc';
      return { sortField: field, sortDirection: newDir, page: 1 };
    }),

  setPage: (page) => set({ page }),

  toggleAutoRefresh: () => set((state) => ({ autoRefresh: !state.autoRefresh })),
}));
