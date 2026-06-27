import { create } from 'zustand';
import type { NodeStore } from '@/types/store';
import { nodeService } from '@/services/node.service';

export const useNodeStore = create<NodeStore>((set, get) => ({
  nodes: [],
  searchQuery: '',
  groupFilter: null,
  sortField: null,
  sortDirection: 'asc',
  isLoading: false,

  fetchNodes: async () => {
    set({ isLoading: true });
    try {
      const nodes = await nodeService.getAll();
      set({ nodes, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  toggleFavorite: async (id) => {
    try {
      const updated = await nodeService.toggleFavorite(id);
      set((state) => ({
        nodes: state.nodes.map((n) => (n.id === id ? updated : n)),
      }));
    } catch {
      // silently fail
    }
  },

  testDelay: async (id) => {
    set((state) => ({
      nodes: state.nodes.map((n) =>
        n.id === id ? { ...n, status: 'untested' as const, delay: null, loss: null } : n,
      ),
    }));
    try {
      const { delay, loss } = await nodeService.testDelay(id);
      set((state) => ({
        nodes: state.nodes.map((n) =>
          n.id === id ? { ...n, delay, loss, status: 'online' as const } : n,
        ),
      }));
    } catch {
      set((state) => ({
        nodes: state.nodes.map((n) => (n.id === id ? { ...n, status: 'offline' as const } : n)),
      }));
    }
  },

  testAllDelay: async () => {
    set((state) => ({
      nodes: state.nodes.map((n) => ({
        ...n,
        status: 'untested' as const,
        delay: null,
        loss: null,
      })),
    }));
    try {
      await nodeService.testAllDelay();
      const nodes = await nodeService.getAll();
      set({ nodes });
    } catch {
      // keep untested status
    }
  },

  connect: async (id) => {
    try {
      const updated = await nodeService.connect(id);
      set((state) => ({
        nodes: state.nodes.map((n) => (n.id === id ? updated : { ...n, isConnected: false })),
      }));
    } catch {
      // silently fail
    }
  },

  disconnect: async (id) => {
    try {
      const updated = await nodeService.disconnect(id);
      set((state) => ({
        nodes: state.nodes.map((n) => (n.id === id ? updated : n)),
      }));
    } catch {
      // silently fail
    }
  },

  setSearchQuery: (q) => set({ searchQuery: q }),

  setGroupFilter: (g) => set({ groupFilter: g }),

  setSort: (field) =>
    set((state) => {
      if (field === null) return { sortField: null, sortDirection: 'asc' };
      const newDir = state.sortField === field && state.sortDirection === 'asc' ? 'desc' : 'asc';
      return { sortField: field, sortDirection: newDir };
    }),
}));
