const BYTE_UNITS = ['B', 'KB', 'MB', 'GB', 'TB'] as const;

export function formatBytes(bytes: number, decimals = 2): string {
  if (bytes === 0) return '0 B';
  if (bytes < 0) return '0 B';

  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const idx = Math.min(i, BYTE_UNITS.length - 1);
  const value = bytes / Math.pow(k, idx);
  return `${value.toFixed(idx === 0 ? 0 : decimals)} ${BYTE_UNITS[idx]}`;
}

export function formatSpeed(bytesPerSec: number, decimals = 1): string {
  return `${formatBytes(bytesPerSec, decimals)}/s`;
}

export function formatDuration(totalSeconds: number): string {
  if (totalSeconds < 0) return '0s';

  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  const parts: string[] = [];
  if (hours > 0) parts.push(`${hours}h`);
  if (minutes > 0) parts.push(`${minutes}m`);
  if (seconds > 0 || parts.length === 0) parts.push(`${seconds}s`);

  return parts.join(' ');
}

export function formatCompactDuration(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;

  if (hours > 0) {
    return `${hours}h ${minutes}m ${seconds}s`;
  }
  if (minutes > 0) {
    return `${minutes}m ${seconds}s`;
  }
  return `${seconds}s`;
}

export function formatTimestamp(
  ts: number,
  fmt: 'time' | 'datetime' | 'relative' = 'time',
): string {
  const date = new Date(ts);

  switch (fmt) {
    case 'time':
      return date.toLocaleTimeString('en-US', {
        hour12: false,
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
      });
    case 'datetime':
      return date.toLocaleString('en-US', {
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
        hour12: false,
      });
    case 'relative': {
      const now = Date.now();
      const diff = now - ts;
      const seconds = Math.floor(diff / 1000);
      if (seconds < 60) return `${seconds}s ago`;
      const minutes = Math.floor(seconds / 60);
      if (minutes < 60) return `${minutes}m ago`;
      const hours = Math.floor(minutes / 60);
      if (hours < 24) return `${hours}h ago`;
      const days = Math.floor(hours / 24);
      return `${days}d ago`;
    }
  }
}

export function formatPercent(value: number, total: number): string {
  if (total === 0) return '0%';
  return `${Math.round((value / total) * 100)}%`;
}

export function formatCpuUsage(value: number): string {
  return `${value.toFixed(1)}%`;
}

export function formatMemoryUsage(mb: number): string {
  if (mb >= 1024) return `${(mb / 1024).toFixed(1)} GB`;
  return `${Math.round(mb)} MB`;
}

export function formatLatency(ms: number | null): string {
  if (ms === null) return '---';
  if (ms === 9999) return 'Timeout';
  return `${ms} ms`;
}

export function formatLoss(percent: number | null): string {
  if (percent === null) return '---';
  return `${percent.toFixed(1)}%`;
}
