import { useEffect, useMemo } from 'react';
import { Search, X, RefreshCw } from 'lucide-react';
import { useConnectionStore } from '@/stores/connectionStore';
import { useAppStore } from '@/stores/appStore';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Badge } from '@/components/common/Badge';
import { Toggle } from '@/components/common/Toggle';
import { Table } from '@/components/common/Table';
import { Pagination } from '@/components/common/Pagination';
import { ConfirmDialog } from '@/components/common/ConfirmDialog';
import { usePolling } from '@/hooks/usePolling';
import { useToast } from '@/hooks/useToast';
import { formatBytes, formatCompactDuration, cn } from '@/utils';
import type { Connection, TableColumn } from '@/types';

export function Connections() {
  const connections = useConnectionStore((s) => s.connections);
  const searchQuery = useConnectionStore((s) => s.searchQuery);
  const sortField = useConnectionStore((s) => s.sortField);
  const sortDirection = useConnectionStore((s) => s.sortDirection);
  const page = useConnectionStore((s) => s.page);
  const isLoading = useConnectionStore((s) => s.isLoading);
  const autoRefresh = useConnectionStore((s) => s.autoRefresh);

  const fetchConnections = useConnectionStore((s) => s.fetchConnections);
  const closeConnection = useConnectionStore((s) => s.closeConnection);
  const closeAll = useConnectionStore((s) => s.closeAll);
  const setSearchQuery = useConnectionStore((s) => s.setSearchQuery);
  const setSortField = useConnectionStore((s) => s.setSortField);
  const setPage = useConnectionStore((s) => s.setPage);
  const toggleAutoRefresh = useConnectionStore((s) => s.toggleAutoRefresh);

  const openModal = useAppStore((s) => s.openModal);
  const closeModal = useAppStore((s) => s.closeModal);
  const activeModal = useAppStore((s) => s.activeModal);

  const toast = useToast();

  useEffect(() => {
    fetchConnections();
  }, [fetchConnections]);

  usePolling(
    () => {
      fetchConnections();
    },
    3000,
    autoRefresh,
  );

  const filteredAndSorted = useMemo(() => {
    let list = connections.filter((c) => {
      if (!searchQuery) return true;
      const q = searchQuery.toLowerCase();
      return (
        c.process.toLowerCase().includes(q) ||
        c.destination.toLowerCase().includes(q) ||
        c.rule.toLowerCase().includes(q)
      );
    });

    if (sortField) {
      const dir = sortDirection === 'asc' ? 1 : -1;
      list = [...list].sort((a, b) => {
        const aVal = a[sortField as keyof Connection] ?? '';
        const bVal = b[sortField as keyof Connection] ?? '';
        if (typeof aVal === 'number' && typeof bVal === 'number') return (aVal - bVal) * dir;
        if (typeof aVal === 'string' && typeof bVal === 'string')
          return aVal.localeCompare(bVal) * dir;
        return 0;
      });
    }

    return list;
  }, [connections, searchQuery, sortField, sortDirection]);

  const pageSize = 25;
  const totalPages = Math.max(1, Math.ceil(filteredAndSorted.length / pageSize));
  const paged = filteredAndSorted.slice((page - 1) * pageSize, page * pageSize);

  async function handleCloseAll() {
    await closeAll();
    toast.info('全部连接已关闭');
    closeModal();
  }

  function getRuleBadge(rule: string) {
    if (rule.includes('Direct')) return { variant: 'success' as const };
    if (rule.includes('Reject')) return { variant: 'error' as const };
    return { variant: 'info' as const };
  }

  const columns: TableColumn<Connection>[] = [
    {
      key: 'process',
      label: '进程',
      render: (c) => (
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-border rounded-sm flex items-center justify-center text-[8px] font-bold text-text-secondary">
            {c.process[0]}
          </div>
          <span className="text-text font-medium">{c.process}</span>
        </div>
      ),
    },
    {
      key: 'destination',
      label: '地址',
      render: (c) => <span className="font-mono text-xs text-text">{c.destination}</span>,
    },
    {
      key: 'rule',
      label: '规则',
      render: (c) => (
        <Badge variant={getRuleBadge(c.rule).variant} size="sm">
          {c.rule}
        </Badge>
      ),
    },
    {
      key: 'network',
      label: '协议',
      width: '60px',
      align: 'center',
      render: (c) => (
        <span className="bg-border px-1.5 py-0.5 rounded text-[10px] font-mono text-text-secondary">
          {c.network}
        </span>
      ),
    },
    {
      key: 'upload',
      label: '上传',
      sortable: true,
      align: 'right',
      width: '90px',
      render: (c) => (
        <span className="text-xs text-text-secondary">
          {c.upload > 0 ? formatBytes(c.upload) : '0 B'}
        </span>
      ),
    },
    {
      key: 'download',
      label: '下载',
      sortable: true,
      align: 'right',
      width: '100px',
      render: (c) => <span className="text-xs text-text-secondary">{formatBytes(c.download)}</span>,
    },
    {
      key: 'duration',
      label: '时间',
      sortable: true,
      align: 'right',
      width: '90px',
      render: (c) => (
        <span className="font-mono text-xs text-text-secondary">
          {formatCompactDuration(c.duration)}
        </span>
      ),
    },
    {
      key: 'actions',
      label: '',
      width: '40px',
      render: (c) => (
        <button
          onClick={(e) => {
            e.stopPropagation();
            closeConnection(c.id);
          }}
          className="p-1 rounded text-text-secondary hover:text-error hover:bg-error/10 transition-colors"
          title="关闭连接"
        >
          <X size={14} />
        </button>
      ),
    },
  ];

  return (
    <div className="app-page flex flex-col">
      {/* Header */}
      <div className="page-header mb-5">
        <h1 className="page-title">活动连接</h1>
        <div className="page-toolbar md:justify-end">
          <div className="flex items-center gap-2 bg-card border border-border rounded-lg px-3 py-1.5">
            <RefreshCw
              size={14}
              className={cn('text-text-secondary', autoRefresh && 'animate-spin')}
            />
            <span className="text-xs text-text-secondary mr-1">自动刷新</span>
            <Toggle enabled={autoRefresh} onChange={toggleAutoRefresh} size="sm" />
          </div>
          <div className="relative">
            <Search
              size={18}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary pointer-events-none"
            />
            <input
              type="text"
              placeholder="按主机、进程或规则筛选..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full rounded-lg border border-border bg-card py-2 pl-10 pr-4 text-sm text-text transition-colors focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/30 sm:w-72"
            />
          </div>
          <Button
            variant="danger"
            icon={X}
            onClick={() => openModal('close-all')}
            disabled={connections.length === 0}
          >
            Close All
          </Button>
        </div>
      </div>

      {/* Content */}
      <Card padding="none" className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <Table
          columns={columns}
          data={paged}
          keyExtractor={(c) => c.id}
          onSort={setSortField}
          sortField={sortField}
          sortDirection={sortDirection}
          isLoading={isLoading}
          emptyMessage="暂无活动连接"
        />
        <Pagination page={page} totalPages={totalPages} onPageChange={setPage} />
      </Card>

      <ConfirmDialog
        open={activeModal === 'close-all'}
        title="关闭全部连接"
        message="确定要关闭所有活动连接吗？这可能会影响正在进行的网络活动。"
        variant="warning"
        confirmLabel="全部关闭"
        onConfirm={handleCloseAll}
        onCancel={closeModal}
      />
    </div>
  );
}
