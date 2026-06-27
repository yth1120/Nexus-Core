import type {
  Profile,
  Node,
  Connection,
  LogEntry,
  LogLevel,
  Rule,
  DashboardStatus,
  StatisticsData,
  TrafficDataPoint,
  NetworkProtocol,
} from '@/types';

// ===== Profiles =====

export const SEED_PROFILES: Profile[] = [
  {
    id: 'profile-1',
    name: 'Global Relay Sub',
    latency: 32,
    updated: new Date(Date.now() - 10 * 60 * 1000).toISOString(),
    status: 'active',
    type: 'Subscription',
    configUrl: 'https://sub.example.com/global.yaml',
    trafficUsed: 45_000_000_000,
    trafficTotal: 100_000_000_000,
  },
  {
    id: 'profile-2',
    name: 'Company Intranet',
    latency: 12,
    updated: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
    status: 'inactive',
    type: 'WireGuard',
    trafficUsed: 12_500_000_000,
  },
  {
    id: 'profile-3',
    name: 'Self-hosted (Oracle JP)',
    latency: 85,
    updated: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
    status: 'inactive',
    type: 'VLESS',
    configUrl: 'https://my-server.jp.example.com/config',
    trafficUsed: 200_000_000_000,
  },
  {
    id: 'profile-4',
    name: 'Backup Nodes (Free)',
    latency: 999,
    updated: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString(),
    status: 'error',
    type: 'Clash Meta',
    configUrl: 'https://free-nodes.example.com/backup',
    trafficUsed: 5_000_000_000,
  },
];

// ===== Nodes =====

export const SEED_NODES: Node[] = [
  {
    id: 'node-1',
    name: 'HK-Azure-01',
    country: 'Hong Kong',
    countryCode: 'HK',
    delay: 12,
    loss: 0.1,
    status: 'online',
    isFavorite: true,
    isConnected: true,
    type: 'V2Ray',
    group: 'Hong Kong',
  },
  {
    id: 'node-2',
    name: 'HK-GCP-02',
    country: 'Hong Kong',
    countryCode: 'HK',
    delay: 18,
    loss: 0.2,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'Trojan',
    group: 'Hong Kong',
  },
  {
    id: 'node-3',
    name: 'JP-Oracle-01',
    country: 'Japan',
    countryCode: 'JP',
    delay: 45,
    loss: 0.5,
    status: 'online',
    isFavorite: true,
    isConnected: false,
    type: 'VLESS',
    group: 'Japan',
  },
  {
    id: 'node-4',
    name: 'JP-Sakura-02',
    country: 'Japan',
    countryCode: 'JP',
    delay: 52,
    loss: 1.2,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'VMess',
    group: 'Japan',
  },
  {
    id: 'node-5',
    name: 'SG-DigitalOcean',
    country: 'Singapore',
    countryCode: 'SG',
    delay: 38,
    loss: 0.0,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'V2Ray',
    group: 'Singapore',
  },
  {
    id: 'node-6',
    name: 'SG-AWS-01',
    country: 'Singapore',
    countryCode: 'SG',
    delay: 42,
    loss: 0.3,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'Trojan',
    group: 'Singapore',
  },
  {
    id: 'node-7',
    name: 'US-LAX-01',
    country: 'United States',
    countryCode: 'US',
    delay: 168,
    loss: 2.5,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'VLESS',
    group: 'United States',
  },
  {
    id: 'node-8',
    name: 'US-NYC-02',
    country: 'United States',
    countryCode: 'US',
    delay: 220,
    loss: 3.8,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'VMess',
    group: 'United States',
  },
  {
    id: 'node-9',
    name: 'KR-Seoul-01',
    country: 'South Korea',
    countryCode: 'KR',
    delay: 35,
    loss: 0.1,
    status: 'online',
    isFavorite: true,
    isConnected: false,
    type: 'V2Ray',
    group: 'South Korea',
  },
  {
    id: 'node-10',
    name: 'TW-Taipei-01',
    country: 'Taiwan',
    countryCode: 'TW',
    delay: 28,
    loss: 0.8,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'Trojan',
    group: 'Taiwan',
  },
  {
    id: 'node-11',
    name: 'DE-Frankfurt-01',
    country: 'Germany',
    countryCode: 'DE',
    delay: 195,
    loss: 1.5,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'VLESS',
    group: 'Germany',
  },
  {
    id: 'node-12',
    name: 'UK-London-01',
    country: 'United Kingdom',
    countryCode: 'UK',
    delay: 180,
    loss: 1.8,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'VMess',
    group: 'United Kingdom',
  },
  {
    id: 'node-13',
    name: 'AU-Sydney-01',
    country: 'Australia',
    countryCode: 'AU',
    delay: 250,
    loss: 4.2,
    status: 'online',
    isFavorite: false,
    isConnected: false,
    type: 'V2Ray',
    group: 'Australia',
  },
  {
    id: 'node-14',
    name: 'IN-Mumbai-01',
    country: 'India',
    countryCode: 'IN',
    delay: 120,
    loss: 1.0,
    status: 'offline',
    isFavorite: false,
    isConnected: false,
    type: 'Trojan',
    group: 'India',
  },
  {
    id: 'node-15',
    name: 'BR-SaoPaulo-01',
    country: 'Brazil',
    countryCode: 'BR',
    delay: 310,
    loss: 5.5,
    status: 'offline',
    isFavorite: false,
    isConnected: false,
    type: 'VLESS',
    group: 'Brazil',
  },
  {
    id: 'node-16',
    name: 'CN-Shanghai-Relay',
    country: 'China',
    countryCode: 'CN',
    delay: 15,
    loss: 0.0,
    status: 'untested',
    isFavorite: false,
    isConnected: false,
    type: 'Shadowsocks',
    group: 'China',
  },
];

// ===== Rules =====

export const SEED_RULES: Rule[] = [
  {
    id: 'rule-1',
    name: 'Proxy: Social Media',
    type: 'DomainSuffix',
    payload: 'twitter.com, facebook.com, instagram.com',
    proxy: 'Proxy (HK-01)',
    enabled: true,
    tags: ['social', 'proxy'],
    createdAt: Date.now() - 86400000 * 30,
  },
  {
    id: 'rule-2',
    name: 'Direct: LAN Traffic',
    type: 'IP-CIDR',
    payload: '192.168.0.0/16, 10.0.0.0/8',
    proxy: 'DIRECT',
    enabled: true,
    tags: ['lan', 'direct'],
    createdAt: Date.now() - 86400000 * 25,
  },
  {
    id: 'rule-3',
    name: 'Proxy: Google Services',
    type: 'DomainSuffix',
    payload: 'google.com, googleapis.com, gstatic.com',
    proxy: 'Proxy (Auto)',
    enabled: true,
    tags: ['google', 'proxy'],
    createdAt: Date.now() - 86400000 * 20,
  },
  {
    id: 'rule-4',
    name: 'Reject: Ad Networks',
    type: 'DomainKeyword',
    payload: 'ad, ads, analytics, tracker',
    proxy: 'REJECT',
    enabled: true,
    tags: ['ads', 'reject'],
    createdAt: Date.now() - 86400000 * 15,
  },
  {
    id: 'rule-5',
    name: 'Direct: China IP',
    type: 'GEOIP',
    payload: 'CN',
    proxy: 'DIRECT',
    enabled: true,
    tags: ['china', 'direct', 'geoip'],
    createdAt: Date.now() - 86400000 * 10,
  },
  {
    id: 'rule-6',
    name: 'Proxy: Telegram',
    type: 'DomainKeyword',
    payload: 'telegram',
    proxy: 'Proxy (HK-01)',
    enabled: true,
    tags: ['telegram', 'proxy'],
    createdAt: Date.now() - 86400000 * 8,
  },
  {
    id: 'rule-7',
    name: 'Proxy: GitHub',
    type: 'DomainSuffix',
    payload: 'github.com, githubusercontent.com',
    proxy: 'Proxy (Auto)',
    enabled: false,
    tags: ['github', 'proxy'],
    createdAt: Date.now() - 86400000 * 5,
  },
  {
    id: 'rule-8',
    name: 'Match: Final',
    type: 'MATCH',
    payload: '*',
    proxy: 'Proxy (Auto)',
    enabled: true,
    tags: ['final', 'default'],
    createdAt: Date.now() - 86400000,
  },
];

// ===== Dashboard Status =====

export const SEED_DASHBOARD_STATUS: DashboardStatus = {
  status: 'running',
  cpuUsage: 12.4,
  memoryUsage: 145,
  uptime: 15780, // 4h 23m
  activeConnections: 25,
  activeProfileName: 'Global Relay Sub',
  activeNodeName: 'HK-Azure-01',
  ipAddress: '203.0.113.45',
  country: 'Hong Kong',
  port: 7890,
};

// ===== Statistics =====

export function generateTrafficHistory(days: number): TrafficDataPoint[] {
  const now = Date.now();
  const pointsPerDay = 24; // hourly points
  const total = days * pointsPerDay;
  const points: TrafficDataPoint[] = [];

  for (let i = 0; i < total; i++) {
    const timestamp = now - (total - i) * 3600 * 1000;
    // Simulate diurnal pattern: higher during day, lower at night
    const hourOfDay = new Date(timestamp).getHours();
    const dayFactor = (Math.sin(((hourOfDay - 6) * Math.PI) / 12) + 1) / 2; // 0-1, peak at noon
    const baseDownload = 5 + dayFactor * 15 + Math.random() * 5; // 5-25 MB/s
    const baseUpload = 1 + dayFactor * 4 + Math.random() * 2; // 1-7 MB/s

    points.push({
      timestamp,
      upload: baseUpload * 1024 * 1024, // bytes/sec
      download: baseDownload * 1024 * 1024,
    });
  }

  return points;
}

export function generateSeedStatistics(): StatisticsData {
  const history = generateTrafficHistory(30);

  const todayTraffic = history.slice(-24).reduce((sum, p) => sum + p.download + p.upload, 0);
  const monthTraffic = history.reduce((sum, p) => sum + p.download + p.upload, 0);
  const maxSpeed = Math.max(...history.map((p) => Math.max(p.upload, p.download)));
  const dailyAverages = history.reduce<number[]>((acc, p, i) => {
    const day = Math.floor(i / 24);
    if (!acc[day]) acc[day] = 0;
    acc[day]! += p.download + p.upload;
    return acc;
  }, []);

  return {
    todayTraffic,
    monthTraffic,
    monthQuota: 322_122_547_200, // 300 GB
    maxSpeed,
    maxSpeedDate: '2026-06-20',
    history,
    dailyAverages: dailyAverages.map((v) => v / 24),
  };
}

// ===== Connection Generator =====

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

export function generateConnections(count: number): Connection[] {
  return Array.from({ length: count }, (_, i) => ({
    id: `conn-${i}`,
    process: PROCESSES[Math.floor(Math.random() * PROCESSES.length)]!,
    source: `127.0.0.1:${50000 + Math.floor(Math.random() * 10000)}`,
    destination: DESTINATIONS[Math.floor(Math.random() * DESTINATIONS.length)]!,
    rule: RULE_NAMES[Math.floor(Math.random() * RULE_NAMES.length)]!,
    network: NETWORKS[Math.floor(Math.random() * NETWORKS.length)]!,
    upload: Math.random() > 0.6 ? Math.floor(Math.random() * 15 * 1024) : 0,
    download: Math.floor(Math.random() * 800 * 1024),
    duration: Math.floor(Math.random() * 600),
    createdAt: Date.now() - Math.floor(Math.random() * 600000),
  }));
}

// ===== Log Generator =====

const LOG_TEMPLATES: Record<LogLevel, string[]> = {
  TRACE: [
    '[TCP] send buffer: 4096 bytes to 127.0.0.1:7890',
    '[DNS] cache lookup: google.com -> 142.250.80.46 (TTL: 245s)',
    '[TLS] ClientHello sent to cdn.discordapp.com',
    '[HTTP] Request headers parsed: GET /api/v1/status',
    '[RULE] Checking domain against rule set (index: 3)',
  ],
  DEBUG: [
    '[PROXY] Outbound connection established to PROXY-HK-01 (203.0.113.1:443)',
    '[DNS] Resolved api.telegram.org -> 149.154.167.50 (12ms)',
    '[ROUTE] Traffic matched rule: Proxy (Auto) via DomainSuffix(google.com)',
    '[CACHE] Rule set cache updated (version: 2026062401)',
    '[TUN] Packet received from 192.168.1.100:54321 (len=1420)',
  ],
  INFO: [
    'Configuration loaded from config.yaml',
    'Mixed(http+socks) proxy listening at: 127.0.0.1:7890',
    '[TCP] 127.0.0.1:54321 --> api.telegram.org:443 match DomainKeyword(telegram) using Proxy[HK-01]',
    '[UDP] 192.168.1.100:5353 --> 224.0.0.251:5353 match IP-CIDR(224.0.0.0/4) using DIRECT',
    'Profile activated: Global Relay Sub (Subscription)',
    'Connection closed: chrome.exe -> gateway.icloud.com (duration: 3m 45s)',
    'Subscription update completed: 156 nodes loaded',
    'TUN device created: utun4 (MTU: 1500)',
  ],
  WARN: [
    '[TCP] dial PROXY (match DomainSuffix/google.com) to mtalk.google.com:5228 error: timeout',
    '[DNS] Upstream server 8.8.8.8 slow response (1245ms), switching to 1.1.1.1',
    '[RULE] No matching rule for 10.0.0.15:8080, using default',
    'Subscription expiry in 3 days: Global Relay Sub',
    '[TLS] Certificate verification warning for self-signed cert on 192.168.1.1',
    '[QUOTA] Monthly traffic at 85% (255GB / 300GB)',
  ],
  ERROR: [
    'Update subscription failed: Get "https://sub.example.com/...": net/http: TLS handshake timeout',
    '[PROXY] Connection refused by remote server 203.0.113.5:443',
    '[DNS] Resolution failed for api.example.com: no such host',
    'Failed to parse rule configuration: unexpected token at line 42',
    '[TUN] Failed to create TUN device: permission denied (are you root?)',
  ],
};

export function generateLogEntry(id: number, level?: LogLevel): LogEntry {
  const levels: LogLevel[] = ['TRACE', 'DEBUG', 'INFO', 'WARN', 'ERROR'];
  const weights = [0.05, 0.15, 0.45, 0.25, 0.1]; // more INFO than others

  const chosenLevel = level ?? weightedRandom(levels, weights);
  const messages = LOG_TEMPLATES[chosenLevel];
  const message = messages[Math.floor(Math.random() * messages.length)]!;

  return {
    id: `log-${id}`,
    timestamp: Date.now(),
    level: chosenLevel,
    message,
  };
}

export function generateLogs(count: number): LogEntry[] {
  const logs: LogEntry[] = [];
  let timeOffset = 0;
  for (let i = 0; i < count; i++) {
    timeOffset += Math.floor(Math.random() * 5000) + 500; // 0.5-5.5s apart
    const entry = generateLogEntry(i + 1);
    entry.timestamp = Date.now() - timeOffset;
    logs.push(entry);
  }
  return logs.reverse();
}

function weightedRandom<T>(items: T[], weights: number[]): T {
  const totalWeight = weights.reduce((a, b) => a + b, 0);
  let random = Math.random() * totalWeight;
  for (let i = 0; i < items.length; i++) {
    random -= weights[i]!;
    if (random <= 0) return items[i]!;
  }
  return items[items.length - 1]!;
}
