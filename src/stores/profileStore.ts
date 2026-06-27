import { create } from 'zustand';
import type { ProfileStore } from '@/types/store';
import type { Profile } from '@/types';
import { profileService } from '@/services/profile.service';

export const useProfileStore = create<ProfileStore>((set, get) => ({
  profiles: [],
  searchQuery: '',
  isLoading: false,
  error: null,

  fetchProfiles: async () => {
    set({ isLoading: true, error: null });
    try {
      const profiles = await profileService.getAll();
      set({ profiles, isLoading: false });
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
    }
  },

  createProfile: async (data) => {
    set({ isLoading: true, error: null });
    try {
      await profileService.create(data);
      const profiles = await profileService.getAll();
      set({ profiles, isLoading: false });
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
    }
  },

  updateProfile: async (id, data) => {
    set({ isLoading: true, error: null });
    try {
      await profileService.update(id, data);
      const profiles = await profileService.getAll();
      set({ profiles, isLoading: false });
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
    }
  },

  deleteProfile: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await profileService.delete(id);
      const profiles = await profileService.getAll();
      set({ profiles, isLoading: false });
    } catch (err) {
      set({ error: (err as Error).message, isLoading: false });
    }
  },

  setSearchQuery: (q) => set({ searchQuery: q }),
}));
