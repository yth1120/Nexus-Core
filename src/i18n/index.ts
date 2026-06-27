import { create } from 'zustand';
import type { Locale } from './types';
import { en } from './en';
import { zhCN } from './zh-CN';
import { LOCALE_STORAGE_KEY } from '@/constants';

const translations: Record<Locale, Record<string, string>> = {
  en,
  'zh-CN': zhCN,
};

function detectLocale(): Locale {
  return 'zh-CN';
}

interface I18nStore {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: (key: string, params?: Record<string, string>) => string;
}

export const useI18n = create<I18nStore>((set, get) => ({
  locale: detectLocale(),

  setLocale: (locale) => {
    try {
      localStorage.setItem(LOCALE_STORAGE_KEY, locale);
    } catch {
      // storage unavailable
    }
    document.documentElement.lang = locale === 'zh-CN' ? 'zh' : 'en';
    set({ locale });
  },

  t: (key, params) => {
    const locale = get().locale;
    const str = translations[locale]?.[key] ?? translations['en']?.[key] ?? key;
    if (!params) return str;
    return str.replace(/\{(\w+)\}/g, (_, p: string) => params[p] ?? `{${p}}`);
  },
}));
