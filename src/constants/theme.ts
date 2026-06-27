export const THEME_STORAGE_KEY = 'nexus-core-theme';
export const SETTINGS_STORAGE_KEY = 'nexus-core-settings';
export const LOCALE_STORAGE_KEY = 'nexus-core-locale';

export const CSS_VARIABLES = [
  'bg',
  'card',
  'border',
  'primary',
  'success',
  'warning',
  'error',
  'text',
  'text-secondary',
] as const;

export type CssVariable = (typeof CSS_VARIABLES)[number];
