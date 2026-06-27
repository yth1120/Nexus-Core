import { create } from 'zustand';
import type { SettingsStore, ThemeMode } from '@/types/store';
import { SETTINGS_STORAGE_KEY, THEME_STORAGE_KEY } from '@/constants';
import { SETTINGS_SECTIONS } from '@/constants/settings';

function getDefaultValues(): Record<string, unknown> {
  const values: Record<string, unknown> = {};
  for (const section of SETTINGS_SECTIONS) {
    for (const field of section.fields) {
      values[field.key] = field.defaultValue;
    }
  }
  return values;
}

function loadFromStorage(): { theme: ThemeMode; values: Record<string, unknown> } {
  try {
    const savedTheme = localStorage.getItem(THEME_STORAGE_KEY) as ThemeMode | null;
    const savedSettings = localStorage.getItem(SETTINGS_STORAGE_KEY);

    const theme =
      savedTheme && ['dark', 'light', 'system'].includes(savedTheme) ? savedTheme : 'dark';

    const values = savedSettings
      ? { ...getDefaultValues(), ...JSON.parse(savedSettings) }
      : getDefaultValues();

    return { theme, values };
  } catch {
    return { theme: 'dark', values: getDefaultValues() };
  }
}

const initial = loadFromStorage();

export const useSettingsStore = create<SettingsStore>((set) => ({
  theme: initial.theme,
  values: initial.values,

  setTheme: (theme) => {
    set({ theme });
    try {
      localStorage.setItem(THEME_STORAGE_KEY, theme);
    } catch {
      // storage unavailable
    }
  },

  updateValue: (key, value) => {
    set((state) => {
      const values = { ...state.values, [key]: value };
      try {
        localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(values));
      } catch {
        // storage unavailable
      }
      return { values };
    });
  },

  resetToDefaults: () => {
    const values = getDefaultValues();
    set({ values });
    try {
      localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(values));
    } catch {
      // storage unavailable
    }
  },

  hydrate: () => {
    const { theme, values } = loadFromStorage();
    set({ theme, values });
  },
}));
