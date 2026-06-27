import type { SettingsCategory, SettingsSection } from '@/types';

export const SETTINGS_CATEGORIES: SettingsCategory[] = [
  { id: 'general', titleKey: 'settings.general', icon: 'Settings' },
  { id: 'network', titleKey: 'settings.network', icon: 'Globe' },
  { id: 'appearance', titleKey: 'settings.appearance', icon: 'Palette' },
  { id: 'advanced', titleKey: 'settings.advanced', icon: 'Terminal' },
];

export const SETTINGS_SECTIONS: SettingsSection[] = [
  {
    categoryId: 'general',
    id: 'system',
    titleKey: 'settings.systemIntegration',
    fields: [
      {
        key: 'launchOnStartup',
        labelKey: 'settings.launchOnStartup',
        descriptionKey: 'settings.launchOnStartupDesc',
        type: 'toggle',
        defaultValue: true,
      },
      {
        key: 'silentMode',
        labelKey: 'settings.silentMode',
        descriptionKey: 'settings.silentModeDesc',
        type: 'toggle',
        defaultValue: true,
      },
      {
        key: 'serviceMode',
        labelKey: 'settings.serviceMode',
        descriptionKey: 'settings.serviceModeDesc',
        type: 'button',
        defaultValue: 'Install',
      },
    ],
  },
  {
    categoryId: 'general',
    id: 'updates',
    titleKey: 'settings.updates',
    fields: [
      {
        key: 'autoCheckUpdates',
        labelKey: 'settings.autoCheckUpdates',
        descriptionKey: 'settings.autoCheckUpdatesDesc',
        type: 'toggle',
        defaultValue: true,
      },
    ],
  },
  {
    categoryId: 'network',
    id: 'inbound',
    titleKey: 'settings.inbound',
    fields: [
      {
        key: 'mixedPort',
        labelKey: 'settings.mixedPort',
        descriptionKey: 'settings.mixedPortDesc',
        type: 'text',
        defaultValue: '7890',
      },
      {
        key: 'allowLan',
        labelKey: 'settings.allowLan',
        descriptionKey: 'settings.allowLanDesc',
        type: 'toggle',
        defaultValue: false,
      },
    ],
  },
  {
    categoryId: 'network',
    id: 'tun',
    titleKey: 'settings.tunMode',
    fields: [
      {
        key: 'tunMode',
        labelKey: 'settings.tunModeEnabled',
        descriptionKey: 'settings.tunModeDesc',
        type: 'toggle',
        defaultValue: false,
      },
      {
        key: 'dnsServer',
        labelKey: 'settings.dnsServer',
        descriptionKey: 'settings.dnsServerDesc',
        type: 'text',
        defaultValue: '1.1.1.1',
      },
    ],
  },
  {
    categoryId: 'appearance',
    id: 'theme',
    titleKey: 'settings.theme',
    fields: [
      {
        key: 'themeMode',
        labelKey: 'settings.themeMode',
        descriptionKey: 'settings.themeModeDesc',
        type: 'select',
        defaultValue: 'dark',
        options: [
          { labelKey: 'settings.dark', value: 'dark' },
          { labelKey: 'settings.light', value: 'light' },
          { labelKey: 'settings.system', value: 'system' },
        ],
      },
    ],
  },
  {
    categoryId: 'advanced',
    id: 'logLevel',
    titleKey: 'settings.logLevel',
    fields: [
      {
        key: 'logLevel',
        labelKey: 'settings.logLevelLabel',
        descriptionKey: 'settings.logLevelDesc',
        type: 'select',
        defaultValue: 'INFO',
        options: [
          { labelKey: 'settings.logLevelTrace', value: 'TRACE' },
          { labelKey: 'settings.logLevelDebug', value: 'DEBUG' },
          { labelKey: 'settings.logLevelInfo', value: 'INFO' },
          { labelKey: 'settings.logLevelWarn', value: 'WARN' },
          { labelKey: 'settings.logLevelError', value: 'ERROR' },
        ],
      },
    ],
  },
];
