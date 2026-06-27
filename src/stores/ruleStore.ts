import { create } from 'zustand';
import type { RuleStore } from '@/types/store';
import type { Rule } from '@/types';
import { ruleService } from '@/services/rule.service';

export const useRuleStore = create<RuleStore>((set) => ({
  rules: [],
  searchQuery: '',
  tagFilter: null,
  isLoading: false,

  fetchRules: async () => {
    set({ isLoading: true });
    try {
      const rules = await ruleService.getAll();
      set({ rules, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  createRule: async (data) => {
    set({ isLoading: true });
    try {
      await ruleService.create(data as Omit<Rule, 'id' | 'createdAt'>);
      const rules = await ruleService.getAll();
      set({ rules, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  updateRule: async (id, data) => {
    set({ isLoading: true });
    try {
      await ruleService.update(id, data);
      const rules = await ruleService.getAll();
      set({ rules, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  deleteRule: async (id) => {
    set({ isLoading: true });
    try {
      await ruleService.delete(id);
      const rules = await ruleService.getAll();
      set({ rules, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  toggleEnabled: async (id) => {
    try {
      const updated = await ruleService.toggleEnabled(id);
      set((state) => ({
        rules: state.rules.map((r) => (r.id === id ? updated : r)),
      }));
    } catch {
      // silently fail
    }
  },

  setSearchQuery: (q) => set({ searchQuery: q }),

  setTagFilter: (tag) => set({ tagFilter: tag }),
}));
