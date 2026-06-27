import { useEffect, useRef, useMemo } from 'react';
import { Search, Download, Trash2, ArrowDown } from 'lucide-react';
import { useLogStore } from '@/stores/logStore';
import { useAppStore } from '@/stores/appStore';
import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { ConfirmDialog } from '@/components/common/ConfirmDialog';
import { useToast } from '@/hooks/useToast';
import { cn, formatTimestamp } from '@/utils';
import type { LogLevel } from '@/types';

const LOG_LEVELS = ['ALL', 'TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR'] as const;

const LEVEL_LABELS: Record<string, string> = {
  ALL: '全部',
  TRACE: 'TRACE',
  DEBUG: 'DEBUG',
  INFO: 'INFO',
  WARN: 'WARN',
  ERROR: 'ERROR',
};

const levelColors: Record<LogLevel | 'ALL', string> = {
  ALL: '',
  TRACE: 'text-text-secondary/50',
  DEBUG: 'text-text-secondary/70',
  INFO: 'text-primary',
  WARN: 'text-warning',
  ERROR: 'text-error',
};

const levelBg: Record<LogLevel | 'ALL', string> = {
  ALL: '',
  TRACE: '',
  DEBUG: '',
  INFO: '',
  WARN: '',
  ERROR: 'text-error/90',
};

export function Logs() {
  const logs = useLogStore((s) => s.logs);
  const levelFilter = useLogStore((s) => s.levelFilter);
  const autoScroll = useLogStore((s) => s.autoScroll);
  const setLevelFilter = useLogStore((s) => s.setLevelFilter);
  const toggleAutoScroll = useLogStore((s) => s.toggleAutoScroll);
  const clearLogs = useLogStore((s) => s.clearLogs);

  const openModal = useAppStore((s) => s.openModal);
  const closeModal = useAppStore((s) => s.closeModal);
  const activeModal = useAppStore((s) => s.activeModal);

  const toast = useToast();
  const scrollRef = useRef<HTMLDivElement>(null);

  const filtered = useMemo(() => {
    if (levelFilter === 'ALL') return logs;
    return logs.filter((l) => l.level === levelFilter);
  }, [logs, levelFilter]);

  // Auto-scroll
  useEffect(() => {
    if (autoScroll && scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [filtered, autoScroll]);

  function handleClear() {
    clearLogs();
    toast.info('日志已清空');
    closeModal();
  }

  function handleExport() {
    const text = filtered
      .map((l) => `[${formatTimestamp(l.timestamp, 'time')}] [${l.level}] ${l.message}`)
      .join('\n');
    const blob = new Blob([text], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `nexus-core-logs-${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.txt`;
    a.click();
    URL.revokeObjectURL(url);
    toast.success('日志已导出');
  }

  return (
    <div className="app-page flex flex-col">
      {/* Header */}
      <div className="page-header mb-5">
        <h1 className="page-title">系统日志</h1>
        <div className="page-toolbar md:justify-end">
          {/* Level Filter */}
          <div className="scrollbar-thin flex max-w-full items-center gap-1 overflow-x-auto rounded-lg border border-border bg-card p-1">
            {LOG_LEVELS.map((level) => (
              <button
                key={level}
                onClick={() => setLevelFilter(level)}
                className={cn(
                  'px-3 py-1.5 rounded-md text-xs font-medium transition-colors',
                  levelFilter === level
                    ? 'bg-primary text-white'
                    : 'text-text-secondary hover:text-text hover:bg-border',
                )}
              >
                {LEVEL_LABELS[level]}
              </button>
            ))}
          </div>
          <Button variant="secondary" icon={Download} onClick={handleExport}>
            Export
          </Button>
          <Button variant="ghost" icon={Trash2} onClick={() => openModal('clear-logs')}>
            Clear
          </Button>
        </div>
      </div>

      {/* Auto-scroll indicator */}
      <div className="flex items-center gap-2 mb-3">
        <button
          onClick={toggleAutoScroll}
          className={cn(
            'flex items-center gap-1.5 text-xs font-medium transition-colors px-2 py-1 rounded',
            autoScroll ? 'text-success' : 'text-text-secondary hover:text-text',
          )}
        >
          <ArrowDown size={12} />
          Auto-scroll {autoScroll ? 'ON' : 'OFF'}
        </button>
        <span className="text-xs text-text-secondary/50">{filtered.length} entries</span>
      </div>

      {/* Log Stream */}
      <Card padding="none" className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <div
          ref={scrollRef}
          className="scrollbar-thin flex flex-1 flex-col gap-1 overflow-y-auto bg-bg p-4 font-mono text-sm"
        >
          {filtered.map((log) => (
            <div
              key={log.id}
              className="flex gap-4 hover:bg-card/50 px-2 py-1 rounded transition-colors"
            >
              <span className="text-text-secondary shrink-0">
                {formatTimestamp(log.timestamp, 'time')}
              </span>
              <span className={cn('shrink-0 w-16 font-bold', levelColors[log.level])}>
                [{log.level}]
              </span>
              <span
                className={cn(log.level === 'ERROR' ? 'text-error/90' : 'text-text', 'break-all')}
              >
                {log.message}
              </span>
            </div>
          ))}
          {/* Blinking cursor */}
          <div className="flex gap-4 px-2 py-1">
            <span className="w-2 h-4 bg-primary animate-cursor-blink mt-1" />
          </div>
        </div>
      </Card>

      <ConfirmDialog
        open={activeModal === 'clear-logs'}
        title="清空日志"
        message="确定要清空所有日志吗？"
        variant="warning"
        confirmLabel="全部清空"
        onConfirm={handleClear}
        onCancel={closeModal}
      />
    </div>
  );
}
