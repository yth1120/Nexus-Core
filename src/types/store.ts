import type {
  Profile,
  Node,
  Connection,
  LogEntry,
  LogLevel,
  Rule,
  DashboardStatus,
  StatisticsData,
  TimeRange,
} from './domain';
import type { Toast } from './component';

// ===== App Store =====

export interface AppStore {
  sidebarOpen: boolean;
  toasts: Toast[];
  activeModal: string | null;
  modalData: unknown;
  addToast: (toast: Omit<Toast, 'id' | 'duration'> & { duration?: number }) => void;
  removeToast: (id: string) => void;
  toggleSidebar: () => void;
  openModal: (id: string, data?: unknown) => void;
  closeModal: () => void;
}

// ===== Dashboard Store =====

export interface DashboardStore {
  isRunning: boolean;
  status: DashboardStatus;
  uploadHistory: number[];
  downloadHistory: number[];
  toggleRunning: () => void;
  updateStatus: (partial: Partial<DashboardStatus>) => void;
  pushTrafficData: (up: number, down: number) => void;
}

// ===== Profile Store =====

export interface ProfileStore {
  profiles: Profile[];
  searchQuery: string;
  isLoading: boolean;
  error: string | null;
  fetchProfiles: () => Promise<void>;
  createProfile: (
    data: Omit<Profile, 'id' | 'status' | 'latency' | 'updated' | 'trafficUsed' | 'trafficTotal'>,
  ) => Promise<void>;
  updateProfile: (id: string, data: Partial<Profile>) => Promise<void>;
  deleteProfile: (id: string) => Promise<void>;
  setSearchQuery: (q: string) => void;
}

// ===== Node Store =====

export interface NodeStore {
  nodes: Node[];
  searchQuery: string;
  groupFilter: string | null;
  sortField: 'delay' | 'loss' | 'name' | null;
  sortDirection: 'asc' | 'desc';
  isLoading: boolean;
  fetchNodes: () => Promise<void>;
  toggleFavorite: (id: string) => Promise<void>;
  testDelay: (id: string) => Promise<void>;
  testAllDelay: () => Promise<void>;
  connect: (id: string) => Promise<void>;
  disconnect: (id: string) => Promise<void>;
  setSearchQuery: (q: string) => void;
  setGroupFilter: (g: string | null) => void;
  setSort: (field: 'delay' | 'loss' | 'name' | null) => void;
}

// ===== Connection Store =====

export interface ConnectionStore {
  connections: Connection[];
  searchQuery: string;
  sortField: string | null;
  sortDirection: 'asc' | 'desc';
  page: number;
  pageSize: number;
  totalCount: number;
  isLoading: boolean;
  autoRefresh: boolean;
  fetchConnections: () => Promise<void>;
  closeConnection: (id: string) => Promise<void>;
  closeAll: () => Promise<void>;
  setSearchQuery: (q: string) => void;
  setSortField: (field: string | null) => void;
  setPage: (page: number) => void;
  toggleAutoRefresh: () => void;
}

// ===== Log Store =====

export interface LogStore {
  logs: LogEntry[];
  levelFilter: LogLevel | 'ALL';
  autoScroll: boolean;
  maxLogs: number;
  addLog: (entry: LogEntry) => void;
  setLevelFilter: (level: LogLevel | 'ALL') => void;
  toggleAutoScroll: () => void;
  clearLogs: () => void;
  setMaxLogs: (n: number) => void;
}

// ===== Statistics Store =====

export interface StatisticsStore {
  data: StatisticsData | null;
  timeRange: TimeRange;
  isLoading: boolean;
  fetchStatistics: () => Promise<void>;
  setTimeRange: (range: TimeRange) => void;
}

// ===== Settings Store =====

export type ThemeMode = 'dark' | 'light' | 'system';

export interface SettingsStore {
  theme: ThemeMode;
  values: Record<string, unknown>;
  setTheme: (theme: ThemeMode) => void;
  updateValue: (key: string, value: unknown) => void;
  resetToDefaults: () => void;
  hydrate: () => void;
}

// ===== Rule Store =====

export interface RuleStore {
  rules: Rule[];
  searchQuery: string;
  tagFilter: string | null;
  isLoading: boolean;
  fetchRules: () => Promise<void>;
  createRule: (data: Omit<Rule, 'id' | 'createdAt'>) => Promise<void>;
  updateRule: (id: string, data: Partial<Rule>) => Promise<void>;
  deleteRule: (id: string) => Promise<void>;
  toggleEnabled: (id: string) => Promise<void>;
  setSearchQuery: (q: string) => void;
  setTagFilter: (tag: string | null) => void;
}
