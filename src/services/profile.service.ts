import type { Profile } from '@/types';
import { SEED_PROFILES } from '@/mock/seed';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

let profiles = [...SEED_PROFILES];

export const profileService = {
  async getAll(): Promise<Profile[]> {
    await delay(300, 800);
    return [...profiles];
  },

  async getById(id: string): Promise<Profile> {
    await delay(100, 400);
    const profile = profiles.find((p) => p.id === id);
    if (!profile) throw new Error(`Profile ${id} not found`);
    return { ...profile };
  },

  async create(
    data: Omit<Profile, 'id' | 'status' | 'latency' | 'updated' | 'trafficUsed' | 'trafficTotal'>,
  ): Promise<Profile> {
    await delay(300, 800);
    const newProfile: Profile = {
      id: `profile-${Date.now()}`,
      status: 'inactive',
      latency: 0,
      updated: new Date().toISOString(),
      ...data,
    };
    profiles = [newProfile, ...profiles];
    return { ...newProfile };
  },

  async update(id: string, data: Partial<Profile>): Promise<Profile> {
    await delay(200, 600);
    const index = profiles.findIndex((p) => p.id === id);
    if (index === -1) throw new Error(`Profile ${id} not found`);
    profiles[index] = { ...profiles[index]!, ...data };
    return { ...profiles[index]! };
  },

  async delete(id: string): Promise<void> {
    await delay(200, 500);
    profiles = profiles.filter((p) => p.id !== id);
  },

  async toggleActive(id: string): Promise<Profile> {
    await delay(150, 400);
    const index = profiles.findIndex((p) => p.id === id);
    if (index === -1) throw new Error(`Profile ${id} not found`);

    if (profiles[index]!.status === 'active') {
      profiles[index] = { ...profiles[index]!, status: 'inactive' };
    } else {
      profiles = profiles.map((p) => ({
        ...p,
        status: p.id === id ? 'active' : ('inactive' as const),
      }));
    }

    return { ...profiles[index]! };
  },
};
