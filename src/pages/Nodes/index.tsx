import { useEffect, useMemo } from 'react';
import { Search, Star, Zap, Plug, PlugZap } from 'lucide-react';
import { useNodeStore } from '@/stores/nodeStore';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { Badge } from '@/components/common/Badge';
import { Table } from '@/components/common/Table';
import { Loading } from '@/components/common/Loading';
import { useToast } from '@/hooks/useToast';
import { formatLatency, formatLoss, cn } from '@/utils';
import type { Node, TableColumn } from '@/types';

const COUNTRY_FLAGS: Record<string, string> = {
  HK: '🇭🇰',
  JP: '🇯🇵',
  SG: '🇸🇬',
  US: '🇺🇸',
  KR: '🇰🇷',
  TW: '🇹🇼',
  DE: '🇩🇪',
  UK: '🇬🇧',
  AU: '🇦🇺',
  IN: '🇮🇳',
  BR: '🇧🇷',
  CN: '🇨🇳',
};

export function Nodes() {
  const nodes = useNodeStore((s) => s.nodes);
  const searchQuery = useNodeStore((s) => s.searchQuery);
  const groupFilter = useNodeStore((s) => s.groupFilter);
  const sortField = useNodeStore((s) => s.sortField);
  const sortDirection = useNodeStore((s) => s.sortDirection);
  const isLoading = useNodeStore((s) => s.isLoading);

  const fetchNodes = useNodeStore((s) => s.fetchNodes);
  const toggleFavorite = useNodeStore((s) => s.toggleFavorite);
  const testDelay = useNodeStore((s) => s.testDelay);
  const testAllDelay = useNodeStore((s) => s.testAllDelay);
  const connectNode = useNodeStore((s) => s.connect);
  const disconnectNode = useNodeStore((s) => s.disconnect);
  const setSearchQuery = useNodeStore((s) => s.setSearchQuery);
  const setGroupFilter = useNodeStore((s) => s.setGroupFilter);
  const setSort = useNodeStore((s) => s.setSort);

  const toast = useToast();

  useEffect(() => {
    fetchNodes();
  }, [fetchNodes]);

  const groups = useMemo(() => {
    const set = new Set(nodes.map((n) => n.group));
    return Array.from(set).sort();
  }, [nodes]);

  const filteredAndSorted = useMemo(() => {
    let list = nodes.filter((n) => {
      if (
        searchQuery &&
        !n.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !n.country.toLowerCase().includes(searchQuery.toLowerCase())
      )
        return false;
      if (groupFilter && n.group !== groupFilter) return false;
      return true;
    });

    if (sortField) {
      const dir = sortDirection === 'asc' ? 1 : -1;
      list = [...list].sort((a, b) => {
        const aVal = sortField === 'name' ? a.name : (a[sortField] ?? 0);
        const bVal = sortField === 'name' ? b.name : (b[sortField] ?? 0);
        if (typeof aVal === 'string' && typeof bVal === 'string')
          return aVal.localeCompare(bVal) * dir;
        return ((aVal as number) - (bVal as number)) * dir;
      });
    }

    return list;
  }, [nodes, searchQuery, groupFilter, sortField, sortDirection]);

  const columns: TableColumn<Node>[] = [
    {
      key: 'country',
      label: '国家',
      width: '80px',
      render: (n) => (
        <span className="text-lg">
          {COUNTRY_FLAGS[n.countryCode] ?? '🏳️'} {n.country}
        </span>
      ),
    },
    {
      key: 'name',
      label: '名称',
      sortable: true,
      render: (n) => (
        <div className="flex items-center gap-2">
          <span className="text-text font-medium">{n.name}</span>
          {n.isFavorite && <Star size={12} className="text-warning fill-warning" />}
        </div>
      ),
    },
    {
      key: 'delay',
      label: '延迟',
      sortable: true,
      align: 'right',
      width: '100px',
      render: (n) => {
        const color =
          n.delay === null
            ? 'text-text-secondary'
            : n.delay < 100
              ? 'text-success'
              : n.delay < 200
                ? 'text-warning'
                : 'text-error';
        return (
          <span className={cn('font-mono text-sm font-medium', color)}>
            {formatLatency(n.delay)}
          </span>
        );
      },
    },
    {
      key: 'loss',
      label: '丢包',
      sortable: true,
      align: 'right',
      width: '80px',
      render: (n) => {
        const color =
          n.loss === null
            ? 'text-text-secondary'
            : n.loss < 1
              ? 'text-success'
              : n.loss < 3
                ? 'text-warning'
                : 'text-error';
        return <span className={cn('font-mono text-sm', color)}>{formatLoss(n.loss)}</span>;
      },
    },
    {
      key: 'status',
      label: '状态',
      align: 'center',
      width: '90px',
      render: (n) => {
        const map = {
          online: { variant: 'success' as const, label: '在线' },
          offline: { variant: 'error' as const, label: '离线' },
          untested: { variant: 'default' as const, label: '未测试' },
        };
        const s = map[n.status];
        return (
          <Badge variant={s.variant} dot size="sm">
            {s.label}
          </Badge>
        );
      },
    },
    {
      key: 'actions',
      label: '',
      width: '180px',
      align: 'right',
      render: (n) => (
        <div className="flex items-center justify-end gap-1">
          <Button
            variant="ghost"
            size="sm"
            onClick={(e) => {
              e?.stopPropagation?.();
              toggleFavorite(n.id);
            }}
          >
            <Star size={14} className={n.isFavorite ? 'fill-warning text-warning' : ''} />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            icon={Zap}
            onClick={(e) => {
              e?.stopPropagation?.();
              testDelay(n.id);
            }}
          />
          {n.isConnected ? (
            <Button
              variant="secondary"
              size="sm"
              icon={PlugZap}
              onClick={(e) => {
                e?.stopPropagation?.();
                disconnectNode(n.id);
              }}
            >
              Disconnect
            </Button>
          ) : (
            <Button
              variant="primary"
              size="sm"
              icon={Plug}
              onClick={(e) => {
                e?.stopPropagation?.();
                connectNode(n.id);
              }}
            >
              Connect
            </Button>
          )}
        </div>
      ),
    },
  ];

  return (
    <div className="app-page flex flex-col">
      {/* Header */}
      <div className="page-header mb-5">
        <h1 className="page-title">节点</h1>
        <div className="page-toolbar md:justify-end">
          <div className="relative">
            <Search
              size={18}
              className="absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary pointer-events-none"
            />
            <input
              type="text"
              placeholder="搜索节点..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full rounded-lg border border-border bg-card py-2 pl-10 pr-4 text-sm text-text transition-colors focus:border-primary focus:outline-none focus:ring-2 focus:ring-primary/30 sm:w-56"
            />
          </div>
          <select
            value={groupFilter ?? ''}
            onChange={(e) => setGroupFilter(e.target.value || null)}
            className="rounded-lg border border-border bg-card px-3 py-2 text-sm text-text focus:outline-none focus:ring-2 focus:ring-primary/30 sm:min-w-36"
          >
            <option value="">所有分组</option>
            {groups.map((g) => (
              <option key={g} value={g}>
                {g}
              </option>
            ))}
          </select>
          <Button
            variant="secondary"
            icon={Zap}
            onClick={() => {
              testAllDelay();
              toast.info('正在测试全部节点...');
            }}
          >
            Test All
          </Button>
        </div>
      </div>

      {/* Content */}
      {isLoading && nodes.length === 0 ? (
        <Loading text="加载中..." />
      ) : (
        <Card padding="none" className="flex min-h-0 flex-1 flex-col overflow-hidden">
          <Table
            columns={columns}
            data={filteredAndSorted}
            keyExtractor={(n) => n.id}
            onSort={(key) => setSort(key as 'delay' | 'loss' | 'name')}
            sortField={sortField}
            sortDirection={sortDirection}
            emptyMessage="未找到节点"
          />
        </Card>
      )}
    </div>
  );
}
