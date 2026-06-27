import React from 'react';
import { ChevronUp, ChevronDown } from 'lucide-react';
import { cn } from '@/utils';
import type { TableColumn, TableSortDirection } from '@/types';
import { Empty } from './Empty';

interface TableProps<T> {
  columns: TableColumn<T>[];
  data: T[];
  keyExtractor: (item: T) => string;
  onSort?: (key: string) => void;
  sortField?: string | null;
  sortDirection?: TableSortDirection;
  isLoading?: boolean;
  emptyMessage?: string;
  onRowClick?: (item: T) => void;
  className?: string;
}

export function Table<T>({
  columns,
  data,
  keyExtractor,
  onSort,
  sortField,
  sortDirection = 'asc',
  isLoading = false,
  emptyMessage = '暂无数据',
  onRowClick,
  className,
}: TableProps<T>) {
  function renderSortIcon(key: string) {
    if (!onSort || !sortField || sortField !== key) {
      return <ChevronDown size={14} className="opacity-0 group-hover:opacity-30" />;
    }
    return sortDirection === 'asc' ? (
      <ChevronUp size={14} className="text-primary" />
    ) : (
      <ChevronDown size={14} className="text-primary" />
    );
  }

  return (
    <div className={cn('flex min-h-0 flex-1 flex-col overflow-hidden', className)}>
      <div className="scrollbar-thin scrollbar-stable flex-1 overflow-auto">
        <table className="w-full min-w-max text-left text-sm whitespace-nowrap">
          <thead className="sticky top-0 z-10 border-b border-border bg-card text-xs uppercase shadow-sm">
            <tr>
              {columns.map((col) => (
                <th
                  key={col.key}
                  className={cn(
                    'px-4 py-3 text-xs font-semibold uppercase tracking-wide text-text-secondary',
                    col.sortable && onSort && 'cursor-pointer select-none hover:text-text',
                    col.align === 'center' && 'text-center',
                    col.align === 'right' && 'text-right',
                    col.className,
                  )}
                  style={col.width ? { width: col.width } : undefined}
                  onClick={() => {
                    if (col.sortable && onSort) onSort(col.key);
                  }}
                >
                  <div
                    className={cn(
                      'flex items-center gap-1',
                      col.align === 'center' && 'justify-center',
                      col.align === 'right' && 'justify-end',
                    )}
                  >
                    {col.label}
                    {col.sortable && renderSortIcon(col.key)}
                  </div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y divide-border/70">
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <tr key={`skeleton-${i}`}>
                  {columns.map((col) => (
                    <td key={col.key} className="px-4 py-2.5">
                      <div className="h-4 bg-border rounded animate-pulse w-3/4" />
                    </td>
                  ))}
                </tr>
              ))
            ) : data.length === 0 ? (
              <tr>
                <td colSpan={columns.length} className="px-5 py-16">
                  <Empty title={emptyMessage} />
                </td>
              </tr>
            ) : (
              data.map((item) => (
                <tr
                  key={keyExtractor(item)}
                  className={cn('transition-colors hover:bg-bg', onRowClick && 'cursor-pointer')}
                  onClick={() => onRowClick?.(item)}
                >
                  {columns.map((col) => (
                    <td
                      key={col.key}
                      className={cn(
                        'px-5 py-3',
                        col.align === 'center' && 'text-center',
                        col.align === 'right' && 'text-right',
                        col.className,
                      )}
                    >
                      {col.render(item)}
                    </td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
