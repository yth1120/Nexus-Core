import type { LogEntry, Connection, DashboardRunStatus, DashboardStatus } from './domain';

/** Engine lifecycle state — mirrors the Rust `EngineState` enum (camelCase). */
export type EngineState = 'stopped' | 'starting' | 'running' | 'stopping' | 'error';

/** Proxy routing mode — mirrors the Rust `CoreMode` enum (camelCase). */
export type CoreMode = 'rule' | 'global' | 'direct';

export interface NexusEventMap {
  // --- Phase 1 ---
  'traffic:update': { upload: number; download: number; timestamp: number };
  'log:new': LogEntry;
  'connection:new': Connection;
  'connection:close': { id: string };
  'status:change': DashboardRunStatus;
  'profile:activate': { profileId: string };
  'theme:change': 'dark' | 'light' | 'system';

  // --- Phase 2 ---
  'config:changed': { path: string };
  'dashboard:update': DashboardStatus;
  'statistics:update': {
    cpuUsage: number;
    memoryUsage: number;
    uptime: number;
    activeConnections: number;
  };
  'connection:update': { connections: Connection[] };

  // --- Phase 3 (network core lifecycle) ---
  'core:started': null;
  'core:stopped': null;
  'session:created': { id: string; profileId: string; nodeId: string | null };
  'session:destroyed': { id: string };
  'profile:activated': { profileId: string };
  'profile:deactivated': { profileId: string };
  'node:changed': { nodeId: string };
  'rule:reloaded': { count: number };
  'engine:state': EngineState;
}

export type NexusEventKey = keyof NexusEventMap;

export type NexusEventHandler<K extends NexusEventKey> = (data: NexusEventMap[K]) => void;
