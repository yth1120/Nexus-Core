import type { Connection } from '@/types';
import { generateConnections } from '@/mock/seed';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

let connections = generateConnections(25);

export const connectionService = {
  async getAll(): Promise<Connection[]> {
    await delay(200, 600);
    // Simulate live network activity with fresh random values each poll
    return connections.map((c) => ({
      ...c,
      upload: Math.floor(Math.random() * 15 * 1024),
      download: Math.floor(Math.random() * 800 * 1024),
      duration: c.duration + 1,
    }));
  },

  async closeById(id: string): Promise<void> {
    await delay(100, 400);
    connections = connections.filter((c) => c.id !== id);
  },

  async closeAll(): Promise<void> {
    await delay(300, 800);
    connections = [];
  },
};
