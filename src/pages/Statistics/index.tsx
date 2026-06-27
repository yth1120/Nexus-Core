import { useEffect, useMemo } from 'react';
import { Activity } from 'lucide-react';
import { useStatisticsStore } from '@/stores/statisticsStore';
import { Card } from '@/components/common/Card';
import { Chart } from '@/components/common/Chart';
import { Loading } from '@/components/common/Loading';
import { formatBytes, formatSpeed, formatPercent } from '@/utils';
import type { TimeRange } from '@/types';

const TIME_RANGES: { value: TimeRange; label: string }[] = [
  { value: '7d', label: '最近 7 天' },
  { value: '30d', label: '最近 30 天' },
  { value: '1y', label: '今年' },
];

export function Statistics() {
  const data = useStatisticsStore((s) => s.data);
  const timeRange = useStatisticsStore((s) => s.timeRange);
  const isLoading = useStatisticsStore((s) => s.isLoading);
  const fetchStatistics = useStatisticsStore((s) => s.fetchStatistics);
  const setTimeRange = useStatisticsStore((s) => s.setTimeRange);

  useEffect(() => {
    fetchStatistics();
  }, [fetchStatistics]);

  const chartData = useMemo(() => {
    if (!data) return [];
    return data.history;
  }, [data]);

  if (isLoading && !data) {
    return (
      <div className="app-page">
        <Loading text="加载中..." fullPage />
      </div>
    );
  }

  if (!data) return null;

  return (
    <div className="app-page-scroll app-page-stack scrollbar-thin">
      <h1 className="page-title">统计</h1>

      {/* Top Stats Cards */}
      <div className="grid grid-cols-1 gap-5 lg:grid-cols-3">
        <Card>
          <h3 className="text-text-secondary font-medium mb-2">今日流量</h3>
          <p className="text-3xl font-bold text-text">{formatBytes(data.todayTraffic)}</p>
          <p className="text-sm text-success mt-2 flex items-center gap-1">
            <Activity size={14} /> +12% from yesterday
          </p>
        </Card>

        <Card>
          <h3 className="text-text-secondary font-medium mb-2">本月</h3>
          <p className="text-3xl font-bold text-text">{formatBytes(data.monthTraffic)}</p>
          <div className="w-full bg-border rounded-full h-1.5 mt-4">
            <div
              className="bg-primary h-1.5 rounded-full"
              style={{ width: formatPercent(data.monthTraffic, data.monthQuota) }}
            />
          </div>
          <p className="text-xs text-text-secondary mt-2 text-right">
            {formatPercent(data.monthTraffic, data.monthQuota)} of {formatBytes(data.monthQuota)}{' '}
            quota
          </p>
        </Card>

        <Card>
          <h3 className="text-text-secondary font-medium mb-2">最高速度记录</h3>
          <p className="text-3xl font-bold text-text">{formatSpeed(data.maxSpeed)}</p>
          <p className="text-sm text-text-secondary mt-2">Recorded on {data.maxSpeedDate}</p>
        </Card>
      </div>

      {/* Traffic History Chart */}
      <Card className="flex min-h-[350px] flex-1 flex-col" padding="lg">
        <div className="mb-6 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <h3 className="text-lg font-semibold text-text">流量历史</h3>
          <div className="flex gap-1 rounded-lg bg-border p-1">
            {TIME_RANGES.map((range) => (
              <button
                key={range.value}
                onClick={() => setTimeRange(range.value)}
                className={`px-3 py-1.5 rounded-md text-xs font-medium transition-colors ${timeRange === range.value ? 'bg-primary text-white' : 'text-text-secondary hover:text-text'}`}
              >
                {range.label}
              </button>
            ))}
          </div>
        </div>
        <div className="flex-1">
          <Chart data={chartData} dataKey="download" height={320} color="#4F8CFF" />
        </div>
      </Card>
    </div>
  );
}
