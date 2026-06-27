import { useMemo } from 'react';
import { Download, Upload, Cpu, HardDrive, Clock, Globe } from 'lucide-react';
import { useDashboardStore } from '@/stores/dashboardStore';
import { Card } from '@/components/common/Card';
import { Toggle } from '@/components/common/Toggle';
import { Sparkline } from '@/components/common/Sparkline';

interface SystemCard {
  key: string;
  icon: React.ComponentType<{ size?: number | string; className?: string }>;
  label: string;
  value: string;
  color: string;
  hoverColor: string;
  highlight?: boolean;
}

export function Dashboard() {
  const isRunning = useDashboardStore((s) => s.isRunning);
  const toggleRunning = useDashboardStore((s) => s.toggleRunning);
  const uploadHistory = useDashboardStore((s) => s.uploadHistory);
  const downloadHistory = useDashboardStore((s) => s.downloadHistory);
  const status = useDashboardStore((s) => s.status);

  const currentDownload = downloadHistory[downloadHistory.length - 1] ?? 0;
  const currentUpload = uploadHistory[uploadHistory.length - 1] ?? 0;

  // Dynamic peaks from actual history data
  const downloadPeak = useMemo(
    () => (downloadHistory.length > 0 ? Math.max(...downloadHistory) : 0),
    [downloadHistory],
  );
  const uploadPeak = useMemo(
    () => (uploadHistory.length > 0 ? Math.max(...uploadHistory) : 0),
    [uploadHistory],
  );

  const SYSTEM_CARDS: SystemCard[] = [
    {
      key: 'cpu',
      icon: Cpu,
      label: 'CPU 使用率',
      value: `${status.cpuUsage.toFixed(1)}%`,
      color: '#9CA3AF',
      hoverColor: '#4F8CFF',
    },
    {
      key: 'memory',
      icon: HardDrive,
      label: '内存',
      value: `${status.memoryUsage.toFixed(0)} MB`,
      color: '#9CA3AF',
      hoverColor: '#4F8CFF',
    },
    {
      key: 'uptime',
      icon: Clock,
      label: '运行时间',
      value: formatUptime(status.uptime),
      color: '#9CA3AF',
      hoverColor: '#4F8CFF',
    },
    {
      key: 'profile',
      icon: Globe,
      label: '当前配置',
      value: status.activeProfileName,
      color: '#4F8CFF',
      hoverColor: '#4F8CFF',
      highlight: true,
    },
  ];

  return (
    <div className="app-page-scroll app-page-stack scrollbar-thin">
      {/* Header */}
      <div className="page-header surface-panel rounded-lg p-5 lg:p-6">
        <div>
          <h1 className="page-title mb-1">总览</h1>
          <p className="text-text-secondary">
            引擎状态：
            <span className={isRunning ? 'text-success' : 'text-error'}>
              {isRunning ? '运行中' : '已停止'}
            </span>
          </p>
        </div>
        <div className="flex w-full items-center justify-between gap-4 rounded-lg border border-border bg-bg p-3 sm:w-auto sm:justify-start sm:pr-4">
          <div className="flex flex-col items-end mr-2">
            <span className="text-sm font-medium text-text">
              系统代理 {isRunning ? 'ON' : 'OFF'}
            </span>
            <span className="text-xs text-text-secondary">端口: {status.port}</span>
          </div>
          <Toggle enabled={isRunning} onChange={toggleRunning} />
        </div>
      </div>

      {/* Main Stats Grid */}
      <div className="grid grid-cols-1 gap-5 xl:grid-cols-2 xl:gap-6">
        {/* Download Speed */}
        <Card className="flex flex-col" padding="none">
          <div className="p-6 pb-2 flex justify-between items-start">
            <div>
              <div className="flex items-center gap-2 mb-1">
                <Download size={18} className="text-primary" />
                <h3 className="text-text-secondary font-medium">下载速度</h3>
              </div>
              <div className="text-3xl font-bold text-text tracking-tight">
                {currentDownload.toFixed(1)}{' '}
                <span className="text-lg text-text-secondary font-normal">MB/s</span>
              </div>
            </div>
            <div className="px-3 py-1 bg-primary/10 text-primary rounded-lg text-sm font-medium">
              峰值 {downloadPeak.toFixed(1)} MB/s
            </div>
          </div>
          <div className="h-32 w-full mt-auto">
            <Sparkline data={downloadHistory} color="var(--color-primary)" />
          </div>
        </Card>

        {/* Upload Speed */}
        <Card className="flex flex-col" padding="none">
          <div className="p-6 pb-2 flex justify-between items-start">
            <div>
              <div className="flex items-center gap-2 mb-1">
                <Upload size={18} className="text-success" />
                <h3 className="text-text-secondary font-medium">上传速度</h3>
              </div>
              <div className="text-3xl font-bold text-text tracking-tight">
                {currentUpload.toFixed(1)}{' '}
                <span className="text-lg text-text-secondary font-normal">MB/s</span>
              </div>
            </div>
            <div className="px-3 py-1 bg-success/10 text-success rounded-lg text-sm font-medium">
              峰值 {uploadPeak.toFixed(1)} MB/s
            </div>
          </div>
          <div className="h-32 w-full mt-auto">
            <Sparkline data={uploadHistory} color="var(--color-success)" />
          </div>
        </Card>
      </div>

      {/* System Status */}
      <h2 className="text-lg font-semibold text-text">系统状态</h2>
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
        {SYSTEM_CARDS.map((card) => (
          <Card key={card.key} hover className={card.highlight ? 'border-primary/50' : ''}>
            <div className="flex items-center gap-4 relative z-10">
              <div
                className={
                  card.highlight
                    ? 'p-3 rounded-lg bg-primary/20 text-primary'
                    : 'p-3 rounded-lg text-text-secondary'
                }
              >
                <card.icon size={24} />
              </div>
              <div>
                <p className="text-sm text-text-secondary">{card.label}</p>
                <p className="text-lg font-bold text-text">{card.value}</p>
              </div>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );
}

function formatUptime(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (h > 0) return `${h} 小时 ${m} 分`;
  return `${m} 分`;
}
