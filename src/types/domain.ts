// ===== Profile =====

export type ProfileStatus = 'active' | 'inactive' | 'error';
export type ProfileType = 'Subscription' | 'WireGuard' | 'VLESS' | 'Clash Meta' | 'Custom';

export interface Profile {
  id: string;
  name: string;
  type: ProfileType;
  status: ProfileStatus;
  latency: number;
  updated: string;
  configUrl?: string;
  trafficUsed?: number;
  trafficTotal?: number;
}

// ===== Node =====

export type NodeStatus = 'online' | 'offline' | 'untested';

export interface Node {
  id: string;
  name: string;
  country: string;
  countryCode: string;
  delay: number | null; // ms
  loss: number | null; // 0-100 %
  status: NodeStatus;
  isFavorite: boolean;
  isConnected: boolean;
  type: string;
  group: string;
}

// ===== Connection =====

export type NetworkProtocol = 'TCP' | 'UDP';

export interface Connection {
  id: string;
  process: string;
  source: string;
  destination: string;
  rule: string;
  network: NetworkProtocol;
  upload: number; // bytes
  download: number; // bytes
  duration: number; // seconds
  createdAt: number; // Unix timestamp
}

// ===== Log =====

export type LogLevel = 'TRACE' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';

export interface LogEntry {
  id: string;
  timestamp: number; // Unix timestamp with ms
  level: LogLevel;
  message: string;
}

// ===== Rule =====

export interface Rule {
  id: string;
  name: string;
  type: string;
  payload: string;
  proxy: string;
  enabled: boolean;
  tags: string[];
  createdAt: number;
}

// ===== Traffic =====

export interface TrafficDataPoint {
  timestamp: number;
  upload: number; // bytes/sec
  download: number; // bytes/sec
}

// ===== Dashboard =====

export type DashboardRunStatus = 'running' | 'stopped' | 'connecting';

export interface DashboardStatus {
  status: DashboardRunStatus;
  cpuUsage: number;
  memoryUsage: number; // MB
  uptime: number; // seconds
  activeConnections: number;
  activeProfileName: string;
  activeNodeName: string;
  ipAddress: string;
  country: string;
  port: number;
}

// ===== Statistics =====

export type TimeRange = '7d' | '30d' | '1y';

export interface StatisticsData {
  todayTraffic: number; // bytes
  monthTraffic: number; // bytes
  monthQuota: number; // bytes
  maxSpeed: number; // bytes/sec
  maxSpeedDate: string;
  history: TrafficDataPoint[];
  dailyAverages: number[];
}

// ===== Settings =====

export interface SettingsCategory {
  id: string;
  titleKey: string;
  icon: string;
}

export type SettingFieldType = 'toggle' | 'text' | 'number' | 'select' | 'button';

export interface SettingSelectOption {
  labelKey: string;
  value: string;
}

export interface SettingField {
  key: string;
  labelKey: string;
  descriptionKey: string;
  type: SettingFieldType;
  defaultValue: unknown;
  options?: SettingSelectOption[];
}

export interface SettingsSection {
  categoryId: string;
  id: string;
  titleKey: string;
  fields: SettingField[];
}
