interface SparklineProps {
  data: number[];
  color: string;
  fillOpacity?: number;
  height?: number;
}

export function Sparkline({ data, color, fillOpacity = 0.1, height = 128 }: SparklineProps) {
  if (!data || data.length === 0) return null;

  const max = Math.max(...data, 1);
  const min = Math.min(...data);
  const range = max - min || 1;

  const width = 100;
  const h = 100;

  const points = data
    .map((d, i) => {
      const x = data.length === 1 ? width / 2 : (i / (data.length - 1)) * width;
      const y = h - ((d - min) / range) * h;
      return `${x},${y}`;
    })
    .join(' ');

  return (
    <svg
      width="100%"
      height={height}
      preserveAspectRatio="none"
      viewBox={`0 -10 ${width} 120`}
      className="opacity-80"
    >
      <polyline
        points={points}
        fill="none"
        stroke={color}
        strokeWidth="2.5"
        strokeLinecap="round"
        strokeLinejoin="round"
        vectorEffect="non-scaling-stroke"
      />
      <path d={`M 0,${h} L ${points} L ${width},${h} Z`} fill={color} fillOpacity={fillOpacity} />
    </svg>
  );
}
