import { useState, useEffect, useMemo } from 'react';
import { Search, Plus, Trash2, Edit3 } from 'lucide-react';
import { useRuleStore } from '@/stores/ruleStore';
import { useAppStore } from '@/stores/appStore';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Badge } from '@/components/common/Badge';
import { Toggle } from '@/components/common/Toggle';
import { Input } from '@/components/common/Input';
import { Select } from '@/components/common/Select';
import { Modal } from '@/components/common/Modal';
import { ConfirmDialog } from '@/components/common/ConfirmDialog';
import { Loading } from '@/components/common/Loading';
import { useToast } from '@/hooks/useToast';
import { cn } from '@/utils';
import type { Rule } from '@/types';

const RULE_TYPES = ['DomainSuffix', 'DomainKeyword', 'IP-CIDR', 'GEOIP', 'MATCH'];
const PROXIES = [
  'DIRECT',
  'REJECT',
  'Proxy (Auto)',
  'Proxy (HK-01)',
  'Proxy (JP-01)',
  'Proxy (SG-01)',
];

const EMPTY_FORM = {
  name: '',
  type: 'DomainSuffix',
  payload: '',
  proxy: 'Proxy (Auto)',
  enabled: true,
  tags: [] as string[],
};

export function Rules() {
  const rules = useRuleStore((s) => s.rules);
  const searchQuery = useRuleStore((s) => s.searchQuery);
  const tagFilter = useRuleStore((s) => s.tagFilter);
  const isLoading = useRuleStore((s) => s.isLoading);
  const fetchRules = useRuleStore((s) => s.fetchRules);
  const createRule = useRuleStore((s) => s.createRule);
  const updateRule = useRuleStore((s) => s.updateRule);
  const deleteRule = useRuleStore((s) => s.deleteRule);
  const toggleEnabled = useRuleStore((s) => s.toggleEnabled);
  const setSearchQuery = useRuleStore((s) => s.setSearchQuery);
  const setTagFilter = useRuleStore((s) => s.setTagFilter);

  const openModal = useAppStore((s) => s.openModal);
  const closeModal = useAppStore((s) => s.closeModal);
  const activeModal = useAppStore((s) => s.activeModal);

  const toast = useToast();

  const [formData, setFormData] = useState(EMPTY_FORM);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [deletingId, setDeletingId] = useState<string | null>(null);

  useEffect(() => {
    fetchRules();
  }, [fetchRules]);

  const allTags = useMemo(() => {
    const set = new Set<string>();
    rules.forEach((r) => r.tags.forEach((t) => set.add(t)));
    return Array.from(set).sort();
  }, [rules]);

  const filtered = useMemo(() => {
    return rules.filter((r) => {
      if (
        searchQuery &&
        !r.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !r.payload.toLowerCase().includes(searchQuery.toLowerCase())
      )
        return false;
      if (tagFilter && !r.tags.includes(tagFilter)) return false;
      return true;
    });
  }, [rules, searchQuery, tagFilter]);

  function openCreate() {
    setEditingId(null);
    setFormData({ ...EMPTY_FORM });
    openModal('rule-form');
  }

  function openEdit(rule: Rule) {
    setEditingId(rule.id);
    setFormData({
      name: rule.name,
      type: rule.type,
      payload: rule.payload,
      proxy: rule.proxy,
      enabled: rule.enabled,
      tags: [...rule.tags],
    });
    openModal('rule-form');
  }

  async function handleSave() {
    try {
      if (editingId) {
        await updateRule(editingId, formData);
        toast.success('规则已更新');
      } else {
        await createRule(formData);
        toast.success('规则已创建');
      }
      closeModal();
    } catch {
      toast.error('保存失败');
    }
  }

  async function handleDelete() {
    if (!deletingId) return;
    try {
      await deleteRule(deletingId);
      toast.info('规则已删除');
      setDeletingId(null);
    } catch {
      toast.error('删除失败');
    }
  }

  function getProxyBadge(proxy: string) {
    if (proxy === 'DIRECT') return { variant: 'success' as const };
    if (proxy === 'REJECT') return { variant: 'error' as const };
    return { variant: 'info' as const };
  }

  return (
    <div className="app-page flex flex-col">
      {/* Header */}
      <div className="page-header mb-5">
        <h1 className="page-title">规则</h1>
        <div className="page-toolbar md:justify-end">
          <div className="relative">
            <Search
              size={18}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary pointer-events-none"
            />
            <input
              type="text"
              placeholder="搜索规则..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full rounded-lg border border-border bg-card py-2 pl-10 pr-4 text-sm text-text transition-colors focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/30 sm:w-56"
            />
          </div>
          <Button icon={Plus} onClick={openCreate}>
            New Rule
          </Button>
        </div>
      </div>

      {/* Tag Filter Chips */}
      {allTags.length > 0 && (
        <div className="flex items-center gap-2 mb-4 flex-wrap">
          <button
            onClick={() => setTagFilter(null)}
            className={cn(
              'px-3 py-1 rounded-full text-xs font-medium transition-colors',
              !tagFilter
                ? 'bg-primary text-white'
                : 'bg-border text-text-secondary hover:text-text',
            )}
          >
            All
          </button>
          {allTags.map((tag) => (
            <button
              key={tag}
              onClick={() => setTagFilter(tag === tagFilter ? null : tag)}
              className={cn(
                'px-3 py-1 rounded-full text-xs font-medium transition-colors',
                tagFilter === tag
                  ? 'bg-primary text-white'
                  : 'bg-border text-text-secondary hover:text-text',
              )}
            >
              {tag}
            </button>
          ))}
        </div>
      )}

      {/* Content */}
      {isLoading && rules.length === 0 ? (
        <Loading text="加载中..." />
      ) : (
        <div className="scrollbar-thin flex flex-1 flex-col gap-2 overflow-y-auto pr-1">
          {filtered.map((rule) => {
            const proxyBadge = getProxyBadge(rule.proxy);
            return (
              <Card key={rule.id} hover padding="md" className="transition-all">
                <div className="flex items-center gap-4">
                  {/* Toggle */}
                  <Toggle
                    enabled={rule.enabled}
                    onChange={() => toggleEnabled(rule.id)}
                    size="sm"
                  />

                  {/* Info */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <h4 className="text-sm font-semibold text-text">{rule.name}</h4>
                      <Badge variant="default" size="sm">
                        {rule.type}
                      </Badge>
                      <Badge variant={proxyBadge.variant} size="sm">
                        {rule.proxy}
                      </Badge>
                    </div>
                    <p className="text-xs text-text-secondary truncate font-mono">{rule.payload}</p>
                    <div className="flex items-center gap-1 mt-1.5">
                      {rule.tags.map((tag) => (
                        <Badge key={tag} variant="default" size="sm">
                          {tag}
                        </Badge>
                      ))}
                    </div>
                  </div>

                  {/* Actions */}
                  <div className="flex items-center gap-1 shrink-0">
                    <Button variant="ghost" size="sm" icon={Edit3} onClick={() => openEdit(rule)} />
                    <Button
                      variant="ghost"
                      size="sm"
                      icon={Trash2}
                      className="hover:text-error"
                      onClick={() => setDeletingId(rule.id)}
                    />
                  </div>
                </div>
              </Card>
            );
          })}
          {filtered.length === 0 && !isLoading && (
            <div className="flex items-center justify-center py-16 text-text-secondary">
              No rules found
            </div>
          )}
        </div>
      )}

      {/* Create/Edit Modal */}
      <Modal
        open={activeModal === 'rule-form'}
        onClose={closeModal}
        title={editingId ? 'Edit Rule' : 'New Rule'}
        size="md"
        footer={
          <>
            <Button variant="ghost" onClick={closeModal}>
              取消
            </Button>
            <Button onClick={handleSave} loading={isLoading}>
              {editingId ? 'Save Changes' : 'Create'}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label="名称"
            placeholder="规则名称"
            value={formData.name}
            onChange={(e) => setFormData((f) => ({ ...f, name: e.target.value }))}
          />
          <Select
            label="类型"
            value={formData.type}
            onChange={(v) => setFormData((f) => ({ ...f, type: v }))}
            options={RULE_TYPES.map((t) => ({ label: t, value: t }))}
          />
          <Input
            label="负载"
            placeholder="e.g. google.com, 192.168.0.0/16, CN"
            value={formData.payload}
            onChange={(e) => setFormData((f) => ({ ...f, payload: e.target.value }))}
          />
          <Select
            label="代理"
            value={formData.proxy}
            onChange={(v) => setFormData((f) => ({ ...f, proxy: v }))}
            options={PROXIES.map((p) => ({ label: p, value: p }))}
          />
          <Input
            label="标签"
            placeholder="comma separated"
            value={formData.tags.join(', ')}
            onChange={(e) =>
              setFormData((f) => ({
                ...f,
                tags: e.target.value
                  .split(',')
                  .map((t) => t.trim())
                  .filter(Boolean),
              }))
            }
          />
        </div>
      </Modal>

      {/* Delete Confirmation */}
      <ConfirmDialog
        open={!!deletingId}
        title="删除规则"
        message="确定要删除此规则吗？"
        variant="danger"
        confirmLabel="删除"
        onConfirm={handleDelete}
        onCancel={() => setDeletingId(null)}
      />
    </div>
  );
}
