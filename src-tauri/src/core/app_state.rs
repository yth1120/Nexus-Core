use std::sync::Arc;

use arc_swap::ArcSwap;
use chrono::Utc;
use parking_lot::RwLock;
use tauri::AppHandle;

use crate::config::AppConfig;
use crate::core::resource_manager::ResourceManager;
use crate::models::{
    Connection, DashboardRunStatus, DashboardStatus, LogEntry, LogLevel, Node, NodeStatus, Profile,
    ProfileStatus, ProfileType, Rule, StatisticsData, TrafficDataPoint,
};

/// Central application state container.
///
/// - `ArcSwap<AppConfig>`: wait-free reads for hot-swappable config
/// - `parking_lot::RwLock`: fast, non-poisoning locks for mutable collections
#[derive(Clone)] // Clone is cheap — all fields are Arc/RwLock
pub struct AppState {
    pub config: Arc<ArcSwap<AppConfig>>,
    pub profiles: Arc<RwLock<Vec<Profile>>>,
    pub nodes: Arc<RwLock<Vec<Node>>>,
    pub connections: Arc<RwLock<Vec<Connection>>>,
    pub rules: Arc<RwLock<Vec<Rule>>>,
    pub logs: Arc<RwLock<Vec<LogEntry>>>,
    pub dashboard_status: Arc<RwLock<DashboardStatus>>,
    pub traffic_history: Arc<RwLock<Vec<TrafficDataPoint>>>,
    pub statistics_data: Arc<RwLock<StatisticsData>>,
    pub app_handle: Arc<RwLock<Option<AppHandle>>>,
    pub resource_manager: Arc<RwLock<Option<Arc<ResourceManager>>>>,
}

impl AppState {
    /// Create a new AppState with default config and seeded data.
    /// Create a new AppState with default config and seeded data.
    pub fn new() -> Self {
        Self {
            config: Arc::new(ArcSwap::from_pointee(AppConfig::default())),
            profiles: Arc::new(RwLock::new(seed_profiles())),
            nodes: Arc::new(RwLock::new(seed_nodes())),
            connections: Arc::new(RwLock::new(Vec::new())),
            rules: Arc::new(RwLock::new(seed_rules())),
            logs: Arc::new(RwLock::new(seed_logs())),
            dashboard_status: Arc::new(RwLock::new(seed_dashboard_status())),
            traffic_history: Arc::new(RwLock::new(seed_traffic_history())),
            statistics_data: Arc::new(RwLock::new(seed_statistics())),
            app_handle: Arc::new(RwLock::new(None)),
            resource_manager: Arc::new(RwLock::new(None)),
        }
    }

    /// Set the Tauri AppHandle for event emission.
    pub fn set_app_handle(&self, handle: AppHandle) {
        *self.app_handle.write() = Some(handle);
    }

    /// Set the ResourceManager reference (after initialization).
    pub fn set_resource_manager(&self, rm: Arc<ResourceManager>) {
        *self.resource_manager.write() = Some(rm);
    }

    /// Get a clone of the ResourceManager, if initialized.
    pub fn get_resource_manager(&self) -> Option<Arc<ResourceManager>> {
        self.resource_manager.read().clone()
    }

    /// Get a clone of the current config.
    pub fn get_config(&self) -> AppConfig {
        (**self.config.load()).clone()
    }

    /// Atomically swap the config.
    pub fn update_config(&self, config: AppConfig) {
        self.config.store(Arc::new(config));
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Seed Data (matching src/mock/seed.ts exactly) =====

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

fn minutes_ago_iso(minutes: i64) -> String {
    (Utc::now() - chrono::Duration::minutes(minutes)).to_rfc3339()
}

fn hours_ago_iso(hours: i64) -> String {
    (Utc::now() - chrono::Duration::hours(hours)).to_rfc3339()
}

fn days_ago_iso(days: i64) -> String {
    (Utc::now() - chrono::Duration::days(days)).to_rfc3339()
}

fn days_ago_ms(days: i64) -> i64 {
    (Utc::now() - chrono::Duration::days(days)).timestamp_millis()
}

pub fn seed_profiles() -> Vec<Profile> {
    vec![
        Profile {
            id: "profile-1".into(),
            name: "Global Relay Sub".into(),
            profile_type: ProfileType::Subscription,
            status: ProfileStatus::Active,
            latency: 32,
            updated: minutes_ago_iso(10),
            config_url: Some("https://sub.example.com/global.yaml".into()),
            traffic_used: Some(45_000_000_000),
            traffic_total: Some(100_000_000_000),
        },
        Profile {
            id: "profile-2".into(),
            name: "Company Intranet".into(),
            profile_type: ProfileType::WireGuard,
            status: ProfileStatus::Inactive,
            latency: 12,
            updated: hours_ago_iso(2),
            config_url: None,
            traffic_used: Some(12_500_000_000),
            traffic_total: None,
        },
        Profile {
            id: "profile-3".into(),
            name: "Self-hosted (Oracle JP)".into(),
            profile_type: ProfileType::Vless,
            status: ProfileStatus::Inactive,
            latency: 85,
            updated: days_ago_iso(5),
            config_url: Some("https://my-server.jp.example.com/config".into()),
            traffic_used: Some(200_000_000_000),
            traffic_total: None,
        },
        Profile {
            id: "profile-4".into(),
            name: "Backup Nodes (Free)".into(),
            profile_type: ProfileType::ClashMeta,
            status: ProfileStatus::Error,
            latency: 999,
            updated: days_ago_iso(7),
            config_url: Some("https://free-nodes.example.com/backup".into()),
            traffic_used: Some(5_000_000_000),
            traffic_total: None,
        },
    ]
}

pub fn seed_nodes() -> Vec<Node> {
    vec![
        Node {
            id: "node-1".into(),
            name: "HK-Azure-01".into(),
            country: "Hong Kong".into(),
            country_code: "HK".into(),
            delay: Some(12),
            loss: Some(0.1),
            status: NodeStatus::Online,
            is_favorite: true,
            is_connected: true,
            node_type: "V2Ray".into(),
            group: "Hong Kong".into(),
        },
        Node {
            id: "node-2".into(),
            name: "HK-GCP-02".into(),
            country: "Hong Kong".into(),
            country_code: "HK".into(),
            delay: Some(18),
            loss: Some(0.2),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "Trojan".into(),
            group: "Hong Kong".into(),
        },
        Node {
            id: "node-3".into(),
            name: "JP-Oracle-01".into(),
            country: "Japan".into(),
            country_code: "JP".into(),
            delay: Some(45),
            loss: Some(0.5),
            status: NodeStatus::Online,
            is_favorite: true,
            is_connected: false,
            node_type: "VLESS".into(),
            group: "Japan".into(),
        },
        Node {
            id: "node-4".into(),
            name: "JP-Sakura-02".into(),
            country: "Japan".into(),
            country_code: "JP".into(),
            delay: Some(52),
            loss: Some(1.2),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "VMess".into(),
            group: "Japan".into(),
        },
        Node {
            id: "node-5".into(),
            name: "SG-DigitalOcean".into(),
            country: "Singapore".into(),
            country_code: "SG".into(),
            delay: Some(38),
            loss: Some(0.0),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "V2Ray".into(),
            group: "Singapore".into(),
        },
        Node {
            id: "node-6".into(),
            name: "SG-AWS-01".into(),
            country: "Singapore".into(),
            country_code: "SG".into(),
            delay: Some(42),
            loss: Some(0.3),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "Trojan".into(),
            group: "Singapore".into(),
        },
        Node {
            id: "node-7".into(),
            name: "US-LAX-01".into(),
            country: "United States".into(),
            country_code: "US".into(),
            delay: Some(168),
            loss: Some(2.5),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "VLESS".into(),
            group: "United States".into(),
        },
        Node {
            id: "node-8".into(),
            name: "US-NYC-02".into(),
            country: "United States".into(),
            country_code: "US".into(),
            delay: Some(220),
            loss: Some(3.8),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "VMess".into(),
            group: "United States".into(),
        },
        Node {
            id: "node-9".into(),
            name: "KR-Seoul-01".into(),
            country: "South Korea".into(),
            country_code: "KR".into(),
            delay: Some(35),
            loss: Some(0.1),
            status: NodeStatus::Online,
            is_favorite: true,
            is_connected: false,
            node_type: "V2Ray".into(),
            group: "South Korea".into(),
        },
        Node {
            id: "node-10".into(),
            name: "TW-Taipei-01".into(),
            country: "Taiwan".into(),
            country_code: "TW".into(),
            delay: Some(28),
            loss: Some(0.8),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "Trojan".into(),
            group: "Taiwan".into(),
        },
        Node {
            id: "node-11".into(),
            name: "DE-Frankfurt-01".into(),
            country: "Germany".into(),
            country_code: "DE".into(),
            delay: Some(195),
            loss: Some(1.5),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "VLESS".into(),
            group: "Germany".into(),
        },
        Node {
            id: "node-12".into(),
            name: "UK-London-01".into(),
            country: "United Kingdom".into(),
            country_code: "UK".into(),
            delay: Some(180),
            loss: Some(1.8),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "VMess".into(),
            group: "United Kingdom".into(),
        },
        Node {
            id: "node-13".into(),
            name: "AU-Sydney-01".into(),
            country: "Australia".into(),
            country_code: "AU".into(),
            delay: Some(250),
            loss: Some(4.2),
            status: NodeStatus::Online,
            is_favorite: false,
            is_connected: false,
            node_type: "V2Ray".into(),
            group: "Australia".into(),
        },
        Node {
            id: "node-14".into(),
            name: "IN-Mumbai-01".into(),
            country: "India".into(),
            country_code: "IN".into(),
            delay: Some(120),
            loss: Some(1.0),
            status: NodeStatus::Offline,
            is_favorite: false,
            is_connected: false,
            node_type: "Trojan".into(),
            group: "India".into(),
        },
        Node {
            id: "node-15".into(),
            name: "BR-SaoPaulo-01".into(),
            country: "Brazil".into(),
            country_code: "BR".into(),
            delay: Some(310),
            loss: Some(5.5),
            status: NodeStatus::Offline,
            is_favorite: false,
            is_connected: false,
            node_type: "VLESS".into(),
            group: "Brazil".into(),
        },
        Node {
            id: "node-16".into(),
            name: "CN-Shanghai-Relay".into(),
            country: "China".into(),
            country_code: "CN".into(),
            delay: None,
            loss: None,
            status: NodeStatus::Untested,
            is_favorite: false,
            is_connected: false,
            node_type: "Shadowsocks".into(),
            group: "China".into(),
        },
    ]
}

pub fn seed_rules() -> Vec<Rule> {
    vec![
        Rule {
            id: "rule-1".into(),
            name: "Proxy: Social Media".into(),
            rule_type: "DomainSuffix".into(),
            payload: "twitter.com, facebook.com, instagram.com".into(),
            proxy: "Proxy (HK-01)".into(),
            enabled: true,
            tags: vec!["social".into(), "proxy".into()],
            created_at: days_ago_ms(30),
        },
        Rule {
            id: "rule-2".into(),
            name: "Direct: LAN Traffic".into(),
            rule_type: "IP-CIDR".into(),
            payload: "192.168.0.0/16, 10.0.0.0/8".into(),
            proxy: "DIRECT".into(),
            enabled: true,
            tags: vec!["lan".into(), "direct".into()],
            created_at: days_ago_ms(25),
        },
        Rule {
            id: "rule-3".into(),
            name: "Proxy: Google Services".into(),
            rule_type: "DomainSuffix".into(),
            payload: "google.com, googleapis.com, gstatic.com".into(),
            proxy: "Proxy (Auto)".into(),
            enabled: true,
            tags: vec!["google".into(), "proxy".into()],
            created_at: days_ago_ms(20),
        },
        Rule {
            id: "rule-4".into(),
            name: "Reject: Ad Networks".into(),
            rule_type: "DomainKeyword".into(),
            payload: "ad, ads, analytics, tracker".into(),
            proxy: "REJECT".into(),
            enabled: true,
            tags: vec!["ads".into(), "reject".into()],
            created_at: days_ago_ms(15),
        },
        Rule {
            id: "rule-5".into(),
            name: "Direct: China IP".into(),
            rule_type: "GEOIP".into(),
            payload: "CN".into(),
            proxy: "DIRECT".into(),
            enabled: true,
            tags: vec!["china".into(), "direct".into(), "geoip".into()],
            created_at: days_ago_ms(10),
        },
        Rule {
            id: "rule-6".into(),
            name: "Proxy: Telegram".into(),
            rule_type: "DomainKeyword".into(),
            payload: "telegram".into(),
            proxy: "Proxy (HK-01)".into(),
            enabled: true,
            tags: vec!["telegram".into(), "proxy".into()],
            created_at: days_ago_ms(8),
        },
        Rule {
            id: "rule-7".into(),
            name: "Proxy: GitHub".into(),
            rule_type: "DomainSuffix".into(),
            payload: "github.com, githubusercontent.com".into(),
            proxy: "Proxy (Auto)".into(),
            enabled: false,
            tags: vec!["github".into(), "proxy".into()],
            created_at: days_ago_ms(5),
        },
        Rule {
            id: "rule-8".into(),
            name: "Match: Final".into(),
            rule_type: "MATCH".into(),
            payload: "*".into(),
            proxy: "Proxy (Auto)".into(),
            enabled: true,
            tags: vec!["final".into(), "default".into()],
            created_at: days_ago_ms(1),
        },
    ]
}

pub fn seed_dashboard_status() -> DashboardStatus {
    DashboardStatus {
        status: DashboardRunStatus::Running,
        cpu_usage: 12.4,
        memory_usage: 145.0,
        uptime: 15780, // 4h 23m
        active_connections: 25,
        active_profile_name: "Global Relay Sub".into(),
        active_node_name: "HK-Azure-01".into(),
        ip_address: "203.0.113.45".into(),
        country: "Hong Kong".into(),
        port: 7890,
    }
}

pub fn seed_traffic_history() -> Vec<TrafficDataPoint> {
    generate_traffic_history(30)
}

pub fn seed_statistics() -> StatisticsData {
    let history = seed_traffic_history();
    let today_traffic: i64 = history
        .iter()
        .rev()
        .take(24)
        .map(|p| p.download + p.upload)
        .sum();
    let month_traffic: i64 = history.iter().map(|p| p.download + p.upload).sum();
    let max_speed = history
        .iter()
        .map(|p| p.upload.max(p.download))
        .max()
        .unwrap_or(0);

    let mut daily_averages: Vec<f64> = Vec::new();
    for chunk in history.chunks(24) {
        let total: i64 = chunk.iter().map(|p| p.download + p.upload).sum();
        daily_averages.push(total as f64 / 24.0);
    }

    StatisticsData {
        today_traffic,
        month_traffic,
        month_quota: 322_122_547_200, // 300 GB
        max_speed,
        max_speed_date: "2026-06-20".into(),
        history,
        daily_averages,
    }
}

pub fn seed_logs() -> Vec<LogEntry> {
    let mut logs = Vec::new();
    let log_messages: Vec<(LogLevel, &str)> = vec![
        (LogLevel::INFO, "Configuration loaded from config.yaml"),
        (
            LogLevel::INFO,
            "Mixed(http+socks) proxy listening at: 127.0.0.1:7890",
        ),
        (
            LogLevel::INFO,
            "Profile activated: Global Relay Sub (Subscription)",
        ),
        (
            LogLevel::DEBUG,
            "[PROXY] Outbound connection established to PROXY-HK-01 (203.0.113.1:443)",
        ),
        (
            LogLevel::DEBUG,
            "[DNS] Resolved api.telegram.org -> 149.154.167.50 (12ms)",
        ),
        (
            LogLevel::TRACE,
            "[TCP] send buffer: 4096 bytes to 127.0.0.1:7890",
        ),
        (
            LogLevel::WARN,
            "[DNS] Upstream server 8.8.8.8 slow response (1245ms), switching to 1.1.1.1",
        ),
        (
            LogLevel::WARN,
            "Subscription expiry in 3 days: Global Relay Sub",
        ),
    ];

    let base_time = now_ms();
    let total = log_messages.len();
    for (i, (level, msg)) in log_messages.into_iter().enumerate() {
        logs.push(LogEntry {
            id: format!("log-init-{}", i + 1),
            timestamp: base_time - (total - i) as i64 * 3000,
            level,
            message: msg.into(),
        });
    }

    logs
}

// ===== Traffic History Generator =====

fn generate_traffic_history(days: i64) -> Vec<TrafficDataPoint> {
    use rand::Rng;

    let now = Utc::now().timestamp_millis();
    let points_per_day: i64 = 24; // hourly
    let total = days * points_per_day;
    let mut rng = rand::thread_rng();
    let mut points: Vec<TrafficDataPoint> = Vec::with_capacity(total as usize);

    for i in 0..total {
        let timestamp = now - (total - i) * 3600 * 1000;
        // Simulate diurnal pattern
        let hour_of_day = ((timestamp / 1000 / 3600) % 24) as f64;
        let day_factor = ((hour_of_day - 6.0) * std::f64::consts::PI / 12.0).sin();
        let day_factor = (day_factor + 1.0) / 2.0; // 0-1, peak at 18:00

        let base_download = 5.0 + day_factor * 15.0 + rng.gen::<f64>() * 5.0; // 5-25 MB/s
        let base_upload = 1.0 + day_factor * 4.0 + rng.gen::<f64>() * 2.0; // 1-7 MB/s

        points.push(TrafficDataPoint {
            timestamp,
            upload: (base_upload * 1024.0 * 1024.0) as i64,
            download: (base_download * 1024.0 * 1024.0) as i64,
        });
    }

    points
}
