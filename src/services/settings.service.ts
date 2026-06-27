import type { SettingField } from '@/types';

const delay = (min = 200, max = 1000): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, Math.random() * (max - min) + min));

export const settingsService = {
  async getDefaults(fields: SettingField[]): Promise<Record<string, unknown>> {
    await delay(50, 200);
    const defaults: Record<string, unknown> = {};
    for (const field of fields) {
      defaults[field.key] = field.defaultValue;
    }
    return defaults;
  },

  async validateSetting(key: string, value: unknown): Promise<boolean> {
    await delay(50, 150);
    if (key === 'mixedPort') {
      const port = Number(value);
      return !isNaN(port) && port >= 1 && port <= 65535;
    }
    return true;
  },
};
