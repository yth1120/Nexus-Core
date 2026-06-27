import { useState } from 'react';
import { Settings as SettingsIcon, Globe, Palette, Terminal, Sun, Moon } from 'lucide-react';
import { useSettingsStore } from '@/stores/settingsStore';
import { useI18n } from '@/i18n';
import { Card } from '@/components/common/Card';
import { Toggle } from '@/components/common/Toggle';
import { Input } from '@/components/common/Input';
import { Select } from '@/components/common/Select';
import { Button } from '@/components/common/Button';
import { cn } from '@/utils';
import type { SettingField, ThemeMode } from '@/types';
import { SETTINGS_CATEGORIES, SETTINGS_SECTIONS } from '@/constants';

const CATEGORY_ICONS: Record<string, typeof SettingsIcon> = {
  Settings: SettingsIcon,
  Globe,
  Palette,
  Terminal,
};

const THEME_OPTIONS = [
  {
    value: 'dark' as const,
    label: '深色',
    preview: (
      <div className="w-16 h-12 bg-[#0F1115] rounded-md border border-[#2A2F3A] flex shadow-inner">
        <div className="w-4 h-full bg-[#13151A] border-r border-[#2A2F3A]" />
      </div>
    ),
  },
  {
    value: 'light' as const,
    label: '浅色',
    preview: (
      <div className="w-16 h-12 bg-[#F3F4F6] rounded-md border border-[#E5E7EB] flex shadow-inner">
        <div className="w-4 h-full bg-[#FFFFFF] border-r border-[#E5E7EB]" />
      </div>
    ),
  },
  {
    value: 'system' as const,
    label: '跟随系统',
    preview: (
      <div className="w-16 h-12 bg-gradient-to-br from-[#0F1115] to-[#F3F4F6] rounded-md border border-[#2A2F3A] flex shadow-inner" />
    ),
  },
];

export function Settings() {
  const [activeCategory, setActiveCategory] = useState('general');
  const { t } = useI18n();

  const theme = useSettingsStore((s) => s.theme);
  const values = useSettingsStore((s) => s.values);
  const setTheme = useSettingsStore((s) => s.setTheme);
  const updateValue = useSettingsStore((s) => s.updateValue);
  const resetToDefaults = useSettingsStore((s) => s.resetToDefaults);

  const sections = SETTINGS_SECTIONS.filter((s) => s.categoryId === activeCategory);

  function renderField(field: SettingField) {
    const currentValue = values[field.key] ?? field.defaultValue;

    switch (field.type) {
      case 'toggle':
        return (
          <div className="flex items-center justify-between gap-4 rounded-lg border border-border bg-bg px-4 py-3">
            <div>
              <p className="text-text font-medium text-sm">{t(field.labelKey)}</p>
              <p className="text-text-secondary text-xs">{t(field.descriptionKey)}</p>
            </div>
            <Toggle
              enabled={currentValue === true}
              onChange={() => updateValue(field.key, !currentValue)}
            />
          </div>
        );

      case 'number':
      case 'text':
        return (
          <div className="flex items-center justify-between gap-4 rounded-lg border border-border bg-bg px-4 py-3">
            <div>
              <p className="text-text font-medium text-sm">{t(field.labelKey)}</p>
              <p className="text-text-secondary text-xs">{t(field.descriptionKey)}</p>
            </div>
            <Input
              value={String(currentValue)}
              onChange={(e) => updateValue(field.key, e.target.value)}
              className="w-24 text-center"
            />
          </div>
        );

      case 'select':
        return (
          <div className="flex items-center justify-between gap-4 rounded-lg border border-border bg-bg px-4 py-3">
            <div>
              <p className="text-text font-medium text-sm">{t(field.labelKey)}</p>
              <p className="text-text-secondary text-xs">{t(field.descriptionKey)}</p>
            </div>
            <Select
              value={String(currentValue)}
              onChange={(v) => updateValue(field.key, v)}
              options={field.options?.map((o) => ({ label: t(o.labelKey), value: o.value })) ?? []}
              className="w-28"
            />
          </div>
        );

      case 'button':
        return (
          <div className="flex items-center justify-between gap-4 rounded-lg border border-border bg-bg px-4 py-3">
            <div>
              <p className="text-text font-medium text-sm">{t(field.labelKey)}</p>
              <p className="text-text-secondary text-xs">{t(field.descriptionKey)}</p>
            </div>
            <Button variant="secondary" size="sm">
              {field.defaultValue as string}
            </Button>
          </div>
        );

      default:
        return null;
    }
  }

  return (
    <div className="app-page flex flex-col">
      <h1 className="page-title mb-5">设置</h1>

      <div className="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden lg:flex-row lg:gap-6">
        {/* Category Sidebar */}
        <div className="scrollbar-thin flex shrink-0 gap-1 overflow-x-auto pb-1 lg:w-48 lg:flex-col lg:overflow-x-visible lg:pb-0">
          {SETTINGS_CATEGORIES.map((cat) => {
            const Icon = CATEGORY_ICONS[cat.icon] ?? SettingsIcon;
            return (
              <button
                key={cat.id}
                onClick={() => setActiveCategory(cat.id)}
                className={cn(
                  'flex shrink-0 items-center gap-3 rounded-lg px-4 py-2.5 text-left text-sm font-medium transition-colors',
                  activeCategory === cat.id
                    ? 'bg-primary/10 text-primary'
                    : 'text-text-secondary hover:bg-border hover:text-text',
                )}
              >
                <Icon size={18} />
                {t(cat.titleKey)}
              </button>
            );
          })}
        </div>

        {/* Content Area */}
        <Card padding="lg" className="scrollbar-thin min-h-0 flex-1 overflow-y-auto">
          <div className="space-y-8">
            {/* Appearance: Quick Theme Toggle */}
            {activeCategory === 'appearance' && (
              <section>
                <h3 className="text-lg font-semibold text-text mb-4">主题设置</h3>

                {/* Quick dark/light toggle switch */}
                <div className="mb-4 flex items-center justify-between gap-4 rounded-lg border border-border bg-bg px-5 py-4">
                  <div className="flex items-center gap-3">
                    {theme === 'dark' ? (
                      <Moon size={20} className="text-primary" />
                    ) : (
                      <Sun size={20} className="text-warning" />
                    )}
                    <div>
                      <p className="text-text font-medium text-sm">深浅主题</p>
                      <p className="text-text-secondary text-xs">
                        {theme === 'dark' ? '当前为深色模式' : '当前为浅色模式'}
                      </p>
                    </div>
                  </div>
                  <Toggle
                    enabled={theme === 'dark'}
                    onChange={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
                  />
                </div>

                {/* Detailed theme options */}
                <p className="text-text-secondary text-xs mb-3">更多选项</p>
                <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
                  {THEME_OPTIONS.map((opt) => (
                    <button
                      key={opt.value}
                      onClick={() => setTheme(opt.value)}
                      className={cn(
                        'flex flex-col items-center gap-3 rounded-lg p-4 transition-all duration-200',
                        theme === opt.value
                          ? 'border border-primary bg-primary/10'
                          : 'border border-border hover:border-primary/50 bg-card opacity-70 hover:opacity-100',
                      )}
                    >
                      {opt.preview}
                      <span
                        className={cn(
                          'text-sm font-medium',
                          theme === opt.value ? 'text-primary' : 'text-text',
                        )}
                      >
                        {opt.label}
                      </span>
                    </button>
                  ))}
                </div>
              </section>
            )}

            {/* Settings Fields from Config */}
            {sections.map((section) => (
              <section key={section.id}>
                <h3 className="text-lg font-semibold text-text mb-4">{t(section.titleKey)}</h3>
                <div className="space-y-3">
                  {section.fields.map((field) => (
                    <div key={field.key}>{renderField(field)}</div>
                  ))}
                </div>
              </section>
            ))}

            {/* Empty state for categories with no configured fields */}
            {activeCategory === 'advanced' && sections.length === 0 && (
              <div className="flex flex-col items-center justify-center text-text-secondary py-16">
                <Terminal size={48} className="opacity-20 mb-4" />
                <p>暂无高级设置</p>
              </div>
            )}
          </div>

          {/* Reset Button */}
          <div className="mt-8 pt-6 border-t border-border">
            <Button variant="ghost" size="sm" onClick={resetToDefaults}>
              恢复默认
            </Button>
          </div>
        </Card>
      </div>
    </div>
  );
}
