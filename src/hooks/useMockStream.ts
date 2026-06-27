import { useEffect, useRef } from 'react';
import type { LogEntry, LogLevel, Connection, NetworkProtocol } from '@/types';
import { useDashboardStore } from '@/stores/dashboardStore';
import { useLogStore } from '@/stores/logStore';
import { useConnectionStore } from '@/stores/connectionStore';
import { eventBus } from '@/events';

let logCounter = 100;
let connCounter = 100;

const PROCESSES = [
  'Telegram.exe',
  'WeChat.exe',
  'svchost.exe',
  'Code.exe',
  'chrome.exe',
  'Discord.exe',
  'CoreSync.exe',
  'msedge.exe',
  'firefox.exe',
  'Slack.exe',
];

const DESTINATIONS = [
  'gateway.icloud.com',
  'mtalk.google.com',
  'cdn.discordapp.com',
  '149.154.167.50:443',
  'api.twitter.com',
  'push.apple.com',
  'stun.l.google.com:19302',
  'api.github.com',
  's3.amazonaws.com',
  'cloudflare-dns.com',
];

const RULE_NAMES = [
  'Proxy (HK-01)',
  'Proxy (Auto)',
  'Direct (LAN)',
  'Reject (Ads)',
  'Direct (CN IP)',
  'Proxy (JP-01)',
  'Proxy (SG-01)',
  'DIRECT',
];

const NETWORKS: NetworkProtocol[] = ['TCP', 'UDP'];

const LOG_MESSAGES: Record<LogLevel, string[]> = {
  TRACE: [
    '[TCP] send buffer: 4096 bytes to 127.0.0.1:7890',
    '[DNS] cache lookup: google.com -> 142.250.80.46 (TTL: 245s)',
    '[TLS] ClientHello sent to cdn.discordapp.com',
    '[RULE] Checking domain against rule set (index: 3)',
  ],
  DEBUG: [
    '[PROXY] Outbound connection established to PROXY-HK-01 (203.0.113.1:443)',
    '[DNS] Resolved api.telegram.org -> 149.154.167.50 (12ms)',
    '[ROUTE] Traffic matched rule: Proxy (Auto) via DomainSuffix(google.com)',
    '[TUN] Packet received from 192.168.1.100:54321 (len=1420)',
  ],
  INFO: [
    'Profile activated: Global Relay Sub (Subscription)',
    'Connection closed: chrome.exe -> gateway.icloud.com (duration: 3m 45s)',
    'Subscription update completed: 156 nodes loaded',
    '[TCP] 127.0.0.1:54321 --> api.telegram.org:443 match DomainKeyword(telegram) using Proxy[HK-01]',
    'Rule set cache refreshed (version: 2026062401)',
  ],
  WARN: [
    '[TCP] dial PROXY (match DomainSuffix/google.com) to mtalk.google.com:5228 error: timeout',
    '[DNS] Upstream server 8.8.8.8 slow response (1245ms), switching to 1.1.1.1',
    'Subscription expiry in 3 days: Global Relay Sub',
    '[QUOTA] Monthly traffic at 85% (255GB / 300GB)',
  ],
  ERROR: [
    'Update subscription failed: Get "https://sub.example.com/...": net/http: TLS handshake timeout',
    '[PROXY] Connection refused by remote server 203.0.113.5:443',
    '[DNS] Resolution failed for api.example.com: no such host',
    'Failed to parse rule configuration: unexpected token at line 42',
  ],
};

function getRandomLogMessage(level: LogLevel): string {
  const messages = LOG_MESSAGES[level];
  return messages[Math.floor(Math.random() * messages.length)]!;
}

function generateConnection(id: number): Connection {
  return {
    id: `conn-${id}`,
    process: PROCESSES[Math.floor(Math.random() * PROCESSES.length)]!,
    source: `127.0.0.1:${50000 + Math.floor(Math.random() * 10000)}`,
    destination: DESTINATIONS[Math.floor(Math.random() * DESTINATIONS.length)]!,
    rule: RULE_NAMES[Math.floor(Math.random() * RULE_NAMES.length)]!,
    network: NETWORKS[Math.floor(Math.random() * NETWORKS.length)]!,
    upload: Math.random() > 0.6 ? Math.floor(Math.random() * 15 * 1024) : 0,
    download: Math.floor(Math.random() * 800 * 1024),
    duration: Math.floor(Math.random() * 600),
    createdAt: Date.now(),
  };
}

export function useMockStream(enabled = true): void {
  const pushTrafficData = useDashboardStore((s) => s.pushTrafficData);
  const addLog = useLogStore((s) => s.addLog);
  const intervalRef = useRef<ReturnType<typeof setInterval>[]>([]);

  useEffect(() => {
    if (!enabled) {
      intervalRef.current.forEach(clearInterval);
      intervalRef.current = [];
      return;
    }

    // Traffic data every second
    const t1 = setInterval(() => {
      const up = Math.random() * 8 + 0.5;
      const down = Math.random() * 20 + 2;
      pushTrafficData(up, down);
      eventBus.emit('traffic:update', { upload: up, download: down, timestamp: Date.now() });
    }, 1000);

    // Log entries every 2-5 seconds
    const t2 = setInterval(
      () => {
        const levels: LogLevel[] = ['TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR'];
        const weights = [0.05, 0.15, 0.45, 0.25, 0.1];
        const totalWeight = weights.reduce((a, b) => a + b, 0);
        let random = Math.random() * totalWeight;
        let level: LogLevel = 'INFO';
        for (let i = 0; i < levels.length; i++) {
          random -= weights[i]!;
          if (random <= 0) {
            level = levels[i]!;
            break;
          }
        }

        const entry: LogEntry = {
          id: `log-stream-${++logCounter}`,
          timestamp: Date.now(),
          level,
          message: getRandomLogMessage(level),
        };
        addLog(entry);
        eventBus.emit('log:new', entry);
      },
      2000 + Math.random() * 3000,
    );

    // Connections every 8-15 seconds
    const t3 = setInterval(
      () => {
        const conn = generateConnection(++connCounter);
        useConnectionStore.getState().fetchConnections();
        eventBus.emit('connection:new', conn);
      },
      8000 + Math.random() * 7000,
    );

    intervalRef.current = [t1, t2, t3];

    return () => {
      intervalRef.current.forEach(clearInterval);
      intervalRef.current = [];
    };
  }, [enabled, pushTrafficData, addLog]);
}
