import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';
import type { TrafficDataPoint } from '@/types';
import { formatBytes } from '@/utils';

interface ChartProps {
  data: TrafficDataPoint[];
  dataKey: 'upload' | 'download';
  height?: number;
  color?: string;
}

export function Chart({ data, dataKey, height = 300, color = '#4F8CFF' }: ChartProps) {
  const chartData = data.map((d) => ({
    ...d,
    formatted: formatBytes(d[dataKey]),
  }));

  return (
    <ResponsiveContainer width="100%" height={height}>
      <AreaChart data={chartData} margin={{ top: 10, right: 10, left: 0, bottom: 0 }}>
        <defs>
          <linearGradient id={`gradient-${dataKey}`} x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor={color} stopOpacity={0.3} />
            <stop offset="95%" stopColor={color} stopOpacity={0.02} />
          </linearGradient>
        </defs>
        <CartesianGrid strokeDasharray="3 3" stroke="#2A2F3A" vertical={false} />
        <XAxis
          dataKey="timestamp"
          tick={{ fill: '#9CA3AF', fontSize: 11 }}
          tickFormatter={(ts: number) => {
            const date = new Date(ts);
            return `${date.getMonth() + 1}/${date.getDate()}`;
          }}
          axisLine={{ stroke: '#2A2F3A' }}
          tickLine={false}
        />
        <YAxis
          tick={{ fill: '#9CA3AF', fontSize: 11 }}
          tickFormatter={(v: number) => formatBytes(v)}
          axisLine={false}
          tickLine={false}
          width={70}
        />
        <Tooltip
          contentStyle={{
            backgroundColor: '#171A21',
            border: '1px solid #2A2F3A',
            borderRadius: '12px',
            color: '#FFFFFF',
            fontSize: '13px',
          }}
          labelFormatter={(ts: number) => new Date(ts).toLocaleString()}
          formatter={(value: number) => [
            formatBytes(value),
            dataKey === 'download' ? 'Download' : 'Upload',
          ]}
        />
        <Area
          type="monotone"
          dataKey={dataKey}
          stroke={color}
          strokeWidth={2}
          fill={`url(#gradient-${dataKey})`}
          dot={false}
          activeDot={{ r: 4, fill: color, stroke: '#0F1115', strokeWidth: 2 }}
        />
      </AreaChart>
    </ResponsiveContainer>
  );
}
