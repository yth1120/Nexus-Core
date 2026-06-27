import { useState, useEffect } from 'react';
import { Search, Plus, Download, Trash2, Edit3, Activity } from 'lucide-react';
import { useProfileStore } from '@/stores/profileStore';
import { useAppStore } from '@/stores/appStore';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Badge } from '@/components/common/Badge';
import { Input } from '@/components/common/Input';
import { Select } from '@/components/common/Select';
import { Modal } from '@/components/common/Modal';
import { ConfirmDialog } from '@/components/common/ConfirmDialog';
import { Loading } from '@/components/common/Loading';
import { useToast } from '@/hooks/useToast';
import type { Profile, ProfileStatus, ProfileType } from '@/types';

const PROFILE_TYPES: ProfileType[] = ['Subscription', 'WireGuard', 'VLESS', 'Clash Meta', 'Custom'];

const EMPTY_FORM: Omit<Profile, 'id'> = {
  name: '',
  type: 'Subscription',
  status: 'inactive',
  latency: 0,
  updated: new Date().toISOString(),
  configUrl: '',
};

export function Profiles() {
  const profiles = useProfileStore((s) => s.profiles);
  const searchQuery = useProfileStore((s) => s.searchQuery);
  const isLoading = useProfileStore((s) => s.isLoading);
  const fetchProfiles = useProfileStore((s) => s.fetchProfiles);
  const createProfile = useProfileStore((s) => s.createProfile);
  const updateProfile = useProfileStore((s) => s.updateProfile);
  const deleteProfile = useProfileStore((s) => s.deleteProfile);
  const setSearchQuery = useProfileStore((s) => s.setSearchQuery);

  const openModal = useAppStore((s) => s.openModal);
  const closeModal = useAppStore((s) => s.closeModal);
  const activeModal = useAppStore((s) => s.activeModal);
  const modalData = useAppStore((s) => s.modalData);

  const toast = useToast();

  const [formData, setFormData] = useState<typeof EMPTY_FORM>(EMPTY_FORM);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [deletingId, setDeletingId] = useState<string | null>(null);

  useEffect(() => {
    fetchProfiles();
  }, [fetchProfiles]);

  const filtered = profiles.filter(
    (p) =>
      !searchQuery ||
      p.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      p.type.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  function openCreate() {
    setEditingId(null);
    setFormData({ ...EMPTY_FORM });
    openModal('profile-form');
  }

  function openEdit(profile: Profile) {
    setEditingId(profile.id);
    setFormData({
      name: profile.name,
      type: profile.type,
      status: profile.status,
      latency: profile.latency,
      updated: profile.updated,
      configUrl: profile.configUrl ?? '',
    });
    openModal('profile-form');
  }

  async function handleSave() {
    try {
      if (editingId) {
        await updateProfile(editingId, formData);
        toast.success('配置已更新');
      } else {
        await createProfile(formData);
        toast.success('配置已创建');
      }
      closeModal();
    } catch {
      toast.error('保存失败');
    }
  }

  async function handleDelete() {
    if (!deletingId) return;
    try {
      await deleteProfile(deletingId);
      toast.info('配置已删除');
      setDeletingId(null);
    } catch {
      toast.error('删除失败');
    }
  }

  async function handleToggleActive(id: string) {
    const profile = profiles.find((p) => p.id === id);
    if (!profile) return;
    try {
      await updateProfile(id, {
        status: profile.status === 'active' ? 'inactive' : 'active',
      });
      toast.info(profile.status === 'active' ? '已断开连接' : '配置已激活');
    } catch {
      toast.error('切换失败');
    }
  }

  function getStatusBadge(status: ProfileStatus) {
    if (status === 'active') return { variant: 'info' as const, label: '活跃' };
    if (status === 'error') return { variant: 'error' as const, label: '错误' };
    return { variant: 'default' as const, label: '未激活' };
  }

  function getLatencyColor(latency: number) {
    if (latency < 100) return 'text-success';
    if (latency < 200) return 'text-warning';
    return 'text-error';
  }

  return (
    <div className="app-page flex flex-col">
      {/* Header */}
      <div className="page-header mb-5">
        <h1 className="page-title">配置</h1>
        <div className="page-toolbar md:justify-end">
          <div className="relative">
            <Search
              size={18}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary pointer-events-none"
            />
            <input
              type="text"
              placeholder="搜索配置..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full rounded-lg border border-border bg-card py-2 pl-10 pr-4 text-sm text-text transition-colors focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/30 sm:w-64"
            />
          </div>
          <Button variant="secondary" icon={Download}>
            Import
          </Button>
          <Button icon={Plus} onClick={openCreate}>
            New Profile
          </Button>
        </div>
      </div>

      {/* Content */}
      {isLoading && profiles.length === 0 ? (
        <Loading text="加载中..." />
      ) : (
        <div className="scrollbar-thin grid flex-1 auto-rows-max grid-cols-1 gap-4 overflow-y-auto pr-1 xl:grid-cols-2">
          {filtered.map((profile) => {
            const statusBadge = getStatusBadge(profile.status);
            return (
              <Card
                key={profile.id}
                hover
                className={profile.status === 'active' ? 'border-primary/50 bg-primary/5' : ''}
              >
                <div className="flex justify-between items-start">
                  <div>
                    <div className="flex items-center gap-2 mb-1">
                      <h3 className="text-lg font-semibold text-text">{profile.name}</h3>
                      <Badge variant={statusBadge.variant} dot size="sm">
                        {statusBadge.label}
                      </Badge>
                    </div>
                    <p className="text-sm text-text-secondary flex items-center gap-2">
                      <span className="bg-border px-2 py-0.5 rounded text-xs">{profile.type}</span>
                      <span>
                        Updated{' '}
                        {new Date(profile.updated).toLocaleTimeString('zh-CN', {
                          hour: '2-digit',
                          minute: '2-digit',
                        })}
                      </span>
                    </p>
                  </div>
                  <div className="flex items-center gap-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      icon={Edit3}
                      onClick={() => openEdit(profile)}
                    />
                    <Button
                      variant="ghost"
                      size="sm"
                      icon={Trash2}
                      className="hover:text-error"
                      onClick={() => setDeletingId(profile.id)}
                    />
                  </div>
                </div>

                <div className="flex justify-between items-center mt-4 pt-4 border-t border-border/50">
                  <div className="flex items-center gap-2">
                    <Activity size={16} className={getLatencyColor(profile.latency)} />
                    <span className="text-sm font-medium text-text">{profile.latency} ms</span>
                  </div>
                  <Button
                    variant={profile.status === 'active' ? 'secondary' : 'primary'}
                    size="sm"
                    onClick={() => handleToggleActive(profile.id)}
                  >
                    {profile.status === 'active' ? 'Disconnect' : 'Connect'}
                  </Button>
                </div>
              </Card>
            );
          })}
        </div>
      )}

      {/* Create/Edit Modal */}
      <Modal
        open={activeModal === 'profile-form'}
        onClose={closeModal}
        title={editingId ? 'Edit Profile' : 'New Profile'}
        size="md"
        footer={
          <>
            <Button variant="ghost" onClick={closeModal}>
              Cancel
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
            placeholder="配置名称"
            value={formData.name}
            onChange={(e) => setFormData((f) => ({ ...f, name: e.target.value }))}
          />
          <Select
            label="类型"
            value={formData.type}
            onChange={(v) => setFormData((f) => ({ ...f, type: v as ProfileType }))}
            options={PROFILE_TYPES.map((t) => ({ label: t, value: t }))}
          />
          <Input
            label="配置地址"
            placeholder="https://..."
            value={formData.configUrl ?? ''}
            onChange={(e) => setFormData((f) => ({ ...f, configUrl: e.target.value }))}
          />
        </div>
      </Modal>

      {/* Delete Confirmation */}
      <ConfirmDialog
        open={!!deletingId}
        title="删除配置"
        message="确定要删除此配置吗？此操作不可撤销。"
        variant="danger"
        confirmLabel="删除"
        onConfirm={handleDelete}
        onCancel={() => setDeletingId(null)}
      />
    </div>
  );
}
