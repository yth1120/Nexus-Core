import type { Rule } from '@/types';
import { SEED_RULES } from '@/mock/seed';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

let rules = [...SEED_RULES];

export const ruleService = {
  async getAll(): Promise<Rule[]> {
    await delay(300, 800);
    return [...rules];
  },

  async create(data: Omit<Rule, 'id' | 'createdAt'>): Promise<Rule> {
    await delay(300, 800);
    const newRule: Rule = {
      id: `rule-${Date.now()}`,
      ...data,
      createdAt: Date.now(),
    };
    rules = [...rules, newRule];
    return { ...newRule };
  },

  async update(id: string, data: Partial<Rule>): Promise<Rule> {
    await delay(200, 600);
    const index = rules.findIndex((r) => r.id === id);
    if (index === -1) throw new Error(`Rule ${id} not found`);
    rules[index] = { ...rules[index]!, ...data };
    return { ...rules[index]! };
  },

  async delete(id: string): Promise<void> {
    await delay(200, 500);
    rules = rules.filter((r) => r.id !== id);
  },

  async toggleEnabled(id: string): Promise<Rule> {
    await delay(100, 300);
    const index = rules.findIndex((r) => r.id === id);
    if (index === -1) throw new Error(`Rule ${id} not found`);
    rules[index] = { ...rules[index]!, enabled: !rules[index]!.enabled };
    return { ...rules[index]! };
  },
};
