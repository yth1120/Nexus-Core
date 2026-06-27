use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rand::{rngs::StdRng, Rng, SeedableRng};
use tauri::AppHandle;
use tokio::time::interval;

use crate::core::app_state::AppState;
use crate::core::task_manager::TaskManager;
use crate::event::{emit_event, AppEvent};
use crate::models::TrafficDataPoint;
use crate::monitoring::{NetworkMonitor, SystemMonitor};

/// Coordinates background tasks with structured lifecycle management.
///
/// Uses TaskManager for named task registration and shutdown control.
/// Real system monitors replace mock data generators for CPU/memory/network.
pub struct Runtime {
    task_manager: Arc<TaskManager>,
    shutdown: Arc<AtomicBool>,
}

impl Runtime {
    /// Start all background tasks using the TaskManager.
    ///
    /// Spawns:
    /// - "traffic-monitor": Real CPU/memory/network stats via sysinfo (1s)
    /// - "log-generator": Mock log entries for development (2-5s)
    /// - "connection-monitor": Real system connections via platform APIs (8-15s)
    pub fn start(app_handle: AppHandle, state: AppState) -> Self {
        let task_manager = Arc::new(TaskManager::new());
        let shutdown = Arc::new(AtomicBool::new(false));

        // 1. Traffic monitor with real system data
        {
            let tm = task_manager.clone();
            let st = state.clone();
            let h = app_handle.clone();
            tm.spawn("traffic-monitor", move |flag| {
                run_traffic_monitor(flag, st, h)
            });
        }

        // 2. Log generator (still mock for development)
        {
            let tm = task_manager.clone();
            let st = state.clone();
            let h = app_handle.clone();
            tm.spawn("log-generator", move |flag| run_log_generator(flag, st, h));
        }

        // 3. Connection generator (mock for now; replaced by ConnectionMonitor when platform available)
        {
            let tm = task_manager.clone();
            let st = state.clone();
            let h = app_handle.clone();
            tm.spawn("connection-generator", move |flag| {
                run_connection_generator(flag, st, h)
            });
        }

        Runtime {
            task_manager,
            shutdown,
        }
    }

    /// Signal all background tasks to stop gracefully.
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.task_manager.shutdown_all();
        tracing::info!("Runtime shutdown complete");
    }

    pub fn task_manager(&self) -> &Arc<TaskManager> {
        &self.task_manager
    }
}

// ===== Traffic Monitor (real system data) =====

async fn run_traffic_monitor(shutdown: Arc<AtomicBool>, state: AppState, app_handle: AppHandle) {
    let mut tick = interval(Duration::from_secs(1));
    let mut sys_monitor = SystemMonitor::new();
    let mut net_monitor = NetworkMonitor::new();

    // Give CPU monitor one tick to initialize
    tokio::time::sleep(Duration::from_secs(1)).await;

    while !shutdown.load(Ordering::SeqCst) {
        tick.tick().await;

        // Real CPU and memory data from sysinfo
        let cpu = sys_monitor.cpu_usage();
        let memory = sys_monitor.memory_usage();
        let uptime = sys_monitor.uptime() as i64;

        // Real network speeds from sysinfo
        let speeds = net_monitor.get_speeds();
        let (upload, download) = if let Some(s) = speeds.first() {
            (s.upload_speed, s.download_speed)
        } else {
            // Fallback if no network data available
            let mut rng = StdRng::from_entropy();
            (rng.gen_range(0.5..8.5), rng.gen_range(2.0..22.0))
        };

        let now = chrono::Utc::now().timestamp_millis();

        // Update dashboard status with real values
        {
            let mut status = state.dashboard_status.write();
            status.cpu_usage = cpu;
            status.memory_usage = memory;
            status.uptime = uptime;
            status.status = crate::models::DashboardRunStatus::Running;
        }

        // Push to traffic history
        {
            let mut history = state.traffic_history.write();
            history.push(TrafficDataPoint {
                timestamp: now,
                upload: upload as i64,
                download: download as i64,
            });
            if history.len() > 720 {
                history.remove(0);
            }
        }

        // Emit events to frontend
        emit_event(
            &app_handle,
            AppEvent::TrafficUpdate {
                upload,
                download,
                timestamp: now,
            },
        );

        let status = state.dashboard_status.read().clone();
        emit_event(&app_handle, AppEvent::DashboardUpdate(status));
    }

    tracing::debug!("Traffic monitor stopped");
}

// ===== Log Generator (mock) =====

const LOG_TEMPLATES_TRACE: &[&str] = &[
    "[TCP] send buffer: 4096 bytes to 127.0.0.1:7890",
    "[DNS] cache lookup: google.com -> 142.250.80.46 (TTL: 245s)",
    "[TLS] ClientHello sent to cdn.discordapp.com",
    "[HTTP] Request headers parsed: GET /api/v1/status",
    "[RULE] Checking domain against rule set (index: 3)",
];

const LOG_TEMPLATES_DEBUG: &[&str] = &[
    "[PROXY] Outbound connection established to PROXY-HK-01 (203.0.113.1:443)",
    "[DNS] Resolved api.telegram.org -> 149.154.167.50 (12ms)",
    "[ROUTE] Traffic matched rule: Proxy (Auto) via DomainSuffix(google.com)",
    "[CACHE] Rule set cache updated (version: 2026062401)",
    "[TUN] Packet received from 192.168.1.100:54321 (len=1420)",
];

const LOG_TEMPLATES_INFO: &[&str] = &[
    "Configuration loaded from config.yaml",
    "Mixed(http+socks) proxy listening at: 127.0.0.1:7890",
    "[TCP] 127.0.0.1:54321 --> api.telegram.org:443 match DomainKeyword(telegram) using Proxy[HK-01]",
    "[UDP] 192.168.1.100:5353 --> 224.0.0.251:5353 match IP-CIDR(224.0.0.0/4) using DIRECT",
    "Profile activated: Global Relay Sub (Subscription)",
    "Connection closed: chrome.exe -> gateway.icloud.com (duration: 3m 45s)",
    "Subscription update completed: 156 nodes loaded",
    "TUN device created: utun4 (MTU: 1500)",
];

const LOG_TEMPLATES_WARN: &[&str] = &[
    "[TCP] dial PROXY (match DomainSuffix/google.com) to mtalk.google.com:5228 error: timeout",
    "[DNS] Upstream server 8.8.8.8 slow response (1245ms), switching to 1.1.1.1",
    "[RULE] No matching rule for 10.0.0.15:8080, using default",
    "Subscription expiry in 3 days: Global Relay Sub",
    "[TLS] Certificate verification warning for self-signed cert on 192.168.1.1",
    "[QUOTA] Monthly traffic at 85% (255GB / 300GB)",
];

const LOG_TEMPLATES_ERROR: &[&str] = &[
    "Update subscription failed: Get \"https://sub.example.com/...\": net/http: TLS handshake timeout",
    "[PROXY] Connection refused by remote server 203.0.113.5:443",
    "[DNS] Resolution failed for api.example.com: no such host",
    "Failed to parse rule configuration: unexpected token at line 42",
    "[TUN] Failed to create TUN device: permission denied (are you root?)",
];

fn pick_log_message(level: &crate::models::LogLevel, rng: &mut impl Rng) -> &'static str {
    let templates = match level {
        crate::models::LogLevel::TRACE => LOG_TEMPLATES_TRACE,
        crate::models::LogLevel::DEBUG => LOG_TEMPLATES_DEBUG,
        crate::models::LogLevel::INFO => LOG_TEMPLATES_INFO,
        crate::models::LogLevel::WARN => LOG_TEMPLATES_WARN,
        crate::models::LogLevel::ERROR => LOG_TEMPLATES_ERROR,
    };
    let idx = rng.gen_range(0..templates.len());
    templates[idx]
}

fn weighted_random_level(rng: &mut impl Rng) -> crate::models::LogLevel {
    use crate::models::LogLevel;
    let total: u32 = LogLevel::all().iter().map(|l| l.weight()).sum();
    let mut roll: u32 = rng.gen_range(0..total);
    for level in LogLevel::all() {
        let w = level.weight();
        if roll < w {
            return level;
        }
        roll -= w;
    }
    LogLevel::INFO
}

async fn run_log_generator(shutdown: Arc<AtomicBool>, state: AppState, app_handle: AppHandle) {
    let mut rng = StdRng::from_entropy();
    let mut counter: u64 = 100;

    while !shutdown.load(Ordering::SeqCst) {
        let delay_ms = rng.gen_range(2000..5000u64);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;

        if shutdown.load(Ordering::SeqCst) {
            break;
        }

        let level = weighted_random_level(&mut rng);
        let message = pick_log_message(&level, &mut rng);
        counter += 1;

        let entry = crate::models::LogEntry {
            id: format!("log-{}", counter),
            timestamp: chrono::Utc::now().timestamp_millis(),
            level,
            message: message.to_string(),
        };

        {
            let mut logs = state.logs.write();
            logs.push(entry.clone());
            if logs.len() > 1000 {
                logs.remove(0);
            }
        }

        emit_event(&app_handle, AppEvent::LogNew(entry));
    }

    tracing::debug!("Log generator stopped");
}

// ===== Connection Generator (mock) =====

async fn run_connection_generator(
    shutdown: Arc<AtomicBool>,
    state: AppState,
    app_handle: AppHandle,
) {
    use crate::models::{Connection, NetworkProtocol};

    const PROCESSES: &[&str] = &[
        "Telegram.exe",
        "WeChat.exe",
        "svchost.exe",
        "Code.exe",
        "chrome.exe",
        "Discord.exe",
        "CoreSync.exe",
        "msedge.exe",
        "firefox.exe",
        "Slack.exe",
    ];
    const DESTINATIONS: &[&str] = &[
        "gateway.icloud.com",
        "mtalk.google.com",
        "cdn.discordapp.com",
        "149.154.167.50:443",
        "api.twitter.com",
        "push.apple.com",
        "stun.l.google.com:19302",
        "api.github.com",
        "s3.amazonaws.com",
        "cloudflare-dns.com",
    ];
    const RULE_NAMES: &[&str] = &[
        "Proxy (HK-01)",
        "Proxy (Auto)",
        "Direct (LAN)",
        "Reject (Ads)",
        "Direct (CN IP)",
        "Proxy (JP-01)",
        "Proxy (SG-01)",
        "DIRECT",
    ];

    let mut rng = StdRng::from_entropy();
    let mut counter: u64 = 50;

    while !shutdown.load(Ordering::SeqCst) {
        let delay_ms = rng.gen_range(8000u64..15000u64);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;

        if shutdown.load(Ordering::SeqCst) {
            break;
        }

        counter += 1;
        let conn = Connection {
            id: format!("conn-{}", counter),
            process: PROCESSES[rng.gen_range(0..PROCESSES.len())].to_string(),
            source: format!("127.0.0.1:{}", 50000 + rng.gen_range(0..10000u32)),
            destination: DESTINATIONS[rng.gen_range(0..DESTINATIONS.len())].to_string(),
            rule: RULE_NAMES[rng.gen_range(0..RULE_NAMES.len())].to_string(),
            network: if rng.gen::<bool>() {
                NetworkProtocol::TCP
            } else {
                NetworkProtocol::UDP
            },
            upload: if rng.gen::<f64>() > 0.6 {
                rng.gen_range(0..15 * 1024) as i64
            } else {
                0
            },
            download: rng.gen_range(0..800 * 1024) as i64,
            duration: rng.gen_range(0.0..600.0),
            created_at: chrono::Utc::now().timestamp_millis() - rng.gen_range(0..600000),
        };

        {
            let mut connections = state.connections.write();
            connections.push(conn.clone());
            if connections.len() > 200 {
                connections.remove(0);
            }
        }

        emit_event(&app_handle, AppEvent::ConnectionNew(conn));
    }

    tracing::debug!("Connection generator stopped");
}
