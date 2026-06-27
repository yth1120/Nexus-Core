import type { Node } from '@/types';
import { SEED_NODES } from '@/mock/seed';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

let nodes = [...SEED_NODES];

export const nodeService = {
  async getAll(): Promise<Node[]> {
    await delay(300, 800);
    return [...nodes];
  },

  async toggleFavorite(id: string): Promise<Node> {
    await delay(100, 300);
    const index = nodes.findIndex((n) => n.id === id);
    if (index === -1) throw new Error(`Node ${id} not found`);
    nodes[index] = { ...nodes[index]!, isFavorite: !nodes[index]!.isFavorite };
    return { ...nodes[index]! };
  },

  async testDelay(id: string): Promise<{ delay: number; loss: number }> {
    await delay(500, 2000);
    const delay_ = Math.floor(Math.random() * 350) + 5;
    const loss = Math.random() * 5;
    const index = nodes.findIndex((n) => n.id === id);
    if (index !== -1) {
      nodes[index] = { ...nodes[index]!, delay: delay_, loss, status: 'online' };
    }
    return { delay: delay_, loss };
  },

  async testAllDelay(): Promise<void> {
    await delay(1000, 5000);
    nodes = nodes.map((node) => ({
      ...node,
      delay: Math.floor(Math.random() * 350) + 5,
      loss: Math.random() * 5,
      status: Math.random() > 0.1 ? ('online' as const) : ('offline' as const),
    }));
  },

  async connect(id: string): Promise<Node> {
    await delay(200, 600);
    nodes = nodes.map((n) => ({
      ...n,
      isConnected: n.id === id,
    }));
    const node = nodes.find((n) => n.id === id);
    if (!node) throw new Error(`Node ${id} not found`);
    return { ...node, isConnected: true };
  },

  async disconnect(id: string): Promise<Node> {
    await delay(100, 300);
    const index = nodes.findIndex((n) => n.id === id);
    if (index === -1) throw new Error(`Node ${id} not found`);
    nodes[index] = { ...nodes[index]!, isConnected: false };
    return { ...nodes[index]! };
  },
};
